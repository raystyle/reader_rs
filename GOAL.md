# GOAL：任务目标管理

> 角色：**工作任务管理**，四个部分：**起点**、**锚点**、**进程**、**历史**。随工作实时更新。
> 与其它文档分工：`ROADMAP.md`=阶段路线；`CHANGELOG.md`=版本成果；`docs\diary\YYYY-MM-DD-*.md`=项目日记；`docs\proven\PNNNN-*.md`=方案与过程经验；`TODO.md`=进度清单；`PLAN.md`=实施指导。

## 起点

> 当前目标的起点：何时发起、为什么发起、要解决什么问题。

- **日期**：2026-09-03。
- **起点**：S006 已实证纯 Rust 内嵌 OCR 可行；用户指示接续开发，点名 OCR 落地实现（P0014：`--ocr` 兜底、模型首用下载缓存）。

## 锚点

> 当前锚定的目标 + 推进时间线。

- **锚定的目标**：P0018 OCR 换引擎 ppocr-rs（进行中：S008 裁决，v6 tiny 0.8s/页且掉字全修，双优胜出；git rev 钉 d07857c 引入，替换 pure-onnx-ocr 加 tract 管线）；v0.3.0 已发布收官。

### 推进时间线

> 倒序：最新进展在最上。

| 日期 | 进展 |
| --- | --- |
| 2026-09-03 | **S008 达成**：四配置同页对比，v5 server 质量好但 125.6s/页出局；v6 tiny 0.8s/页加掉字全修双优胜出、v6 small 3.2s 更干净留档候选；裁决换引擎（S008 落盘） |
| 2026-09-03 | S008 立项：OCR 质量升级研究（mobile 掉字问题：server 模型与 ppocr-rs v6 双路线）；G004 存量清零（38 文件 205 处 swarm 清理、基线删除） |
| 2026-09-03 | **v0.3.0 发布**：封版 tag 触发 release 五 job 全绿（musl 首验 mq-lang/tract/hayro 过）、10 资产齐；windows 资产 .sha256 官方校验 OK、解包 `reader 0.3.0` 冒烟过、发行件 self update 报已最新；本机与 CI 二进制 sha 不同（构建机差异，P0013 时的一致性不可复现） |
| 2026-09-03 | 全平台收口：推送至 8cf02e3，CI 双 run 三系统全绿；lan-mac（arm64）与 lan-linux（12 核）实机门禁加 OCR 真样本双过（mac 5.2s/2 页、linux 9.7s/2 页，全进 5-10s 档）；lan-linux 修两坑（1.97 工具链 ICE 换 stable 1.98、registry 损坏缓存清除） |
| 2026-09-03 | **P0017 达成**：rec 动态宽度加桶化加宽度分组分批加组间并行（每 worker 独立会话）；batch 按核数自适应；质量持平且部分掉字修复；SKILL 重构（用户裁定常用例子加渐进引导） |
| 2026-09-03 | P0017 立项：OCR 性能剖析：det 仅 0.73s、rec infer 19.6s 占绝对大头；引擎 run_with_metrics 拆三段计时 |
| 2026-09-03 | CI 三系统全绿：P0014-P0016 推送后 run 33623776472 windows/ubuntu/macos 全过（全平台回归闭环）；SKILL 重构为常用例子加渐进引导（用户裁定） |
| 2026-09-03 | **P0016 达成**：.md/.markdown 进 search/extract 格式面（零新依赖复用 split_markdown）；`query` 子命令嵌 mq-lang 全引擎全格式面；批量目录搜索自动覆盖 .md；README/AGENTS/INDEX/CHANGELOG 同步 |
| 2026-09-03 | **S007 达成**：mq 双通道核实加 PoC 实测（`.h`/`.code`/select 组合中文样本全对，miette 结构化报错）；裁决：.md 零新依赖复用 split_markdown，query 嵌 mq-lang；mq-markdown 不直用（拍平序列无增量）；踩坑 docs.rs 示例过时（RuntimeValue 改名） |
| 2026-09-03 | S007 立项：用户点名学习 mq 加 markdown 搜索与结构化提取；双通道核实 mq（MIT、1023 星、活跃），mq-markdown 0.8.4 轻量纯 Rust、mq-lang 0.8.4 约 30 依赖全引擎；anydoc.rs 的 split_markdown 可零新依赖复用给 .md 分节 |
| 2026-09-03 | **P0015 达成**：self update 落地（19 单元加 47 集成全绿）；临时目录 `--force` 端到端实测：下载加 digest 校验加解包加双名替换，换上件 sha256 与官方 v0.2.1 资产逐一一致；M012 沉淀（flate2 0.2 无 rust_backend，钉 1.x） |
| 2026-09-03 | **P0014 达成**：`--ocr` 兜底落地（16 单元加 46 集成全绿）；真样本两页出正文、首用下载全流程实测；三坑沉淀 M009-M011（ureq 10MB 上限、vendor println 污染 stdout、OcrEngine 非 Send/Sync）；stripped 哈希改钉 prost 输出；二进制 7.3MB 到 32.9MB |
| 2026-09-02 | **S006 达成**：内嵌 OCR 双通道普查九候选，纯 Rust 管线（hayro 0.7 渲染加 pure-onnx-ocr 0.1 跑 PP-OCRv5 mobile 20.5MB）真样本端到端实证；ocrs 拉丁限定出局、RapidOCR 系全绑 ort 破边界；两坑沉淀（tract value_info 剥离、rec max_width 硬编码 320） |
| 2026-09-01 | **v0.2.1 发布**：CI 绿后打 tag（M005 流程修正版）；Release 五 job 首跑全绿（musl 一次过）、10 资产齐；windows 与 musl 双 sha256 一致、`reader 0.2.1` 冒烟过；P0013 收官，三项目标全闭环（封版中记 M008：Git Bash 落出 nul 保留名文件） |
| 2026-09-01 | mac 接管验收（R005）闭环：门禁三件与文档门禁全绿（15 单元 + 44 集成、rumdl 43 文件零告警）、真样本五路过（M007 现场验 141 静默）、x86_64 交叉预建 file 判形过（本机无 Rosetta 2）；三平台对账闭环，mac 接管开发就位 |
| 2026-09-01 | mac 接管移交（R005）：仓已 pull 到 154df2b、样本 docx 已 SCP 就位、工具面实测齐（arm64 macOS 26.5.2、rumdl 可跑 M003 已解）；推送后 CI 三系统绿（M007 测试在 ubuntu/macos job 均过） |
| 2026-09-01 | Linux 实机验收（R004）闭环：门禁与文档门禁全绿、真样本五路过、musl 预建静态件成（zstd-sys 假设转实证）；发现并当轮修复 M007（SIGPIPE 管道 panic），v0.2.1 tag 可发 |
| 2026-09-01 | 开发验收移交 Linux（用户实机）：门禁三件与文档三件复跑、真样本四路冒烟、musl 本地预建（可选，为 P0013 首跑去险）；v0.2.1 tag 待验收后定夺 |
| 2026-09-01 | P0011 加 P0012 达成：超长节切 part（单元号跨 kind 连续）、批量目录搜索（材料目录真样本递归命中 pptx 与 docx）；P0013 musl 矩阵就位，验收挂 v0.2.1 tag 首跑 |
| 2026-09-01 | P0010 达成：无标题文档按 200 行分片为 part 单元，12 单元加 38 集成全绿，真样本回归（渗透方案单 part、测试V2 仍 15 section）；S005 达成：TOON 三真样本双编码器实测收益为负或 <2% 且 0.5.0 往返破损，裁定不引入销候选 |
| 2026-09-01 | v0.2.0 发布：CI 首跑暴露 M005（反斜杠路径 join 致 linux/macOS 测试红），修复重切 tag 后四 job 全绿 8 资产齐，本机 sha256 与三格式实测过 |
| 2026-09-01 | P0009 达成：anydoc 统一引擎大重构，格式面 2 到 14 种，37 集成加 9 单元全绿；S004 决策变更记录（用户推翻 docx 自解初判） |
| 2026-09-01 | S004 选型研究：docx 双路线 PoC（office_oxide 实体丢字出局）；用户裁定大重构选 anydoc，补测全绿（实体保真、legacy .doc 直读） |
| 2026-08-31 | P0008 达成：v0.1.0 已发布，run 33461625241 四 job 全绿、8 资产齐出；本机实测 sha256 一致、`reader 0.1.0`、真样本 25 命中与 S001 基线一致；macos-13 退役坑记 M004（Intel 改交叉编译） |
| 2026-08-31 | P0008 立项：tag v* 触发 release 流水线，四目标矩阵（win/linux x86_64、macos aarch64/x86_64），gh CLI 幂等上传，版本一致性闸；CHANGELOG 封版 0.1.0 |
| 2026-08-31 | P0007 达成：39 测全绿（单元 7 加集成 32）；`--llms`/`skill`/help examples 落地；仓根 SKILL.md 提交加双漂移守卫；README 加 Agent 发现节 |
| 2026-08-31 | P0007 立项：`--llms` 紧凑索引、`skill` 子命令生成 SKILL.md（仓根提交加漂移守卫）、help examples 节；curated 文本加双漂移测试，不做命令树全自动生成 |
| 2026-08-31 | mac 本地接管验收：门禁六件全绿（34 测与 P0006 收官一致），rumdl 架构坑记 M003；lan-mac 与 lan-linux 实机验收通过（用户确认），三平台开发测试面全部实证 |
| 2026-08-31 | 推送 5 笔提交，CI run 33389883062 三系统全绿（P0005/P0006 跨平台验证闭环）；开发测试移交 mac 接管 |
| 2026-08-31 | P0006 达成：34 测全绿（单元 7 加集成 27）；包膜、分页（next_offset 加 cta）、filter 三件落地；真样本中英文抽查过（书 25 命中序列与 S001 一致，中文原样 UTF-8） |
| 2026-08-31 | P0006 立项：`--format json` 包膜（ok/data/error 加 meta）、extract `--offset/--limit` 分页（next_offset 加 cta）、`--filter` 点路径裁剪；serde/serde_json 进依赖；agent 发现与 MCP 留候选 |
| 2026-08-31 | P0004 达成：CI 首跑三系统全绿（run 33378905306）；mac/Linux 接管面就位 |
| 2026-08-31 | P0004 立项：CI 三系统矩阵（fmt/clippy/test --locked）、.gitattributes 钉 LF、README bash 双形态；验收以 CI 首跑为准 |
| 2026-08-31 | P0003 达成：EPUB 支持全链路绿（格式分派 TextUnit、cargo test 20 过、真实样本 37 章回归）；quick-xml 0.42 版本敏感点记 S003 |
| 2026-08-31 | EPUB 选型研究 S003：`epub` crate 为 GPL-3.0 一票出局，选 rbook（Apache-2.0、活跃）加 quick-xml 解章正文 |
| 2026-08-31 | P0002 达成：全仓定位对齐 Reader（Agent 原生文档阅读、搜索和提取工具）；R001 改名重写；门禁回归全绿 |
| 2026-08-31 | P0001 达成：search/extract 全链路绿（cargo test 13 过、门禁三件与 rumdl 三件套过、真实 PDF 冒烟过）；断链误判坑记 M001 |

## 进程

> 当前目标的进程：只记录当前这一个目标的进行状态。

- 当前目标（已达成）：P0017 OCR 性能优化，见锚点与 P0017 归档。

## 历史

> 所有已完成目标的轨迹，按日期倒序。

| 日期 | 目标 | 结果 |
| --- | --- | --- |
| 2026-09-03 | P0018 OCR 换引擎 ppocr-rs | 达成：v6 tiny 落地；掉字全修、0.8s/页量级；二进制回落 28.3MB；门禁全绿 |
| 2026-09-03 | S008 OCR 质量升级研究 | 达成：v6 tiny 双优胜出（0.8s/页加掉字全修）；server 出局；转 P0018 |
| 2026-09-03 | P0017 OCR 性能优化 | 达成：20.5s/页到 3.0s/页（32 核），真样本约 5.5 秒/页达标 5-10s 目标；质量持平 |
| 2026-09-03 | P0016 markdown 支持与 mq 结构化提取 | 达成：.md 进格式面加 query 子命令；23 单元加 52 集成全绿、真样本回归过 |
| 2026-09-03 | S007 markdown 支持选型（学习 mq） | 达成：嵌 mq-lang 全引擎加零依赖分节裁决；转 P0016 落地 |
| 2026-09-03 | P0015 self update | 达成：stable 通道版本判新加 `--force` 重装、资产 digest 钉死、staged 加 rename 原子替换自身与兄弟；临时目录实测通过；M012 沉淀 |
| 2026-09-03 | P0015 self update | 达成：stable 通道版本判新加 `--force` 重装、资产 digest 钉死、staged 加 rename 原子替换自身与兄弟；临时目录实测通过；M012 沉淀 |
| 2026-09-03 | P0014 OCR 兜底落地 | 达成：`--ocr`/`--offline` 进 extract 加 search（仅 PDF 单文件）；模型三件 ModelScope 下载加双套 SHA-256 钉死、进程内 prost strip；16 单元加 46 集成全绿、真样本回归过；M009-M011 沉淀；二进制 7.3MB 到 32.9MB |
| 2026-09-02 | S006 内嵌 OCR 选型研究 | 达成：纯 Rust 管线（hayro 加 pure-onnx-ocr 跑 PP-OCRv5 mobile）真样本端到端实证可行；九候选裁决与四坑沉淀；OCR 落地转 P0014 |
| 2026-09-01 | 无标题长文档行分片加 TOON 验证（P0010、S005） | 达成：part 分片落地（12 单元加 38 集成全绿）；TOON 实测不引入（中文更费 token、往返破损）销候选 |
| 2026-09-01 | anydoc 统一引擎大重构与 v0.2.0 发布（P0009） | 达成：格式面 2 到 14 种；v0.2.0 三端发布（M005 修复后重切 tag，8 资产实测过）；EPUB 章改节为破坏性变更 |
| 2026-08-31 | 封版 v0.1.0 与三端二进制 release（P0008） | 达成：tag 触发四目标流水线；8 资产齐出；本机实测通过；M004 记 macos-13 退役 |
| 2026-08-31 | Agent 自省与发现：llms 索引、SKILL 生成与 help 示例（P0007） | 达成：`--llms`、`skill`、help examples；仓根 SKILL.md 加双漂移守卫；39 测全绿 |
| 2026-08-31 | 输出形态第一刀：json 包膜与分页裁剪（P0006） | 达成：`--format json` 包膜、extract 分页（next_offset 加 cta）、`--filter` 裁剪；34 测全绿 |
| 2026-08-31 | PDF 提取质量：markdown 管线与 needs_ocr 提示（P0005） | 达成：多栏阅读序、needs_ocr 检出提示、S001 瑕疵修复；中英文真样本双验；22 测全绿 |
| 2026-08-31 | mac 与 Linux 接管开发与跨平台兼容（P0004） | 达成：CI 三系统门禁首跑全绿；LF 钉死；文档双形态 |
| 2026-08-31 | EPUB 支持（P0003） | 达成：格式分派 TextUnit；cargo test 20 过；真实样本 37 章回归正确 |
| 2026-08-31 | 项目重新定位 Reader（P0002） | 达成：Agent 原生定位落成三条设计约束；全仓文档对齐；R001 改名重写 |
| 2026-08-31 | PDF 文本搜索与提取 CLI 最小闭环（P0001） | 达成：search/extract 双命令加 reader/rr 双 bin；cargo test 13 过；门禁全绿 |
| 2026-08-31 | 对照 ohmyagents 建立项目结构与项目文档 | 达成：四段 AGENTS、三原语、docs 六目录、guide 三件、`.tools` 三件套、rumdl 门禁 |

## 维护规则

- **起点**：开工时写一句「何时发起 + 为什么发起」。
- **锚点**：每完成一个节点补一行（日期 + 进展）。
- **进程**：只记当前目标；达成后整条移入「历史」。
- **历史**：日期 + 目标 + 结果，倒序。
- **一目标一路径**：起点、锚点、进程、历史同属一个目标轨迹。
- **日记与方案**：当天流水账进 `docs\diary\`；方案与过程经验进 `docs\proven\`。
