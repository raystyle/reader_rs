# Reader

> Reader（命令 `reader`，等价缩写 `rr`，同一二进制双名）：Agent 原生文档阅读、搜索和提取工具。为 Agent 管线设计的 Rust 单二进制 CLI：从本地 PDF、markdown、图片（png / jpg 等 8 种扩展名）与 anydoc 家族（Word 含 legacy .doc、EPUB、ODT、RTF、Office、CSV 等 14 种格式）读文本层，按页/节读、字面与正则搜、目录批量搜、mq 结构化提取（query）、OCR 兜底识图（`--ocr`，PP-OCRv6）、图片本体导出与一键完整提取（figures / export）。输出稳定可解析：行式标记、grep 语义退出码 0/1/2、`--format json` 包膜加 `--filter` 裁剪；机器可读优先于人类美观，单调用完成一件事，无交互无守护进程，错误走 stderr。质量承诺面向英文与中文；只读不渲染不编辑；图表模型理解归调用方多模态侧（S010 T3 已拒）。命令契约以 `--llms` 与 `--help` 为准（`--llms` 即 agent 说明书，漂移守卫兜底）。

## Commands

- `cargo fmt --all -- --check` 加 `cargo clippy --all-targets -- -D warnings` 提交前必跑（CI 三系统同口径；missing_docs 是 deny，缺 `///` 即红）
- `cargo test --locked` 全量测试（单元加集成，六层体系见 G006）
- `cargo test --doc` doctest 示例冒烟（纯函数面在册示例）
- 文档门禁四件：`rumdl check .` 加 `uv run --script .tools/md-char-scan.py` 加 `uv run --script .tools/md-heading-scan.py` 加 `uv run --script .tools/md-ref-scan.py`（文档结构变更末件必跑）
- `cargo aidoc` 生成库面投影进 `docs/aidoc/`；`cargo aidoc --check --strict` 投影漂移门禁（改 pub 项或 `///` 后先 `cargo aidoc` 再同一次提交 docs/aidoc；tool-rust 强制口径：Rust 栈 aidoc 投影强制化，bin-only 不豁免，受众是维护者与 agent，ADR-0005）
- `PEVO_CHECK_ALLOW='^docs/aidoc/'` 加 `uv run /home/ray/repos/ProjectEvo/plugins/project-evo/skills/dev-evo/scripts/check.py .` 骨架合规自检（豁免正则在册：aidoc 条目分隔符 em dash 是渲染格式无开关，漂移真门禁是 cargo aidoc --check --strict；机制见 docs/README 存量禁字债节）
- `cargo build --release --locked` 发布构建
- 公开契约双面：CLI 面走 `--llms`（curated agent 说明书）加 tests/ 集成测试（clap 命令树旗标全覆盖 `--llms` 断言即漂移门禁）；库面走 `///` 与类型签名加 `docs/aidoc/` 投影（missing_docs deny 强制）

## Must

- 改 pub 项：同步 `///` 与 doctest（missing_docs 是 deny，CI 必红），并 `cargo aidoc` 后与 `docs/aidoc/` 同一次提交
- 契约注释守格式纪律（dev-evo 第六十批，tool-rust 与 base-projection 契约注释通用准则）：首句成句（做什么加何时用加边界，不以项名开头，细节隔空行）；返 `Result` 必 `# Errors`、可能 panic 必 `# Panics`、unsafe 项必 `# Safety` 列全 UB 前置（clippy missing_errors_doc 与 missing_panics_doc 与 missing_safety_doc 已开）；示例断言收尾，不执行块显式标注并注明原因
- 命令面改动三处同步：README、`src/introspect.rs` curated 文本、`--help`（漂移守卫集成测试与 `--llms` 快照兜底）
- 不可逆技术选择先立 `docs/adr/`；新需求先立 `docs/requirements/` REQ 再实现，实现后回填 trace（编号 D 号即 REQ 号口径：PRD 存量 D01 至 D47 留档，活跃队列已按 D 号转登记，新需求自 REQ-050 接编）
- 事实性断言标六态（`[实证]` 至 `[直觉]`，规范见 G002）；实证滥用即未完成
- 踩坑当场记 `docs/diary/`（过程留痕）或立 ADR（被否决的选择也是决策）；`docs/mistakes/` M 编号体系留档不再接编
- 一事一提交（feat/docs/fix/chore/test 前缀加中文描述）；每次提交 diary 当天记钩子
- 版本分支模型（D45）：版本工作落 `dev/v<版本>` 分支承载与验收，全绿后 fast-forward 合并 main 打 tag 发布；main 唯一发版源，发版窗口冻结
- 版本载体唯一权威 `Cargo.toml`（单包 version 一处，flow-release 第七节）：Cargo.lock 是生成投影随封版重生，CHANGELOG 是历史记录非载体；载体外版本号即第二真相，发现即清理；semver 判据（文档批与修复取 patch、能力新增取 minor、契约破裂取 major）写进封版 REQ 不凭感觉

## Must not

- 手改生成物（`--llms` 的 curated 输出面、`docs/aidoc/` 整目录）
- 另写第二真相（测试规范唯一权威在 G005/G006，封版流程在 R008，选型细则在 R002；库 API 叙述以 `///` 为准不另写 API.md）
- emoji、破折号、Unicode 箭头、智能引号、全角字母数字（G004 四类禁字，豁免区外零容忍；`docs/aidoc/` 渲染格式走在册豁免）
- 未经指示推远端或做 git 变更；在 main 上直接开发版本工作或打未经合并的 tag
- Windows 侧默认 powershell.exe 5.1；sed 批改中文与反斜杠路径（用 `.tools/md-replace.py`）

## Read first

- 库 API 面：`docs/aidoc/llms.txt`（Agent 入口索引）到 `docs/aidoc/reader_rs/<模块>.md`，仍不确定再开源码 `///`
- 文档地图：`docs/README.md`（全仓索引，承接旧 INDEX 职责）；需求与队列：`docs/requirements/README.md`；架构决策：`docs/adr/README.md`
- 做事的流程：`docs/references/`（R002 选型双通道、R007 五步工作流、R008 封版发布）；为什么：`docs/research/S00x`
- 规范与禁令：`docs/guide/`（G001 命名写作、G002 六态、G004 禁字、G005/G006 测试、G007 工程基线、G008 README 规范）
- 代码定位：`INDEX.md` 代码文件位置表（迁移期保留仍有效）加 `rg` / `ast-grep`；旧三节协作规则全文：`docs/guides/agents-legacy-three-sections.md`

## 环境

- 三平台矩阵（Windows / Linux / macOS，CI 三系统含 dev/** 推送触发）；Windows 优先验证
- 全平台测试基建（原语五端4机，ohmycloud 总台周知，用户宣言 2026-09-16）：WSL 总台加 lan-win（Windows 宿主，与 WSL 同机两面）加 lan-ubuntu（Linux NUC，全运行时）加 lan-linux（Linux server）加 lan-mac（macOS arm64），五端跨四机均可跑本仓验收，lan-linux2 不在矩阵；实机清单 R004（Linux 面，lan-ubuntu 与 lan-linux 同适用）与 R005（mac 面）；验收支撑按需向 ohmycloud 总台要端点，回执纪律照旧（conclusion 自取）
- 连接姿势（env-platform 第十节口径）：WSL 到宿主恒走 `127.0.0.1` 回环 ssh 加 interop 直调（powershell.exe），不走宿主 mesh IP（mirrored 网络共身份自连 RST 属结构性）；lan 三端 mesh 地址随时随地；连接问题先查姿势再查配置。多仓派单与回执走 herdr 飞轮协议（flow-flywheel.md，本仓工位实践即实证源）
- 验收与运维脚本统一载体 pwsh（五端 7.6.6 在位，env-platform 第十一节）：新增验收与运维面脚本一律 pwsh 一份，不再各写 bash 加 cmd 加 zsh；既有 `.tools` PEP 723 Python 脚本（uv 运行时）按标准不强制迁移
- 本仓开发侧 WSL `~/repos/reader_rs`（Windows 盘 `D:\reader_rs`）；md 与 Rust 源 UTF-8；Windows 兼容 5.1 的脚本带 UTF-8 BOM
- 发布走 R008：tag 触发 release.yml（五平台资产加 sha256 边车）加镜像腿（mirror job 与 mirror-models.yml 周更）；模型与升级镜像 `reader.ohmygh.com` 归 ohmycloud 承载
- dev-evo 标准权威在 `~/repos/ProjectEvo`（本仓文档体系 2026-09-16 全量迁移自旧四原语体系，映射口径见 docs/README）
