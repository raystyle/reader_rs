# P0001-PDF文本搜索与提取CLI最小闭环

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：已完成
- 日期：2026-08-31
- 关联：TODO.md / 研究 `docs\research\S001-PDF文本提取crate选型-pdf-inspector双通道核实.md` / 参考 `docs\references\R001-项目定位-Agent原生文档阅读搜索和提取工具.md`（定位现役版，P0002 后）、`docs\references\R003-测试标准细则-分层断言与门禁流程.md`

## 背景与问题

用户要一个 Rust 编写的 PDF 文档文本搜索与提取工具。仓库从零起，文档体系对照 ohmyagents；提取引擎经双通道核选定为 pdf-inspector（用户点名研究，核实后成立）。本方案为首期切面：两个命令跑通、可测试、可验收。

## 目标与非目标

- 目标：
  - `reader search <文件> <关键词>`：按页输出命中行，带页码；支持 `--regex`、`-i`、`-C N`、`--pages`。
  - `reader extract <文件>`：按页分节输出全文到 stdout；支持 `--pages`、`-o` 写文件。
  - `rr` 与 `reader` 同入口双 bin。
  - 集成测试覆盖冒烟、正负例、页过滤；本地门禁三件过。
- 非目标：
  - 不做 OCR、不做渲染与编辑、不做 Markdown 转换、不做批量目录扫描（见 ROADMAP 阶段 2/3）。
  - 不做栏检测级阅读序（行重建为 y 聚类加 x 排序的朴素实现，天花板写进代码注释）。

## 方案

提取层包 pdf-inspector：`extract_text_with_positions_pages(path, page_filter)` 拿带坐标的 TextItem（page 1 起），按页分组后 y 聚类成行、行内 x 排序拼接（间隙超字号 1/4 补空格）。搜索层对重建行做字面/正则匹配，输出行仿 grep：`页:行号:文本`（命中）与 `页-行号-文本`（上下文）。退出码仿 grep：命中 0、无命中 1、出错 2。

```text
reader_rs/
  src\
    main.rs    薄壳，双 bin 同入口（reader / rr）
    lib.rs     clap CLI 定义、run()、页范围解析
    pdf.rs     按页提取与行重建（包 pdf-inspector）
    search.rs  匹配器与命中收集
  tests\
    cli.rs     assert_cmd 集成；lopdf 现造两页测试 PDF
```

关键命令：

```powershell
reader search .\doc.pdf "关键词" -i -C 1
reader search .\doc.pdf "r.st" --regex --pages 2-4
reader extract .\doc.pdf --pages 1-3,5 -o out.txt
```

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| pdf-inspector（选定） | 双通道信号强；位置感知提取；纯 Rust 默认构建。证据见 S001 |
| lopdf 自写提取 | 更底层，要自己处理编码与 CMap，重复造轮子，违反写 Rust 规则 |
| pdf-extract | 无位置信息，做不出按页搜索 |
| pdfium-render | 需外挂 pdfium 二进制，违反纯 Rust 边界 |

## 实施步骤

1. 文档骨架与立项（本方案、S001、三原语）。
2. `src\pdf.rs` / `src\search.rs` / `src\lib.rs` / `src\main.rs`。
3. `tests\cli.rs`：lopdf 现造测试 PDF，冒烟加正负例。
4. 门禁三件加 rumdl 三件套；登记 INDEX 与 diary。

## 风险与回滚

- 风险：lopdf 现造的 PDF 提取不出文本（字体编码不全）。缓解：标准 Helvetica 加 WinAnsiEncoding 是解析器覆盖最完备的路径；若失败改用内嵌最小 PDF 字节模板。回滚：测试夹具独立，换夹具不动产品代码。
- 风险：朴素行重建在多栏排版下串行。缓解：注释写明天花板，阶段 2 换 pdf-inspector layout 管线。

## 实施过程与经验

- 实际怎么做：按计划四步走完，无偏差。`src\pdf.rs` 行重建一次跑通；lopdf 现造测试 PDF 的方案有效（生成、pdf-inspector 提取回路通畅，期望值独立于被测实现）。
- 踩了什么坑 + 怎么解决：
  - 测试夹具 `TestResult` 类型别名写死 `Result<(), _>` 却用作 `TestResult<Command>`，26 个编译错；正解是泛型默认参数 `type TestResult<T = ()> = Result<T, ...>`。编译期即拦截，未进 mistakes。
  - lopdf `Object::from` 需要 owned ObjectId，`page_ids.iter()` 要 `.copied()`。编译期即拦截。
  - 断链扫描把 S001 里外链 URL 内嵌的 `docs/rust-api.md` 当仓内引用报断链；走豁免清单正解，记 `docs\mistakes\M101-文档门禁扫描错误.md`（M001）。
- 沉淀的经验：
  - G001 配套机检真实拦到两处违规（标题带括号、外链路径误判），收尾跑门禁三件套不是形式。
  - 「测试 PDF 现造」免维护二进制样本，且天然满足期望值独立来源，可套用到后续方案。
  - 真实样本冒烟（W3C dummy.pdf）确认双 bin 与提取链路在非自造文件上同样工作。

## 验收标准

- `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked` 全过。[实证: 2026-08-31]
- search 对测试 PDF 命中退出 0 且输出含页码标记行；无命中退出 1；缺文件退出 2。[实证: tests\cli.rs 11 例 2026-08-31]
- extract 输出 `== page N ==` 分节与内嵌文本；`--pages` 过滤正确。[实证: 同上]
- `rumdl check .` 零告警；断链与标题扫描过；INDEX 登记完成。[实证: 2026-08-31]
