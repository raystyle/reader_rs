# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

Agent 自省与发现：llms 索引、SKILL 生成与 help 示例（对应 `GOAL.md`，方案 P0007，登记日 2026-08-31）。**2026-08-31 达成**：39 测全绿。阶段 3 余下候选（MCP、TOON、token 计数、pager、completions、批量目录）待立项。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| 立项 P0007 | 已完成 | `docs\proven\P0007-Agent自省与发现-llms索引SKILL生成与help示例.md`；三原语登记 | 2026-08-31 |
| introspect 模块与 CLI 接线 | 已完成 | `src\introspect.rs` 的 `llms_text()`/`skill_md()`；顶层 `--llms` 旗标与 `skill` 子命令；help examples 节；裸 reader 语义保持 | 2026-08-31 |
| 测试补强 | 已完成 | 正例 3 例加双漂移守卫（clap 树旗标覆盖、仓根 SKILL.md 逐字节一致）；39 测全绿 | 2026-08-31 |
| 仓根 SKILL.md 生成提交 | 已完成 | `reader skill > SKILL.md`；与运行时输出逐字节一致 | 2026-08-31 |
| 收官登记 | 已完成 | P0007 回填、README Agent 发现节、AGENTS 意图路由、INDEX、CHANGELOG、diary、门禁与提交 | 2026-08-31 |
