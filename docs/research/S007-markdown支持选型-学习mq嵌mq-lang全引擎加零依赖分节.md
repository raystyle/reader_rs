# S007-markdown支持选型-学习mq嵌mq-lang全引擎加零依赖分节

> 2026-09-03。触发点：用户点名「学习 https://github.com/harehare/mq 加入 markdown 文档的搜索和结构化提取能力」。流程按 R002；六态标准见 G002。PoC 工程 `target\poc-mq\`（gitignored，证据以本文记录为准）。

## 背景

Reader 格式面缺 .md（PDF 与 anydoc 家族之外的纯 markdown 文档）；mq 是 jq 风格的 markdown 查询工具（Rust 写），其结构化提取思路（节点选择器加管道）正合「结构化提取」诉求。待裁决：.md 基础支持怎么进、结构化提取嵌入多深。

## 关键结论

1. **.md 基础支持零新依赖落地**：`anydoc.rs` 的 `split_markdown`（按顶层 ATX 标题分节、代码围栏内不分节）加 `to_unit_bodies`（超 200 行切 part）原样适用于 .md 原文——读文件进同一函数即可，行为与 anydoc 家族完全同构（section/part 语义、`--pages`、分页全继承）。[实证: 2026-09-03 读 src\anydoc.rs 确认函数与格式无关]
2. **结构化提取嵌 mq-lang 全引擎**：`reader query <文件> '<mq 表达式>'`。mq-lang 0.8.4（MIT）API 直给：`DefaultEngine` 加 `load_builtin_module` 加 `parse_markdown_input` 加 `eval`；非匹配节点产出空渲染，过滤空串即得干净结果。[实证: 2026-09-03 target\poc-mq 中文样本实测：`.h`/`.h2`/`.code`/`.link`/`.[] | select(contains("nmap"))` 全出正确 markdown 片段；语法错误出结构化 Error（miette）]
3. **mq-markdown 不单独引入**：其 `.nodes` 是拍平序列（段落拆成 Text/Link/Strong、表格拆成 TableCell/TableAlign），对分节无增量价值（split_markdown 已够用），只随 mq-lang 传递进入。[实证: 2026-09-03 poc-mq nodes 模式输出]
4. **依赖代价认账**：mq-lang 约 30 个正常依赖（ammonia、nom、miette、csv、yaml-rust2 等），全 MIT 系纯 Rust；对已经 32.9MB 的二进制增量有限。[实证: 2026-09-03 docs.rs 依赖清单；推断: 体积增量未实测]
5. **mq 生态裁决**：mq 主仓 1023 星、MIT、2026-09-01 仍推（活跃）；mq-markdown crates.io 17.5 万下载、mq-lang 1.17 万。学习对象是查询语义与节点选择器设计，不是自研一套查询语言。[实证: 2026-09-03 gh repo view 加 crates.io API]

## 现状或实测

### 双通道核实

| 候选 | crates.io | GitHub | 裁决 |
| --- | --- | --- | --- |
| mq-lang | 0.8.4，MIT，1.17 万下载 | harehare/mq 主仓内 crates/mq-lang，1023 星活跃 | 选中：query 子命令引擎 |
| mq-markdown | 0.8.4，MIT，17.5 万下载 | 同上 crates/mq-markdown | 随 mq-lang 传递，不直用 |
| 自研 md 分节 | — | — | 不新写：split_markdown 复用 |

[实证: 2026-09-03 cargo info 加 gh api 逐条]

### PoC 实测

样本：中文标题加段落（含链接加粗）、列表、rust 代码围栏、GFM 表格、三级标题。

| 验证点 | 结果 |
| --- | --- |
| mq-markdown 节点 | 拍平序列带 Position（line/column），表格拆 TableCell/TableAlign [实证] |
| `.h` | 出全部 4 级标题原文 [实证] |
| `.h2` | 只出 2 个二级标题 [实证] |
| `.code` | 出完整代码围栏（含语言标记） [实证] |
| `.link` | 出 `[链接示例](https://example.com)` [实证] |
| `.[] \| select(contains("nmap"))` | 出 `- nmap` 列表项 [实证] |
| 非法表达式 | miette 结构化错误（行列号加 UnexpectedToken） [实证] |
| 非匹配节点 | 产空渲染，过滤空串即干净 [实证] |

## 踩坑沉淀

| 现象 | 根因 | 正确处理 |
| --- | --- | --- |
| docs.rs 示例编译不过（`Value::Markdown`、`engine.eval` 返回单值断言） | 0.8.4 已改名 `RuntimeValue`、eval 收 `Iterator<Item = RuntimeValue>` 返回集合 | 以仓内源码为准（parse_markdown_input 加 load_builtin_module），不信 docs.rs 旧示例 |

## 待办

1. 转 P0016 落地：.md 进格式面（search/extract 零新依赖）加 `query` 子命令（mq-lang）。
2. mq 查询语法文档面：skill/llms 给常用例（`.h2`、`.code`、`.link`、select 组合），完整语法指去 mqlang.org。
