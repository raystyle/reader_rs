//! 批量目录搜索（P0012）：递归走目录、逐文件 search、聚合 text/json 输出。
//! 单文件 search 的输出与退出码契约不变；目录模式命中行带路径前缀，坏文件 stderr 跳过后继续。
//! 天花板：顺序遍历不并发；符号链接跟随（依赖文件系统无环）；needs_ocr 走 stderr 不进 json。

use crate::document;
use crate::search::{self, Matcher};
use crate::Format;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::time::Instant;

/// 目录批量搜索：返回是否任一文件命中（退出码 0/1 由调用方映射；Err 映射 2）。
pub(crate) fn run(
    dir: &Path,
    matcher: &Matcher,
    context: usize,
    format: Format,
    filter: Option<&str>,
    started: Instant,
) -> Result<bool, String> {
    let mut files = Vec::new();
    walk(dir, &mut files)?;
    if files.is_empty() {
        return Err(format!("目录无支持格式文件（{}）", dir.display()));
    }
    let mut any_hit = false;
    let mut skipped = 0u32;
    let mut json_hits: Vec<Value> = Vec::new();
    for file in &files {
        let units = match document::extract(file, None) {
            Ok(units) => units,
            Err(err) => {
                eprintln!("reader: 跳过 {}: {err}", file.display());
                skipped += 1;
                continue;
            }
        };
        crate::warn_unreliable(&units);
        let hits = search::search(&units, matcher, context);
        if hits.is_empty() {
            continue;
        }
        any_hit = true;
        match format {
            Format::Text => {
                for hit in &hits {
                    for (line_no, text) in &hit.before {
                        println!("{}:{}-{}-{}", file.display(), hit.unit, line_no, text);
                    }
                    println!(
                        "{}:{}:{}:{}",
                        file.display(),
                        hit.unit,
                        hit.line_no,
                        hit.text
                    );
                    for (line_no, text) in &hit.after {
                        println!("{}:{}-{}-{}", file.display(), hit.unit, line_no, text);
                    }
                }
            }
            Format::Json => json_hits.extend(hits.iter().map(|h| {
                json!({
                    "file": file.display().to_string(),
                    "unit": h.unit,
                    "line": h.line_no,
                    "text": &h.text,
                    "before": h.before.iter().map(|(l, t)| json!({"line": l, "text": t})).collect::<Vec<_>>(),
                    "after": h.after.iter().map(|(l, t)| json!({"line": l, "text": t})).collect::<Vec<_>>(),
                })
            })),
        }
    }
    if format == Format::Json {
        let mut data = json!({
            "hits": json_hits,
            "files": { "scanned": files.len(), "skipped": skipped },
        });
        if let Some(path) = filter {
            data = crate::output::filter_value(&data, path)?;
        }
        println!("{}", crate::output::ok_json("search", started, data)?);
    }
    Ok(any_hit)
}

/// 递归走目录收支持格式的文件，路径排序保证输出稳定。
fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("无法读取目录 {}: {e}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            walk(&path, out)?;
        } else if document::is_supported(&path) {
            out.push(path);
        }
    }
    Ok(())
}
