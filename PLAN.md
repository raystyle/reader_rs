# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**——基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：输出形态——json 包膜与分页裁剪

> 对应 `GOAL.md`，方案 `docs\proven\P0006-输出形态-json包膜与分页裁剪.md`，登记日 2026-08-31。

### 1. 闸门

S002 落地映射前三条已设计到可落地程度（包膜字段、分页原语、点路径裁剪）；serde / serde_json 为序列化事实标准，选型免另立研究（P0006 方案 1 条）。退出码与默认文本形态不动，R001 约束保持。

### 2. 实施件与依据

| 件 | 要做什么 | 依据 |
| --- | --- | --- |
| `Cargo.toml` | 加 `serde`（derive）与 `serde_json` | P0006 方案 1 条；R002 懒人阶梯 |
| `src\output.rs`（新） | `Envelope`/`Meta`（Serialize）、`duration_ms` 计时、`cta` 生成、`filter` 点路径求值器（键访问、`[]` 映射、`[N]` 下标；非法路径报错） | P0006 方案 2 条；S002 模块表 filter 行 |
| `src\lib.rs` | 两子命令 `--format <text|json>`；extract `--offset/--limit`（0 起单元位置序）；`--filter`（非 json 报错）；JSON 错误路径（stdout 包膜加 stderr 人读行，退出 2） | P0006 方案 3、4 条；R001 约束一、三 |
| `tests\cli.rs` | JSON 正例（serde_json 解析后按字段断言，期望值独立来源）、无命中包膜、错误包膜、分页 next_offset/cta、filter 正负例；既有回归 | `docs\references\R003-测试标准细则-分层断言与门禁流程.md` |

### 3. 每件验收

门禁三件全绿；包膜字段与方案 1 条清单一致；退出码 0/1/2 语义不变；filter 非法路径走错误包膜。失败当场记 `docs\mistakes\`。验收通用口径见 G003 第四节。

### 4. 边界

不改默认文本输出与既有参数语义；不做 TOON/多格式、token 计数、search 分页、agent 发现、MCP；filter 只做路径取值不做谓词。[依据: P0006 非目标节]

## 完成的定义

> 本目标验收口径。

- TODO 表全部已完成或明确跳过（跳过须写 mistakes 原因）
- `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked` 三件过
- `rumdl check .` 尽量零告警；P0006 与 INDEX 已登记；CHANGELOG 记新能力
- 研究与测试文档的关键结论标六态（AGENTS 写研究与测试文档规则）
