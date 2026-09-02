# Changelog

本文件只记录**大版本里程碑**：定位变更、发布、阶段完成、核心能力整体落地。细碎条目由 `docs\diary\YYYY-MM-DD-*.md` 与 git 历史承载。

## [Unreleased]

## [0.3.0] - 2026-09-03

> 能力面三连加性能翻身：OCR 兜底（P0014）、self update（P0015）、markdown 与 mq query（P0016）、OCR 提速进 5-10 秒/页档（P0017）。三平台 CI 加 lan-mac/lan-linux 实机双路回归全绿。

- **OCR 性能优化（P0017）**：`--ocr` 单页 20.5s 提速到约 3-5.5s（多核）：rec 张量动态宽度加 320 桶化、按行宽分组分批、组间并行（每 worker 独立推理会话），rec_batch_size 按核数自适应；正文识别质量持平，水印碎片行由空串变为少量噪声短行（needs_ocr 域内）。
- **SKILL 重构**：SKILL.md 改为常用例子加输出契约加渐进引导（`--llms` / `<子命令> --help`）三节式；全量参数面由 `--help` 与 README 承载。
- **markdown 支持与 mq 结构化提取（P0016）**：`.md` / `.markdown` 进 search/extract 格式面（原文直读进分节管线，节语义与 anydoc 家族一致，批量目录搜索自动覆盖）；新子命令 `reader query <文件> <mq表达式>` 嵌 mq-lang 全引擎（学习 harehare/mq），全格式面转 markdown 后结构化提取（`.h2` / `.code` / `.link` / `.table` / select 管道），退出码 0/1/2 同 search，json 形态 `results[]` 加 `count`。
- **self update（P0015）**：`reader self update [--force]` 从 GitHub Releases 最新正式版自升级：版本判新（`--force` 同版本重装）、资产 sha256 digest 钉死校验（缺失拒升）、staged 加 rename 原子替换当前运行二进制与同目录兄弟（reader/rr 双名）；`GH_TOKEN` 注入认证、403 限流回退 gh api。只 stable 通道，不自动更新。
- **OCR 兜底（P0014）**：`extract` 与 `search` 对 PDF 单文件的 needs_ocr 页加 `--ocr` 兜底识别（hayro 渲染加 tract 推理 PP-OCRv5 mobile，vendored pure-onnx-ocr 修 max_width）；OCR 文本回填 lines、`needs_ocr` 标记保留（mobile 模型有系统性掉字）；模型三件约 20.5MB 首用从 ModelScope 下载进平台缓存目录、SHA-256 钉死校验，`--offline` 禁下载，`READER_OCR_CACHE_DIR` 覆盖缓存目录；目录批量搜索加 `--ocr` 报错。默认行为零变化；二进制体积 7.3MB 涨到 32.9MB（release）。

## [0.2.1] - 2026-09-01

> 三平台实机验收轮（R004 Linux、R005 mac）：分片与批量搜索落地、SIGPIPE 修复、musl 静态资产首发。

- **Unix 管道截断行为修复（M007）**：Linux/macOS 上输出接 `| head` 等早退管道时由 panic（exit 101 + stderr 栈）改为按 Unix 惯例死于 SIGPIPE（shell 报 141、零输出噪音，同 grep/rg）；Windows 行为零变化。Linux 实机验收（R004）发现并当轮修复，回归测试入集成面。
- **批量目录搜索（P0012）**：`reader search <目录> <关键词>` 递归批量搜支持格式（路径排序稳定）；text 命中行 `路径:单元:行号:文本`，json `hits[]` 带 `file` 字段加 `files.scanned / files.skipped` 统计；坏文件 stderr 跳过后继续，目录无支持格式文件退出 2；`--pages` 目录下不可用。单文件模式输出与退出码零变化。
- **超长节再分片（P0011）**：有标题文档中超过 200 行的节按同预算切为 `part` 单元，单元号跨 kind 全局连续；短节与无标题文档行为不变（P0010 口径）。
- **musl 静态 Linux 资产（P0013）**：release 流水线新增 `x86_64-unknown-linux-musl` 目标（ubuntu runner 加 musl-tools 交叉构建），资产 `reader-v<版本>-x86_64-unknown-linux-musl.tar.gz`；随下版 tag 首发验证。
- **无标题长文档行分片（P0010）**：全文无顶层标题的文档按 200 行预算分片为 `part` 单元（节头 `== part N ==`），`--pages` 与 `--offset/--limit` 分页恢复可用；有标题文档的 `section` 行为零变化。CSV 等无标题小文档的单元标签由 `section` 改为 `part`（行为变化，1 个既有用例随行为更新）。

## [0.2.0] - 2026-09-01

> anydoc 统一文档引擎大重构（P0009）：格式面 2 种扩到 14 种。**破坏性变更**：EPUB 单元由 spine 章（`== chapter N ==`）改为标题节（`== section N ==`）。

- **统一文档引擎（破坏性变更）**：Word（含 legacy .doc 直读）/ EPUB / ODT / RTF / PowerPoint / Excel / ODF / CSV 统一走 anydoc 0.2.4（firecrawl，MIT）出 GFM markdown，按顶层标题分节；PDF 保持 pdf-inspector 直连（页契约、页级 needs_ocr 原样，与 anydoc 自身对 PDF 的架构一致）。选型双通道核实与保真实测记 `docs\research\S004-Word文档读取选型-docx自解与doc直读双路线实测.md`（含决策变更记录），方案 `docs\proven\P0009-anydoc统一文档引擎大重构.md`。
- **依赖树**：`rbook` 与 `quick-xml` 退出主依赖（rbook 转 dev-dependency 造 EPUB 夹具；`zip` 进 dev-dependency 造 docx 夹具）。
- **已知限制**：无标题长文档整篇一节（`--pages` 不可细分；行式搜索与命中行格式不受影响）。

## [0.1.0] - 2026-08-31

> 首个发布：文档地基、最小闭环与 Agent 原生输出面整体落地；GitHub Release 出三端预编译二进制（P0008）。

### 里程碑

- **项目定位**：Reader，Agent 原生文档阅读、搜索和提取工具。定位变更方案 `docs\proven\P0002-项目重新定位-Agent原生文档阅读搜索和提取工具.md`；首期切面 `docs\proven\P0001-PDF文本搜索与提取CLI最小闭环.md`。
- **文档骨架**：对照 `D:\ohmyagents` 的四段 AGENTS、三原语、docs 六目录、`.tools` 三件套。
- **CLI 名**：二进制 `reader` 与 `rr`（同入口双 bin）；项目名 `reader_rs`。
- **提取引擎**：firecrawl/pdf-inspector 1.17.0（纯 Rust 默认构建，lopdf 解析；选型 `docs\research\S001-PDF文本提取crate选型-pdf-inspector双通道核实.md`）。
- **最小闭环**：`reader search` / `reader extract` 落地；门禁三件与 rumdl 三件套全绿。
- **EPUB 支持**：格式分派（`TextUnit` 统一页/章），rbook 加 quick-xml 提取；cargo test 20 过；真实样本回归。方案 `docs\proven\P0003-EPUB支持与格式分派.md`，选型 `docs\research\S003-EPUB解析crate选型-rbook双通道核实.md`。
- **跨平台接管**：GitHub Actions 三系统门禁（windows/ubuntu/macos 跑 fmt/clippy/test --locked）与 LF 钉死。方案 `docs\proven\P0004-mac与Linux接管开发与跨平台兼容.md`。
- **提取质量（破坏性变更）**：PDF 通道整体切 pdf-inspector markdown 布局管线：多栏阅读序、needs_ocr 检出提示（extract 页节提示行、search stderr 警示）、逐页分节补齐；PDF 输出行带 markdown 语法（标题、链接、表格），v0.1 朴素行重建退役。质量承诺面向英文与中文。方案 `docs\proven\P0005-PDF提取质量-markdown管线与needs_ocr提示.md`。
- **Agent 原生输出（P0006）**：`--format json` 包膜（`{ok,data,error}` 加 meta；错误 stdout 包膜加 stderr 人读行，退出码 0/1/2 不变）、extract `--offset/--limit` 分页（meta 带 `next_offset` 与 `cta`）、`--filter` 点路径裁剪；serde / serde_json 进依赖。方案 `docs\proven\P0006-输出形态-json包膜与分页裁剪.md`。
- **Agent 自省与发现（P0007）**：`reader --llms` 紧凑命令索引、`reader skill` 生成 SKILL.md（仓根提交，漂移双守卫：clap 命令树旗标全覆盖、仓内文件与运行时输出逐字节一致）、search/extract help 补 examples 节；零新依赖。方案 `docs\proven\P0007-Agent自省与发现-llms索引SKILL生成与help示例.md`。
