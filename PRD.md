# PRD：需求清单管理

> 角色：**需求清单**，四原语之首：需求驱动目标。GOAL 是理解 PRD 后定下的目标和达成标准，GOAL 的每个目标应能回指本清单条目；条目经「追问链」人机交互逐条澄清后登记，禁止静默假设。
> 与其它原语分工：`PRD.md` = 需求清单（要什么）；`GOAL.md` = 理解 PRD 后定下的目标和达成标准（要达成什么）；`PLAN.md` = 规划（怎么做）；`TODO.md` = 进度（做到哪）。

## 生命周期

```text
新需求 到 待澄清 到 已澄清 到 已采纳 到 已交付
拒绝路径：任一状态 到 已拒绝（记原因防复问）
```

## 需求清单

> 一条需求一行：编号接编 D 加两位；「派生去向」回指 GOAL 锚点 / PLAN 切片 / P 编号 / S 编号。
> D01 至 D20 为 2026-09-03 追溯登记：需求措辞依据 GOAL 时间线、docs\diary\ 与会话记录重建 [推断]，澄清轮次记为第 0 轮（用户点名即定）。

| 编号 | 需求 | 状态 | 澄清轮次 | 派生去向 |
| --- | --- | --- | --- | --- |
| D01 | 对照 ohmyagents 建立项目结构与文档体系（四段 AGENTS、原语、docs 六目录、rumdl 门禁） | 已交付 | 第 0 轮 | 文档骨架；diary 2026-08-31 |
| D02 | PDF 文本搜索与提取 CLI 最小闭环（search / extract、reader 与 rr 双 bin） | 已交付 | 第 0 轮 | P0001；S001 选型 |
| D03 | 重新定位为 Agent 原生文档阅读、搜索和提取工具 | 已交付 | 第 0 轮 | P0002；R001 |
| D04 | EPUB 支持 | 已交付 | 第 0 轮 | P0003；S003 |
| D05 | mac 与 Linux 接管开发，CI 三系统门禁 | 已交付 | 第 0 轮 | P0004；R004 / R005 验收清单 |
| D06 | PDF 提取质量：markdown 布局管线（多栏阅读序）、needs_ocr 检出提示 | 已交付 | 第 0 轮 | P0005 |
| D07 | 输出契约：`--format json` 包膜、extract `--offset/--limit` 分页带 next_offset 与 cta、`--filter` 点路径裁剪 | 已交付 | 第 0 轮 | P0006 |
| D08 | Agent 自省与发现：`--llms` 紧凑索引、`skill` 生成 SKILL.md、help examples | 已交付 | 第 0 轮 | P0007；S002 |
| D09 | 封版 v0.1.0，git tag 触发三端二进制 release 流水线 | 已交付 | 第 0 轮 | P0008 |
| D10 | anydoc 统一文档引擎，格式面扩到 Word（含 legacy .doc）/ EPUB / ODT / RTF / Office / CSV 等 14 种 | 已交付 | 第 1 轮（S004 决策变更：用户推翻 docx 自解初判） | P0009；S004 |
| D11 | 无标题长文档按 200 行分片为 part 单元 | 已交付 | 第 0 轮 | P0010 |
| D12 | 超长节再分片 part（单元号跨 kind 连续） | 已交付 | 第 0 轮 | P0011 |
| D13 | 目录批量搜索（递归、命中行带路径前缀） | 已交付 | 第 0 轮 | P0012 |
| D14 | musl 静态 Linux 资产进 release 矩阵 | 已交付 | 第 0 轮 | P0013 |
| D15 | TOON 输出形态 | 已拒绝 | 第 0 轮 | S005：三真样本双编码器实测收益为负或小于 2%，且 0.5.0 往返破损；防复问 |
| D16 | OCR 兜底：`--ocr` 对 PDF needs_ocr 页识别，模型首用下载加缓存，`--offline` 禁下载 | 已交付 | 第 1 轮（S006 选型后用户点名落地） | P0014；S006 |
| D17 | self update：判断自身路径，GitHub Releases 最新正式版判新加原子替换自身与 rr 兄弟 | 已交付 | 第 0 轮 | P0015 |
| D18 | markdown 进 search / extract 格式面；学习 mq 加结构化提取 query 子命令 | 已交付 | 第 0 轮 | P0016；S007 |
| D19 | OCR 提速：全页 15 至 30 分钟不可接受，目标 5 至 10 秒一页；速度性能按系统核数自适应（空闲核多极限策略、核少平衡策略） | 已交付 | 第 1 轮（用户给验收口径） | P0017（达成 3 至 5.5 秒每页） |
| D20 | SKILL 重构为常用例子加输出契约，扩展用法渐进引导命令行 `--help` | 已交付 | 第 1 轮（用户裁定） | P0017 期间落地（e73e7bb） |
| D21 | OCR 质量升级：mobile 掉字修复，同页四配置对比选型 | 已交付 | 第 1 轮（用户点名 OCR 质量任务） | P0018；S008（v6 tiny 双优胜出） |
| D22 | 中英文 Markdown 技术文档写作规范转为项目规范，存量渐进清理 | 已交付 | 第 0 轮 | G004；存量清零（be59688） |
| D23 | MCP 服务能力 | 待澄清 | 未发起 | 候选方向 |
| D24 | 分发面扩展：crates.io / brew / scoop | 待澄清 | 未发起 | 候选方向 |
| D25 | OCR v6 small 质量档旗标（更干净输出，3.2s/页量级） | 待澄清 | 未发起 | S008 留档候选；A/B 首跑证据 tests\ab\reports\2026-09-03-tiny-vs-small.md（合成样本 small 4/5 对 tiny 1/5，真样本 51 对 37 行）；README 已说明 env 档位开关（2026-09-03） |
| D26 | query 边界扩展（目录输入、更多输出形态） | 待澄清 | 未发起 | 候选方向 |
| D27 | 学习 pve-harness PRD 做项目自省：需求层独立成第四原语，需求驱动目标 | 已交付 | 第 1 轮（用户逐条定四原语分工：PRD 需求清单、GOAL 目标与达成标准、PLAN 规划、TODO 进度） | 本文件；AGENTS / INDEX / R007 同步 |
| D28 | 测试体系定标准规范：单元、集成、冒烟、回归、验收、A/B 六层各定落点与口径 | 已交付 | 第 1 轮（用户点名六层） | G006 |
| D29 | 补齐 A/B 测试目录，翻入可供测试验证的对象资源，跑质量与性能 A/B 对比 | 已交付 | 第 1 轮（用户点名） | tests\ab\；`.tools\ab_run.py`；首跑报告 tests\ab\reports\2026-09-03-tiny-vs-small.md |
| D30 | 研究产物是 PoC 原型，对应 poc 目录：规范落点并迁移存量 PoC | 已交付 | 第 1 轮（用户点名） | `poc\`；G002 八节 |
| D31 | 测试程序载体裁定：A/B 跑批用 uv 运行时 Python 脚本（PEP 723 单文件），冒烟/回归/验收归 cargo test 体系（独立 test target） | 已交付 | 第 2 轮（第 1 轮四层全 uv，用户裁定收拢为 A/B 独占 uv、其余归 cargo） | tests\smoke.rs / regress.rs / accept.rs；`.tools\ab_run.py`；G006 载体规则 |
| D32 | AGENTS 自省调整各目录和文档职责、规范、规则和意图路由；采纳 Rust 测试框架供稿（原生三层单元/集成/文档测试为地基，冒烟/回归/验收/A/B 为目的流程层靠约定拼装） | 已交付 | 第 1 轮（用户供稿加点名） | AGENTS 一/二/四节；G006 一节框架修正 |
| D33 | 验收层 BDD 化：cucumber Gherkin 场景驱动（testcontainers-rs 裁定不适用：纯 CLI 无可容器化依赖） | 已交付 | 第 1 轮（用户供稿点名 cucumber） | tests\features\accept.feature 8 场景加 tests\accept.rs 步骤绑定 |
| D34 | 回归层增强：insta 快照加 proptest 属性（trybuild 裁定不适用：公开面是 CLI 不是库 API，无编译期契约要守） | 已交付 | 第 1 轮（用户供稿点名） | tests\snapshot.rs 3 快照加 tests\snapshots\；src\lib.rs 页范围 3 属性 |
| D35 | Rust CLI 工程基线供稿消化：逐项裁定（已符合/已落地/候选/不适用）落 G007；release profile strip 加 thin LTO 即落 | 已交付 | 第 1 轮（用户供稿） | G007；Cargo.toml profile.release |
| D36 | 错误与诊断体系：库内 thiserror、边界 anyhow 加 cause chain、tracing 结构化诊断 | 待澄清 | 未发起 | G007 一节；触发：公开库 API 或错误分类需求 |
| D37 | 发布面扩展：aarch64-linux-musl 资产、cargo-dist 安装器（brew/winget/deb） | 待澄清 | 未发起 | G007 二节；触发：ARM Linux 用户或安装器需求 |
| D38 | trycmd 跑帮助与文档示例测试 | 待澄清 | 未发起 | G007 三节；触发：README 示例数量上来后 |
| D39 | 工程效率件：edition 2024 加 rust-toolchain.toml 钉死、nextest 运行器、llvm-cov 覆盖率、cargo deny/audit 供应链闸 | 待澄清 | 未发起 | G007 一/三节；触发：CI 门禁扩展轮 |
| D40 | README 规范供稿消化：定 G008（结构顺序、About 一致性、写作增量、反模式、同步义务）并按此整改 README | 已交付 | 第 4 轮（About 纯中文；部署首要；self update 匿名即可用；开头速览化；模型档位、来源与手动部署说清；致谢节补齐） | G008；README 整改（部署集群提升、License 置末、速览化、模型三问、致谢） |
| D41 | 封版发布流程确定：先本地全平台编译、全平台测试验收，后封版触发 GitHub Action 发布 release | 已交付 | 第 0 轮（用户点名流程骨架） | R008；v0.4.0 首轮执行 |
| D42 | OCR 模型分发自维护：不再依赖 HuggingFace 直连（国内机器首用下载不可达） | 已采纳 | 第 3 轮（第 1 轮用户裁定自维护分发；第 2 轮 ISSUE #1 载体裁决：reader.ohmygh.com 镜像默认、镜像 到 HF 直连 到 GitHub Releases 模型 tag 三级回退、self update 先读 latest.json、minisign 首轮不上；第 3 轮用户点名 ocr init / doctor / switch 三件套：下载、诊断、切换两档） | 证据：v0.4.0 封版验收 lan-linux 首用下载 tiny-rec 三连重试失败、scp 手动放置后 `--offline` 全通（2026-09-03）；裁决与基建回执 ISSUE #1；立项见 GOAL 锚点与 PLAN 完成的定义 |

## 维护规则

- **登记**：新需求先入本清单（状态新需求），再走追问链澄清；澄清完转已澄清，采纳立项转已采纳，交付验收转已交付。
- **回指**：GOAL 锚点与 PLAN 立项须注明所服务的需求编号（如 D19）。
- **拒绝**：裁定不做的需求转已拒绝并记原因，防复问；实证依据挂 S 编号。
- **追溯**：历史需求补登记时措辞属重建，须按 G002 标六态。
