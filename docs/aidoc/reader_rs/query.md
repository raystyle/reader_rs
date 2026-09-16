# reader_rs::query

mq 结构化提取（P0016；学习 harehare/mq，选型 S007）：全部支持格式转 markdown 文本后
跑 mq 表达式（jq 风格节点选择器与管道）。引擎嵌 mq-lang 全量；非匹配节点产空渲染，
过滤空串即得干净结果（S007 PoC 实证）。

## Functions

- `run_query` — 跑 mq 表达式，返回非空渲染结果集（markdown 片段原文）。
- `to_markdown` — 任意支持格式转 markdown 文本,供 mq 表达式求值消费。

