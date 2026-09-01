//! anydoc 统一引擎提取（P0009）：Word / EPUB / ODT / RTF / Office / CSV 家族出 GFM markdown，
//! 按顶层标题分节映射 `TextUnit`。PDF 例外——走 `pdf.rs` 直连 pdf-inspector 保页契约
//! （anydoc 自身对 PDF 也直连 pdf-inspector 绕过文档模型，架构同构）。

use crate::document::{TextUnit, UnitKind};
use std::collections::HashSet;
use std::path::Path;

/// 提取 anydoc 家族文档为分节单元（`filter` 为 1 起单元号集合，`None` 为全部）。
pub fn extract_sections(
    path: &Path,
    filter: Option<&HashSet<u32>>,
) -> Result<Vec<TextUnit>, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("无法读取文档 {}: {e}", path.display()))?;
    let format = ::anydoc::Format::from_bytes(&bytes).or_else(|| ::anydoc::Format::from_path(path));
    let Some(format) = format else {
        return Err(format!("无法识别文档格式（{}）", path.display()));
    };
    let markdown = ::anydoc::to_markdown_bytes(&bytes, format)
        .map_err(|e| format!("无法解析文档 {}: {e}", path.display()))?;
    Ok(markdown_to_units(&markdown)
        .into_iter()
        .enumerate()
        .filter(|(i, _)| filter.map(|f| f.contains(&(*i as u32 + 1))).unwrap_or(true))
        .map(|(i, lines)| TextUnit {
            no: i as u32 + 1,
            kind: UnitKind::Section,
            lines,
            needs_ocr: None,
        })
        .collect())
}

// 简化注记：GFM markdown 按顶层 ATX 标题行分节，代码围栏（``` 开合）内的 # 行不分节；
// 节内行原样保留（表格、列表、代码块保形）；空行丢弃（行号紧凑，与 EPUB 通道口径一致）；
// 全文无标题则整篇一单元。天花板：不做 ~~~ 围栏与行内转义 # 的特判（罕见），升级路径：引解析器。
fn markdown_to_units(markdown: &str) -> Vec<Vec<String>> {
    let mut sections: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut in_fence = false;
    for line in markdown.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
        } else if !in_fence && is_atx_heading(trimmed) {
            sections.push(std::mem::take(&mut current));
        }
        current.push(line.trim_end().to_string());
    }
    sections.push(current);
    sections
        .into_iter()
        .map(|lines| -> Vec<String> { lines.into_iter().filter(|l| !l.is_empty()).collect() })
        .filter(|lines: &Vec<String>| !lines.is_empty())
        .collect()
}

/// GFM ATX 标题：行首 1 到 6 个 `#`，后跟空白或行尾。
fn is_atx_heading(trimmed: &str) -> bool {
    let hashes = trimmed.chars().take_while(|&c| c == '#').count();
    if !(1..=6).contains(&hashes) {
        return false;
    }
    let rest = &trimmed[hashes..];
    rest.is_empty() || rest.starts_with([' ', '\t'])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_splits_at_top_level_headings() {
        let md = "# Title\npara one\n\n## Sub\npara two\n";
        let units = markdown_to_units(md);
        assert_eq!(
            units,
            vec![
                vec!["# Title".to_string(), "para one".to_string()],
                vec!["## Sub".to_string(), "para two".to_string()]
            ]
        );
    }

    #[test]
    fn markdown_hash_inside_fence_stays_in_section() {
        let md = "# Title\n```rust\n# not a heading\nlet x = 1;\n```\nafter\n";
        let units = markdown_to_units(md);
        assert_eq!(units.len(), 1);
        assert!(units[0].contains(&"# not a heading".to_string()));
    }

    #[test]
    fn markdown_without_headings_is_single_unit() {
        let units = markdown_to_units("line one\n\nline two\n");
        assert_eq!(
            units,
            vec![vec!["line one".to_string(), "line two".to_string()]]
        );
    }

    #[test]
    fn markdown_leading_blank_section_dropped() {
        let units = markdown_to_units("\n\n# Only\nbody\n");
        assert_eq!(units.len(), 1);
        assert_eq!(units[0][0], "# Only");
    }

    #[test]
    fn atx_requires_space_or_end() {
        assert!(is_atx_heading("# a"));
        assert!(is_atx_heading("###### a"));
        assert!(!is_atx_heading("####### a")); // 7 个 # 非标题
        assert!(!is_atx_heading("#a"));
        assert!(is_atx_heading("#"));
    }
}
