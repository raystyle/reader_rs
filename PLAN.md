# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**——基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：anydoc 统一文档引擎大重构

> 对应 `GOAL.md`，方案 `docs\proven\P0009-anydoc统一文档引擎大重构.md`，登记日 2026-09-01。选型证据 `docs\research\S004-Word文档读取选型-docx自解与doc直读双路线实测.md`（决策变更节）。

### 1. 闸门

S004 anydoc 补测全绿（实体保真、legacy .doc 直读、真样本三方一致）；anydoc 对 PDF 直连 pdf-inspector 绕过自身模型，故「PDF 留直连加其余走 anydoc」与上游架构同构，无选型风险。

### 2. 实施件与依据

| 件 | 要做什么 | 依据 |
| --- | --- | --- |
| `Cargo.toml` | `anydoc = "0.2.4"` 进主依赖；`rbook`、`quick-xml` 移出主依赖；`rbook` 转 dev-dep（EPUB 夹具）、`zip` 进 dev-dep（docx 夹具） | P0009 依赖树节 |
| `src\anydoc.rs`（新） | `to_markdown_bytes` → `markdown_to_units`：代码围栏外 ATX 行分节，无标题单单元；`ConvertError` 转中文错误串 | P0009 分节规则 |
| `src\document.rs` | 分派重写：`pdf` → pdf.rs；`anydoc::Format::from_extension` 命中 → anydoc.rs；`UnitKind::Section` | P0009 方案节 |
| `src\epub.rs` | 删除（rbook 加 quick-xml 手写文本化退役） | P0009 方案节 |
| `src\lib.rs` / `src\introspect.rs` | mod 表、help、llms、skill 文案更新（格式清单、section 单元语义） | AGENTS 意图路由同步 |
| `SKILL.md` | `reader skill` 再生成（bash 原始重定向保 LF；逐字节守卫测试兜底） | P0007 守卫 |
| `tests\cli.rs` | docx 夹具（zip 现造 OOXML）用例、csv 用例、`tests\assets\legacy.doc` 用例、EPUB 断言改 section；PDF 用例零改动 | R003 现造夹具与独立期望 |
| `CHANGELOG.md` 等 | Unreleased 破坏性变更（EPUB 章变节）；README/AGENTS/INDEX/ROADMAP/diary 收口 | G003 交付变更规则 |

### 3. 每件验收

门禁三件加 rumdl 三件套全绿；新增格式用例与改写 EPUB 用例全绿、PDF 用例零改动全绿；真样本回归（docx 27 行基线、Word COM 造 .doc、既有 PDF）。失败当场记 `docs\mistakes\`。验收通用口径见 G003 第四节。

### 4. 边界

不改 PDF 行为；不做 OCR/网络；不做表格公式深加工；不做 .doc 夹具 COM 自动化（提交二进制资产）；pptx/xlsx 与 docx 同通道不加专属形态。[依据: P0009 非目标节]

## 完成的定义

> 本目标验收口径。

- TODO 表全部已完成或明确跳过（跳过须写 mistakes 原因）
- `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked` 三件过
- `rumdl check .` 尽量零告警；P0009 与 INDEX 已登记；CHANGELOG Unreleased 记破坏性变更
- 研究与测试文档的关键结论标六态（AGENTS 写研究与测试文档规则）
