# docs 地图

> 全仓文档索引（2026-09-16 起承接旧根 INDEX 职责，dev-evo 文档即代码体系）。旧 INDEX 已于 2026-09-17 清退（代码文件位置表并入本地图，ADR-0006），新文档按本地图落位。
>
> 红线（dev-evo 用户裁定 2026-09-16）：`diary/` 与 `research/` 是保留核心结构，不可裁撤。
> 双目录并存裁定：`guide/`（G 编号元规范，被 AGENTS 与 G006 等全仓引用的活档体系）与 `guides/`（dev-evo 任务导向指南与留档）语义不同故并存，不合并。

## 活跃体系

> dev-evo 文档即代码体系。

| 位置 | 讲什么 | 何时看 |
|---|---|---|
| `../AGENTS.md` | 五节合同（Commands / Must / Must not / Read first / 环境） | 每轮开工前 |
| `adr/README.md` | 架构决策索引（ADR-NNNN，仍约束现状的决策择要） | 立不可逆选择前 |
| `requirements/README.md` | 需求登记索引（REQ-NNN，draft 到 implemented 带 trace；D 号即 REQ 号口径见其 README） | 立需求或查验收时 |
| `guides/` | 任务指南（getting-started、旧三节协作规则留档） | 做事前查方法 |
| `diary/` | YYYY-MM-DD 一天一篇过程与自省 | 查当天做了什么 |
| `research/` 加 README | SNNN 研究档案（六态标注） | 找为什么时 |
| `../CHANGELOG.md` 加 `../ROADMAP.md` | 版本成果与阶段 | 查历史与进度 |
| `references/` | R 编号做事的流程（R002 选型、R007 五步工作流、R008 封版仍现役） | 做事前查流程 |
| `guide/` | G 编号元规范（G001 命名写作、G002 六态、G004 禁字、G005/G006 测试、G007 基线、G008 README 规范） | 写文档与测试前查规范 |
| `poc/` | S 编号 PoC 原型产物（登记表见其 README） | 复现研究结论时 |
| `../.tools/` | 项目脚本工具（md 四件门禁加跑批与镜像件，清单见其 README） | 跑文档门禁与验收脚本时 |
| `aidoc/` | 库面 API 投影（llms.txt 入口加分模块 md 加 api JSON，cargo aidoc 生成物进 Git） | 查库 API 时 |

> 三栈投影（2026-09-16 撤换当日早间的「无自有 API 面」不适用裁定：dev-evo 第五十九批 tool-rust 强制口径，aidoc 投影强制化，bin-only 不豁免，受众是维护者与 agent，ADR-0005、REQ-049）：库面公开契约以 `///` 与类型签名为准，missing_docs 设 deny（Cargo.toml lints）；`docs/aidoc/` 全部生成物进 Git，漂移真门禁为 `cargo aidoc --check --strict`；CLI 面契约另走 `--llms` 运行时断言（clap 命令树旗标全覆盖，tests/cli.rs 漂移守卫）。

> agent CLI 面对照（2026-09-16，tool-cli-agents 十一节输出面增量，第六十一批终态对齐）：发现通道已落（`--llms` 紧凑索引为 agent 说明书唯一面，skill 子命令与仓根 SKILL.md 已退役（2026-09-16 用户裁定），clap 命令树覆盖断言兜漂移；MCP 通道在册 REQ-023）；输出信封已落（`{ok,data,meta}` 包膜，错误信封 `{ok:false,error,meta}` 加退出码 0/1/2）；CTA 已落（分页 meta 带 `next_offset` 与 `cta`）；单元级分页已落（`--offset/--limit` 加 `--filter` 点路径、数组映射、下标）。裁定不适用三件：token 计量与 token 分页（token 口径随模型族漂移，reader 无模型知识，职责归调用方；单元级分页已是稳定子集取回）；错误信封 typed 错误码与 retryable 字段（本地只读 CLI 错误确定性高、无重试消费者；错误分类需求实名时走 REQ-036）；TTY 探测输出分叉（单形态行式输出人机两用，stderr 人读行已分流）。

## 代码文件位置

> 原 INDEX 代码文件位置表承接面（2026-09-17 并入，ADR-0006）。行内 P / D / M / S 编号为历史沿革标签（档案清退见历史体系节）。

| 文件 | 职责 |
|---|---|
| `.tools/` | 项目自定义脚本工具归档（`README.md` 含清单与规则；`uv run --script` 载体） |
| `.tools/md-ref-scan.py` | markdown 仓内引用断链扫描（文档大改后回归门禁；豁免清单 `md-ref-allow.txt`） |
| `.tools/md-heading-scan.py` | 标题括号规范机检 |
| `.tools/md-char-scan.py` | G004 禁用字符机检 |
| `.tools/md-replace.py` | 中文与反斜杠路径安全的字面批量替换 |
| `.tools/make-scan-sample.py` | tests/ab 合成扫描件样本生成（无文本层 PDF 加独立检查点） |
| `.tools/ab_run.py` | A/B 对比跑批器（tests/ab 层，质量加性能报告） |
| `.tools/gen-latest-json.py` | 镜像升级清单生成（D42）：release API JSON 加 `.sha256` 边车出 `reader/latest.json`（五平台白名单校验） |
| `.tools/mirror-models.py` | 模型镜像 staging（D42）：按 Cargo.toml 钉的 ppocr-rs rev 取 models.json，HF 下载校验后出 R2 上传树与 gh 兜底资产 |
| `src/main.rs` | 薄壳入口（reader / rr 双 bin 共用） |
| `src/lib.rs` | clap CLI 定义、`run()` 分发、页/章范围解析 |
| `src/document.rs` | 格式分派与统一文本单元 TextUnit（页/节，含 needs_ocr 信号）；图片八扩展名单页提取（D43，恒标 needs_ocr:image，`--ocr` 兜底） |
| `src/pdf.rs` | PDF 页提取（pdf-inspector markdown 布局管线：多栏阅读序、needs_ocr） |
| `src/anydoc.rs` | anydoc 家族提取（Word/EPUB/ODT/RTF/Office/CSV 出 GFM，按顶层标题分节，超 200 行单元切 part；P0009 至 P0011） |
| `src/batch.rs` | 批量目录搜索（递归走查加两形态聚合；P0012） |
| `src/search.rs` | 匹配器（字面/正则/忽略大小写）与命中收集 |
| `src/output.rs` | JSON 包膜（ok/data/error 加 meta）、filter 点路径裁剪、cta 生成 |
| `src/introspect.rs` | agent 自省：`--llms` 紧凑索引（curated 文本，agent 说明书唯一面；skill 生成已退役 REQ-052） |
| `src/ocr.rs` | OCR 兜底（P0014、P0018 换引擎、D42 源链、D43 图片）：hayro 渲染 needs_ocr 页与图片文件直解码（首帧、EXIF 方向、alpha 白底）加 ppocr-rs 原生 CPU 内核跑 PP-OCRv6；引擎构建共用 helper；首用三级回退预取（镜像到HF到GitHub，`mirror` 模块）、缓存先零网络探测；`ocr init / doctor / switch` 三子命令与档位三级（env > model-size 设置 > tiny）；`READER_OCR_CACHE_DIR` 覆盖缓存目录 |
| `src/mirror.rs` | 镜像源链与清单（D42）：四包 pin 表（与 ppocr-rs rev 同步换，单测钉）、三级回退单件下载（`.part` 加校验加 rename）、只读 assess、latest.json 拉取解析；`READER_MIRROR` 覆盖基址 |
| `src/selfupdate.rs` | self update（P0015、D42 加镜像通道）：镜像 latest.json 优先、GitHub API 加 gh api 兜底、版本判新、资产 sha256 校验、zip/tar.gz 解包、staged 加 rename 替换自身与兄弟 |
| `src/query.rs` | mq 结构化提取（P0016）：格式转 markdown 文本（md 原文/anydoc GFM/PDF 管线）加 mq-lang eval，空渲染过滤；图片拒入并指路 --ocr（D43） |
| `src/figures.rs` | 图片本体导出与文本元数据对齐（D47）：PDF 内嵌位图 XObject 直抽（DCT jpg 原字节 / Flate 按色彩空间解码 png；扫描页回退 hayro 整页渲染；图题从页文本对齐）、md 引用复制、anydoc zip 内嵌件直读、图片文件自复制；`figure:` 行式与 json figures[]；不做图表模型理解（S010 T3 已拒） |
| `tests/cli.rs` | CLI 集成冒烟与正负例（夹具现造；legacy .doc 仓内资产） |
| `tests/smoke.rs` / `regress.rs` / `accept.rs` | 冒烟/回归/验收三层 cargo 独立 test target（D31 第 2 轮；accept 为 cucumber BDD，场景 tests/features/，D33；smoke 自 D44 起全格式活体：现造 pdf/md/csv/epub 加 anydoc 官方语料九族；G006 载体规则） |
| `tests/snapshot.rs` | 回归层 insta 输出快照（extract 全量、search 命中格式、--llms；快照在 tests/snapshots/，D34） |
| `tests/mirror.rs` | 回归层 mirror 公开 API 直测（本机一次性 HTTP 服务加合成 pin；READER_MIRROR env 变更需独立 target 隔离，下载器自建父目录回归，M017） |
| `tests/corpus.rs` | 回归层 anydoc 官方语料 63 件逐件全量快照加负例与滥用断言（stderr 绝对路径归一 `<repo>`；快照在 tests/snapshots/corpus__*，D44 第 3 轮） |
| `tests/materials.rs` | 回归层 E:\研究资料 全语料 gated 基线核验（D46 第 2 轮：弃 E:\ebook 改此；manifest 钉 sha256，盘缺失整体跳过 CI 免跑；工具 `.tools/materials-corpus.py`） |
| `tests/assets/legacy.doc` | legacy Word 二进制测试资产（Word COM 现造，CI 无 Word 不能现造；P0009） |
| `tests/assets/anydoc/` | anydoc 官方测试 fixtures 语料 71 件全量非 pdf corpus（firecrawl/anydoc@261fc25，MIT，镜像上游布局；含 malformed 负例与 abuse 滥用件；来源与 sha256 见目录内 README，D44 第 3 轮扩全量） |
| `tests/assets/ocr-text.png` | 图片 OCR 端到端门控资产（GDI+ 现造 480x140 文字图 READER SMOKE 12345，tiny 档实测全识；D43） |
| `tests/assets/tiny.jpg` | figures 内嵌图直抽测试资产（GDI+ 现造 4x4 JPEG，DCT 原字节断言用；D47） |
| `Cargo.toml` | package reader_rs；依赖 pin 与双 bin 定义 |

## 历史体系已清退

> 旧四原语（根 PRD / GOAL / PLAN / TODO）与根 INDEX、`proven/`（P 编号方案档案）、`mistakes/`（M 编号错误档案）已于 2026-09-17 清退（REQ-054、ADR-0006）：存量知识融入 ADR-0006 与活跃面，全文历史见 git（清退前最后完整态在 v0.7.0）。P / M / D 编号此后仅作历史沿革标签，不再产生新条目。

## 迁移映射

> dev-evo base-init 存量迁移口径（2026-09-16 全量迁移，楷模仓 hst_rs；2026-09-17 二次清退，REQ-054 与 ADR-0006）。

PRD 条目对应 REQ（编号连续性用 D 号即 REQ 号：D01 至 D47 历史，活跃队列 D23 至 D26 与 D36 至 D39 按 D 号转 REQ，新需求自 REQ-049 接编）；PLAN 与 TODO 对应 REQ 的 Criteria 与 trace 或 ROADMAP；GOAL 定位句并入 AGENTS 头部与 README；INDEX 职责由本地图（含代码文件位置表）加 AGENTS Read first 承接；proven 语义由 implemented 需求加关联 ADR 承接；mistakes 由 ADR-0006 蒸馏表承接。迁移不是搬运是重审：历史不回填、活档不搬家、断链必回归。迁移本身立 REQ-048，清退立 REQ-054。

### 存量禁字债口径

PE-11 历史档案豁免走 `PEVO_CHECK_ALLOW` 机制（分号分隔正则，匹配 `docs/` 下相对路径加行号，命中报 SKIP 带处数；根三件 AGENTS / README / CHANGELOG 永不受益）。本仓手写面存量禁字为零（G004 存量清零，D22）；唯一在册豁免正则是 `^docs/aidoc/`（cargo aidoc 渲染格式：条目分隔符 em dash 无开关可改，漂移真门禁是 `cargo aidoc --check --strict`，rumdl 与 md-char-scan 同步整目录豁免），标准命令：

```bash
PEVO_CHECK_ALLOW='^docs/aidoc/' uv run ~/.claude/plugins/marketplaces/project-evo/plugins/evo-adr/skills/code-kit/scripts/check.py .
```

日后历史档案留档面若新增禁字（历史不改写），按路径级豁免追加登记在册（只作用 `docs/` 档案，活跃面零容忍），例如 `PEVO_CHECK_ALLOW='^docs/aidoc/;^docs/diary/'`；正则口径与 scan 的 `PEVO_SCAN_ALLOW` 同一惯例。
