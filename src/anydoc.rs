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
    let (sections, has_heading) = split_markdown(&markdown);
    Ok(sections_to_units(sections, has_heading, filter))
}

/// 提取 markdown 原文文档（.md，P0016）：读 UTF-8 文本直接进同一条分节管线，
/// 节语义与 anydoc 家族完全一致（section/part、`--pages`、分页全继承）。
pub fn extract_markdown(
    path: &Path,
    filter: Option<&HashSet<u32>>,
) -> Result<Vec<TextUnit>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        format!(
            "无法读取 markdown 文档 {}（须为 UTF-8）: {e}",
            path.display()
        )
    })?;
    let (sections, has_heading) = split_markdown(&text);
    Ok(sections_to_units(sections, has_heading, filter))
}

/// 节体映射 TextUnit：part 分片（P0010/P0011）后按 1 起序号过滤。
fn sections_to_units(
    sections: Vec<Vec<String>>,
    has_heading: bool,
    filter: Option<&HashSet<u32>>,
) -> Vec<TextUnit> {
    to_unit_bodies(sections, has_heading)
        .into_iter()
        .enumerate()
        .filter(|(i, _)| filter.map(|f| f.contains(&(*i as u32 + 1))).unwrap_or(true))
        .map(|(i, (kind, lines))| TextUnit {
            no: i as u32 + 1,
            kind,
            lines,
            needs_ocr: None,
        })
        .collect()
}

/// 节体到单元体的判定：超预算的节切 part（P0011），短节保持 section；
/// 无标题文档整篇走 part（P0010 行为不变）。
fn to_unit_bodies(sections: Vec<Vec<String>>, has_heading: bool) -> Vec<(UnitKind, Vec<String>)> {
    if !has_heading {
        return chunk_lines(sections.into_iter().next().unwrap_or_default())
            .into_iter()
            .map(|lines| (UnitKind::Part, lines))
            .collect();
    }
    sections
        .into_iter()
        .flat_map(|lines| {
            if lines.len() > PART_LINE_BUDGET {
                chunk_lines(lines)
                    .into_iter()
                    .map(|p| (UnitKind::Part, p))
                    .collect::<Vec<_>>()
            } else {
                vec![(UnitKind::Section, lines)]
            }
        })
        .collect()
}

// 简化注记：GFM markdown 按顶层 ATX 标题行分节，代码围栏（``` 开合）内的 # 行不分节；
// 节内行原样保留（表格、列表、代码块保形）；空行丢弃（行号紧凑，与 EPUB 通道口径一致）；
// 全文无标题则整篇一单元、由调用方按行预算分片为 part。天花板：不做 ~~~ 围栏与行内转义 # 的
// 特判（罕见），升级路径：引解析器。
fn split_markdown(markdown: &str) -> (Vec<Vec<String>>, bool) {
    let mut sections: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut in_fence = false;
    let mut has_heading = false;
    for line in markdown.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
        } else if !in_fence && is_atx_heading(trimmed) {
            has_heading = true;
            sections.push(std::mem::take(&mut current));
        }
        current.push(line.trim_end().to_string());
    }
    sections.push(current);
    let sections = sections
        .into_iter()
        .map(|lines| -> Vec<String> { lines.into_iter().filter(|l| !l.is_empty()).collect() })
        .filter(|lines: &Vec<String>| !lines.is_empty())
        .collect();
    (sections, has_heading)
}

/// 行分片预算：200 行一个 part（PDF 页约 40 到 60 行，量级相称；P0010 起，P0011 推广到超长节）。
const PART_LINE_BUDGET: usize = 200;

/// 整篇行按预算切 part；空文档不出单元。
fn chunk_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    if lines.is_empty() {
        return Vec::new();
    }
    lines.chunks(PART_LINE_BUDGET).map(|c| c.to_vec()).collect()
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
        let (units, has_heading) = split_markdown("# Title\npara one\n\n## Sub\npara two\n");
        assert!(has_heading);
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
        let (units, has_heading) = split_markdown(md);
        assert!(has_heading);
        assert_eq!(units.len(), 1);
        assert!(units[0].contains(&"# not a heading".to_string()));
    }

    #[test]
    fn markdown_without_headings_is_single_section() {
        let (units, has_heading) = split_markdown("line one\n\nline two\n");
        assert!(!has_heading);
        assert_eq!(
            units,
            vec![vec!["line one".to_string(), "line two".to_string()]]
        );
    }

    #[test]
    fn markdown_leading_blank_section_dropped() {
        let (units, has_heading) = split_markdown("\n\n# Only\nbody\n");
        assert!(has_heading);
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

    #[test]
    fn headingless_long_document_chunks_into_parts() {
        let md: String = (0..450)
            .map(|i| format!("line-{i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let (sections, has_heading) = split_markdown(&md);
        assert!(!has_heading);
        let parts = chunk_lines(sections.into_iter().next().unwrap());
        assert_eq!(parts.len(), 3);
        assert_eq!(
            (parts[0].len(), parts[1].len(), parts[2].len()),
            (200, 200, 50)
        );
        assert_eq!(parts[2].last().unwrap(), "line-449");
    }

    #[test]
    fn headingless_short_document_stays_single_part() {
        let (sections, has_heading) = split_markdown("only\nthree\nlines\n");
        assert!(!has_heading);
        let parts = chunk_lines(sections.into_iter().next().unwrap());
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].len(), 3);
    }

    #[test]
    fn empty_markdown_yields_no_parts() {
        let (sections, has_heading) = split_markdown("");
        assert!(!has_heading && sections.is_empty());
        assert!(chunk_lines(sections.into_iter().next().unwrap_or_default()).is_empty());
    }

    #[test]
    fn overlong_section_splits_into_parts_among_sections() {
        let long: Vec<String> = (0..450).map(|i| format!("l{i}")).collect();
        let sections = vec![
            vec!["# A".to_string(), "x".to_string()],
            {
                let mut s = vec!["# B".to_string()];
                s.extend(long);
                s
            },
            vec!["# C".to_string(), "y".to_string()],
        ];
        let units = to_unit_bodies(sections, true);
        assert_eq!(units.len(), 5, "短节1 + 长节切3 + 短节1");
        assert_eq!(units[0].0, UnitKind::Section);
        assert_eq!(
            (units[1].0, units[2].0, units[3].0),
            (UnitKind::Part, UnitKind::Part, UnitKind::Part)
        );
        assert_eq!(
            (units[1].1.len(), units[2].1.len(), units[3].1.len()),
            (200, 200, 51)
        );
        assert_eq!(units[4].0, UnitKind::Section);
    }

    #[test]
    fn all_short_sections_stay_sections() {
        let sections = vec![
            vec!["# A".into(), "x".into()],
            vec!["# B".into(), "y".into()],
        ];
        let units = to_unit_bodies(sections, true);
        assert_eq!(units.len(), 2);
        assert!(units.iter().all(|(k, _)| *k == UnitKind::Section));
    }

    #[test]
    fn headingless_maps_whole_body_to_parts() {
        let units = to_unit_bodies(vec![(0..3).map(|i| format!("l{i}")).collect()], false);
        assert_eq!(units.len(), 1);
        assert_eq!(units[0].0, UnitKind::Part);
        assert_eq!(units[0].1.len(), 3);
    }
}
