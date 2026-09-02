# P0016-markdown支持与mq结构化提取

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：已完成
- 日期：2026-09-03
- 关联：TODO.md / 研究 `docs\research\S007-markdown支持选型-学习mq嵌mq-lang全引擎加零依赖分节.md` / 前序 P0009（anydoc 分节管线）

## 背景与问题

格式面缺 .md 纯 markdown 文档；用户点名学习 harehare/mq 加入 markdown 搜索与结构化提取能力。S007 裁决：.md 零新依赖复用 split_markdown 进格式面；结构化提取嵌 mq-lang 全引擎出 `query` 子命令。

## 目标与非目标

- 目标：
  - `.md` / `.markdown` 进 search/extract 格式面，节语义与 anydoc 家族完全一致（section/part、`--pages`、分页、批量目录搜索全继承）。
  - `reader query <文件> <mq表达式>`：全格式面（md 原文、anydoc 家族 GFM、PDF markdown 管线）跑 mq 查询；退出码 0/1/2 同 search 语义；text 出 markdown 片段、json 出 `results[]` 加 `count`。
- 非目标：不自研查询语法子集；不做 query 的 --pages/--ocr 组合；不发版本。

## 方案

`anydoc.rs` 抽 `sections_to_units` 公共尾部，新增 `extract_markdown`（读 UTF-8 原文进 split_markdown/to_unit_bodies）；`document.rs` 分派加 `is_supported` 加 .md/.markdown；新模块 `query.rs`（to_markdown 按格式取 markdown 文本，run_query 嵌 mq-lang：DefaultEngine 加 load_builtin_module 加 parse_markdown_input 加 eval，过滤空渲染）；`lib.rs` 挂 `query` 子命令。

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| 嵌 mq-lang 全引擎（选定） | 中文样本实测全对、错误结构化；依赖约 30 个全 MIT 纯 Rust |
| 只用 mq-markdown 自写选择器 | 节点拍平无增量（S007 关键结论 3），自研语法是维护黑洞 |
| mq-lang 裁 feature | 默认面已够小；裁剪收益不配复杂度 |

## 实施步骤

1. S007 研究落盘后立项，三原语同步。
2. anydoc/document 加 .md 路径；query.rs 加 lib.rs 接线；introspect 双文本。
3. tests\cli.rs：md 夹具（分节、part 分片、搜索命中）、query 正例（.h2、select 管道）加负例（无命中 1、坏表达式 2、目录 2）加 json 形态。
4. 真样本回归（仓内 README/SKILL/proven md 搜加 query）；门禁全绿。
5. 文案收口（AGENTS/INDEX/README/CHANGELOG/diary/本归档）。

## 风险与回滚

- 风险：mq-lang 约 30 依赖推高构建时间与体积。缓解：接受（增量有限，S007 关键结论 4）；回滚 `git revert` 单模块加依赖行。
- 风险：mq 表达式错误体验差。缓解：miette 结构化错误透传 stderr（行列号齐全，S007 PoC 实证）。

## 实施过程与经验

- 实际怎么做：按步骤走完；query 对仓内 md 真样本 `.h2`/`.code` 全对，批量目录搜索自动覆盖 .md。
- 与计划偏差：skill 文本是纯字符串字面量，新节里 `contains("关键词")` 的引号没转义当场编译红：curated 文本改动要先想转义。
- 沉淀的经验：format! 系长文本里的示例代码引号是高频坑；docs.rs 首页示例可能滞后于 crate 版本（S007 踩坑：RuntimeValue 改名），以仓内源码为准。

## 验收标准

- 门禁三件加 rumdl 三件套全绿，既有用例零改动。[实证: 2026-09-03 fmt/clippy -D warnings/test --locked（23 单元加 52 集成）全绿；rumdl 零告警]
- 新用例：md 分节加 part 分片加搜索命中、query 正负例与 json、目录拒绝。[实证: 2026-09-03 cargo test]
- 真样本：README.md 搜索命中、P0015 方案 extract 分节、`.h2` 与 `.code` 查询、docs 目录批量搜覆盖 .md。[实证: 2026-09-03 本机]
- 文案与登记收口（AGENTS/INDEX/README/SKILL 漂移守卫/CHANGELOG/diary）。[实证: 2026-09-03]
