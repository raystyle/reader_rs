//! 图片本体导出与文本元数据对齐（D47；S010 定界：只负责提取存储，理解归调用方 Agent）。
//! 四路分派：PDF 按页渲染 PNG（扫描书页即图本体，图题与上下文从页文本层对齐）；
//! markdown 解析 `![alt](path)` 引用复制；anydoc 家族 zip 直读内嵌图片部件（原字节）；
//! 图片文件本体即自身。输出行式清单与 json 包膜，退出码 0 有图 / 1 无图 / 2 出错。

use crate::document::is_image_ext;
use std::collections::HashSet;
use std::io::Read as _;
use std::path::{Path, PathBuf};

/// 一件导出的图本体：kind 加锚定位回文档，caption 加 context 是与文本元数据的对齐面。
pub struct FigureOut {
    /// 导出路径:page（PDF 页渲染）/ md-ref（markdown 引用）/ zip-asset（家族内嵌件）/ file（图片文件自身）
    pub kind: &'static str,
    /// 文档内锚:page N / 引用原样 / zip 部件路径 / 文件名
    pub anchor: String,
    /// 图题候选(页文本或引用 alt 的「图N-M」形态行;扫描页无文本层时为 None)
    pub caption: Option<String>,
    /// 图题后的上下文文本行(对齐面;无则空)
    pub context: Vec<String>,
    /// 落盘路径
    pub file: PathBuf,
    pub bytes: u64,
    /// 图片格式(png / jpg 等,按落盘扩展名)
    pub format: String,
}

/// 图题形态:行首「图N-M」(允许页码/空白前缀如「12 图1-1」;编号分隔兼容 - — – . ．,
/// 编号前后允许空格)。正文行内引用(如「如图1-2所示」)不算图题行。
fn is_caption_line(line: &str) -> bool {
    let t = line.trim();
    let core = t.trim_start_matches(|c: char| c.is_ascii_digit() || c.is_whitespace());
    let Some(tag) = core.strip_prefix('图').or_else(|| core.strip_prefix('表')) else {
        return false;
    };
    let mut chars = tag.chars().peekable();
    while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
        chars.next();
    }
    let mut saw_digit = false;
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            saw_digit = true;
            chars.next();
        } else {
            break;
        }
    }
    if !saw_digit {
        return false;
    }
    while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
        chars.next();
    }
    matches!(
        chars.peek(),
        Some('-') | Some('—') | Some('–') | Some('.') | Some('．')
    )
}

/// 提取图本体到 `out_dir`,按文档格式分派;`filter` 为 1 起页集合(仅 PDF 生效)。
pub fn extract_figures(
    path: &Path,
    filter: Option<&HashSet<u32>>,
    out_dir: &Path,
) -> Result<Vec<FigureOut>, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .unwrap_or_default();
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("doc")
        .to_string();
    std::fs::create_dir_all(out_dir)
        .map_err(|e| format!("建输出目录失败 {}: {e}", out_dir.display()))?;
    match ext.as_str() {
        "pdf" => figures_from_pdf(path, filter, out_dir, &stem),
        "md" | "markdown" => figures_from_markdown(path, out_dir, &stem),
        _ if is_image_ext(&ext) => {
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("image")
                .to_string();
            Ok(vec![copy_as(
                &std::fs::read(path)
                    .map_err(|e| format!("无法读取图片 {}: {e}", path.display()))?,
                out_dir,
                name.clone(),
                "file",
                &name,
                None,
                vec![],
            )?])
        }
        _ if ::anydoc::Format::from_extension(&ext).is_some() => {
            figures_from_zip(path, out_dir, &stem, &ext)
        }
        _ => Err(format!(
            "不支持的格式 {ext}（{}）;figures 支持 .pdf、markdown、图片与 anydoc 家族",
            path.display()
        )),
    }
}

/// PDF:逐页渲染 PNG(页即图本体,扫描书形态);图题与上下文从 pdf-inspector 页文本对齐
/// (文本层才有;扫描页 caption 为 None,交给调用方配合 --ocr 文本)。
fn figures_from_pdf(
    path: &Path,
    filter: Option<&HashSet<u32>>,
    out_dir: &Path,
    stem: &str,
) -> Result<Vec<FigureOut>, String> {
    let zero_based: Option<Vec<u32>> = filter.map(|set| {
        let mut list: Vec<u32> = set.iter().map(|n| n.saturating_sub(1)).collect();
        list.sort_unstable();
        list.dedup();
        list
    });
    let result = pdf_inspector::extract_pages_markdown(path, zero_based.as_deref())
        .map_err(|e| format!("无法读取 PDF {}: {e}", path.display()))?;
    let file = std::fs::read(path).map_err(|e| format!("无法读取 PDF {}: {e}", path.display()))?;
    let pdf = hayro::hayro_syntax::Pdf::new(file)
        .map_err(|e| format!("无法解析 PDF {}: {e:?}", path.display()))?;
    let pages = pdf.pages();
    let mut out = Vec::new();
    for page in result.pages {
        let no = page.page + 1;
        let lines: Vec<&str> = page.markdown.lines().collect();
        let (caption, context) = match lines.iter().position(|l| is_caption_line(l)) {
            Some(i) => (
                Some(lines[i].trim().to_string()),
                lines
                    .iter()
                    .skip(i + 1)
                    .take(2)
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty())
                    .collect(),
            ),
            None => (None, vec![]),
        };
        let png = pages
            .get(page.page as usize)
            .map(|p| crate::ocr::page_png(p, no))
            .transpose()?
            .ok_or_else(|| format!("页 {no} 超范围"))?;
        out.push(copy_as(
            &png,
            out_dir,
            format!("{stem}-p{no}.png"),
            "page",
            &format!("page {no}"),
            caption,
            context,
        )?);
    }
    Ok(out)
}

/// markdown:解析 `![alt](path)` 引用;相对路径按 md 所在目录解析,存在即复制。
fn figures_from_markdown(
    path: &Path,
    out_dir: &Path,
    _stem: &str,
) -> Result<Vec<FigureOut>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("无法读取 markdown {}: {e}", path.display()))?;
    let re = regex::Regex::new(r#"!\[([^\]]*)\]\(([^)\s]+)(?:\s+"[^"]*")?\)"#)
        .map_err(|e| format!("图片引用正则异常: {e}"))?;
    let base = path.parent().unwrap_or(Path::new("."));
    let mut out = Vec::new();
    for caps in re.captures_iter(&text) {
        let (alt, target) = (
            caps.get(1).map(|m| m.as_str()).unwrap_or(""),
            caps.get(2).map(|m| m.as_str()).unwrap_or(""),
        );
        if target.starts_with("http://") || target.starts_with("https://") {
            continue; // 远程引用不抓取,单调用零网络
        }
        let src = base.join(target);
        let Ok(bytes) = std::fs::read(&src) else {
            continue; // 引用悬空跳过,不算错误
        };
        let name = src
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("md-image")
            .to_string();
        let caption = (!alt.trim().is_empty()).then(|| alt.trim().to_string());
        out.push(copy_as(
            &bytes,
            out_dir,
            name.clone(),
            "md-ref",
            target,
            caption,
            vec![],
        )?);
    }
    Ok(out)
}

/// anydoc 家族:zip 直读图片扩展名部件(media 原字节;anydoc 公开 API 丢弃 assets,S010 实证)。
fn figures_from_zip(
    path: &Path,
    out_dir: &Path,
    stem: &str,
    ext: &str,
) -> Result<Vec<FigureOut>, String> {
    if ext == "doc" || ext == "ppt" || ext == "pps" || ext == "pot" || ext == "xls" || ext == "xlsb"
    {
        // 二进制容器族(zip 直读不适用;OLE/BIFF):v1 无图可导,如实返回空
        return Ok(vec![]);
    }
    let file =
        std::fs::File::open(path).map_err(|e| format!("无法读取文档 {}: {e}", path.display()))?;
    let mut zip = zip::ZipArchive::new(file)
        .map_err(|e| format!("无法读取 zip 容器 {}: {e}", path.display()))?;
    let mut out = Vec::new();
    for i in 0..zip.len() {
        let mut entry = zip
            .by_index(i)
            .map_err(|e| format!("zip 件 {i} 读取失败: {e}"))?;
        let Some(name) = entry.enclosed_name() else {
            continue;
        };
        // 锚统一正斜杠(zip 部件路径的规范形态;Windows 下 to_string_lossy 出反斜杠会跨机漂)
        let part = name.to_string_lossy().replace('\\', "/");
        let Some(e) = part.rsplit('.').next() else {
            continue;
        };
        let e = e.to_lowercase();
        if !is_image_ext(&e) {
            continue;
        }
        let mut bytes = Vec::with_capacity(entry.size() as usize);
        entry
            .read_to_end(&mut bytes)
            .map_err(|err| format!("zip 件 {part} 读字节失败: {err}"))?;
        let flat = part.replace(['/', '\\', ':'], "__");
        out.push(copy_as(
            &bytes,
            out_dir,
            format!("{stem}__{flat}"),
            "zip-asset",
            &part,
            None,
            vec![],
        )?);
    }
    Ok(out)
}

fn copy_as(
    bytes: &[u8],
    out_dir: &Path,
    name: String,
    kind: &'static str,
    anchor: &str,
    caption: Option<String>,
    context: Vec<String>,
) -> Result<FigureOut, String> {
    let file = out_dir.join(sanitize(&name));
    std::fs::write(&file, bytes).map_err(|e| format!("写图失败 {}: {e}", file.display()))?;
    Ok(FigureOut {
        kind,
        anchor: anchor.to_string(),
        caption,
        context,
        bytes: bytes.len() as u64,
        format: file
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase(),
        file,
    })
}

/// 落盘名清洗:只换路径分隔与 Windows 非法字符(保留 CJK;全 ASCII 化会把中文书名
/// 变全下划线且互相撞名,D47 首验踩到)。
fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 图题判定:标准形态、全角横线、行首页码前缀、非图题行(纯编号/正文)。
    #[test]
    fn caption_line_matches_figures_and_tables() {
        assert!(is_caption_line("图1-1全国安全生产事故的事故起数和死亡人数"));
        assert!(is_caption_line("图 2-3 载荷谱"));
        assert!(is_caption_line("表3.2 相似元集合"));
        assert!(is_caption_line("12 图1-1 有页码前缀"));
        assert!(is_caption_line("图1—1 全角横线"));
        assert!(!is_caption_line("如图1-2所示的趋势继续"));
        assert!(!is_caption_line("1.1.1 安全系统分析方法的需求"));
        assert!(!is_caption_line("图11(无编号分隔)"));
    }

    /// 落盘名清洗:路径分隔与非法字符换下划线,CJK 保留(防中文书名全下划线撞名)。
    #[test]
    fn sanitize_keeps_cjk_and_drops_separators() {
        assert_eq!(sanitize("安全相似系统学-p13.png"), "安全相似系统学-p13.png");
        assert_eq!(sanitize("word/media/image1.png"), "word_media_image1.png");
        assert_eq!(sanitize("a:b*c?d"), "a_b_c_d");
    }
}
