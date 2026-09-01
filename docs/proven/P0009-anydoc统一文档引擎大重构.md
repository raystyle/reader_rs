# P0009-anydoc统一文档引擎大重构

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：进行中
- 日期：2026-09-01
- 关联：TODO.md / 研究 `docs\research\S004-Word文档读取选型-docx自解与doc直读双路线实测.md`（含 anydoc 补测与决策变更记录）/ 参考 `docs\references\R001-项目定位-Agent原生文档阅读搜索和提取工具.md`

## 背景与问题

用户裁定：以 anydoc（firecrawl，MIT）为统一文档引擎做大重构，取代逐格式自写提取。S004 补测证实 anydoc 是当前最强可用引擎：doc/docx/odt/rtf/epub/ppt/pptx/xls/xlsx/ods/odp/csv 十三格式进同一块级模型或出 GFM markdown，实体保真（`a & b 中文` 全对，office_oxide 同题丢实体）、legacy .doc 直读保真（真 Word 二进制样本）、表格出 GFM 管道表。

同时实测确认 anydoc 的 PDF 路径是**整篇单串 markdown**：无页边界标记、无页级 API，含扫描页时整篇报 `NeedsOcr`；anydoc 自身对 PDF 也绕过其文档模型直连 pdf-inspector（`to_document` 对 PDF 明确不支持）。

## 目标与非目标

- 目标：
  - 格式面从 2 种扩到 14 种：`.pdf` 保留 pdf-inspector 直连（页契约原样）；其余十三种走 anydoc。
  - `TextUnit` 映射：anydoc markdown 按**顶层标题分节**（代码围栏外的 ATX 行开新单元），无标题文档整篇一单元；新 `UnitKind::Section`（label `section`）。
  - 依赖树瘦身：`rbook` 与 `quick-xml` 退出主依赖；`rbook` 转 dev-dependency（EPUB 测试夹具），`zip` 进 dev-dependency（docx 测试夹具）；`anydoc = "0.2.4"` 进主依赖。
  - CLI 契约：退出码、行式命中、JSON 包膜、分页、filter 全不变；`--pages` 语义保持「选单元」。
- 非目标：
  - 不做 OCR 与任何网络调用（anydoc Rust 侧零网络，天然合规）。
  - 不改 PDF 侧行为（`src\pdf.rs` 一行不动）。
  - 不做表格/公式语义深加工（GFM 原样行输出）。
  - 不做 .doc 夹具的 COM 自动化（CI 无 Word；提交小型 .doc 二进制测试资产）。
  - 不做 pptx/xlsx 专属输出形态（与 docx 同一 markdown 通道）。

## 方案

```text
src\
  document.rs  分派重写: "pdf" → pdf.rs；anydoc::Format::from_extension 命中 → anydoc.rs；否则不支持报错（附格式清单）
  pdf.rs       不变（页契约: UnitKind::Page、页级 needs_ocr）
  anydoc.rs    新: 读 bytes → ::anydoc::to_markdown_bytes → markdown_to_units（标题分节）→ Vec<TextUnit>
  epub.rs      删除（rbook 加 quick-xml 手写文本化退役）
```

分节规则（`markdown_to_units`）：ATX 标题行（`^#{1,6} `）在代码围栏（``` 开合状态机）外开启新单元；节内行原样保留（GFM 表格、列表、代码块）；全文无标题则整篇单单元；`filter` 按 1 起单元号过滤（与 PDF 页过滤同机制）。

错误映射：`ConvertError` 统一转中文错误串走既有 stderr 路径（退出 2）。anydoc 家族 `needs_ocr` 恒 `None`。

EPUB 单元语义变化：spine 章（`== chapter N ==`）改为标题分节（`== section N ==`），属破坏性变更，记 CHANGELOG Unreleased。

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| anydoc 统一加 PDF 直连保留（选定） | 页契约零损失；与 anydoc 自身对 PDF 的架构决策一致（其 PDF 也直连 pdf-inspector 绕过模型） |
| 全面 anydoc（含 PDF） | `--pages` / `== page N ==` / 页级 needs_ocr 全失，扫描 PDF 整篇报错劣于现行为；破坏 v0.1.0 已发布契约，弃 |
| anydoc 只接新格式族、EPUB 留 rbook | 三引擎并存，非大重构；rbook/quick-xml 留主依赖，弃 |
| docx 自解路线（S004 初判） | 用户裁定推翻；保真虽可但格式面窄，表格/列表/脚注/公式全要自写 |

## 实施步骤

1. 立项与本方案登记，三原语同步。
2. 依赖手术（Cargo.toml）；`src\anydoc.rs`；`document.rs` 分派重写；删 `epub.rs`；`lib.rs`/`introspect.rs` 文案与 mod 表更新。
3. `tests\cli.rs`：docx 夹具（zip 现造 OOXML）与用例、csv 用例、`tests\assets\legacy.doc` 提交与用例、EPUB 用例改 section 断言、不支持格式负例保持。
4. `SKILL.md` 再生成（bash 原始重定向保 LF，逐字节守卫测试兜底）。
5. 门禁三件加 rumdl 三件套；真样本回归（docx / legacy .doc / PDF 各一）。
6. 收官：S004 回填、CHANGELOG Unreleased、README/AGENTS/INDEX/ROADMAP、diary。

## 风险与回滚

- 风险：anydoc 0.2.4 年轻（2026-08 建仓）。缓解：主版本锁 `^0.2`、真样本回归、引擎隔离在单模块，`git revert` 即回。
- 风险：标题分节误切代码围栏内的 `#` 行。缓解：围栏状态机加单元测试（围栏内 `# bash` 注释不分节）。
- 风险：EPUB 章语义变化破坏存量调用。缓解：CHANGELOG 记破坏性变更；行式命中格式不变，仅节头与单元号语义变化。
- 风险：无标题长文档单单元过长，`--offset/--limit` 失去意义。缓解：记录为已知限制（与 S004 待办同源），真实需求出现再评行数分片。

## 实施过程与经验

> 完成时补全，不是留空。

## 验收标准

- 门禁三件加 rumdl 三件套全绿。
- 新格式用例绿：docx（搜索/提取/分节/过滤）、legacy .doc（提取含中英文与 `&`）、csv；EPUB 用例改 section 后绿；PDF 用例零改动绿。
- 真样本回归：Desktop 测试V2.docx（27 行基线）、Word COM 造 .doc、既有 PDF 样本。
- S004 回填 anydoc 实测与决策变更；CHANGELOG/README/AGENTS/INDEX/ROADMAP/diary 收口。
