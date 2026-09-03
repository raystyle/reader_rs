# AGENTS.md

本文件是协作规则的**最高约束**，四段职责依次为：**项目定位**、**工作规则**、**意图路由**、**文档索引**。

## 一、项目定位

> 本项目的本质与边界。根为定位，下分本质、边界、交互对象。

1. **本质**
   - Reader 是 Agent 原生文档阅读、搜索和提取工具：为 Agent 管线设计的 Rust CLI，从本地文档读文本层：按页读、按词/正则搜、按页取。

2. **边界**
   - 服务对象先 Agent 后人：输出稳定可解析（行式标记、grep 语义退出码 0/1/2）、单调用完成一件事、无交互无守护进程、错误走 stderr；机器可读优先于人类美观。
   - 当前只读支持 PDF、markdown（.md/.markdown）与 anydoc 家族（Word 含 legacy .doc、EPUB、ODT、RTF、PowerPoint、Excel、ODF、CSV），不做渲染、编辑；OCR 仅以 `--ocr` 兜底形式支持（仅 PDF 单文件 needs_ocr 页，hayro 渲染加 ppocr-rs 原生 CPU 内核跑 PP-OCRv6 tiny，P0014/P0018；默认不识别只提示）；结构化提取以 `query` 子命令嵌 mq-lang（mq 表达式，P0016）；其它格式按需另立项。
   - 文本质量只承诺英文与中文；其它语言不做质量承诺，不可靠页以 needs_ocr 提示兜底。
   - CLI 是唯一交互面；纯 Rust 单二进制，不外挂 pdfium 等二进制运行时。
   - Windows 优先验证；依赖均跨平台，不主动破坏其它平台。

3. **交互对象**
   - 本地文档文件（只读交互：读文本层，不渲染不编辑；首版 PDF）。
   - 调用方（Agent 或人）经 CLI：`reader` 命令（等价缩写 `rr`，同一二进制两个名字）。
   - 定位展开见 `docs\references\R001-项目定位-Agent原生文档阅读搜索和提取工具.md`；历史方案与研究一律查 `INDEX.md`，不在本节维护清单。

## 二、工作规则

> 四类场景：**对话**、**操作**、**编码**、**文档**。前三类先列动作清单，再定规则（可以 / 禁止 / 参考）；文档类是动作后的对齐义务：什么动作、到什么状态、必须对齐撰写哪些文档。

### 对话

**动作清单**：新需求提出、追问链澄清、裁定与验收反馈、踩坑上报、问题发现。

**规则**：

1. **每轮对话**
   - 可以：先核对四原语 `PRD.md`（需求清单，要什么）、`GOAL.md`（理解 PRD 后定下的目标和达成标准）、`PLAN.md`（规划，怎么做）、`TODO.md`（进度，做到哪）；需求驱动目标：新需求先入 PRD 走追问链，澄清后登记；GOAL 目标须回指 PRD 编号；实质推进当场更新原语；踩坑当场落 `docs\mistakes\`（接编 MNNN，同根因合并）；发现问题走五步闭环。
   - 禁止：不核对四原语就干活；偏离当前目标；推进了不更新原语；替用户静默假设需求；问题与坑只留在对话里。
   - 参考：四原语；`docs\references\R007-工作流标准细则-从登记到归档五步.md`。

### 操作

**动作清单**：执行 shell 命令与读写文件、跑门禁（cargo 三件加 rumdl 四件）、运行项目脚本工具、git 变更（add/commit/push/tag）、发布（tag 触发流水线）。

**规则**：

2. **执行命令与写文件时**
   - 可以：Windows 命令用 PowerShell 7（`pwsh`），Linux / macOS / WSL 用该平台常规 shell；Markdown / Rust 源码 UTF-8；Windows 上需兼容 5.1 的脚本用 UTF-8 BOM。
   - 禁止：Windows 上默认用 `powershell.exe` 5.1；无 BOM 的中文 ps1 给 5.1 读。
   - 参考：`docs\mistakes\` M101 / M102（路径、编码、管道坑）。

3. **提交与 git 管理时**
   - 可以：`feat:` / `docs:` / `fix:` / `chore:` / `test:` 前缀加中文描述；一次提交只做一件事。
   - 禁止：多事混一提交；未经指示做 `git commit` / `push` / `reset` / `rebase` 等 git 变更操作。
   - 参考：发布流程见 `.github\workflows\release.yml`（tag 与 Cargo.toml version 一致性闸）。

### 编码

**动作清单**：写改 Rust 源码、改依赖与构建配置、写测试（六层各载体）、写临时/工具脚本、做研究出 PoC 原型。

**规则**：

4. **写 Rust 时**
   - 可以：先查 crates.io / docs.rs / GitHub 上是否已有最流行、最稳定、或已经覆盖本需求的库，检索走双通道细则 `docs\references\R002-选型研究细则-cratesio与github双通道.md`；选定后用最少代码接上，优先组合而不是自写协议、解析、CLI 解析；实现取舍照懒人阶梯（YAGNI、仓里已有复用、标准库优先、已装依赖优先、最少代码，见 ohmyagents S012 同源经验）。
   - 禁止：在现成库已能稳定完成的前提下从零实现；为风格引入冷门或实验 crate；一次拉一堆用不上的依赖。
   - 参考：R002；`docs\guide\G007-RustCLI工程基线-供稿要点逐项裁定.md`。

5. **写测试时**
   - 可以：遵守 `docs\guide\G005-测试标准细则-分层断言与门禁流程.md`（原生地基：单元/集成/文档测试，集成优先、期望值独立来源、稳定字段断言、`dies_` 负例前缀、`TestResult` 错误传播）与 `docs\guide\G006-测试体系细则-六层分层与各层标准.md`（目的流程四层：冒烟/回归/验收/A/B 落点与口径）；载体裁定（D31 第 2 轮）：冒烟/回归/验收归 cargo test 体系（`tests\smoke.rs` 等独立 target），只有 A/B 跑批用 uv 运行时 Python（`.tools\ab_run.py`）。测试资源：夹具一律现造（lopdf/rbook/zip）落系统临时目录；入仓资产只放 `tests\assets\`（如 legacy.doc）与 `tests\ab\assets\`（合成样本）；外部真样本（CLR 书、安全牛 PDF）不入仓，钉 sha256 登记在 `tests\regress.rs` 与 `tests\ab\manifest.json`，缺失即跳过不算失败。
   - 禁止：重言式断言；公开 API 测试塞 `mod tests{}` 不进 `tests\`；默认 mock；计时进断言；只测 happy path；A/B 检查点从被测输出反抄；外部版权样本进仓。
   - 参考：G005、G006；A/B 协议见 `tests\ab\README.md`。

6. **写临时脚本时**
   - 可以：按需自定义的 ps1 / py / Rust 工具，有复用价值即归档 `.tools\`（规则与清单见 `.tools\README.md`；Python 带 PEP 723 头，用 `uv run --script` 运行）；文档结构大改后跑 `uv run --script .tools\md-ref-scan.py` 做断链回归。
   - 禁止：可复用脚本散落仓库根或只留在对话里；归档不带自述与用法。
   - 参考：`.tools\README.md`。

7. **做研究出原型时**
   - 可以：研究的产物是 PoC 原型（D30）：可运行验证落 `poc\<S编号-主题短名>\`（约定与登记表见 `poc\README.md`；源码、清单、脚本、自述入仓，构建产物与模型大件 gitignore）；实测结论回填对应 S 文档并标六态；上游 clone 实测类不搬源码，只在登记表记指针。
   - 禁止：PoC 散落在 `target\` 或仓根临时工区；模型与大文件进 git；有 PoC 无 S 文档结论。
   - 参考：`poc\README.md`；G002 八节。

### 文档

> 对齐义务：什么动作后、到什么状态、必须对齐撰写哪些文档。写完即过文档门禁四件。

| 动作（类别） | 状态时机 | 文档义务 |
| --- | --- | --- |
| 新需求提出（对话） | 提出时 | `PRD.md` 登记新需求行 |
| 追问链澄清（对话） | 澄清完成 | PRD 状态流转加澄清轮次与裁定 |
| 目标立项（对话） | 开工前 | GOAL 起点与锚点、PLAN 方案、TODO 清单 |
| 选型与调研（编码前置） | 研究完成 | S 文档（六态）加 PoC 登记（poc\）加 INDEX 研究节 |
| 写改源码与配置（编码） | 改动完成 | README / SKILL / 意图路由同步；行为基线变化同步 G006 三节；版本级成果进 CHANGELOG |
| 写测试（编码） | 新层或新面 | G006 落点与基线表同步；INDEX 测试行 |
| 写脚本（编码） | 归档时 | `.tools\README.md` 清单行 |
| 出 PoC（编码） | 原型完成 | `poc\README.md` 登记表加 S 文档结论回填 |
| 踩坑（任何动作中） | 当场 | `docs\mistakes\` 接编一行；INDEX 错误速查节同步 |
| 方案达成（操作收尾） | 验收全绿 | proven 归档（P 编号）、GOAL 历史行、INDEX 归档节 |
| 每次提交（操作） | 提交后 | diary 当天记钩子 |
| 发布（操作） | tag 推送后 | CHANGELOG 封版、ROADMAP 阶段状态 |
| 文档结构变更（文档） | 改名移目录后 | INDEX 同步；`md-ref-scan.py` 断链回归必跑 |

**写作标准**（写任何文档时）：

- 可以：遵守 `docs\guide\G001-文档标准细则-命名写作规范与rumdl检查.md`（树形、标题干净、文件名即标题、rumdl）与 `docs\guide\G004-写作规范细则-禁用字符与机械判定.md`（四类禁用字符：破折号、箭头、emoji、非法全角；全部文档须过 md-char-scan）；事实性断言标六态（标准见 G002）。
- 禁止：标题带括号、口号或破折号（解释放标题下一行引用 `>`）；整段混杂不成树；把「没验证」写成「已验证」；断言不标六态。

## 三、意图路由

> 需求意图与操作方法的映射。显示名 Reader；仓库 `reader_rs`；CLI 二进制 `reader`（缩写 `rr`）。

- **搜文本**：`reader search <文件|目录> <关键词>`（.pdf、.md/.markdown 及 anydoc 家族 .doc/.docx/.epub/.odt/.rtf/.ppt(x)/.xls(x)/.ods/.odp/.csv；`--regex` 正则、`-i` 忽略大小写、`-C N` 上下文、`--pages 1-3,5` 限页/节；命中退出 0、无命中退出 1、出错退出 2；`--format json` 包膜、`--filter` 点路径裁剪；`--ocr` 对 PDF 单文件 needs_ocr 页兜底识别（PP-OCRv6 默认 tiny 约 6.2MB，`READER_OCR_MODEL_SIZE=small` 切质量档；首用下载进缓存目录，`--offline` 禁下载）；目录输入递归批量搜，命中行带路径前缀，`--pages` 与 `--ocr` 不可用）
- **结构化提取**：`reader query <文件> <mq表达式>`（jq 风格：`.h2`/`.code`/`.link`/`.table`/select 管道，完整语法见 mqlang.org；全格式面转 markdown 后查询；退出码 0/1/2 同 search；不支持目录）
- **提文本**：`reader extract <文件>`（`--pages` 限页/节、`-o` 写文件；按单元输出 `== page N ==`（PDF）/ `== section N ==`（其余，标题分节）分节；输出 markdown 形态（PDF 布局管线 / anydoc GFM），不可靠页节头后给 `[needs_ocr: 原因]` 提示行；`--ocr`/`--offline` 同上对 PDF needs_ocr 页兜底；`--format json` 包膜、`--filter` 裁剪、`--offset/--limit` 分页带 `next_offset` 与 `cta`）
- **agent 发现**：`reader --llms`（紧凑命令索引）、`reader skill`（生成 SKILL.md；仓根 `SKILL.md` 与运行时输出逐字节一致，测试守卫）
- **自升级**：`reader self update [--force]`（GitHub Releases 最新正式版，资产 sha256 digest 校验后替换自身与兄弟二进制；GH_TOKEN 注入认证，限流回退 gh api）
- **发布**：`git tag v<版本>` 推 tag 触发 `.github\workflows\release.yml`（tag 须与 Cargo.toml version 一致，流水线有闸）；资产命名 `reader-v<版本>-<target>.<zip|tar.gz>`
- **构建测试**：`cargo build` / `cargo test`；本地门禁 `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked`
- **分层跑批**：`cargo test --test smoke`（冒烟）、`cargo test --test regress`（真样本回归）、`cargo test --test snapshot`（insta 输出快照）、`cargo test --test accept`（验收 cucumber BDD，场景在 tests\features\）；六层口径见 G006
- **A/B 对比**：`uv run --script .tools\ab_run.py --a tiny --b small`（tests\ab\ 对象资源加检查点，OCR 模型档位质量与性能对比，报告落 tests\ab\reports\；标准见 G006）
- **查文档**：先搜 `INDEX.md` 定位编号，再读文件
- **文档门禁**：`rumdl check .`、`uv run --script .tools\md-ref-scan.py`、`uv run --script .tools\md-heading-scan.py`、`uv run --script .tools\md-char-scan.py`（G004 禁用字符）

已落地：`search`（含目录批量）、`extract`、`query`（mq 结构化提取，P0016）、`skill`、`--llms`、超长单元 part 分片、`--ocr` OCR 兜底（仅 PDF 单文件 needs_ocr 页，P0014）、`self update`（P0015）。候选方向与已拒绝项（MCP、TOON 等）登记在 `PRD.md` 需求清单，禁止假装未落地能力已经可跑。

## 四、文档索引

> 定位看 `INDEX.md`（项目根目录，唯一索引：编号表、目录结构、代码文件位置）。本节是配合 INDEX 的搜索方法。

**速记**：前缀定位 `P`（proven 归档）/ `S`（research 研究）/ `R`（references 做事的流程）/ `G`（guide 做事的规范）/ `M`（mistakes 错误；文件 M1xx、行级 M0xx）；根目录四原语 `PRD`（需求清单）/ `GOAL`（目标与达成标准）/ `PLAN`（规划）/ `TODO`（进度）。

**目录职责**：`docs\proven\` 已完成方案的**历史归档**（封存：做成了什么、当时的方案与依据，不再更新）；`docs\diary\` 一天一篇总结自省；`docs\research\` 研究（为什么，六态）；`docs\references\` **现役**做事的流程（操作手册与流程细则，下次照着做，持续更新）；`docs\guide\` 做事的规范（标准与禁令）；`docs\mistakes\` 出错怎么纠；`poc\` 研究原型产物（S 编号前缀子目录，产物与模型 gitignore）；`tests\` 集成测试加三层跑批 target（smoke / regress / accept）加快照（snapshot）加 `tests\ab\` A/B 层；`.tools\` 项目脚本工具（清单见 `.tools\README.md`）。分界：方案做成归档进 proven（历史）；可复用的流程提炼进 references（现役）；标准禁令进 guide。

**搜索方法（文档）**：

```powershell
rg -n "关键词" INDEX.md                        # 1 先搜总索引，定位编号或文件
rg --files docs | rg 关键词                     # 2 按文件名搜文档
rg -n "关键词" docs\research docs\references    # 3 全文搜研究参考
rg -n "关键词" docs\mistakes\                   # 4 搜错误处理
```

**分析路径**：改产品行为先读 `docs\references\`（做事的流程）再回 `docs\research\`（为什么）；规范禁令查 `docs\guide\`（做事的规范）；踩坑查 `docs\mistakes\`；写码选库走 R002；写测试走 G005 加 G006；新想法走 R007 五步；定位代码先 INDEX 模块表。
