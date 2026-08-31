# R001-项目定位-PDF文本搜索与提取工具

> 角色：**现役定位展开**。AGENTS 一、项目定位的全文版；定位变更走 proven 方案归档。

## 一、一句话定位

Reader RS 是 PDF 文档文本搜索与提取工具：Rust 编写的 CLI，对本地 PDF 做按页文本提取与关键词/正则搜索。[实证: 2026-08-31 用户原始需求]

## 二、本质与边界

1. **本质**：读 PDF 文本层，两个动作——`search`（找，带页码与上下文）与 `extract`（取，按页分节输出）。[实证: P0001 方案]
2. **边界**：
   - 首版不做渲染、编辑、OCR；扫描件与编码问题页检出后提示，不识别。[推断: pdf-inspector 提供 `needs_ocr` / `has_encoding_issues` 信号可支撑提示]
   - 纯 Rust 依赖；不外挂 pdfium 等二进制运行时（pdf-inspector 默认构建即纯 Rust）。[实证: cargo info pdf-inspector 2026-08-31，default features 为空]
   - Windows 优先验证；依赖均跨平台，不主动破坏其它平台。[推断: lopdf 与 pdf-inspector 无平台专属代码路径]

## 三、命名

- 显示名 Reader RS；仓库 `reader_rs`；CLI 二进制 `reader`，缩写 `rr`（同一 `src\main.rs` 两个 `[[bin]]`）。[实证: 2026-08-31 用户指定]

## 四、关键依赖

| 依赖 | 角色 | 依据 |
| --- | --- | --- |
| pdf-inspector | PDF 解析与文本提取（含页码、坐标、字号） | `docs\research\S001-PDF文本提取crate选型-pdf-inspector双通道核实.md` |
| clap | CLI 参数解析（derive） | 官方生态标准 |
| regex | `--regex` 搜索模式 | 官方生态标准 |

## 五、演进方向

见 `ROADMAP.md`：阶段 2 提取质量（多栏阅读序、问题页提示），阶段 3 输出形态（Markdown / JSON / 批量）。均按需立项，不预写代码。
