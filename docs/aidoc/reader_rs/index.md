# reader_rs 0.8.1

Reader：Agent 原生文档阅读、搜索和提取工具。为 Agent 管线设计的 Rust 单二进制 CLI
（`reader` 与 `rr` 双名，同一 `main` 薄壳）：从本地 PDF、markdown、图片与 anydoc 家族
（Word 含 legacy .doc、EPUB、ODT、RTF、Office、CSV 等 14 种格式）读文本层。
能力面：按页/节读（extract）、字面与正则搜加目录批量（search）、mq 结构化提取（query）、
OCR 兜底识图（`--ocr`，PP-OCRv6 三级回退源链）、图片本体导出与一键完整提取（figures/export）。
输出契约：行式标记、grep 语义退出码 0/1/2、`--format json` 包膜加 `--filter` 裁剪，
机器可读优先于人类美观。本文件承载 CLI 定义与 `run()` 分发；各模块以
`document::TextUnit` 为统一文本单元。

## Modules

- [`anydoc`](anydoc.md): anydoc 统一引擎提取（P0009）：Word / EPUB / ODT / RTF / Office / CSV 家族出 GFM markdown，
- [`batch`](batch.md): 批量目录搜索（P0012）：递归走目录、逐文件 search、聚合 text/json 输出。
- [`document`](document.md): 格式分派与统一文本单元：PDF 的页、其余格式的标题节与无标题分片，对上同为 `TextUnit`。
- [`figures`](figures.md): 图片本体导出与文本元数据对齐（D47；S010 定界：只负责提取存储，理解归调用方 Agent）。
- [`introspect`](introspect.md): Agent 自省与发现（P0007；三面统一批 REQ-057 对齐总台 REQ-060）：`--llms` 旗标
- [`issue`](issue.md): issue 域（总台统一入口 issues.ohmygh.com，ohmycloud REQ-057 契约对齐）：
- [`mirror`](mirror.md): 镜像源链与镜像清单(D42):OCR 模型三级回退下载(镜像 到 HF 直连 到 GitHub
- [`ocr`](ocr.md): OCR 兜底管线（P0014 落地、P0018 换引擎、D42 镜像源链、D43 图片文件）：needs_ocr 页经
- [`output`](output.md): 输出层：JSON 包膜（ok/data/error 加 meta）与 filter 点路径裁剪。设计依据 S002（P0006）。
- [`pdf`](pdf.md): PDF 页提取：包 pdf-inspector 的 markdown 布局管线（多栏阅读序、needs_ocr 检出）。
- [`query`](query.md): mq 结构化提取（P0016；学习 harehare/mq，选型 S007）：全部支持格式转 markdown 文本后
- [`search`](search.md): 搜索层：匹配器（字面 / 正则 / 忽略大小写）与命中收集。
- [`selfupdate`](selfupdate.md): self update（P0015；D42 加镜像通道）：`reader self update` 先读镜像

