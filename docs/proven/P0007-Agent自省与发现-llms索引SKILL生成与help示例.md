# P0007-Agent自省与发现-llms索引SKILL生成与help示例

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：已完成
- 日期：2026-08-31
- 关联：TODO.md 当前目标 / `docs\research\S002-incurs模块经验研究-Agent原生CLI的命令输出与帮助设计.md`（落地映射 4、5）

## 背景与问题

P0006 已吸收 incurs 输出契约侧（包膜、cta、分页、filter），但自省发现侧一条未做：Agent 拿到 `reader` 二进制后无法自助发现能力——`--llms` 紧凑索引、SKILL.md 生成、help 的 examples 节都缺。用户点名补齐，并要求命令集成 SKILL。

## 目标与非目标

- 目标：
  - `reader --llms`：stdout 紧凑命令索引（子命令、参数、退出码、输出契约），退出 0。
  - `reader skill` 子命令：stdout 生成 SKILL.md（frontmatter 加使用契约），退出 0。
  - 仓根提交 `SKILL.md`（由命令生成），集成测试做漂移守卫（运行时输出与仓内文件一致）。
  - search / extract help 补 examples 节（clap `after_long_help`，零新依赖）。
- 非目标：
  - 不做 SKILL.md 命令树自动生成与 SHA-256 过期检测（incurs 全套）； curated 文本加漂移测试替代。
  - 不做 MCP、TOON、token 计数、pager、completions（仍候选）。
  - 不动 search/extract 既有输出与退出码语义。

## 方案

1. 新模块 `src\introspect.rs`：`llms_text()` 与 `skill_md()` 返回 curated 文本（版本号取 `env!("CARGO_PKG_VERSION")`）。
2. `src\lib.rs`：顶层加 `--llms` 旗标与 `skill` 子命令（`command` 改 `Option<Commands>`，`--llms` 优先于子命令分派）；search/extract 加 `after_long_help` examples 节。
3. 漂移守卫双保险（tests\cli.rs）：
   - 遍历 clap `Command` 树的每个 long 旗标，断言出现在 `--llms` 与 `skill` 输出中（防新参数漏登记）。
   - 仓根 `SKILL.md` 与 `reader skill` 输出逐字节一致（防文档漂移）。
4. 仓根 `SKILL.md` 由 `reader skill > SKILL.md` 生成提交；README 加「Agent 发现」节。

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| curated 文本加漂移测试（选定） | 内容含退出码、输出契约、示例等 clap 不知道的语义；KISS |
| 从 clap 命令树全量生成（incurs 路线） | 单一事实源，但生成器复杂度高，语义节仍需手工；现阶段过度工程 |
| 引 incurs 依赖 | S002 结论一已否决（信号弱不进依赖） |

## 实施步骤

1. 本方案立项，三原语登记。
2. `src\introspect.rs` 加 CLI 接线；help examples。
3. 集成测试（正例加双漂移守卫）；仓根 SKILL.md 生成。
4. 门禁回归；README/AGENTS/INDEX/CHANGELOG/diary 登记。

## 风险与回滚

- 风险：`command` 改 Option 影响既有分派与帮助形态。缓解：集成测试覆盖裸 `reader`（出帮助）与既有子命令全回归。回滚：revert 单提交即回。

## 实施过程与经验

- 实际怎么做：按步骤走完，一处计划外改动——`command` 改 `Option<Commands>` 后裸 `reader` 不再由 clap 自动报错，补 None 分支手写「帮助走 stderr、退出 2」，保持原语义（`dies_no_args` 回归不破）。
- 踩了什么坑 + 怎么解决：`Some(Commands::Search { ... })` 嵌套 struct 模式漏右括号，编译器报 mismatched delimiter，补上 `})` 即过；无新坑入 mistakes。
- 沉淀的经验：
  - curated 文本加「clap 命令树遍历断言」是防文档漂移的便宜组合：文本可以写 clap 不知道的语义（退出码、输出契约），而旗标全覆盖由命令树反向兜底，新增参数漏登记当场红。
  - SKILL.md 仓根提交加逐字节一致测试，让「文档即代码产物」可验收；版本号取 `env!("CARGO_PKG_VERSION")`，升版本后测试会逼着重生成。

## 验收标准

- `reader --llms` 与 `reader skill` 输出完整覆盖两子命令全部参数（clap 树遍历断言过）。[实证: 2026-08-31 introspection_texts_cover_all_clap_flags 绿]
- 仓根 `SKILL.md` 与运行时输出逐字节一致（测试过）。[实证: 2026-08-31 committed_skill_md_matches_runtime_output 绿]
- `reader search --help` 含 examples 节。[实证: 2026-08-31 search_help_contains_examples 绿]
- 门禁三件加 rumdl 三件套全绿；INDEX 与三原语登记完整。[实证: 2026-08-31，39 测全绿（单元 7 加集成 32）]
