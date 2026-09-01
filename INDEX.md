# INDEX：项目总索引

> 角色：全仓**唯一索引**——只做定位：编号表、目录结构、代码文件位置。搜索方法见 `AGENTS.md` 四、资源索引。规则权威源见 `AGENTS.md`；命名与编号规则见 `docs\guide\G001-文档标准细则-命名写作规范与rumdl检查.md`。

## 一、编号体系

**前缀定位**：`P`（proven，已完成 plan 归档，4 位）；`S`（research，研究原型过程，3 位）；`R`（references，开发测试参考，3 位）；`G`（guide，元规范，3 位）；`M`（mistakes，分类文件 M1xx、行级错误 M0xx 全局递增不复用）。根目录三原语：`GOAL`（目标轨迹）/ `PLAN`（当前目标方案）/ `TODO`（进度清单）。

**目录职能**：`proven` 已完成 plan 归档；`diary` 一天一篇总结与自省；`research` 研究原型过程（为什么，六态对齐，规范见 G002）；`references` 开发测试参考（要做什么怎么做，六态溯源）；`guide` 元规范（含 `template.md`）；`mistakes` 出错怎么纠（与 references 是经验教训的两面）。

新文档按类别落位，编号接当前最大号，登记进本索引对应节。

## 二、目录结构与代码文件位置

| 类别 | 目录 | 说明 |
| --- | --- | --- |
| 文档 | `docs\`（proven/diary/research/guide/references/mistakes）+ 根目录 GOAL/PLAN/TODO/INDEX/AGENTS/README/CHANGELOG/ROADMAP | 见上节职能 |
| 代码 | `src\` | Rust CLI `reader`（`rr` 同入口双 bin） |
| 测试 | `tests\` | assert_cmd 集成测试；测试 PDF 由 lopdf 现造 |

**代码文件位置**：

| 文件 | 职责 |
| --- | --- |
| `.tools\` | 项目自定义脚本工具归档（`README.md` 含清单与规则；`uv run --script` 载体） |
| `.tools\md-ref-scan.py` | markdown 仓内引用断链扫描（文档大改后回归门禁；豁免清单 `md-ref-allow.txt`） |
| `.tools\md-heading-scan.py` | 标题括号规范机检 |
| `.tools\md-replace.py` | 中文与反斜杠路径安全的字面批量替换 |
| `src\main.rs` | 薄壳入口（reader / rr 双 bin 共用） |
| `src\lib.rs` | clap CLI 定义、`run()` 分发、页/章范围解析 |
| `src\document.rs` | 格式分派与统一文本单元 TextUnit（页/章，含 needs_ocr 信号） |
| `src\pdf.rs` | PDF 页提取（pdf-inspector markdown 布局管线：多栏阅读序、needs_ocr） |
| `src\epub.rs` | EPUB 章提取（rbook spine 序）与 XHTML 文本化（quick-xml） |
| `src\search.rs` | 匹配器（字面/正则/忽略大小写）与命中收集 |
| `src\output.rs` | JSON 包膜（ok/data/error 加 meta）、filter 点路径裁剪、cta 生成 |
| `src\introspect.rs` | agent 自省：`--llms` 紧凑索引与 `skill` SKILL.md 生成（curated 文本） |
| `tests\cli.rs` | CLI 集成冒烟与正负例 |
| `Cargo.toml` | package reader_rs；依赖 pin 与双 bin 定义 |

```text
reader_rs/
  GOAL.md / PLAN.md / TODO.md / INDEX.md   三原语加总索引
  AGENTS.md / README.md / CHANGELOG.md / ROADMAP.md / SKILL.md
  Cargo.toml / LICENSE / .rumdl.toml
  .tools\            自定义脚本工具（md-ref-scan / md-heading-scan / md-replace）
  src\
    main.rs  lib.rs  document.rs  pdf.rs  epub.rs  search.rs  output.rs  introspect.rs
  tests\
    cli.rs
  docs\
    proven\      P 编号，已完成 plan 归档
    diary\       一天一篇总结自省
    research\    S 编号，研究原型过程（六态）
    references\  R 编号，开发测试参考
    guide\       G 编号，元规范；template.md
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

## 四、项目日记

> 位置 `docs\diary\`；一天一篇总结自省。

- `2026-08-31-对照ohmyagents建立文档骨架.md`

## 五、研究文档

> 位置 `docs\research\`；S 编号。

| 编号 | 文件 | 主题 |
| --- | --- | --- |
| S001 | `S001-PDF文本提取crate选型-pdf-inspector双通道核实.md` | 提取引擎选型（双通道） |
| S002 | `S002-incurs模块经验研究-Agent原生CLI的命令输出与帮助设计.md` | Agent 原生输出/帮助设计借鉴 |
| S003 | `S003-EPUB解析crate选型-rbook双通道核实.md` | EPUB 解析选型（rbook 加 quick-xml） |

## 六、开发测试参考

> 位置 `docs\references\`；R 编号。

| 编号 | 文件 | 用途 |
| --- | --- | --- |
| R001 | `R001-项目定位-Agent原生文档阅读搜索和提取工具.md` | 现役定位展开（P0002 后） |
| R002 | `R002-选型研究细则-cratesio与github双通道.md` | 选库检索双通道 |
| R003 | `R003-测试标准细则-分层断言与门禁流程.md` | 测试分层、断言、门禁 |

## 七、元规范

> 位置 `docs\guide\`；G 编号。

| 编号 | 文件 | 用途 |
| --- | --- | --- |
| G001 | `G001-文档标准细则-命名写作规范与rumdl检查.md` | 命名与编号、写作、rumdl |
| G002 | `G002-研究标准细则-结构与六态标记.md` | 研究结构与六态 |
| G003 | `G003-工作流标准细则-从登记到归档五步.md` | 五步工作流与优先级 |
| — | `template.md` | 方案模板（不编号） |

## 八、错误速查

> 位置 `docs\mistakes\`；分类文件 M1xx，行级 M0xx。

| 编号 | 分类文件 | 覆盖关键词 | 行级编号段 |
| --- | --- | --- | --- |
| M101 | `M101-文档门禁扫描错误.md` | rumdl、断链扫描、标题括号、豁免清单 | M001 |
| M102 | `M102-Windows路径与shell错误.md` | MSYS 路径、os error 3、引号、原生二进制 | M002 |
| M103 | `M103-开发环境安装错误.md` | 架构不配、bad CPU type、接管机装工具 | M003 |
| M104 | `M104-CI与发布流水线错误.md` | runner 退役、上传竞态、资产命名 | M004 |

迭代规则：踩坑按当前最大号接编 MNNN 进对应分类文件；一行一事；同根因或同型坑可合并聚合进已有条目（保留最早编号与首踩日期）；反复踩落 `docs\research\`；新分类文件登记本节。

## 九、阶段与版本

- `ROADMAP.md`：阶段路线
- `CHANGELOG.md`：版本里程碑
