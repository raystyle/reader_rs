# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**——基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：无标题分节（P0010）加 TOON 前置研究（S005）

> 用户点名两项按序推进，登记日 2026-09-01。

### 1. 闸门

P0009 已收官发布，分节限制是其在册已知限制；TOON 有 S002 遗留假设待验（中文 token 收益）。均无选型风险：前者纯实现，后者先研究后定。

### 2. 实施件与依据

| 件 | 要做什么 | 依据 |
| --- | --- | --- |
| `src\document.rs` | `UnitKind::Part`（label `part`） | P0010 方案节 |
| `src\anydoc.rs` | `markdown_to_units` 带出「有无顶层标题」信号；无标题走 `chunk_lines(200)` 分片；单元测试更新与新添 | P0010 方案节 |
| `tests\cli.rs` | 300 行 CSV 夹具：>= 2 part、part 2 含尾行、`--pages 2` 只出 part 2；有标题 docx 仍 section | R003 现造夹具 |
| introspect / SKILL / README / CHANGELOG | 节头口径加 part；SKILL 再生成（bash 原始重定向） | P0007 守卫 |
| `docs\research\S005-*`（新） | toon-format 双通道核实（R002）；Reader 真样本（中英文）`--format json` 输出做 JSON vs TOON 字节与 token 对照；结论按六态落档 | S002 待办 3 |

### 3. 每件验收

门禁三件加 rumdl 三件套全绿；既有用例零改动全绿；新用例过；真样本回归（渗透方案 docx 180 行单 part、测试V2.docx 仍 15 section）。S005 结论附 PoC 证据，收益不成立则明确「不引入」并销候选。验收通用口径见 G003 第四节。

### 4. 边界

不做行预算旗标；不做有标题单节超长再分片；不做语义分节；S005 只研究不实现 `--format toon`（收益成立另立项）。[依据: P0010 非目标节]

## 完成的定义

> 本目标验收口径。

- TODO 表全部已完成或明确跳过（跳过须写 mistakes 原因）
- `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked` 三件过
- `rumdl check .` 尽量零告警；P0010 与 S005 已登记 INDEX；CHANGELOG Unreleased 记 part 行为变化
- 研究与测试文档的关键结论标六态（AGENTS 写研究与测试文档规则）
