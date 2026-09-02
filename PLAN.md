# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**——基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：P0016 markdown 支持落地

> 登记日 2026-09-03。用户点名学习 mq 加 markdown 搜索与结构化提取；依据 `docs\research\S007-markdown支持选型-学习mq嵌mq-lang全引擎加零依赖分节.md`（双通道核实加 PoC 实测裁决）。

## 1. 闸门

S007 实证：split_markdown 零新依赖复用可行；mq-lang 0.8.4 API 直给、中文样本查询全对、错误结构化。无新选型。

## 2. 关键裁决

1. **.md 进格式面零新依赖**：document.rs 分派 `.md` 走 anydoc.rs 的 markdown 直读路径（读文件 → split_markdown → to_unit_bodies）；section/part 语义、`--pages`、分页、批量目录搜索全部继承。[依据: S007 关键结论 1]
2. **`reader query <文件> <表达式>`**：嵌 mq-lang 全引擎（DefaultEngine 加 load_builtin_module 加 parse_markdown_input 加 eval，过滤空渲染）；输入面为全部已支持格式——md 原文、anydoc 家族 GFM、PDF markdown 管线。[依据: S007 关键结论 2]
3. **输出契约同 search 语义**：退出码 0 有命中 / 1 无命中 / 2 出错；text 形态逐结果原样输出（markdown 片段），json 形态 `results[]` 加 `count`；`--format`/`--filter` 沿用。[依据: 边界「输出稳定可解析」]
4. **mq-markdown 不直用**：拍平节点序列对分节无增量（S007 关键结论 3），只随 mq-lang 传递进树。

## 3. 实施件与依据

| 件 | 要做什么 | 依据 |
| --- | --- | --- |
| `src\anydoc.rs` | 出 markdown 直读路径（.md 文件读文本进 split_markdown/to_unit_bodies） | 裁决 1 |
| `src\document.rs` | 分派加 is_supported 加 .md；错误文案格式清单同步 | 裁决 1 |
| `src\query.rs`（新） | to_markdown（按格式取 markdown 文本）加 run_query（eval 加空值过滤） | 裁决 2 |
| `src\lib.rs` | `query` 子命令（file 加 expression 位置参，--format/--filter）；退出码映射 | 裁决 3 |
| `src\introspect.rs` | llms 与 skill 双文本登记 query 与 .md 格式面 | P0007 守卫 |
| `tests\cli.rs` | md 夹具（标题分节、长文 part、搜索命中）；query 正例加负例加 json | R003 |
| Cargo.toml | mq-lang = "0.8" 进主依赖 | S007 裁决 |

## 4. 每件验收

门禁三件加 rumdl 三件套全绿；既有用例零改动；真样本回归（仓内 md 文档：README/SKILL/proven 方案文件搜加 query）；批量目录搜索自动覆盖 .md。

## 5. 边界

不做 mq 查询语法自研子集；不做 query 的 --pages/--ocr 组合；不改 release 流水线；不发版本。

## 完成的定义

> 本目标验收口径。

- `.md` 可 search/extract（节语义与 anydoc 家族一致），`query` 对全格式面可跑 mq 表达式
- 门禁与真样本回归全绿；文档回填（AGENTS、INDEX、SKILL、README、CHANGELOG、diary、P0016 归档）
