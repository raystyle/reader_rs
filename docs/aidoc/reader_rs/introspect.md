# reader_rs::introspect

Agent 自省与发现（P0007）：`--llms` 紧凑索引，agent 说明书唯一面。
文本为 curated 内容（含退出码、输出契约等 clap 不知道的语义）；
漂移由 tests\cli.rs 守卫兜底：clap 命令树旗标全覆盖断言（skill 长形态与仓根
SKILL.md 已于 2026-09-16 用户裁定退役，`--help` 与 README 承接渐进深入）。

## Functions

- `llms_text` — `reader --llms`：紧凑命令索引（agent 发现用，单行一句、稳定可解析）。

