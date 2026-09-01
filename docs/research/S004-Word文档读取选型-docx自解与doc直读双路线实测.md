# S004-Word文档读取选型-docx自解与doc直读双路线实测

> 2026-09-01，触发点：用户提出增加 Office Word 文档（.doc / .docx）读取能力。流程按 `docs\references\R002-选型研究细则-cratesio与github双通道.md`；六态标准见 G002。PoC 工程在 `target\poc-docx\`（gitignored，证据以本文记录为准）。

## 背景

Reader 现支持 .pdf / .epub，要评估 Word 文档读取。问题分两层：.docx（OOXML，ZIP 封装的 XML 集）与 legacy .doc（OLE CFB 二进制）。选库候选与自解路线双通道核实后 PoC 实测对照。

## 关键结论

1. **.docx 选自解路线：zip 8.6 提为直接依赖加 quick-xml 0.42 事件流读 `word/document.xml`**，与 `src\epub.rs` 的 XHTML 文本化同构。zip 已是 rbook 传递依赖（Cargo.lock 树内 8.6.0），quick-xml 已是直接依赖，**零新增 crate**。PoC 保真全过：中文、实体（含数字字符引用）、`w:tab` / `w:br`、`xml:space` 语义、AlternateContent 去重。[实证: 2026-09-01 target\poc-docx cargo run，fixture 与真样本双验]
2. **候选 crate 无一适配 .docx 主通道**：`docx` 1.1.2（2020-04 停更）、`dotext` 0.1.1（2017-12 停更）稳度出局；`rdocx` 0.11.1、`rwml` 0.1.4、`zavora-docx` 0.1.4 太新太窄；`office_oxide` 0.1.9 与 `anydoc` 0.2.4 活跃但均为 2026 年新仓。office_oxide 实测**丢实体**（`a & b 中文` 提成 `a  b 文`，`&amp;` 与 `&#x4e2d;` 直接消失）且**丢 AlternateContent 文本框整段**，文本保真不过关，不作主引擎。[实证: 2026-09-01 PoC 双路线同 fixture 对照]
3. **legacy .doc 首版不做，明确报错提示**。office_oxide 实测可直读真 Word 产的 .doc（Word COM 现造样本，中英文与 `&` 均正确），是候选中唯一像样的纯 Rust .doc 直读 [实证: 同上]；但 0.1.x 太新，且本机用户文档检索全为 .docx、未见 .doc 实例，按 YAGNI 首版不做，真需求出现时以本研究 PoC 结论立项（届时 office_oxide 仍需带保真 fixture 复测版本）。[推断: 需求侧未见实例]
4. `cfb` 0.14（recent 18.6M/90d）极稳但只到 OLE 容器层；.doc 文本层（FIB、piece table、CP 到 FC 映射）协议深，自写不值。[经验: 协议复杂度；推断: 收益不配成本]
5. `anydoc`（firecrawl，19.7k stars，2026-08 建）为全格式转 Markdown 方向，与本项目 TextUnit 管线架构错位，不采用；其底层 PDF 引擎选了同款 pdf-inspector，佐证 S001 选型。[实证: 2026-09-01 crates.io API 依赖清单]

## 现状或实测

### crates.io 通道

| crate | 版本 | license | updated | recent/90d | 判断 |
| --- | --- | --- | --- | --- | --- |
| docx | 1.1.2 | MIT | 2020-04-27 | 1,316 | 停更出局 |
| dotext | 0.1.1 | MIT | 2017-12-03 | 15,400 | 停更出局 |
| office_oxide | 0.1.9 | MIT OR Apache-2.0 | 2026-09-01 | 432,822 | 活跃但 0.1.x；保真硬伤（见 PoC） |
| anydoc | 0.2.4 | MIT | 2026-08-27 | 238,826 | 全格式转换器，架构错位 |
| rdocx | 0.11.1 | MIT OR Apache-2.0 | 2026-08-29 | 4,874 | 2026-02 新仓 |
| rwml | 0.1.4 | MIT | 2026-08-29 | 4,395 | 0 stars 新仓 |
| cfb | 0.14.0 | MIT | 2026-02-13 | 18,627,915 | 容器层稳，文本层不管 |

[实证: 2026-09-01 cargo search / cargo info / crates.io API（UA 带：reader-rs-research）]

### GitHub 通道

| 仓库 | stars | created | pushed | 备注 |
| --- | --- | --- | --- | --- |
| firecrawl/anydoc | 19,748 | 2026-08-03 | 2026-08-28 | 一个月新仓，品牌带动 |
| yfedoseev/office_oxide | 117 | 2026-03-01 | 2026-09-01 | 六格式宣称，营销口径强 |
| tensorbee/rdocx | 37 | 2026-02-22 | 2026-08-31 | 新 |
| iyulab/undoc | 34 | 2025-12-20 | 2026-08-21 | DOCX/XLSX/PPTX 转 Markdown |
| mdsteele/rust-cfb | 63 | 2017-03-12 | 2026-08-28 | 老而稳的 CFB 容器库 |
| PoiScript/docx-rs | 73 | 2018-05-06 | 2024-06-11 | crate 侧 2020 后未发版 |
| HyunjoJung/rwml | 0 | 2026-06-22 | 2026-08-31 | 无星，出局 |

[实证: 2026-09-01 gh repo view 逐仓]

### PoC 双路线实测

> A = zip 加 quick-xml 自解（PoC 里仿 epub.rs 事件流）；B = office_oxide `Document::open().plain_text()`。fixture 由 zip 现造（英文段落、xml:space 两态、中文、tab/br、实体、AlternateContent、2x2 中文表格），legacy .doc 由 Word COM 现造，真样本为本机 Word 产的中文 .docx。

| 用例 | A 自解 | B office_oxide |
| --- | --- | --- |
| 无 preserve 尾空格 | `HelloWorld`（按 OOXML spec 剥） | `Hello World`（保留，宽松） |
| `xml:space="preserve"` | `Hello World` | 同 |
| 中文 | 正确 | 正确 |
| `w:tab` / `w:br` | 制表符 / 断行 | 同 |
| `&amp;` 加 `&#x4e2d;` | `a & b 中文` | `a  b 文`（**实体丢失**） |
| AlternateContent 文本框 | 出一次（skip Fallback） | **整段丢失** |
| 表格 | 每格一行 | 行内 `\t` 拼接（更利搜索上下文） |
| 真样本 .docx（Desktop 测试V2.docx） | 27 行，首三行与 B 一致 | 27 行 |
| legacy .doc（Word COM 现造） | 干净报错（非 zip） | 正确读出（中英文与 `&`） |

[实证: 2026-09-01 target\poc-docx cargo run 三样本]

## 踩坑沉淀

| 现象 | 根因 | 正确处理 |
| --- | --- | --- |
| quick-xml 0.42 属性比较编译错（`str` 对 `[u8; N]`） | 0.42 属性 key/value 与 tag 名一样是 `&str`（S003 已记 tag 名，属性同源） | 与 `&str` 字面量比较；版本敏感 API 以本地源码为准 |
| `w:t` 分段 trim 吞空格（`a&b中文`） | 实体报为独立 GeneralRef 事件，逐片段 trim 破坏整段语义 | 按 `w:t` 整体缓冲（Text 加 GeneralRef 解析），End 时一次 trim |
| office_oxide 实体与文本框丢字 | 其 docx 文本管线未解实体、未处理 mc 回退标记 | 引入前必须带保真 fixture 对照，不自采信营销口径（「100% pass rate」） |
| AlternateContent 同文双份 | Choice 与 Fallback 是新旧两套等价标记 | skip `mc:Fallback` 子树（注意所有输出位点都要挂 skip 守卫，含 tab / tc 补位符） |

## 待办

1. 立项 P0009 时定夺：`UnitKind` 对 docx 无页概念，整篇一单元（新增 `Body` 类标签）还是按 `w:sectPr` 分节；表格行拼接（B 形态）需要在表格内抑制按 `w:p` 断行。纯方案问题，无选型风险。
2. `word/footnotes.xml` / `endnotes.xml` / 页眉页脚暂不读（主正文优先），真实需求出现再评。[假设: 阅读场景正文为主]
3. office_oxide 若日后引入（.doc 直读），复测其 0.2+ 版本实体保真。[假设: 届时已修]
