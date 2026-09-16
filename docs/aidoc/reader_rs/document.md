# reader_rs::document

格式分派与统一文本单元：PDF 的页、其余格式的标题节与无标题分片，对上同为 `TextUnit`。

## Functions

- `extract` — 按扩展名分派提取，返回统一文本单元。
- `is_image_ext` — 扩展名是否图片面（分派、批量遍历与 query 的专属错误共用；D43）。
- `is_supported` — 扩展名是否命中支持面（分派与批量目录遍历共用同一真源；P0012）。

## Types

- `OcrOpts` — OCR 兜底选项（P0014）：`ocr` 开兜底、`offline` 禁模型下载；仅对 PDF 的 needs_ocr 页生效。
- `TextUnit` — 一个文本单元。`no` 为 1 起序号（PDF 页码 / 其余格式节序），`lines` 按阅读序排列；
- `UnitKind` — 文本单元种类，决定输出分节标记（`== page N ==` / `== section N ==` / `== part N ==`）。

