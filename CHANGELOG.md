# Changelog

本文件只记录**大版本里程碑**：定位变更、发布、阶段完成、核心能力整体落地。细碎条目由 `docs\diary\YYYY-MM-DD-*.md` 与 git 历史承载。

## [Unreleased]

### 里程碑

> 文档地基与最小闭环已完成（2026-08-31）。代码尚未发布。

- **项目定位**：Reader，Agent 原生文档阅读、搜索和提取工具。定位变更方案 `docs\proven\P0002-项目重新定位-Agent原生文档阅读搜索和提取工具.md`；首期切面 `docs\proven\P0001-PDF文本搜索与提取CLI最小闭环.md`。
- **文档骨架**：对照 `D:\ohmyagents` 的四段 AGENTS、三原语、docs 六目录、`.tools` 三件套。
- **CLI 名**：二进制 `reader` 与 `rr`（同入口双 bin）；项目名 `reader_rs`。
- **提取引擎**：firecrawl/pdf-inspector 1.17.0（纯 Rust 默认构建，lopdf 解析；选型 `docs\research\S001-PDF文本提取crate选型-pdf-inspector双通道核实.md`）。
- **最小闭环**：`reader search` / `reader extract` 落地；门禁三件与 rumdl 三件套全绿。
- **EPUB 支持**：格式分派（`TextUnit` 统一页/章），rbook 加 quick-xml 提取；cargo test 20 过；真实样本回归。方案 `docs\proven\P0003-EPUB支持与格式分派.md`，选型 `docs\research\S003-EPUB解析crate选型-rbook双通道核实.md`。
- **跨平台接管**：GitHub Actions 三系统门禁（windows/ubuntu/macos 跑 fmt/clippy/test --locked）与 LF 钉死。方案 `docs\proven\P0004-mac与Linux接管开发与跨平台兼容.md`。
