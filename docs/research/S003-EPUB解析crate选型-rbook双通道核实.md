# S003-EPUB解析crate选型-rbook双通道核实

> 2026-08-31。触发点：用户追加 EPUB 支持需求。流程按 `docs\references\R002-选型研究细则-cratesio与github双通道.md`；六态标准见 G002。

## 背景

Reader 要支持 EPUB。EPUB 即 ZIP 封装的 XHTML 集（spine 定义阅读序），需要：解容器、按 spine 序读章、XHTML 转纯文本。选库问题分两层：EPUB 容器解析、XHTML 文本化。

## 关键结论

1. **EPUB 容器选定 rbook 0.7.10**：Apache-2.0（与 MIT 兼容）、2022-12 创建、2026-07-31 仍活跃、49 stars；高层 API 现成（`Epub::open`、`reader()` 按 spine 序出章、`read_str` 取资源）。[实证: 2026-08-31 cargo info 加 gh repo view DevinSterling/rbook；docs.rs rbook 0.7.10]
2. **`epub` crate（2.1.5）一票出局：GPL-3.0**。下载量再高也不能进 MIT 项目。[实证: 2026-08-31 cargo info epub]
3. `epub-stream`（0.1.0，no_std 嵌入式向）与 `epubie-lib`（0.1.1）太新太窄；mdbook-epub 是写出方向。均不选。[实证: 2026-08-31 cargo search/info]
4. **XHTML 文本化用 quick-xml 自写最小解**：XHTML 在 EPUB 2/3 里要求是良构 XML，quick-xml 走事件流收 text 节点、块级标签断行即可；不引 html2text/htmd 等整转换器（梯子第五档：已够用不加依赖）。quick-xml 是生态事实标准。[推断: EPUB 规范口径属公开常识；quick-xml 地位 [经验]]
5. 测试 EPUB 不存档：rbook 自带 builder（`Epub::builder().chapter(...).save()`），测试现造两章 EPUB，期望值独立。[实证: 2026-08-31 tests\cli.rs 用例全绿]
6. quick-xml 0.42 有两处版本敏感点：tag 名是 `&str`（不再 `&[u8]`）；实体引用报为独立 `GeneralRef` 事件，要显式解析（预定义实体走 `resolve_predefined_entity`，字符引用自解，`nbsp` 特判）。[实证: 2026-08-31 本地源码加编译修正]

## 现状或实测

### crates.io 通道

| crate | 版本 | license | 判断 |
| --- | --- | --- | --- |
| epub | 2.1.5 | GPL-3.0 | 出局（license 不兼容 MIT） |
| rbook | 0.7.10 | Apache-2.0 | 选定 |
| epub-stream | 0.1.0 | MIT | 嵌入式向，0.1.0 太新 |
| epubie-lib | 0.1.1 | 未深挖 | 太新 |

[实证: 2026-08-31 `cargo search epub --registry crates-io` 与逐个 `cargo info`]

### GitHub 通道

- `DevinSterling/rbook`：49 stars，2022-12-30 建，2026-07-31 推，未归档。[实证: 2026-08-31 gh repo view]
- `aeosynth/epub` 仓库名查无（gh 解析失败），不影响结论（GPL 已出局）。[实证: 2026-08-31]
- 终端 EPUB 阅读器生态（bk 338 stars 等）属应用层，非库选型对象。[推断]

### rbook API 核实

- 打开：`Epub::open(path)` / `Epub::read(Read+Seek)`；解析行为可配（strict、skip_toc 等）。[实证: docs.rs 与本地源码 rbook-0.7.10]
- 按序读章：`epub.reader()` 后 `read_next()` 迭代出内容项（inherent 方法，`src\epub\reader.rs:219`），`data.content()` 得 XHTML 串；`LinearBehavior::LinearOnly` 可跳过非线性内容。[实证: 同上]
- builder：`Epub::builder().identifier/title/language/chapter(...).write().save(path)`；`EpubChapter::new("标题")` 会把标题生成为章内首个 heading 行。[实证: 2026-08-31 本机生成加提取回路]

### 真实样本回归

> 样本：《Powershell For Sysadmins - Workflow Automation Made Easy》（用户提供，EPUB，2026-08-31 本机）。

- 提取：37 章 3583 行；章标题、推荐语、正文段落结构正确。[实证: 2026-08-31]
- 搜索：`Get-Process` 命中 14/22 章多行；`-i` 加 `--pages 1-5` 加 `-C 1` 组合正确；`rr` 缩写正常。[实证: 同上]
- 首轮回归发现 pre 代码块被折叠成单行（块内换行被空白折叠吞掉），当场修为 pre 保留换行与行首缩进；修后代码逐行输出。[实证: 同上，tests\epub::tests::xhtml_pre_keeps_line_breaks]
- 次要观察：行间粘连未见；实体（®、’）解码正确；长行主要出现在本就单段的正文。[实证: 同上]

## 踩坑沉淀

| 现象 | 根因 | 正确处理 |
| --- | --- | --- |
| `cargo info epub` 首查空输出 | rsproxy 镜像偶发 DNS 解析失败（lf9-static.rsproxy.cn），重试即得 | 镜像网络抖动时重跑；license 一查到底，GPL 一票出局 |
| quick-xml 0.42 编译错 7 处 | 按 0.3x 记忆写：`local_name()` 返回 `&[u8]`、实体自动解码 | 0.42 tag 名是 `&str`；实体走 GeneralRef 显式解析；版本敏感 API 以本地源码为准不凭记忆 |
| pre 代码块塌成一行 | 文本化对一切块折叠空白 | pre 单列通道：保换行保缩进（真实样本回归暴露） |

## 待办

1. 其它 HTML 实体（非预定义、非 nbsp）直接丢弃；若真实书出现缺字符，再扩实体表或换 html 实体库。[假设: 正规 EPUB 少用]
2. 表格（table/tr/td）目前按行断行不成结构；阶段 2 提取质量时评。
