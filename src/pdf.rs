//! PDF 页提取与行重建：包 pdf-inspector 的位置感知提取。

use crate::document::{TextUnit, UnitKind};
use pdf_inspector::{extract_text_with_positions_pages, TextItem};
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

/// 提取指定页（`None` 为全部页，页码 1 起）并重建文本行，行按页面从上往下排序。
pub fn extract_pages(path: &Path, pages: Option<&HashSet<u32>>) -> Result<Vec<TextUnit>, String> {
    let items = extract_text_with_positions_pages(path, pages)
        .map_err(|e| format!("无法读取 PDF {}: {e}", path.display()))?;
    Ok(group_into_pages(items))
}

fn group_into_pages(items: Vec<TextItem>) -> Vec<TextUnit> {
    let mut by_page: BTreeMap<u32, Vec<TextItem>> = BTreeMap::new();
    for item in items {
        by_page.entry(item.page).or_default().push(item);
    }
    by_page
        .into_iter()
        .map(|(page, items)| TextUnit {
            no: page,
            kind: UnitKind::Page,
            lines: items_to_lines(items),
        })
        .collect()
}

// 简化注记：行重建为朴素几何法——按 y 降序（PDF 原点左下，y 大者在页面上方）扫描，
// y 差在容差内归入同行，行内按 x 升序拼接，间隙超字号 1/4 补一个空格。
// 天花板：多栏排版不做栏检测，跨栏同行会串成一行；升级路径：换 pdf-inspector 的
// layout/markdown 管线（extract_pages_markdown）。
fn items_to_lines(mut items: Vec<TextItem>) -> Vec<String> {
    items.sort_by(|a, b| b.y.total_cmp(&a.y).then(a.x.total_cmp(&b.x)));
    let mut lines: Vec<Vec<TextItem>> = Vec::new();
    for item in items {
        if item.text.trim().is_empty() {
            continue;
        }
        let tol = (item.height * 0.5).max(1.0);
        match lines.last_mut() {
            Some(line) if (line[0].y - item.y).abs() <= tol => line.push(item),
            _ => lines.push(vec![item]),
        }
    }
    lines
        .into_iter()
        .map(join_line)
        .filter(|line| !line.is_empty())
        .collect()
}

fn join_line(mut items: Vec<TextItem>) -> String {
    items.sort_by(|a, b| a.x.total_cmp(&b.x));
    let mut out = String::new();
    let mut prev: Option<&TextItem> = None;
    for item in &items {
        if let Some(p) = prev {
            let gap = item.x - (p.x + p.width);
            if gap > p.font_size * 0.25
                && !out.ends_with(char::is_whitespace)
                && !item.text.starts_with(char::is_whitespace)
            {
                out.push(' ');
            }
        }
        out.push_str(&item.text);
        prev = Some(item);
    }
    out.trim().to_string()
}
