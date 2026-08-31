# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**——基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：PDF 提取质量——markdown 管线与 needs_ocr 提示

> 对应 `GOAL.md`，方案 `docs\proven\P0005-PDF提取质量-markdown管线与needs_ocr提示.md`，登记日 2026-08-31。**2026-08-31 已达成**（验收记录见方案「验收标准」节）；本文件在下一目标立项时整体替换。

### 1. 闸门

pdf-inspector 1.17.0 现成 API 已核实 [实证: 2026-08-31 本地 registry 源码]：`extract_pages_markdown` 返回逐页 `PageMarkdown{ page(0 基), markdown, needs_ocr, ocr_reason }` 与文档级栏/OCR 信号；`needs_ocr` 覆盖 GID 编码字体、编码问题、乱码、空提取。不引新依赖，无版本升级。

### 2. 实施件与依据

| 件 | 要做什么 | 依据 |
| --- | --- | --- |
| `src\pdf.rs` | 切 `extract_pages_markdown`：1 基 `HashSet` 过滤换算 0 基有序切片；`TextUnit.no = page + 1`；markdown 按 `lines()` 拆行；删除 `items_to_lines` / `join_line` | P0005 方案；pdf-inspector rust-api |
| `src\document.rs` | `TextUnit` 加 `pub needs_ocr: Option<String>` | P0005 方案 |
| `src\epub.rs` | 构造 `TextUnit` 处补 `needs_ocr: None` | P0005 方案（EPUB 文本层语义可靠） |
| `src\lib.rs` | extract 页节后 `[needs_ocr: 原因]` 行；search 命中输出前 stderr 汇总警示（列页号） | P0005 方案；R001 边界（检出后提示） |
| `tests\cli.rs` | 两栏 PDF 阅读序正例（lopdf 造，期望值独立来源）；无文本页 `[needs_ocr` 例；search 警示例；既有断言锚稳定字段回归 | `docs\references\R003-测试标准细则-分层断言与门禁流程.md` |

### 3. 每件验收

门禁三件全绿；两栏阅读序为左上左下右上右下；无文本页有提示行且 search 有警示；既有语义不降级。失败当场记 `docs\mistakes\`。验收通用口径见 G003 第四节。

### 4. 边界

不做 OCR；不动行式输出协议与退出码语义；不调 MarkdownOptions（用管线默认）；乱码修复（tounicode 层）不在本期，只做检出提示。[依据: P0005 非目标节]

## 完成的定义

> 本目标验收口径。

- TODO 表全部已完成或明确跳过（跳过须写 mistakes 原因）
- `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked` 三件过
- `rumdl check .` 尽量零告警；P0005 与 INDEX 已登记；CHANGELOG 记破坏性变更
- 研究与测试文档的关键结论标六态（AGENTS 写研究与测试文档规则）
