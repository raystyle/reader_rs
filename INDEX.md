# INDEX：项目总索引

> 角色：全仓**唯一索引**：只做定位：编号表、目录结构、代码文件位置。搜索方法见 `AGENTS.md` 四、资源索引。规则权威源见 `AGENTS.md`；命名与编号规则见 `docs\guide\G001-文档标准细则-命名写作规范与rumdl检查.md`。

## 一、编号体系

**前缀定位**：`P`（proven，已完成 plan 归档，4 位）；`S`（research，研究原型过程，3 位）；`R`（references，做事的流程，3 位）；`G`（guide，做事的规范，3 位）；`M`（mistakes，分类文件 M1xx、行级错误 M0xx 全局递增不复用）。根目录四原语：`PRD`（需求清单）/ `GOAL`（目标与达成标准）/ `PLAN`（当前目标规划）/ `TODO`（进度清单）。

**目录职能**：`proven` 已完成 plan 归档；`diary` 一天一篇总结与自省；`research` 研究原型过程（为什么，六态对齐，规范见 G002；PoC 产物落 `poc\`）；`references` 做事的流程（要做什么怎么做，操作手册与流程细则）；`guide` 做事的规范（标准与禁令，含 `template.md`）；`mistakes` 出错怎么纠（与 references 是经验教训的两面）。

新文档按类别落位，编号接当前最大号，登记进本索引对应节。

## 二、目录结构与代码文件位置

| 类别 | 目录 | 说明 |
| --- | --- | --- |
| 文档 | `docs\`（proven/diary/research/guide/references/mistakes）+ 根目录 PRD/GOAL/PLAN/TODO/INDEX/AGENTS/README/CHANGELOG/ROADMAP | 见上节职能 |
| 代码 | `src\` | Rust CLI `reader`（`rr` 同入口双 bin） |
| 测试 | `tests\` | assert_cmd 集成测试（cli.rs；测试 PDF 由 lopdf 现造、EPUB 由 rbook 现造、docx 由 zip 现造；legacy .doc 用仓内资产）；冒烟/回归/验收三层 uv 脚本（smoke.py / regress.py / accept.py，D31）；`tests\ab\` A/B 对比层（manifest 对象资源加 expectations 检查点加 reports 报告，G006） |

**代码文件位置**：

| 文件 | 职责 |
| --- | --- |
| `.tools\` | 项目自定义脚本工具归档（`README.md` 含清单与规则；`uv run --script` 载体） |
| `.tools\md-ref-scan.py` | markdown 仓内引用断链扫描（文档大改后回归门禁；豁免清单 `md-ref-allow.txt`） |
| `.tools\md-heading-scan.py` | 标题括号规范机检 |
| `.tools\md-char-scan.py` | G004 禁用字符机检 |
| `.tools\md-replace.py` | 中文与反斜杠路径安全的字面批量替换 |
| `.tools\make-scan-sample.py` | tests\ab 合成扫描件样本生成（无文本层 PDF 加独立检查点） |
| `.tools\ab_run.py` | A/B 对比跑批器（tests\ab 层，质量加性能报告） |
| `src\main.rs` | 薄壳入口（reader / rr 双 bin 共用） |
| `src\lib.rs` | clap CLI 定义、`run()` 分发、页/章范围解析 |
| `src\document.rs` | 格式分派与统一文本单元 TextUnit（页/节，含 needs_ocr 信号） |
| `src\pdf.rs` | PDF 页提取（pdf-inspector markdown 布局管线：多栏阅读序、needs_ocr） |
| `src\anydoc.rs` | anydoc 家族提取（Word/EPUB/ODT/RTF/Office/CSV 出 GFM，按顶层标题分节，超 200 行单元切 part；P0009-P0011） |
| `src\batch.rs` | 批量目录搜索（递归走查加两形态聚合；P0012） |
| `src\search.rs` | 匹配器（字面/正则/忽略大小写）与命中收集 |
| `src\output.rs` | JSON 包膜（ok/data/error 加 meta）、filter 点路径裁剪、cta 生成 |
| `src\introspect.rs` | agent 自省：`--llms` 紧凑索引与 `skill` SKILL.md 生成（curated 文本） |
| `src\ocr.rs` | OCR 兜底（P0014）：hayro 渲染 needs_ocr 页加 tract 推理 PP-OCRv5 mobile；模型三件 ModelScope 下载加 SHA-256 钉死、进程内 strip value_info；`READER_OCR_CACHE_DIR` 覆盖缓存目录 |
| `src\selfupdate.rs` | self update（P0015）：latest 元数据（GH_TOKEN 加 gh api 兜底）、版本判新、资产 digest 校验、zip/tar.gz 解包、staged 加 rename 替换自身与兄弟 |
| `src\query.rs` | mq 结构化提取（P0016）：格式转 markdown 文本（md 原文/anydoc GFM/PDF 管线）加 mq-lang eval，空渲染过滤 |
| `tests\cli.rs` | CLI 集成冒烟与正负例（夹具现造；legacy .doc 仓内资产） |
| `tests\smoke.py` / `regress.py` / `accept.py` | 冒烟/回归/验收三层 uv 跑批脚本（PEP 723 单文件，D31；G006 载体规则） |
| `tests\assets\legacy.doc` | legacy Word 二进制测试资产（Word COM 现造，CI 无 Word 不能现造；P0009） |
| `Cargo.toml` | package reader_rs；依赖 pin 与双 bin 定义 |

```text
reader_rs/
  PRD.md / GOAL.md / PLAN.md / TODO.md / INDEX.md   四原语加总索引
  AGENTS.md / README.md / CHANGELOG.md / ROADMAP.md / SKILL.md
  Cargo.toml / LICENSE / .rumdl.toml
  .tools\            自定义脚本工具（md 四件门禁加 make-scan-sample / ab_run）
  poc\               研究原型产物（S 编号前缀子目录；产物与模型 gitignore）
  src\
    main.rs  lib.rs  document.rs  pdf.rs  anydoc.rs  search.rs  output.rs  introspect.rs  ocr.rs  selfupdate.rs  query.rs
  tests\
    cli.rs  assets\legacy.doc
    smoke.py / regress.py / accept.py   冒烟/回归/验收 uv 跑批脚本
    ab\      A/B 对比层（manifest / expectations / assets / reports）
  docs\
    proven\      P 编号，已完成 plan 归档
    diary\       一天一篇总结自省
    research\    S 编号，研究原型过程（六态）
    references\  R 编号，做事的流程
    guide\       G 编号，做事的规范；template.md
    mistakes\    M1xx 分类文件，行级 M0xx
```

## 三、方案归档

> 位置 `docs\proven\`。

| 编号 | 文件 | 主题 |
| --- | --- | --- |
| P0001 | `P0001-PDF文本搜索与提取CLI最小闭环.md` | 首期切面（已完成 2026-08-31） |
| P0002 | `P0002-项目重新定位-Agent原生文档阅读搜索和提取工具.md` | 现役定位（已完成 2026-08-31） |
| P0003 | `P0003-EPUB支持与格式分派.md` | EPUB 支持（已完成 2026-08-31） |
| P0004 | `P0004-mac与Linux接管开发与跨平台兼容.md` | CI 三系统门禁（已完成 2026-08-31） |
| P0005 | `P0005-PDF提取质量-markdown管线与needs_ocr提示.md` | 阶段 2 提取质量（已完成 2026-08-31） |
| P0006 | `P0006-输出形态-json包膜与分页裁剪.md` | 阶段 3 第一刀（已完成 2026-08-31） |
| P0007 | `P0007-Agent自省与发现-llms索引SKILL生成与help示例.md` | 阶段 3 第二刀（已完成 2026-08-31） |
| P0008 | `P0008-封版v0.1与三端二进制release.md` | 封版 v0.1.0 与 release 流水线（已完成 2026-08-31） |
| P0009 | `P0009-anydoc统一文档引擎大重构.md` | anydoc 统一引擎与 14 格式（已完成 2026-09-01，v0.2.0 已发布） |
| P0010 | `P0010-无标题长文档行分片.md` | 无标题文档 part 分片（已完成 2026-09-01） |
| P0011 | `P0011-超长节再分片.md` | 超长节 part 再分片（已完成 2026-09-01） |
| P0012 | `P0012-批量目录搜索.md` | 批量目录搜索（已完成 2026-09-01） |
| P0013 | `P0013-musl静态Linux资产.md` | musl 静态资产（已完成 2026-09-01，v0.2.1 首发实测） |
| P0014 | `P0014-OCR兜底落地.md` | `--ocr` 兜底与模型管理（已完成 2026-09-03） |
| P0015 | `P0015-self-update.md` | self update（已完成 2026-09-03） |
| P0016 | `P0016-markdown支持与mq结构化提取.md` | .md 进格式面加 mq query（已完成 2026-09-03） |
| P0017 | `P0017-OCR性能优化-宽度分组分批加组间并行.md` | OCR 提速 20.5s 到 3-5.5s/页（已完成 2026-09-03；P0018 换引擎后代码面退役） |
| P0018 | `P0018-OCR换引擎ppocr-rs.md` | OCR 换 ppocr-rs PP-OCRv6 tiny（已完成 2026-09-03） |

## 四、项目日记

> 位置 `docs\diary\`；一天一篇总结自省。

- `2026-08-31-对照ohmyagents建立文档骨架.md`
- `2026-09-01-S004选型反复与P0009-anydoc大重构.md`
- `2026-09-02-S006内嵌OCR选型研究与PoC实证.md`
- `2026-09-03-P0014-OCR兜底落地.md`

## 五、研究文档

> 位置 `docs\research\`；S 编号。

| 编号 | 文件 | 主题 |
| --- | --- | --- |
| S001 | `S001-PDF文本提取crate选型-pdf-inspector双通道核实.md` | 提取引擎选型（双通道） |
| S002 | `S002-incurs模块经验研究-Agent原生CLI的命令输出与帮助设计.md` | Agent 原生输出/帮助设计借鉴 |
| S003 | `S003-EPUB解析crate选型-rbook双通道核实.md` | EPUB 解析选型（rbook 加 quick-xml） |
| S004 | `S004-Word文档读取选型-docx自解与doc直读双路线实测.md` | Word 支持选型（anydoc 统一引擎，决策变更记录在内） |
| S005 | `S005-TOON输出形态收益实测-Reader真实样本上不成立.md` | TOON 收益实测（不引入，销候选） |
| S006 | `S006-内嵌OCR选型-纯Rust管线hayro加pure-onnx-ocr实测可行.md` | 内嵌 OCR 选型（纯 Rust 管线实测可行，已落地 P0014） |
| S007 | `S007-markdown支持选型-学习mq嵌mq-lang全引擎加零依赖分节.md` | markdown 支持选型（学习 mq，已落地 P0016） |
| S008 | `S008-OCR质量升级-ppocr-rs的PP-OCRv6原生内核双优胜出现管线换引擎.md` | OCR 质量升级（v6 tiny 双优，已落地 P0018） |

## 六、references：做事的流程

> 位置 `docs\references\`；R 编号。**做事的流程**：操作手册、流程细则。

| 编号 | 文件 | 用途 |
| --- | --- | --- |
| R001 | `R001-项目定位-Agent原生文档阅读搜索和提取工具.md` | 现役定位展开（P0002 后） |
| R002 | `R002-选型研究细则-cratesio与github双通道.md` | 选库检索双通道 |
| R004 | `R004-Linux实机验收清单-门禁真样本与musl预建.md` | Linux 接管验收操作手册（P0011-P0013 轮，已回填全绿含 M007） |
| R005 | `R005-mac接管开发验收清单-门禁真样本与交叉预建.md` | mac 接管开发验收操作手册（P0011-P0013 轮接续，已回填全绿含 M007 验点） |
| R007 | `R007-工作流标准细则-从登记到归档五步.md` | 五步工作流与优先级（2026-09-03 自 guide 迁入，原 G003） |

## 七、guide：做事的规范

> 位置 `docs\guide\`；G 编号。**做事的规范**：标准与禁令。

| 编号 | 文件 | 用途 |
| --- | --- | --- |
| G001 | `G001-文档标准细则-命名写作规范与rumdl检查.md` | 命名与编号、写作、rumdl |
| G002 | `G002-研究标准细则-结构与六态标记.md` | 研究结构与六态；PoC 产物约定（八节） |
| G004 | `G004-写作规范细则-禁用字符与机械判定.md` | 四类禁用字符、豁免区、md-char-scan 门禁与存量基线 |
| G005 | `G005-测试标准细则-分层断言与门禁流程.md` | 测试分层、断言、门禁（2026-09-03 自 references 迁入，原 R003） |
| G006 | `G006-测试体系细则-六层分层与各层标准.md` | 六层测试体系（单元/集成/冒烟/回归/验收/A/B）落点与口径；真样本基线登记（D28/D29；2026-09-03 自 references 迁入，原 R006） |
| - | `template.md` | 方案模板（不编号） |

编号退役注记：2026-09-03 按「references 是做事的流程、guide 是做事的规范」归类修正，R003 改 G005、R006 改 G006、G003 改 R007；退役编号不复用。

## 八、错误速查

> 位置 `docs\mistakes\`；分类文件 M1xx，行级 M0xx。

| 编号 | 分类文件 | 覆盖关键词 | 行级编号段 |
| --- | --- | --- | --- |
| M101 | `M101-文档门禁扫描错误.md` | rumdl、断链扫描、标题括号、豁免清单、门禁退出码被管道吞 | M001 M006 |
| M102 | `M102-Windows路径与shell错误.md` | MSYS 路径、os error 3、引号、原生二进制、跨平台路径拼接、SIGPIPE 管道早退、cmd 风格重定向 | M002 M005 M007 M008 |
| M103 | `M103-开发环境安装错误.md` | 架构不配、bad CPU type、接管机装工具 | M003 |
| M104 | `M104-CI与发布流水线错误.md` | runner 退役、上传竞态、资产命名 | M004 |
| M105 | `M105-Rust依赖与库行为错误.md` | ureq 响应上限、vendor 库 println 污染 stdout、引擎非 Send/Sync、flate2 后端 | M009 M010 M011 M012 |

迭代规则：踩坑按当前最大号接编 MNNN 进对应分类文件；一行一事；同根因或同型坑可合并聚合进已有条目（保留最早编号与首踩日期）；反复踩落 `docs\research\`；新分类文件登记本节。

## 九、阶段与版本

- `ROADMAP.md`：阶段路线
- `CHANGELOG.md`：版本里程碑
