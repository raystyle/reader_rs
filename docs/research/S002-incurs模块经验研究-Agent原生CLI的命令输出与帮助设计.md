# S002-incurs模块经验研究-Agent原生CLI的命令输出与帮助设计

> 2026-08-31。触发点：用户指定研究 incurs（<https://docs.rs/incurs/latest/incurs/index.html>），吸收其模块经验构建 Reader 的命令、输出与帮助。流程按 R002 双通道；六态标准见 G002。本研究只产出设计取舍，不改代码。

## 背景

Reader 定位 Agent 原生（P0002），当前输出是行式标记文本。incurs 是「CLI framework for humans and AI agents」的 Rust 移植，模块表恰好覆盖我们要演进的方向（输出形态、agent 发现、MCP），值得拆解开看哪些经验可吸收。

## 关键结论

1. **不引为依赖，吸收设计**。incurs 本体信号弱：GitHub 1 star、2026-03 创建、0.5.x 早期版本；其设计源头 wevm/incur（TypeScript）有 605 stars 且活跃。按 R002 稳度四信号，0.5.x 加极低采用度不宜进生产依赖；设计模式自实现最小子集即可。[实证: 2026-08-31 gh repo view 两仓 + cargo info]
2. **输出包膜是最值得吸收的一条**：每条命令返回 `ExecuteResult`（`Ok{ok,data,cta}` / `Error{ok,error,cta,exit_code}`），外层 `OutputEnvelope{result, meta}`，meta 带 command、duration、cta、next_offset。错误也是结构化数据而不只是 stderr 文案。[实证: docs.rs output 模块 0.5.3]
3. **cta 与 next_offset 是 Agent 原生的精髓**：cta（call-to-action）在输出里直接给出下一步建议命令，Agent 可读着链式推进；next_offset 支撑大结果集分页。Reader 的大文档 extract 正需要这对原语。[实证: OutputMeta/CtaBlock 字段，docs.rs 0.5.3；价值判断为 [推断]]
4. **多格式输出的默认是 TOON 不是 JSON**：`Format` 七变体（Toon/Json/Yaml/Markdown/Jsonl/Table/Csv），formatter 把 serde_json::Value 串行化到目标格式，默认 TOON（Token-Oriented Object Notation，面向 LLM 的省 token JSON 替代，toon-format 0.5.0，MIT）。配套 tokens feature 默认开（tiktoken-rs 计 token）——Agent 场景把上下文预算当一等公民。[实证: docs.rs formatter/output 与 cargo info 两 crate]
5. **agent 发现机制可整套借鉴**：skill 模块从命令树生成 SKILL.md（供 AI 编码 agent 发现 CLI），`--llms` 紧凑命令索引，按深度拆分多文件，SHA-256 哈希做过期检测。[实证: docs.rs skill 模块]
6. **MCP 是命令树的投影不是另写服务**：mcp 模块把 CLI 叶子命令经 stdio 暴露为 MCP 工具（rmcp 实现），带 include/exclude 过滤；sync_mcp 反向把远端 MCP server 投影为本地命令。Reader 将来 `reader` 命令可直接成 MCP 工具。[实证: docs.rs mcp 模块]
7. **help 分 router 与 leaf 两型**，节齐全（header/synopsis/arguments/options/examples/hints/subcommands/global options/env vars），examples 与 hints 是 agent 读帮助时的关键节。[实证: docs.rs help 模块]
8. **OutputPolicy 区分受众**（All / AgentOnly），pager 只管人类输出——「机器可读与人读分路」有现成的类型级表达。[实证: docs.rs output/pager 模块]

## 现状或实测

### 双通道信号

| 通道 | 证据 | 值 |
| --- | --- | --- |
| crates.io | `cargo info incurs` | 0.5.3，MIT，rust 1.88；default = cli+toon+tokens [实证: 2026-08-31] |
| crates.io | `cargo info toon-format` | 0.5.0，MIT [实证: 同上] |
| GitHub | douglance/incurs | 1 star，2026-03-22 建，2026-08-17 推，未归档 [实证: gh repo view] |
| GitHub | wevm/incur（设计源头） | 605 stars，2026-02-26 建，2026-08-16 推，MIT [实证: gh repo view] |

### 模块清单与可吸收点

| 模块 | 职责 | Reader 吸收判断 |
| --- | --- | --- |
| output | ExecuteResult/OutputEnvelope/Format/OutputPolicy/CtaBlock/StreamRecord | 全盘吸收为输出层设计（结论 2/3/8） |
| formatter | Value 到七格式串行化，默认 TOON | 阶段 3 先 Json，Toon 按需（toon-format 可单引） |
| filter | 点路径加数组切片裁剪 Value 树 | 吸收为 `--filter`（`hits[].text` 式裁剪） |
| help | router/leaf 两型帮助，examples/hints 节 | 吸收：help 加 examples 节 |
| tokens | tiktoken 计 token（默认 feature） | 候选：extract 输出附 token 估算，引 tiktoken-rs 再定 |
| skill / sync_skills | SKILL.md 生成与同步、--llms 索引、SHA-256 过期检测 | 整套借鉴：`reader --llms` 加 SKILL.md 生成 |
| mcp / sync_mcp | 命令树经 stdio 成 MCP 工具；远端 MCP 反投影 | 远期：`reader` 暴露为 MCP 工具（rmcp） |
| pager | 人类输出分页 | 候选：人读长 extract 时分页，与 agent 输出分路 |
| completions | bash/zsh/fish/nushell 补全 | 候选：clap_complete 生态现成 |
| cli/command/parser/middleware/schema/config/openapi/fetch/agent_plugin/agents/streaming/tool | 框架骨架与 HTTP/OpenAPI 方向 | 不吸收：Reader 无 HTTP/OpenAPI 面；clap 已覆盖 parser/cli 职责 |

### 对 Reader 的落地映射

> 以下均为候选，逐条待立项。

1. **输出层**（阶段 3 首选）：search/extract 结构化结果 + `--format json`；包膜 `{ok, data, error}` 加 meta `{command, duration_ms}`；exit_code 保留 grep 语义。[推断: 与 P0002 设计约束一、三一致]
2. **分页原语**：extract 大文档加 `--offset/--limit`，meta 带 next_offset；cta 给出下一条建议命令（如下一页的 extract 调用）。[推断]
3. **裁剪**：`--filter` 点路径（如 `hits[].text`），基于 serde_json::Value 实现，不引 incurs。[推断]
4. **agent 发现**：`reader --llms` 紧凑索引 + SKILL.md 生成（命令树驱动）。[推断]
5. **远期**：MCP stdio 暴露（rmcp）；help 的 examples 节先行（clap 的 after_long_help 即可，零新依赖）。[推断]

## 踩坑沉淀

| 现象 | 根因 | 正确处理 |
| --- | --- | --- |
| 差点按「库即依赖」思路评估 incurs | 用户给的是 docs.rs 链接，第一反应是选型 | 双通道先分「选依赖」与「选学习对象」（R002 一节）：incurs 属后者，信号弱不进依赖，设计照学 |
| ExecuteResult 按 struct 抓 404 | docs.rs 上它是 enum | docs.rs 类型页 URL 带类别（struct./enum.），拿不准先回模块索引页核对 |

## 待办

1. 落地映射五条全部待立项：阶段 3 输出形态方案（P0003 候选）立项时以本研究为依据，逐条取舍。
2. wevm/incur（TS 源头）未读源码，若对 cta/help 节的具体文案格式要深挖，走 GitHub 通道 clone 深读。
3. TOON 对中文内容的 token 收益未验证；引 toon-format 前用 Reader 真实输出测一轮。[假设: 对结构化命中列表有收益]
