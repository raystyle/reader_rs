# AGENTS.md

本文件是协作规则的**最高约束**，四段职责依次为：**项目定位**、**操作规则**、**意图路由**、**资源索引**。

## 一、项目定位

> 本项目的本质与边界。根为定位，下分本质、边界、管理对象、方案索引。

1. **本质**
   - Reader 是 Agent 原生文档阅读、搜索和提取工具：为 Agent 管线设计的 Rust CLI，从本地文档读文本层：按页读、按词/正则搜、按页取。

2. **边界**
   - 服务对象先 Agent 后人：输出稳定可解析（行式标记、grep 语义退出码 0/1/2）、单调用完成一件事、无交互无守护进程、错误走 stderr；机器可读优先于人类美观。
   - 当前只读支持 PDF、markdown（.md/.markdown）与 anydoc 家族（Word 含 legacy .doc、EPUB、ODT、RTF、PowerPoint、Excel、ODF、CSV），不做渲染、编辑；OCR 仅以 `--ocr` 兜底形式支持（仅 PDF 单文件 needs_ocr 页，hayro 渲染加 ppocr-rs 原生 CPU 内核跑 PP-OCRv6 tiny，P0014/P0018；默认不识别只提示）；结构化提取以 `query` 子命令嵌 mq-lang（mq 表达式，P0016）；其它格式按需另立项。
   - 文本质量只承诺英文与中文；其它语言不做质量承诺，不可靠页以 needs_ocr 提示兜底。
   - CLI 是唯一交互面；纯 Rust 单二进制，不外挂 pdfium 等二进制运行时。
   - Windows 优先验证；依赖均跨平台，不主动破坏其它平台。

3. **管理对象**
   - 本地文档文件（只读；首版 PDF）。
   - 调用方（Agent 或人）经 CLI：`reader` 命令（等价缩写 `rr`，同一二进制两个名字）。

4. **方案索引**
   - 定位：`docs\references\R001-项目定位-Agent原生文档阅读搜索和提取工具.md`
   - 定位变更：`docs\proven\P0002-项目重新定位-Agent原生文档阅读搜索和提取工具.md`；首期切面 `docs\proven\P0001-PDF文本搜索与提取CLI最小闭环.md`
   - 选型研究：`docs\research\S001-PDF文本提取crate选型-pdf-inspector双通道核实.md`
   - 统一引擎：anydoc 重构方案 `docs\proven\P0009-anydoc统一文档引擎大重构.md`；选型 `docs\research\S004-Word文档读取选型-docx自解与doc直读双路线实测.md`（含决策变更）
   - 研究：`docs\research\`（文件名即标题，按关键词搜）

## 二、操作规则

> 两类场景：**工作节奏**（何时做什么）与**写作编码**（写什么按什么标准）。每条下分可以与禁止。

### 工作节奏

1. **每轮对话**
   - 可以：先核对四原语 `PRD.md`、`GOAL.md`、`TODO.md`、`PLAN.md`（需求驱动目标：新需求先入 PRD 走追问链，澄清后登记；GOAL 目标须回指 PRD 编号）；实质推进当场更新 todo 与 plan。
   - 禁止：不核对四原语就干活；偏离当前目标；推进了不更新 todo/plan；替用户静默假设需求。

2. **踩坑时**
   - 可以：当场按当前最大号接编 MNNN，落 `docs\mistakes\` 对应分类文件一行（文件名即错误主题，分类表见 `INDEX.md`）；同根因或同型坑合并聚合进已有条目（保留最早编号与首踩日期）；主题深挖落 `docs\research\`。
   - 禁止：只留在对话里反复试错。

3. **发现问题时**
   - 可以：走五步闭环（定位、归类、修正、验证、提交），细则见 `docs\references\R007-工作流标准细则-从登记到归档五步.md`。
   - 禁止：跳过定位直接改；只修表象不回写体系；问题只留在对话或记忆里；修完不跑验证；把临时补丁当最终方案不归档。

4. **交付变更时**
   - 可以：改代码同步对应文档，改文档同步索引与 `docs\diary\`；遵守命名标准；技术文档按文档标准细则写。
   - 禁止：只改代码不落文档；改了文档不更新索引。

5. **提交时**
   - 可以：`feat:` / `docs:` / `fix:` / `chore:` 前缀加中文描述；一次提交只做一件事。
   - 禁止：多事混一提交；未经指示推远端。

### 写作编码

6. **执行命令与写文件时**
   - 可以：Windows 命令用 PowerShell 7（`pwsh`），Linux / macOS / WSL 用该平台常规 shell；Markdown / Rust 源码 UTF-8；Windows 上需兼容 5.1 的脚本用 UTF-8 BOM。
   - 禁止：Windows 上默认用 `powershell.exe` 5.1；无 BOM 的中文 ps1 给 5.1 读。

7. **写 Rust 时**
   - 可以：先查 crates.io / docs.rs / GitHub 上是否已有最流行、最稳定、或已经覆盖本需求的库，检索走双通道细则 `docs\references\R002-选型研究细则-cratesio与github双通道.md`；选定后用最少代码接上，优先组合而不是自写协议、解析、CLI 解析；实现取舍照懒人阶梯（YAGNI、仓里已有复用、标准库优先、已装依赖优先、最少代码，见 ohmyagents S012 同源经验）。
   - 禁止：在现成库已能稳定完成的前提下从零实现；为风格引入冷门或实验 crate；一次拉一堆用不上的依赖。

8. **写文档时**
   - 可以：遵守 `docs\guide\G001-文档标准细则-命名写作规范与rumdl检查.md`（树形、标题干净、文件名即标题、rumdl）与 `docs\guide\G004-写作规范细则-禁用字符与机械判定.md`（四类禁用字符：破折号、箭头、emoji、非法全角；全部文档须过 md-char-scan（2026-09-03 存量已清零））。
   - 禁止：标题带括号、口号或破折号（解释放标题下一行引用 `>`）；整段混杂不成树。

9. **写研究与测试文档时**
   - 可以：事实性断言必须标六态之一：`[实证]`、`[推断]`、`[经验]`、`[记忆]`、`[假设]`、`[直觉]`；标准见 `docs\guide\G002-研究标准细则-结构与六态标记.md`。
   - 禁止：把「没验证」写成「已验证」（实证滥用）；断言不标六态；用猜测冒充结论。

10. **写测试时**
    - 可以：遵守 `docs\guide\G005-测试标准细则-分层断言与门禁流程.md`（三层分层、集成优先、期望值独立来源、稳定字段断言、`dies_` 负例前缀、`TestResult` 错误传播）与 `docs\guide\G006-测试体系细则-六层分层与各层标准.md`（单元/集成/冒烟/回归/验收/A/B 六层落点与口径）。
    - 禁止：重言式断言；公开 API 测试塞 `mod tests{}` 不进 `tests\`；默认 mock；计时进断言；只测 happy path；A/B 检查点从被测输出反抄。

11. **写临时脚本时**
    - 可以：按需自定义的 ps1 / py / Rust 工具，有复用价值即归档 `.tools\`（规则与清单见 `.tools\README.md`；Python 带 PEP 723 头，用 `uv run --script` 运行）；文档结构大改后跑 `uv run --script .tools\md-ref-scan.py` 做断链回归。
    - 禁止：可复用脚本散落仓库根或只留在对话里；归档不带自述与用法。

## 三、意图路由

> 需求意图与操作方法的映射。显示名 Reader；仓库 `reader_rs`；CLI 二进制 `reader`（缩写 `rr`）。

- **搜文本**：`reader search <文件|目录> <关键词>`（.pdf、.md/.markdown 及 anydoc 家族 .doc/.docx/.epub/.odt/.rtf/.ppt(x)/.xls(x)/.ods/.odp/.csv；`--regex` 正则、`-i` 忽略大小写、`-C N` 上下文、`--pages 1-3,5` 限页/节；命中退出 0、无命中退出 1、出错退出 2；`--format json` 包膜、`--filter` 点路径裁剪；`--ocr` 对 PDF 单文件 needs_ocr 页兜底识别（PP-OCRv6 默认 tiny 约 6.2MB，`READER_OCR_MODEL_SIZE=small` 切质量档；首用下载进缓存目录，`--offline` 禁下载）；目录输入递归批量搜，命中行带路径前缀，`--pages` 与 `--ocr` 不可用）
- **结构化提取**：`reader query <文件> <mq表达式>`（jq 风格：`.h2`/`.code`/`.link`/`.table`/select 管道，完整语法见 mqlang.org；全格式面转 markdown 后查询；退出码 0/1/2 同 search；不支持目录）
- **提文本**：`reader extract <文件>`（`--pages` 限页/节、`-o` 写文件；按单元输出 `== page N ==`（PDF）/ `== section N ==`（其余，标题分节）分节；输出 markdown 形态（PDF 布局管线 / anydoc GFM），不可靠页节头后给 `[needs_ocr: 原因]` 提示行；`--ocr`/`--offline` 同上对 PDF needs_ocr 页兜底；`--format json` 包膜、`--filter` 裁剪、`--offset/--limit` 分页带 `next_offset` 与 `cta`）
- **agent 发现**：`reader --llms`（紧凑命令索引）、`reader skill`（生成 SKILL.md；仓根 `SKILL.md` 与运行时输出逐字节一致，测试守卫）
- **自升级**：`reader self update [--force]`（GitHub Releases 最新正式版，资产 sha256 digest 校验后替换自身与兄弟二进制；GH_TOKEN 注入认证，限流回退 gh api）
- **发布**：`git tag v<版本>` 推 tag 触发 `.github\workflows\release.yml`（tag 须与 Cargo.toml version 一致，流水线有闸）；资产命名 `reader-v<版本>-<target>.<zip|tar.gz>`
- **构建测试**：`cargo build` / `cargo test`；本地门禁 `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked`
- **A/B 对比**：`uv run --script .tools\ab_run.py --a tiny --b small`（tests\ab\ 对象资源加检查点，OCR 模型档位质量与性能对比，报告落 tests\ab\reports\；标准见 G006）
- **查文档**：先搜 `INDEX.md` 定位编号，再读文件
- **文档门禁**：`rumdl check .`、`uv run --script .tools\md-ref-scan.py`、`uv run --script .tools\md-heading-scan.py`、`uv run --script .tools\md-char-scan.py`（G004 禁用字符）

已落地：`search`（含目录批量）、`extract`、`query`（mq 结构化提取，P0016）、`skill`、`--llms`、超长单元 part 分片、`--ocr` OCR 兜底（仅 PDF 单文件 needs_ocr 页，P0014）、`self update`（P0015）。候选方向与已拒绝项（MCP、TOON 等）登记在 `PRD.md` 需求清单，禁止假装未落地能力已经可跑。

## 四、资源索引

> 定位看 `INDEX.md`（项目根目录，唯一索引：编号表、目录结构、代码文件位置）。本节是配合 INDEX 的搜索方法。

**速记**：前缀定位 `P`（proven 归档）/ `S`（research 研究）/ `R`（references 做事的流程）/ `G`（guide 做事的规范）/ `M`（mistakes 错误；文件 M1xx、行级 M0xx）；根目录四原语 `PRD`（需求清单）/ `GOAL`（目标与达成标准）/ `PLAN`（规划）/ `TODO`（进度）。

**搜索方法（文档）**：

```powershell
rg -n "关键词" INDEX.md                        # 1 先搜总索引，定位编号或文件
rg --files docs | rg 关键词                     # 2 按文件名搜文档
rg -n "关键词" docs\research docs\references    # 3 全文搜研究参考
rg -n "关键词" docs\mistakes\                   # 4 搜错误处理
```

**分析路径**：改产品行为先读 `docs\references\`（怎么做）再回 `docs\research\`（为什么）；踩坑查 `docs\mistakes\`；写码选库走 R002；测试规范 G005；新想法走 R007 五步；定位代码先 INDEX 模块表。
