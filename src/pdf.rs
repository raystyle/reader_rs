//! PDF 页提取：包 pdf-inspector 的 markdown 布局管线（多栏阅读序、needs_ocr 检出）。

use crate::document::{OcrOpts, TextUnit, UnitKind};
use pdf_inspector::extract_pages_markdown;
use std::collections::HashSet;
use std::path::Path;

/// 提取指定页（`None` 为全部页，页码 1 起）为 markdown 行，行按阅读序排列；
/// 文本层不可靠的页（扫描件、编码问题、乱码、空提取）带 `needs_ocr` 原因。
/// `ocr.ocr` 为真时对 needs_ocr 页走 OCR 兜底回填 lines（P0014；`needs_ocr` 标记保留，
/// OCR 文本仍属不可靠）。
pub fn extract_pages(
    path: &Path,
    pages: Option<&HashSet<u32>>,
    ocr: OcrOpts,
) -> Result<Vec<TextUnit>, String> {
    // pdf-inspector 的 pages 参数是 0 基有序切片；外部过滤是 1 基集合，换算为升序去重 Vec。
    let zero_based: Option<Vec<u32>> = pages.map(|set| {
        let mut list: Vec<u32> = set.iter().map(|n| n.saturating_sub(1)).collect();
        list.sort_unstable();
        list.dedup();
        list
    });
    let result = extract_pages_markdown(path, zero_based.as_deref())
        .map_err(|e| format!("无法读取 PDF {}: {e}", path.display()))?;
    let mut units: Vec<TextUnit> = result
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
        .collect();
    if ocr.ocr {
        let bad: Vec<u32> = units
            .iter()
            .filter(|u| u.needs_ocr.is_some())
            .map(|u| u.no)
            .collect();
        if !bad.is_empty() {
            for (no, lines) in crate::ocr::ocr_pages(path, &bad, ocr.offline)? {
                if let Some(unit) = units.iter_mut().find(|u| u.no == no) {
                    if !lines.is_empty() {
                        unit.lines = lines;
                    }
                }
            }
        }
    }
    Ok(units)
}
