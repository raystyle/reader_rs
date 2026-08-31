# P0005-PDF提取质量-markdown管线与needs_ocr提示

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：已完成
- 日期：2026-08-31
- 关联：TODO.md 当前目标 / `docs\research\S001-PDF文本提取crate选型-pdf-inspector双通道核实.md`（真实样本回归节记两处行重建瑕疵，归本方案处理）/ ROADMAP 阶段 2

## 背景与问题

PDF 通道现用朴素几何法重建行（`src\pdf.rs`：y 降序扫描加容差归行、行内 x 升序拼接），三处天花板 [实证: S001 真实样本回归]：

1. 多栏排版无栏检测，跨栏同行串成一行，阅读序错。
2. 旁注与正文同 y 时粘连（URL 粘连、图片占位符粘连）。
3. 扫描件页与编码问题页（CJK、GID 字体）无任何信号——Agent 拿到空提取或乱码只能猜。

ROADMAP 阶段 2 立三项：多栏阅读序、问题页检出提示、大文档性能观察。

## 目标与非目标

- 目标：
  - PDF 通道整体切换 `pdf_inspector::extract_pages_markdown` 管线：多栏阅读序由其 zone/column 排序承担，行重建交库。
  - `TextUnit` 扩展 `needs_ocr: Option<String>`（页文本层不可靠信号，含原因）；EPUB 通道恒 `None`。
  - 输出提示：extract 在页节标记后给 `[needs_ocr: 原因]` 行；search 对含 needs_ocr 页的文档 stderr 一条警示，stdout 保持纯 grep 语义。
  - 真实样本回归（390 页 PDF 多栏页抽查）与大文档性能观察。
- 非目标：
  - 不做 OCR（R001 边界：检出后提示，不识别）。
  - 质量承诺只面向英文与中文（2026-08-31 用户裁定）；其它语言不做专项验证与调优，不可靠时由 needs_ocr 提示兜底。
  - 不引新依赖（pdf-inspector 1.17.0 现成 API，`extract_pages_markdown_mem` 免参数）。
  - 不动 search/extract 的行式输出协议与退出码语义（0/1/2）；`== page N ==` 分节不变。
  - MarkdownOptions 调优（标题/列表/表格检测开关）不在本期，用管线默认值，观察真实样本后再定是否立项。

## 方案

1. `src\pdf.rs` 重写：`extract_pages` 调 `extract_pages_markdown`；其 `pages` 参数为 0 基有序切片，外部 1 基 `HashSet` 过滤需换算（减一、升序、去重）；`PageMarkdown.page` 0 基，`TextUnit.no = page + 1`；`PageMarkdown.markdown` 按 `lines()` 拆入 `TextUnit.lines`；`needs_ocr` 映射 `TextUnit.needs_ocr`（真则装 `ocr_reason.unwrap_or("原因未明")`）。朴素几何法（`items_to_lines` / `join_line`）删除。
2. `src\document.rs`：`TextUnit` 加 `pub needs_ocr: Option<String>`；EPUB 构造处补 `None`。
3. `src\lib.rs`：`run_extract` 页节标记后若 `needs_ocr` 为 `Some` 输出 `[needs_ocr: {原因}]`；`run_search` 在有命中输出前，对 `needs_ocr` 页 stderr 一条汇总警示（列页号）。
4. 测试：lopdf 造两栏 PDF（左栏上下两句、右栏上下两句）断言阅读序为左上左下右上右下（期望值来自写入文本与坐标，独立来源）；造无文本页 PDF 断言 extract 输出 `[needs_ocr`；既有 20 测回归。

依据：pdf-inspector 1.17.0 源码核实 [实证: 2026-08-31 本地 registry 源码]——`PagesExtractionResult{ pages: Vec<PageMarkdown>, pages_with_columns, pages_needing_ocr, ocr_reasons_by_page, is_complex }`；`PageMarkdown{ page: 0 基, markdown, needs_ocr, ocr_reason }`，其 `needs_ocr` 文档明言覆盖 GID 编码字体、编码问题、乱码、空提取；`extract_pages_markdown_mem` 内部计算字体统计与布局复杂度，页过滤不影响阈值一致性。

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| 整体切 markdown 管线（选定，用户裁决） | search/extract 共享同一文本层（搜到即提取到）；三问题一次解决；阶段 3 Markdown 导出顺带半达成。代价：v0.1 输出行为变（行内容带 markdown 语法），CHANGELOG 记破坏性变更 |
| extract 加 --markdown 旗标 | 契约不变；但双管线并存，search 命中行与 extract 行不一致，两套测试面 |
| 只做检出提示不切管线 | 最小改动；多栏与旁注瑕疵继续留着，阶段 2 主体空转 |
| 自写栏检测（x 谷分割） | 违反懒人阶梯：库已覆盖，不重造 |

## 实施步骤

1. 本方案立项，三原语与 ROADMAP/INDEX 登记。
2. pdf.rs 切管线；TextUnit 扩展；document.rs/epub.rs 跟随。
3. lib.rs 输出提示两条路径。
4. 测试补强与既有回归。
5. 真实样本回归与性能观察。
6. 收官登记（回填本方案、CHANGELOG 破坏性变更行、diary）。

## 风险与回滚

- 风险：markdown 管线对简单测试 PDF 的输出可能加结构语法或改行合并，既有行级断言（如 `1:1:Hello Reader World`）可能需跟随更新。缓解：断言改锚定稳定字段（页号、命中文本本身），不锚定整行格式。
- 风险：管线默认 MarkdownOptions 未暴露调参口，个别文档输出风格不合意。缓解：本期接受默认，真实样本观察后另立项（非目标已列）。
- 风险：性能较朴素法变慢（管线做布局分析）。缓解：性能观察项兜底，390 页样本计时对比 S001 基线 0.57s；劣化一个数量级内可接受（质量换时间）。
- 回滚：git revert 单提交即回朴素法。

## 实施过程与经验

> 2026-08-31 回填。

- 实际怎么做（与计划偏差、关键决策点）：
  - 管线切换一次过；既有三个行级断言按预案改锚稳定字段（`页:行:` 前缀加同行文本），语义未降级。
  - 两栏测试 fixture 两轮才触发栏检测：第一版 3 乘 2 等距网格被管线判成 markdown 表格（等宽等距网格本就是表）；读 `split_side_by_side` 源码阈值后重设计——每栏 22 行变宽散文、栏间沟约 90pt、两栏 y 网格错开 5pt（阈值：items 至少 40、沟至少 30pt、两侧各至少 20% 项、无跨界项），一次命中。
  - needs_ocr 两路径（extract 提示行、search stderr 警示）一次成形，真实样本即中。
- 踩了什么坑 + 怎么解决：无新 mistakes 级坑（测试借用期 E0716/E0597 属常规修正）。fixture 被判表格不算坑，是对管线行为的认知更新，已写进测试注释。
- 沉淀的经验：
  - 阈值门控行为的测试先读实现阈值再设计 fixture，比盲试快；「造两栏」不足以触发检测器，等距短网格反而进表格通道。
  - `needs_ocr` 真实有效：390 页书 24 个图像页（封面、章扉）全部命中；中文论文封面页 reason 为 scanned。Agent 拿到的是信号而非静默空页。
  - 全页分节补齐：旧法只给有文本项的页建节（390 节），新管线逐页出节（399 节），扫描页不再从页码序列里缺席——对 Agent 的页定位是净改善。
  - markdown 管线对整页单行会加 `##` 标题装饰；行级断言别锚装饰前缀，锚 `页:行:` 与内容本身。
  - 性能：布局分析有成本（399 页 0.92s 对 S001 基线 0.57s，约 1.6 倍），质量换时间，在验收带内。

## 验收标准

- 两栏测试 PDF 阅读序正确（左栏自上而下后右栏），非跨栏串行。[实证: 2026-08-31 extract_orders_two_columns_by_reading_order 绿]
- 无文本页（扫描件形态）extract 输出 `[needs_ocr: ...]` 提示行；search 对此类文档 stderr 有警示且 stdout 无提示混入。[实证: 2026-08-31 needs_ocr_page_hinted_in_extract_and_search 绿]
- 测试全绿（单元 5 加集成 17，共 22）；门禁三件与 rumdl 三件套过。[实证: 2026-08-31 WSL 本地]
- 真实样本：S001 两处瑕疵修复（旁注 URL 成形 markdown 链接不再粘连、图片占位符不再混入正文）；`assert_cmd` 25 行命中与 S001 记录一致；399 页全量 0.92s（基线 0.57s）；EPUB 37 章 0.083s；中文论文（360 Quake x-quake paper）标题、摘要、正文提取通顺，中文关键词搜索命中正确，全文 0 个 needs_ocr。[实证: 2026-08-31 本机 release]
- INDEX 与三原语登记完整，CHANGELOG 记破坏性变更，README 输出格式与边界更新。[实证: 2026-08-31]
