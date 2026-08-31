# P0003-EPUB支持与格式分派

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：已完成
- 日期：2026-08-31
- 关联：TODO.md / 研究 `docs\research\S003-EPUB解析crate选型-rbook双通道核实.md` / 参考 `docs\references\R001-项目定位-Agent原生文档阅读搜索和提取工具.md`

## 背景与问题

用户追加需求：除 PDF 外支持 EPUB。当前提取层（`src\pdf.rs`）钉死 PDF，CLI 无格式概念。EPUB 无页码，自然单位是章（spine 序）。需要一次小的结构升级：提取层按扩展名分派，页/章统一为「文本单元」。

## 目标与非目标

- 目标：
  - `reader search/extract` 对 `.epub` 文件开箱可用，参数语义不变（`--pages` 对 EPUB 选章）。
  - 输出分节标记按格式区分：PDF 用 `== page N ==`，EPUB 用 `== chapter N ==`；命中行格式不变（`单元号:行号:文本`）。
  - 测试 EPUB 由 rbook builder 现造；真实样本回归。
- 非目标：
  - 不做 EPUB 元数据/封面/目录命令；不做 DRM 解密；不改 PDF 侧行为。
  - 不做更多格式（docx/mobi 等按需另立项）。

## 方案

```text
src\
  document.rs   格式分派：按扩展名 .pdf/.epub；统一 TextUnit{no, kind, lines}
  pdf.rs        调整为产出 TextUnit（行重建逻辑不变）
  epub.rs       rbook reader 按 spine 序出章；quick-xml 事件流把 XHTML 文本化为行
  search.rs     不变（操作 TextUnit 切片）
  lib.rs        不变加分派调用；未知扩展名报错退出 2
```

XHTML 文本化规则（最小解）：text 节点直接收集；块级标签（p/div/h1-h6/li/tr/section 等）结束处断行；pre 内保留换行与行首缩进；行内标签不断行；实体显式解析（GeneralRef 事件）。天花板注记写代码注释（不处理表格结构、不保留链接目标）。

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| rbook + quick-xml（选定） | license 兼容、活跃、builder 兼做测试夹具。证据见 S003 |
| epub crate | GPL-3.0，出局 |
| 自写 ZIP+XML 容器解析 | 重复造轮子，违反写 Rust 规则 |
| html2text 整转换器 | 超出最小需求（梯子第五档），文本化自写约 60 行 |

## 实施步骤

1. S003 与本方案立项，三原语登记。
2. `src\document.rs` / `src\epub.rs`，改造 `pdf.rs`/`lib.rs` 分派。
3. `tests\cli.rs` 加 EPUB 用例（rbook builder 现造）；真实样本回归回填 S003。
4. 门禁与登记；CHANGELOG 里程碑。

## 风险与回滚

- 风险：真实 EPUB 的 XHTML 非良构导致 quick-xml 解析中断。缓解：非良构处截断本章剩余（尽力而为）；实测样本验证。回滚：EPUB 路径独立成模块，`git revert` 即回。
- 风险：rbook 对某些 EPUB 2 老书兼容性问题。缓解：真实样本回归先行暴露。

## 实施过程与经验

- 实际怎么做：按步骤走完。结构升级为 `TextUnit{no, kind, lines}` 统一页/章，search 层只改字段名（page 改 unit），输出格式对 PDF 完全不变。
- 踩了什么坑 + 怎么解决：
  - quick-xml 0.42 与记忆不符（tag 名 `&str`、实体 GeneralRef 化），7 处编译错；以本地源码为准修正，记 S003 坑表。
  - 行内标签边界丢空格（`Hello <b>EPUB</b>` 变 `HelloEPUB`）：push_text 初版把前块行尾空白丢了；单元测试当场抓到，改为边界空白保留为间隔。
  - pre 代码块塌成一行：真实样本回归暴露；加 pre 专用通道（保换行保缩进），配单元测试 `xhtml_pre_keeps_line_breaks`。
  - rbook builder 会把章标题生成为章内首行 heading，测试期望从 `1:1:` 订正为 `1:2:`（先核实生成物再改断言）。
- 沉淀的经验：
  - 格式扩展的接缝放在「统一文本单元」一层，搜索与输出零改动——第二个格式进来的成本验证了这个抽象恰好够用。
  - 真实样本回归在单元测试全绿后仍能抓到两类问题（pre 折叠、章标题行），「自造夹具 + 真实样本」双层都要。

## 验收标准

- 门禁三件加 rumdl 三件套全过。[实证: 2026-08-31]
- EPUB 测试用例与 PDF 用例对称（命中、过滤、负例、不支持格式退出 2）；cargo test 20 例全绿。[实证: 2026-08-31]
- 真实样本《Powershell For Sysadmins》EPUB：37 章 3583 行，`Get-Process` 等搜索命中正确，回填 S003「真实样本回归」节。[实证: 2026-08-31]
