# reader_rs::anydoc

anydoc 统一引擎提取（P0009）：Word / EPUB / ODT / RTF / Office / CSV 家族出 GFM markdown，
按顶层标题分节映射 `TextUnit`。PDF 例外——走 `pdf.rs` 直连 pdf-inspector 保页契约
（anydoc 自身对 PDF 也直连 pdf-inspector 绕过文档模型，架构同构）。

## Functions

- `extract_markdown` — 提取 markdown 原文文档（.md，P0016）为分节单元。
- `extract_sections` — 提取 anydoc 家族文档为分节单元。

