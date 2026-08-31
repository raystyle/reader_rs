//! PDF 页提取：包 pdf-inspector 的 markdown 布局管线（多栏阅读序、needs_ocr 检出）。

use crate::document::{TextUnit, UnitKind};
use pdf_inspector::extract_pages_markdown;
use std::collections::HashSet;
use std::path::Path;

/// 提取指定页（`None` 为全部页，页码 1 起）为 markdown 行，行按阅读序排列；
/// 文本层不可靠的页（扫描件、编码问题、乱码、空提取）带 `needs_ocr` 原因。
pub fn extract_pages(path: &Path, pages: Option<&HashSet<u32>>) -> Result<Vec<TextUnit>, String> {
    // pdf-inspector 的 pages 参数是 0 基有序切片；外部过滤是 1 基集合，换算为升序去重 Vec。
    let zero_based: Option<Vec<u32>> = pages.map(|set| {
        let mut list: Vec<u32> = set.iter().map(|n| n.saturating_sub(1)).collect();
        list.sort_unstable();
        list.dedup();
        list
    });
    let result = extract_pages_markdown(path, zero_based.as_deref())
        .map_err(|e| format!("无法读取 PDF {}: {e}", path.display()))?;
    Ok(result
        .pages
        .into_iter()
        .map(|page| TextUnit {
            no: page.page + 1,
            kind: UnitKind::Page,
            lines: page.markdown.lines().map(str::to_string).collect(),
            needs_ocr: page
                .needs_ocr
                .then(|| page.ocr_reason.unwrap_or_else(|| "原因未明".to_string())),
        })
        .collect())
}
