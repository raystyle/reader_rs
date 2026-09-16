# reader_rs::introspect

Agent 自省与发现（P0007）：`--llms` 紧凑索引与 `skill` 子命令的 SKILL.md 生成。
文本为 curated 内容（含退出码、输出契约等 clap 不知道的语义）；
漂移由 tests\cli.rs 双守卫兜底：clap 命令树旗标全覆盖断言 + 仓根 SKILL.md 逐字节一致断言。

## Functions

- `llms_text` — `reader --llms`：紧凑命令索引（agent 发现用，单行一句、稳定可解析）。
- `skill_md` — `reader skill`：生成 SKILL.md（仓根提交同名文件，漂移由测试守卫）。

