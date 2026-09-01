//! 格式分派与统一文本单元：PDF 的页、其余格式的标题节与无标题分片，对上同为 `TextUnit`。

use std::collections::HashSet;
use std::path::Path;

/// 一个文本单元。`no` 为 1 起序号（PDF 页码 / 其余格式节序），`lines` 按阅读序排列；
/// `needs_ocr` 为 `Some(原因)` 表示该单元文本层不可靠（扫描件、编码问题、乱码、空提取）。
pub struct TextUnit {
    pub no: u32,
    pub kind: UnitKind,
    pub lines: Vec<String>,
    pub needs_ocr: Option<String>,
}

/// 文本单元种类，决定输出分节标记（`== page N ==` / `== section N ==` / `== part N ==`）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnitKind {
    Page,
    Section,
    /// 无标题文档与超长节的固定行分片（P0010、P0011）。
    Part,
}

impl UnitKind {
    pub fn label(self) -> &'static str {
        match self {
            UnitKind::Page => "page",
            UnitKind::Section => "section",
            UnitKind::Part => "part",
        }
    }
}

/// 按扩展名分派提取；`filter` 为 1 起序号集合（`None` 为全部）。
/// PDF 直连 pdf-inspector 保页契约；anydoc 家族（Word / EPUB / ODT / RTF / Office / CSV）
/// 走统一引擎按标题分节（P0009）。
pub fn extract(path: &Path, filter: Option<&HashSet<u32>>) -> Result<Vec<TextUnit>, String> {
    let ext = ext_of(path);
    if ext == "pdf" {
        return crate::pdf::extract_pages(path, filter);
    }
    if ::anydoc::Format::from_extension(&ext).is_some() {
        return crate::anydoc::extract_sections(path, filter);
    }
    Err(format!(
        "不支持的格式 {}（{}）；当前支持 .pdf 与 anydoc 家族（.doc / .docx / .epub / .odt / .rtf / .ppt(x) / .xls(x) / .ods / .odp / .csv）",
        if ext.is_empty() {
            "<无扩展名>"
        } else {
            &ext
        },
        path.display()
    ))
}

/// 扩展名是否命中支持面（分派与批量目录遍历共用同一真源；P0012）。
pub fn is_supported(path: &Path) -> bool {
    let ext = ext_of(path);
    ext == "pdf" || ::anydoc::Format::from_extension(&ext).is_some()
}

fn ext_of(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .unwrap_or_default()
}
