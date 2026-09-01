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
- **提取质量（破坏性变更）**：PDF 通道整体切 pdf-inspector markdown 布局管线——多栏阅读序、needs_ocr 检出提示（extract 页节提示行、search stderr 警示）、逐页分节补齐；PDF 输出行带 markdown 语法（标题、链接、表格），v0.1 朴素行重建退役。质量承诺面向英文与中文。方案 `docs\proven\P0005-PDF提取质量-markdown管线与needs_ocr提示.md`。
- **Agent 原生输出（P0006）**：`--format json` 包膜（`{ok,data,error}` 加 meta；错误 stdout 包膜加 stderr 人读行，退出码 0/1/2 不变）、extract `--offset/--limit` 分页（meta 带 `next_offset` 与 `cta`）、`--filter` 点路径裁剪；serde / serde_json 进依赖。方案 `docs\proven\P0006-输出形态-json包膜与分页裁剪.md`。
- **Agent 自省与发现（P0007）**：`reader --llms` 紧凑命令索引、`reader skill` 生成 SKILL.md（仓根提交，漂移双守卫：clap 命令树旗标全覆盖、仓内文件与运行时输出逐字节一致）、search/extract help 补 examples 节；零新依赖。方案 `docs\proven\P0007-Agent自省与发现-llms索引SKILL生成与help示例.md`。
