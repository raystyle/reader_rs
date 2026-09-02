//! Reader：Agent 原生文档阅读、搜索和提取工具（PDF 按页；markdown 与 Word / EPUB / ODT / RTF / Office / CSV 按标题节）。
//! 薄壳在 `src\main.rs`；本文件承载 CLI 定义、`run()` 分发与页/节范围解析。

pub mod anydoc;
pub mod batch;
pub mod document;
pub mod introspect;
pub mod ocr;
pub mod output;
pub mod pdf;
pub mod query;
pub mod search;
pub mod selfupdate;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use document::OcrOpts;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// 输出形态：text 行式（缺省）或 json 包膜（P0006）。
#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum Format {
    Text,
    Json,
}

/// 输出选项：形态与 filter 裁剪路径（两子命令共用）。
struct OutputOpts {
    format: Format,
    filter: Option<String>,
}

/// 匹配选项：正则开关、大小写、上下文行数（search 专用）。
struct SearchOpts {
    regex_mode: bool,
    ignore_case: bool,
    context: usize,
}

#[derive(Parser)]
#[command(
    name = "reader",
    version,
    about = "Agent 原生文档阅读、搜索和提取工具（PDF 按页；markdown 与 Word / EPUB / ODT / RTF / Office / CSV 按节）"
)]
struct Cli {
    /// 输出紧凑命令索引（agent 发现用；skill 子命令给长形态 SKILL.md）
    #[arg(long)]
    llms: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 按页/节搜索文档文本（命中退出 0，无命中退出 1，出错退出 2）；目录输入递归批量搜
    #[command(after_long_help = "\
示例:
  reader search ./doc.pdf \"error\" -i -C 1
  reader search ./doc.pdf \"err(or|code)\" --regex --pages 2-10
  reader search ./docs \"配置\" --format json --filter 'hits[].file'")]
    Search {
        /// 文档或目录路径（.pdf、.md/.markdown 及 Word / EPUB / ODT / RTF / Office / CSV 家族；目录递归批量搜）
        file: PathBuf,
        /// 关键词；`--regex` 时按正则解释
        pattern: String,
        /// 按正则匹配
        #[arg(long)]
        regex: bool,
        /// 忽略大小写
        #[arg(short = 'i', long)]
        ignore_case: bool,
        /// 命中行前后各带 N 行上下文
        #[arg(short = 'C', long, default_value_t = 0)]
        context: usize,
        /// 限定页/节范围（1 起），如 1-3,5
        #[arg(long)]
        pages: Option<String>,
        /// 输出形态：text（行式，缺省）或 json（包膜）
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// 裁剪 JSON data 的点路径（如 hits[].text）；仅 --format json 下可用
        #[arg(long)]
        filter: Option<String>,
        /// 对 needs_ocr 页走 OCR 兜底（仅 PDF 单文件；首用下载约 20.5MB 模型，约 19-42 秒/页）
        #[arg(long)]
        ocr: bool,
        /// 禁模型下载（须与 --ocr 同用；模型未就位时报错）
        #[arg(long)]
        offline: bool,
    },
    /// 按页/节提取文档文本（默认输出到 stdout）
    #[command(after_long_help = "\
示例:
  reader extract ./doc.pdf
  reader extract ./doc.pdf --pages 1-3,5
  reader extract ./scan.pdf --ocr
  reader extract ./report.docx --format json --offset 0 --limit 5")]
    Extract {
        /// 文档路径（.pdf、.md/.markdown 及 Word / EPUB / ODT / RTF / Office / CSV 家族）
        file: PathBuf,
        /// 限定页/节范围（1 起），如 1-3,5
        #[arg(long)]
        pages: Option<String>,
        /// 写入文件（缺省输出到 stdout）
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// 输出形态：text（行式，缺省）或 json（包膜）
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// 裁剪 JSON data 的点路径（如 units[].no）；仅 --format json 下可用
        #[arg(long)]
        filter: Option<String>,
        /// 跳过前 N 个单元（0 起，两形态同用）
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// 最多输出 M 个单元
        #[arg(long)]
        limit: Option<usize>,
        /// 对 needs_ocr 页走 OCR 兜底（仅 PDF；首用下载约 20.5MB 模型，约 19-42 秒/页）
        #[arg(long)]
        ocr: bool,
        /// 禁模型下载（须与 --ocr 同用；模型未就位时报错）
        #[arg(long)]
        offline: bool,
    },
    /// 生成 SKILL.md（agent 发现与接入文档；--llms 给紧凑索引）
    Skill,
    /// 用 mq 表达式结构化提取文档（jq 风格：.h2 标题、.code 代码块、.link 链接、select 管道；命中退出 0，无命中退出 1，出错退出 2）
    #[command(after_long_help = "\
示例:
  reader query ./README.md \".h2\"
  reader query ./doc.pdf \".code\" --format json
  reader query ./notes.md \".[] | select(contains(\\\"关键词\\\"))\" --filter 'results[]'")]
    Query {
        /// 文档路径（.md/.markdown 原文、.pdf 及 anydoc 家族转 markdown 后查询）
        file: PathBuf,
        /// mq 表达式（完整语法见 mqlang.org）
        expression: String,
        /// 输出形态：text（markdown 片段，缺省）或 json（包膜）
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// 裁剪 JSON data 的点路径（如 results[]）；仅 --format json 下可用
        #[arg(long)]
        filter: Option<String>,
    },
    /// 自升级（GitHub Releases 最新正式版，下载校验后替换自身与兄弟二进制）
    #[command(name = "self")]
    SelfCmd {
        #[command(subcommand)]
        command: SelfCommands,
    },
}

#[derive(Subcommand)]
enum SelfCommands {
    /// 升级到最新正式版（已最新时明示；--force 同版本重装）
    Update {
        /// 版本相同也强制重装
        #[arg(long)]
        force: bool,
    },
}

/// CLI 入口：返回进程退出码。
pub fn run() -> i32 {
    let cli = Cli::parse();
    if cli.llms {
        print!("{}", introspect::llms_text());
        return 0;
    }
    match cli.command {
        Some(Commands::Skill) => {
            print!("{}", introspect::skill_md());
            0
        }
        Some(Commands::Query {
            file,
            expression,
            format,
            filter,
        }) => {
            let opts = OutputOpts { format, filter };
            match run_query(&file, &expression, &opts) {
                Ok(true) => 0,
                Ok(false) => 1,
                Err(err) => fail("query", opts.format, err),
            }
        }
        Some(Commands::SelfCmd {
            command: SelfCommands::Update { force },
        }) => match selfupdate::self_update(force) {
            Ok(outcome) => {
                match outcome.action {
                    "current" => println!(
                        "self_update: current {}（latest {}，已是最新）",
                        outcome.current, outcome.latest
                    ),
                    _ => {
                        println!(
                            "self_update: updated {} -> {}",
                            outcome.current, outcome.latest
                        );
                        for path in &outcome.replaced {
                            println!("path: {}", path.display());
                        }
                    }
                }
                0
            }
            Err(err) => fail("self update", Format::Text, err),
        },
        Some(Commands::Search {
            file,
            pattern,
            regex,
            ignore_case,
            context,
            pages,
            format,
            filter,
            ocr,
            offline,
        }) => {
            let opts = OutputOpts { format, filter };
            let ocr = OcrOpts { ocr, offline };
            let search_opts = SearchOpts {
                regex_mode: regex,
                ignore_case,
                context,
            };
            match run_search(&file, &pattern, &search_opts, pages, &opts, ocr) {
                Ok(true) => 0,
                Ok(false) => 1,
                Err(err) => fail("search", opts.format, err),
            }
        }
        Some(Commands::Extract {
            file,
            pages,
            out,
            format,
            filter,
            offset,
            limit,
            ocr,
            offline,
        }) => {
            let opts = OutputOpts { format, filter };
            let ocr = OcrOpts { ocr, offline };
            match run_extract(&file, pages, out, &opts, offset, limit, ocr) {
                Ok(()) => 0,
                Err(err) => fail("extract", opts.format, err),
            }
        }
        // 裸 reader（无子命令无旗标）：帮助走 stderr，退出 2（保持 clap 必填子命令时的语义）
        None => {
            eprintln!("{}", Cli::command().render_help());
            2
        }
    }
}

/// 失败出口：stderr 人读行恒出；json 形态下 stdout 补错误包膜（R001 错误走 stderr 不破）。
fn fail(command: &'static str, format: Format, err: String) -> i32 {
    if format == Format::Json {
        println!("{}", output::err_json(command, Instant::now(), err.clone()));
    }
    eprintln!("reader: {err}");
    2
}

/// 暴露 clap 命令树（tests 的旗标漂移守卫用；P0007）。
pub fn command_tree() -> clap::Command {
    Cli::command()
}

fn run_search(
    file: &Path,
    pattern: &str,
    search_opts: &SearchOpts,
    pages: Option<String>,
    opts: &OutputOpts,
    ocr: OcrOpts,
) -> Result<bool, String> {
    let started = Instant::now();
    let page_set = parse_optional_pages(pages)?;
    check_ocr_opts(ocr)?;
    if file.is_dir() {
        if page_set.is_some() {
            return Err("--pages 不适用于目录搜索".to_string());
        }
        if ocr.ocr {
            return Err("--ocr 不适用于目录搜索（请对单个 PDF 文件使用）".to_string());
        }
        check_filter(opts)?;
        let matcher =
            search::Matcher::new(pattern, search_opts.regex_mode, search_opts.ignore_case)?;
        return batch::run(
            file,
            &matcher,
            search_opts.context,
            opts.format,
            opts.filter.as_deref(),
            started,
        );
    }
    let matcher = search::Matcher::new(pattern, search_opts.regex_mode, search_opts.ignore_case)?;
    let extracted = document::extract(file, page_set.as_ref(), ocr)?;
    warn_unreliable(&extracted, ocr.ocr);
    let hits = search::search(&extracted, &matcher, search_opts.context);
    check_filter(opts)?;
    match opts.format {
        Format::Text => {
            for hit in &hits {
                for (line_no, text) in &hit.before {
                    println!("{}-{}-{}", hit.unit, line_no, text);
                }
                println!("{}:{}:{}", hit.unit, hit.line_no, hit.text);
                for (line_no, text) in &hit.after {
                    println!("{}-{}-{}", hit.unit, line_no, text);
                }
            }
        }
        Format::Json => {
            let mut data = search_data(&extracted, &hits);
            if let Some(path) = opts.filter.as_deref() {
                data = output::filter_value(&data, path)?;
            }
            println!("{}", output::ok_json("search", started, data)?);
        }
    }
    Ok(!hits.is_empty())
}

fn run_extract(
    file: &Path,
    pages: Option<String>,
    out: Option<PathBuf>,
    opts: &OutputOpts,
    offset: usize,
    limit: Option<usize>,
    ocr: OcrOpts,
) -> Result<(), String> {
    let started = Instant::now();
    let page_set = parse_optional_pages(pages)?;
    check_filter(opts)?;
    check_ocr_opts(ocr)?;
    if limit == Some(0) {
        return Err("无效 --limit: 须为正整数".to_string());
    }
    let extracted = document::extract(file, page_set.as_ref(), ocr)?;
    let total = extracted.len();
    let visible: Vec<&document::TextUnit> = extracted
        .iter()
        .skip(offset)
        .take(limit.unwrap_or(usize::MAX))
        .collect();
    let next_offset = (offset + visible.len() < total).then_some(offset + visible.len());
    let content = match opts.format {
        Format::Text => {
            let mut buf = String::new();
            for unit in &visible {
                buf.push_str(&format!("== {} {} ==\n", unit.kind.label(), unit.no));
                if let Some(reason) = &unit.needs_ocr {
                    buf.push_str(&format!("[needs_ocr: {reason}]\n"));
                }
                for line in &unit.lines {
                    buf.push_str(line);
                    buf.push('\n');
                }
            }
            buf
        }
        Format::Json => {
            let mut data =
                json!({ "units": visible.iter().map(|u| unit_value(u)).collect::<Vec<_>>() });
            if let Some(path) = opts.filter.as_deref() {
                data = output::filter_value(&data, path)?;
            }
            let cta = next_offset.map(|next| {
                let limit_arg = limit.map(|l| format!(" --limit {l}")).unwrap_or_default();
                format!(
                    "reader extract {} --offset {next}{limit_arg} --format json",
                    file.display()
                )
            });
            format!(
                "{}\n",
                output::ok_json_paged("extract", started, data, next_offset, cta)?
            )
        }
    };
    match out {
        Some(path) => {
            std::fs::write(&path, content).map_err(|e| format!("写入 {} 失败: {e}", path.display()))
        }
        None => {
            print!("{content}");
            Ok(())
        }
    }
}

/// filter 仅在 json 形态下可用。
fn check_filter(opts: &OutputOpts) -> Result<(), String> {
    if opts.filter.is_some() && opts.format != Format::Json {
        return Err("--filter 仅在 --format json 下可用".to_string());
    }
    Ok(())
}

/// query 子命令：格式转 markdown 后跑 mq 表达式；命中与否映射退出码 0/1（P0016）。
fn run_query(file: &Path, expression: &str, opts: &OutputOpts) -> Result<bool, String> {
    let started = Instant::now();
    check_filter(opts)?;
    if file.is_dir() {
        return Err("query 不支持目录（请对单个文件使用；批量找内容用 search <目录>）".to_string());
    }
    let markdown = query::to_markdown(file)?;
    let results = query::run_query(&markdown, expression)?;
    match opts.format {
        Format::Text => {
            for r in &results {
                println!("{r}");
            }
        }
        Format::Json => {
            let mut data = json!({ "results": results, "count": results.len() });
            if let Some(path) = opts.filter.as_deref() {
                data = output::filter_value(&data, path)?;
            }
            println!("{}", output::ok_json("query", started, data)?);
        }
    }
    Ok(!results.is_empty())
}

/// search 的 data 树：hits 加 needs_ocr_units（不可靠页序号）。
fn search_data(units: &[document::TextUnit], hits: &[search::Hit]) -> Value {
    json!({
        "hits": hits.iter().map(|h| json!({
            "unit": h.unit,
            "line": h.line_no,
            "text": &h.text,
            "before": h.before.iter().map(|(l, t)| json!({"line": l, "text": t})).collect::<Vec<_>>(),
            "after": h.after.iter().map(|(l, t)| json!({"line": l, "text": t})).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "needs_ocr_units": units
            .iter()
            .filter(|u| u.needs_ocr.is_some())
            .map(|u| u.no)
            .collect::<Vec<_>>(),
    })
}

/// extract 的 data 树：units（kind / no / needs_ocr / lines）。
fn unit_value(unit: &document::TextUnit) -> Value {
    json!({
        "kind": unit.kind.label(),
        "no": unit.no,
        "needs_ocr": &unit.needs_ocr,
        "lines": &unit.lines,
    })
}

/// `--offline` 须与 `--ocr` 同用（单挂无意义，属于旗标误用）。
fn check_ocr_opts(ocr: OcrOpts) -> Result<(), String> {
    if ocr.offline && !ocr.ocr {
        return Err("--offline 须与 --ocr 同用".to_string());
    }
    Ok(())
}

/// 文本层不可靠的单元给一条 stderr 警示（stdout 保持纯命中输出；退出码语义不变）。
/// `ocr` 为真时这些页已经 OCR 兜底（`needs_ocr` 标记保留，mobile 模型有掉字）。
pub(crate) fn warn_unreliable(units: &[document::TextUnit], ocr: bool) {
    let bad: Vec<&document::TextUnit> = units.iter().filter(|u| u.needs_ocr.is_some()).collect();
    if bad.is_empty() {
        return;
    }
    let label = bad[0].kind.label();
    let list = bad
        .iter()
        .map(|u| u.no.to_string())
        .collect::<Vec<_>>()
        .join(",");
    if ocr {
        eprintln!(
            "reader: 提示: {label} {list} 已经 OCR 兜底（needs_ocr 标记保留，mobile 模型有系统性掉字，命中可能失真）"
        );
    } else {
        eprintln!(
            "reader: 提示: {label} {list} 文本层不可靠（needs_ocr，疑似扫描件或编码问题），命中可能失真；可加 --ocr 兜底识别（仅 PDF 单文件，慢）"
        );
    }
}

fn parse_optional_pages(pages: Option<String>) -> Result<Option<HashSet<u32>>, String> {
    pages.map(|spec| parse_page_spec(&spec)).transpose()
}

/// 解析页范围串（如 `1-3,5`）为 1 起页码集合。
pub fn parse_page_spec(spec: &str) -> Result<HashSet<u32>, String> {
    let mut set = HashSet::new();
    for part in spec.split(',') {
        let part = part.trim();
        if let Some((lo, hi)) = part.split_once('-') {
            let lo = parse_page_no(lo, part)?;
            let hi = parse_page_no(hi, part)?;
            if lo > hi {
                return Err(format!("无效页范围 {part:?}: 起页大于止页"));
            }
            set.extend(lo..=hi);
        } else {
            set.insert(parse_page_no(part, part)?);
        }
    }
    if set.is_empty() {
        return Err(format!("无效页范围 {spec:?}: 为空"));
    }
    Ok(set)
}

fn parse_page_no(text: &str, part: &str) -> Result<u32, String> {
    let no: u32 = text
        .trim()
        .parse()
        .map_err(|_| format!("无效页范围 {part:?}: {text:?} 不是正整数"))?;
    if no == 0 {
        return Err(format!("无效页范围 {part:?}: 页码从 1 起"));
    }
    Ok(no)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_spec_single_and_range() {
        let set = parse_page_spec("1-3,5").unwrap();
        assert_eq!(set, HashSet::from([1, 2, 3, 5]));
    }

    #[test]
    fn page_spec_dies_zero_and_inverted() {
        assert!(parse_page_spec("0").is_err());
        assert!(parse_page_spec("3-1").is_err());
        assert!(parse_page_spec("a").is_err());
        assert!(parse_page_spec("").is_err());
    }
}
