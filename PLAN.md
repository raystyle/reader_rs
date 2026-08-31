# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**——基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：PDF 文本搜索与提取 CLI 最小闭环

> 对应 `GOAL.md`，方案 `docs\proven\P0001-PDF文本搜索与提取CLI最小闭环.md`，登记日 2026-08-31。**2026-08-31 已达成**（验收记录见方案「验收标准」节）；本文件在下一目标立项时整体替换。

### 1. 闸门

文档骨架已就位（AGENTS 四段、三原语、docs 六目录、`.tools` 三件套）。选型已定：pdf-inspector（研究 `docs\research\S001-PDF文本提取crate选型-pdf-inspector双通道核实.md`）。

### 2. 实施件与依据

| 件 | 要做什么 | 依据 |
| --- | --- | --- |
| `src\pdf.rs` | 按页提取与行重建（`extract_text_with_positions_pages`，y 聚类 x 排序） | S001 结论；pdf-inspector rust-api 文档 |
| `src\search.rs` | 匹配器（字面 / 正则 / 忽略大小写）与上下文命中 | P0001 方案 |
| `src\lib.rs` | clap CLI 定义、`run()`、页范围解析 | 《Command-Line Rust》薄壳模式（R003 溯源） |
| `src\main.rs` | 薄壳：`reader` 与 `rr` 双 bin 同入口 | 用户指定命令名 |
| `tests\cli.rs` | assert_cmd 集成：冒烟、正负例、页过滤；测试 PDF 用 lopdf 现造 | `docs\references\R003-测试标准细则-分层断言与门禁流程.md` |

### 3. 每件验收

`cargo test` 全绿；`reader search` 命中退出 0、无命中退出 1、出错退出 2；失败当场记 `docs\mistakes\`。验收通用口径见 G003 第四节。[实证: 2026-08-31 全部通过]

### 4. 边界

首版不做 OCR、不做 Markdown 转换、不做批量目录；测试 PDF 由 lopdf 现造，不依赖外部样本文件。[依据: P0001 非目标节]

## 完成的定义

> 本目标验收口径（2026-08-31 全部满足）。

- TODO 表全部已完成或明确跳过（跳过须写 mistakes 原因）
- `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked` 三件过
- `rumdl check .` 尽量零告警；P0001 与 INDEX 已登记
- 研究与测试文档的关键结论标六态（AGENTS 写研究与测试文档规则）
