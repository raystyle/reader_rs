# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

D31 测试程序载体裁定（登记日 2026-09-03；依据 `PRD.md`）。**2026-09-03 已达成**（第 2 轮裁定）：A/B 跑批用 uv 运行时 Python（`.tools\ab_run.py`），冒烟/回归/验收归 cargo test 体系（`tests\smoke.rs` 3 测 / `regress.rs` 4 测 / `accept.rs` 5 测全绿，独立 test target 调度）；G006 载体规则与基线归因修正（25 命中属 CLR 书）。同日先行达成 D28 / D29 / D30（六层规范 G006、tests\ab\ A/B 层、poc\ 目录）与归类修正（references 流程 / guide 规范，三件搬家）。下一目标待用户点名（候选见 `PRD.md` D23 至 D26）。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| D31 三层归 cargo（第 2 轮裁定） | 已完成 | smoke.rs 3 测、regress.rs 4 测；uv 只留 A/B | 2026-09-03 |
| D34 回归层增强 | 已完成 | insta 快照 3 枚人工审入库；proptest 页范围 3 属性；trybuild 不适用 | 2026-09-03 |
| D35 工程基线供稿消化 | 已完成 | G007 四态裁定（已符合 6 / 已落地 1 / 候选 4 / 不适用 3）；release strip 加 thin LTO | 2026-09-03 |
| D33 验收层 BDD 化 | 已完成 | cucumber 0.23 加 futures；accept.feature 8 场景 21 步全绿；harness=false；testcontainers 裁定不适用 | 2026-09-03 |
| G006 载体规则与基线修正 | 已完成 | uv 运行时 Python 规则；CLR 书 25 命中归因修正、399 页标记实测 | 2026-09-03 |
| 归类修正（references 流程 / guide 规范） | 已完成 | R003 改 G005、R006 改 G006、G003 改 R007；56 处引用替换 | 2026-09-03 |
| D28-D30 落地 | 已完成 | G006 六层规范；tests\ab\ A/B 首跑；poc\ 目录与 S006 迁移 | 2026-09-03 |
| 同步与门禁 | 已完成 | INDEX、AGENTS、PRD、GOAL/diary；门禁全绿 | 2026-09-03 |
