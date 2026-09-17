# reader_rs::introspect

Agent 自省与发现（P0007；三面统一批 REQ-057 对齐总台 REQ-060）：`--llms` 旗标
裸出 markdown 紧凑手册（命令表自活 clap 命令树渲染，零手维护双份；curated 段
承载退出码、行式契约与 env 等 clap 不知道的语义），`--llms --json` 出机器形态。
漂移由 tests/cli.rs 守卫兜底：clap 命令树旗标全覆盖 `--llms` 输出断言。

## Functions

- `llms_json` — `reader --llms --json`：机器形态（结构对齐族标准：name、version、description、
- `llms_text` — `reader --llms`：markdown 紧凑手册（命令表自活命令树渲染；总长目标至多 120 行）。

