# P0006-输出形态-json包膜与分页裁剪

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：进行中
- 日期：2026-08-31
- 关联：TODO.md 当前目标 / `docs\research\S002-incurs模块经验研究-Agent原生CLI的命令输出与帮助设计.md`（落地映射 1-3 条）/ ROADMAP 阶段 3 / R001 设计约束一、三

## 背景与问题

当前 search/extract 只有行式文本输出。人对 grep 形态够用，Agent 拿行式要做二次解析：命中结构（页、行、文本、上下文）与不可靠页信号都埋在文本里。S002 研究结论：输出包膜（`{ok,data,error}` 加 meta）、分页原语（offset/limit 加 next_offset 与 cta）、点路径裁剪（filter）是 Agent 原生输出的三件套，均已设计到可落地程度。

## 目标与非目标

- 目标：
  - `--format json`（search / extract）：包膜输出 `{ok, data, meta}`；错误也是包膜 `{ok:false, error, meta}`（stdout 给 JSON，stderr 保留人读行）。
  - 退出码不变：命中 0 / 无命中 1 / 出错 2；`ok` 表执行成败，无命中是 `ok:true` 加空 `hits`。
  - extract 分页：`--offset N`（0 起，跳过前 N 个单元）与 `--limit M`，文本与 JSON 两形态同用；JSON meta 带 `next_offset` 与 `cta`（下一页建议命令），无更多页时不带。
  - `--filter`（仅 `--format json` 下合法）：点路径裁剪 `data` 树，支持键访问、`[]` 数组映射（`hits[].text`）、`[N]` 下标。
  - search JSON 的 `data`：`hits[]`（`unit` / `line` / `text` / `before[]` / `after[]`）加 `needs_ocr_units[]`；extract JSON 的 `data`：`units[]`（`kind` / `no` / `needs_ocr` / `lines[]`）。
  - meta：`command` 与 `duration_ms` 两稳定字段起步。
- 非目标：
  - 不改默认行式文本输出与既有参数语义（`--format` 缺省 text）。
  - 不做 TOON / YAML / CSV 等多格式串行化（S002 映射注明按需）。
  - 不做 token 计数（引 tiktoken-rs 前先用真实输出验证收益，S002 待办 3）。
  - 不做 search 分页（命中集分页与大结果集策略另立项观察）。
  - 不做 agent 发现（`--llms` / SKILL.md）与 MCP 暴露（S002 映射 4、5 条，后续候选）。
  - filter 不做通配、过滤谓词、递归下降，只做路径取值。

## 方案

1. 依赖：加 `serde`（derive）与 `serde_json`。选型免研究文档：crates.io 序列化事实标准、Reader 全生态（含 clap 传递树）通用 [实证: 2026-08-31 crates.io]，符合 R002 懒人阶梯「最流行稳定优先」。
2. `src\output.rs`（新）：包膜类型（serde Serialize；`Envelope<T>` 加 `Meta`）；`Instant` 计时装 `duration_ms`；`cta` 生成（extract 有 `next_offset` 时给下一条命令文本）；`filter` 点路径求值器（`Value` 递归取值，语法 `a.b`、`a[]`、`a[0]`，非法路径报错）。
3. `src\lib.rs`：两子命令加 `--format <text|json>`；extract 加 `--offset` / `--limit`；两子命令加 `--filter`（非 json 形态报错）。文本路径除分页外不动；JSON 路径构造 data 后经 filter 再包膜输出。错误路径：`--format json` 时 stdout 出错误包膜（stderr 人读行保留），退出码 2。
4. 分页语义：`--offset/--limit` 作用在单元列表（页/章）上，0 起位置序；`next_offset = offset + 实取单元数`，仅当还有剩余单元时出现在 meta。
5. 测试：JSON 各形态断言以 serde_json 解析后按字段取值（期望值来自写入文本，独立来源）；退出码与 stderr 断言沿用 predicates。

依据：S002 关键结论 2、3（包膜与 cta/next_offset 原语）、模块表 filter 行（点路径加数组切片）；R001 约束一（输出稳定可解析）与约束三（机器可读优先）。[推断: S002 落地映射 1-3]

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| 自实现 JSON 输出（手拼字符串） | 违反懒人阶梯；serde_json 转义（中文、引号）手拼必踩 |
| 引 incurs 用其 formatter | S002 结论 1：0.5.x 加 1 star 不进生产依赖 |
| 默认切 JSON、文本形态加旗标 | 破坏 v0.1 契约与既有调用方；S002 默认 TOON 的前提是 incurs 生态，Reader 缺省保守 |
| filter 做成 jq 子集 | 引 jaq 类依赖或自写解析器，超出一刀范围；YAGNI |

## 实施步骤

1. 本方案立项，三原语与 ROADMAP/INDEX 登记。
2. 依赖加 serde / serde_json；output.rs 包膜与 filter 落地。
3. lib.rs 参数接线与两条输出路径改造。
4. 测试补强与既有回归。
5. 真样本抽查（中英文、filter 组合、分页翻页）。
6. 收官登记（README JSON 形态与参数表、AGENTS 意图路由、CHANGELOG、diary）。

## 风险与回滚

- 风险：包膜字段名一旦有人依赖即成契约，后改名成本高。缓解：字段集最小起步（方案 1 条清单），README 定格式样例；后续加字段不删不改名。
- 风险：JSON 错误路径与 R001「错误走 stderr」表面冲突。缓解：stderr 人读行保留不动，stdout 包膜是补充通道；方案与 README 明示。
- 风险：filter 求值器边界（不存在路径、类型不符）。缓解：报错走错误包膜加退出 2，不静默空值。
- 回滚：git revert 单提交即回；无数据迁移。

## 实施过程与经验

> 完成时补全，不是留空。

- 实际怎么做（与计划偏差、关键决策点）：待回填。
- 踩了什么坑 + 怎么解决：待回填。
- 沉淀的经验：待回填。

## 验收标准

- search / extract `--format json` 输出可被 serde_json 解析，包膜字段符合方案；无命中 `ok:true` 加空 `hits`，退出码 1。[待验]
- 错误路径（缺文件、坏参数）：JSON 形态下 stdout 出 `{ok:false,error,meta}`，stderr 保留人读行，退出 2。[待验]
- extract `--offset/--limit` 两形态分页正确；JSON meta 的 `next_offset` 与 `cta` 仅在有剩余页时出现。[待验]
- `--filter hits[].text`、`units[0].lines` 式裁剪正确；`--filter` 无 `--format json` 报错退出 2；非法路径走错误包膜。[待验]
- 既有测试全绿；门禁三件与 rumdl 三件套过；真样本（中英文）抽查过。[待验]
- README / AGENTS / CHANGELOG / 三原语 / INDEX 登记完整。[待验]
