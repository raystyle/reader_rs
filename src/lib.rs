//! Reader RS：PDF 文档文本搜索与提取工具。
//! 薄壳在 `src\main.rs`；本文件承载 CLI 定义、`run()` 分发与页范围解析。

pub mod pdf;
pub mod search;

use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "reader", version, about = "PDF 文档文本搜索与提取工具")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 按页搜索 PDF 文本（命中退出 0，无命中退出 1，出错退出 2）
    Search {
        /// PDF 文件路径
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
        /// 限定页范围（1 起），如 1-3,5
        #[arg(long)]
        pages: Option<String>,
    },
    /// 按页提取 PDF 文本（默认输出到 stdout）
    Extract {
        /// PDF 文件路径
        file: PathBuf,
        /// 限定页范围（1 起），如 1-3,5
        #[arg(long)]
        pages: Option<String>,
        /// 写入文件（缺省输出到 stdout）
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

/// CLI 入口：返回进程退出码。
pub fn run() -> i32 {
    match Cli::parse().command {
        Commands::Search {
            file,
            pattern,
            regex,
            ignore_case,
            context,
            pages,
        } => match run_search(&file, &pattern, regex, ignore_case, context, pages) {
            Ok(true) => 0,
            Ok(false) => 1,
            Err(err) => {
                eprintln!("reader: {err}");
                2
            }
        },
        Commands::Extract { file, pages, out } => match run_extract(&file, pages, out) {
            Ok(()) => 0,
            Err(err) => {
                eprintln!("reader: {err}");
                2
            }
        },
    }
}

fn run_search(
    file: &Path,
    pattern: &str,
    regex_mode: bool,
    ignore_case: bool,
    context: usize,
    pages: Option<String>,
) -> Result<bool, String> {
    let page_set = parse_optional_pages(pages)?;
    let matcher = search::Matcher::new(pattern, regex_mode, ignore_case)?;
    let extracted = pdf::extract_pages(file, page_set.as_ref())?;
    let hits = search::search(&extracted, &matcher, context);
    for hit in &hits {
        for (line_no, text) in &hit.before {
            println!("{}-{}-{}", hit.page, line_no, text);
        }
        println!("{}:{}:{}", hit.page, hit.line_no, hit.text);
        for (line_no, text) in &hit.after {
            println!("{}-{}-{}", hit.page, line_no, text);
        }
    }
    Ok(!hits.is_empty())
}

fn run_extract(file: &Path, pages: Option<String>, out: Option<PathBuf>) -> Result<(), String> {
    let page_set = parse_optional_pages(pages)?;
    let extracted = pdf::extract_pages(file, page_set.as_ref())?;
    let mut buf = String::new();
    for page in &extracted {
        buf.push_str(&format!("== page {} ==\n", page.page));
        for line in &page.lines {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    match out {
        Some(path) => {
            std::fs::write(&path, buf).map_err(|e| format!("写入 {} 失败: {e}", path.display()))
        }
        None => {
            print!("{buf}");
            Ok(())
        }
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
