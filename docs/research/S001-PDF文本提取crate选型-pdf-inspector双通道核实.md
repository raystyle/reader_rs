# S001-PDF文本提取crate选型-pdf-inspector双通道核实

> 2026-08-31。触发点：用户要 PDF 文本搜索提取工具，点名研究 firecrawl/pdf-inspector。流程按 `docs\references\R002-选型研究细则-cratesio与github双通道.md`；六态标准见 `docs\guide\G002-研究标准细则-结构与六态标记.md`。

## 背景

为 reader_rs 选 PDF 文本提取引擎。要求：纯 Rust（不外挂二进制）、按页提取、最好带文本坐标（支撑按页搜索与行重建）、持续维护。

## 关键结论

1. **选定 pdf-inspector 1.17.0**：MIT、纯 Rust 默认构建（default features 为空，解析走 lopdf）、位置感知提取（TextItem 带 page/x/y/width/font_size）、按页过滤 API（`extract_text_with_positions_pages`）现成。[实证: 2026-08-31 cargo info 与本地 registry 源码 `src\extractor\mod.rs`、`src\types.rs`]
2. 双通道信号均强：GitHub 17067 stars、未归档、2026-08-21 仍有提交、最近 release v1.15.0（2026-08-17）；crates.io 已发到 1.17.0。[实证: 2026-08-31 gh repo view 与 cargo search]
3. 用户点名在前，核实在后：点名方向经双通道核实成立，非盲采。[实证: 2026-08-31 对话与上述查询]
4. 备选均不成立：lopdf 太底层（需自写编码与 CMap 处理）；pdf-extract 无位置信息；pdfium-render 需外挂 pdfium 二进制（且本机 rsproxy 镜像 `cargo info pdfium-render` 查无结果，未深究）。[实证: cargo info 三个候选 2026-08-31；pdfium 外挂为其官方文档口径，[记忆] 待复核]
5. 官方基准（opendataloader-bench 200 PDF 语料）自称 overall 0.875、200 篇 0.470s，优于 liteparse/opendataloader/pymupdf4llm。[经验: firecrawl README 2026-07-31 刷新数据，厂商自测未本机复核]

## 现状或实测

### crates.io 通道

| crate | 版本 | license | rust-version | 备注 |
| --- | --- | --- | --- | --- |
| pdf-inspector | 1.17.0 | MIT | 1.88 | default features 空；ocr/render-pdfium 等为可选 feature |
| pdf-extract | 0.12.0 | MIT | unknown | 只出整段文本 |
| lopdf | 0.44.0 | MIT | 1.88 | pdf-inspector 的底层解析依赖 |
| hayro | 0.7.1 | Apache-2.0 OR MIT | 1.92 | 定位是 rasterizer，非文本提取 |

[实证: 2026-08-31 `cargo search` / `cargo info --registry crates-io`（rsproxy 镜像）]

### GitHub 通道

- `gh repo view firecrawl/pdf-inspector`：stargazerCount 17067、pushedAt 2026-08-21、isArchived false、license MIT、latestRelease v1.15.0（2026-08-17）、语言分布以 Rust 为主（约 3.5MB）。[实证: 2026-08-31]
- crates.io 版本 1.17.0 新于 GitHub latest release v1.15.0，release 页可能滞后于 crates 发布，不视为风险。[推断]

### API 核实

> 以下行号与字段核自本机 cargo registry 的 pdf-inspector-1.17.0 源码。

- `extract_text_with_positions_pages(path, Option<&HashSet<u32>>) -> Result<Vec<TextItem>, PdfError>`：`src\extractor\mod.rs:86`（page_filter 为 1 起页码集合）。[实证: pdf-inspector-1.17.0 本地源码]
- `TextItem` 字段：text、x、y（PDF 坐标，原点左下）、width、height、font、font_tag、font_size、page（1 起）、is_bold/is_italic 等：`src\types.rs:98`。[实证: 同上]
- 公开函数表（process_pdf / detect_pdf / extract_text / extract_pages_markdown 等）见上游仓库文档 <https://github.com/firecrawl/pdf-inspector/blob/main/docs/rust-api.md>。[经验: 官方文档，与本地源码抽样一致]

## 踩坑沉淀

| 现象 | 根因 | 正确处理 |
| --- | --- | --- |
| `cargo info` 不带 `--registry` 走镜像查询异常（pdfium-render 查无结果） | 本机 rsproxy 镜像通道 | 一律 `cargo search/info --registry crates-io`（R002 坑表首条） |

## 待办

1. pdf-inspector 官方基准数据未本机复核，保持 [经验] 标记；阶段 2 做提取质量时可用真实样本回归。
2. hayro 是 2026 新秀（0.7.1、偏渲染），若将来要做渲染或发现 pdf-inspector 解析缺口，重评。
