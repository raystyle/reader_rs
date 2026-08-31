//! 格式分派与统一文本单元：PDF 的页、EPUB 的章，对上同为 `TextUnit`。

use std::collections::HashSet;
use std::path::Path;

/// 一个文本单元。`no` 为 1 起序号（PDF 页码 / EPUB 章序），`lines` 按阅读序排列；
/// `needs_ocr` 为 `Some(原因)` 表示该单元文本层不可靠（扫描件、编码问题、乱码、空提取）。
pub struct TextUnit {
    pub no: u32,
    pub kind: UnitKind,
    pub lines: Vec<String>,
    pub needs_ocr: Option<String>,
}

/// 文本单元种类，决定输出分节标记（`== page N ==` / `== chapter N ==`）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnitKind {
    Page,
    Chapter,
}

impl UnitKind {
    pub fn label(self) -> &'static str {
        match self {
            UnitKind::Page => "page",
            UnitKind::Chapter => "chapter",
        }
    }
}

/// 按扩展名分派提取；`filter` 为 1 起序号集合（`None` 为全部）。
pub fn extract(path: &Path, filter: Option<&HashSet<u32>>) -> Result<Vec<TextUnit>, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .unwrap_or_default();
    match ext.as_str() {
        "pdf" => crate::pdf::extract_pages(path, filter),
        "epub" => crate::epub::extract_chapters(path, filter),
        other => Err(format!(
            "不支持的格式 {}（{}）；当前支持 .pdf / .epub",
            if other.is_empty() {
                "<无扩展名>"
            } else {
                other
            },
            path.display()
        )),
    }
}
