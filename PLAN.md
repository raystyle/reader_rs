# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**——基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：Agent 自省与发现——llms 索引、SKILL 生成与 help 示例

> 对应 `GOAL.md`，方案 `docs\proven\P0007-Agent自省与发现-llms索引SKILL生成与help示例.md`，登记日 2026-08-31。

### 1. 闸门

S002 落地映射 4、5 已设计到可落地程度（`--llms` 紧凑索引、SKILL.md 生成、help examples 节）；零新依赖（clap `after_long_help` 与 `CommandFactory` 现成），选型免另立研究（P0007 方案 1 条）。

### 2. 实施件与依据

| 件 | 要做什么 | 依据 |
| --- | --- | --- |
| `src\introspect.rs`（新） | `llms_text()` 紧凑索引；`skill_md()` 生成 SKILL.md（frontmatter 加使用契约，版本取 `env!("CARGO_PKG_VERSION")`） | P0007 方案 1 条；S002 模块表 skill 行 |
| `src\lib.rs` | 顶层 `--llms` 旗标与 `skill` 子命令（`command` 改 `Option<Commands>`，`--llms` 优先分派）；search/extract 加 `after_long_help` examples 节 | P0007 方案 2 条；S002 结论 7（examples 是 agent 读帮助的关键节） |
| `SKILL.md`（仓根，新） | 由 `reader skill > SKILL.md` 生成提交 | P0007 方案 4 条；用户「命令集成 SKILL」 |
| `tests\cli.rs` | 正例（llms/skill 内容断言）加双漂移守卫（clap 树 long 旗标全覆盖、仓根 SKILL.md 与运行时输出逐字节一致）；既有回归 | `docs\references\R003-测试标准细则-分层断言与门禁流程.md` |

### 3. 每件验收

门禁三件全绿；`--llms`/`skill` 退出 0 且输出覆盖全部旗标；裸 `reader` 仍出帮助；search/extract 帮助含 examples 节；退出码 0/1/2 语义不变。失败当场记 `docs\mistakes\`。验收通用口径见 G003 第四节。

### 4. 边界

不动 search/extract 既有输出与参数语义；不做命令树全自动生成与 SHA-256 过期检测；不做 MCP/TOON/token 计数/pager/completions。[依据: P0007 非目标节]

## 完成的定义

> 本目标验收口径。

- TODO 表全部已完成或明确跳过（跳过须写 mistakes 原因）
- `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked` 三件过
- `rumdl check .` 尽量零告警；P0007 与 INDEX 已登记；CHANGELOG 记新能力
- 研究与测试文档的关键结论标六态（AGENTS 写研究与测试文档规则）
