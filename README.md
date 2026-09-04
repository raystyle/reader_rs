# Reader

Agent 原生文档阅读、搜索与提取 CLI：PDF / Word（含 .doc）/ EPUB / Office / CSV 等 14 种文档格式加图片（png / jpg 等 8 种），按页/节读、正则搜、目录批量搜、OCR 识图；grep 语义退出码，Rust 单二进制。

[![CI](https://github.com/raystyle/reader_rs/actions/workflows/ci.yml/badge.svg)](https://github.com/raystyle/reader_rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

从本地 PDF、markdown、图片与 Office 家族文档读文本层，给 Agent 稳定可解析的输出。Rust 单二进制、单调用无交互；命令 `reader`（缩写 `rr`）。

| 做什么 | 命令示例 |
| --- | --- |
| 读：按页或标题节取正文 | `reader extract ./doc.pdf --pages 1-3` |
| 搜：字面、正则、目录批量 | `rr search ./材料 "配置" -C 2` |
| 结构化提取：mq 表达式（jq 风格） | `reader query ./notes.md ".h2"` |
| 取图：图片本体导出与元数据对齐 | `reader figures ./scan.pdf --pages 12-32` |
| 一键：文本+图片+元数据落一目录 | `reader export ./paper.pdf --ocr` |

- PDF 按页读；markdown 与 Word / EPUB / ODT / RTF / Office / CSV 等 14 种格式按标题节读；图片文件（png / jpg / bmp / gif / webp / tiff 等 8 种扩展名）单图即单页
- 扫描件与图片以 `needs_ocr` 检出，`--ocr` 兜底识别（PP-OCRv6 tiny，首用下载约 6.2 MB 模型）
- 行式标记、grep 语义退出码 0/1/2；`--format json` 包膜加 `--filter` 点路径裁剪、分页 `next_offset`
- `--llms` 紧凑命令索引；`reader skill` 生成 SKILL.md 给编码 agent 自动发现

## 全平台安装

前置：Rust 工具链（1.88+，推荐 <https://rustup.rs>）。支持 Windows / macOS / Linux，CI 三系统门禁见 [.github/workflows/ci.yml](.github/workflows/ci.yml)。

预编译二进制（无 Rust 工具链时用）：[GitHub Releases](https://github.com/raystyle/reader_rs/releases) 页按平台取资产，解压即得 `reader` 与 `rr`（Windows 为 `.exe`），同附 README、LICENSE、SKILL.md 与 `.sha256` 校验文件：

| 资产 | 平台 |
| --- | --- |
| `reader-v<版本>-x86_64-pc-windows-msvc.zip` | Windows x86_64 |
| `reader-v<版本>-x86_64-unknown-linux-gnu.tar.gz` | Linux x86_64（glibc） |
| `reader-v<版本>-x86_64-unknown-linux-musl.tar.gz` | Linux x86_64 静态（Alpine / 容器 / 最小镜像；v0.2.1 起） |
| `reader-v<版本>-aarch64-apple-darwin.tar.gz` | macOS Apple 芯片 |
| `reader-v<版本>-x86_64-apple-darwin.tar.gz` | macOS Intel |

从源码安装（`reader` 与 `rr` 两个命令一起装）：

```bash
git clone https://github.com/raystyle/reader_rs
cd reader_rs
cargo install --path .
```

或直接远端安装：

```bash
cargo install --git https://github.com/raystyle/reader_rs
```

只构建不安装：`cargo build --release`，产物在 `target/release/reader` 与 `rr`（Windows 为 `target\release\reader.exe` 与 `rr.exe`）。

验证：

```bash
reader --version
reader --help
```

## 升级

自升级（推荐）：

```text
reader self update [--force]
```

从镜像 `reader.ohmygh.com/reader/latest.json` 查新版并下载本平台资产（国内可达；`READER_MIRROR` 可覆盖基址），sha256 钉死校验后替换当前运行的二进制与同目录兄弟（`reader` / `rr` 双名一次替换）；已最新时明示。`--force` 同版本重装。镜像不可用时自动回退 GitHub API：默认匿名（有配额），撞限流 403 再回退 `gh api`（用本机 gh CLI 登录态），配 `GH_TOKEN` 可提高配额。只走 stable 通道，不做自动更新。

源码安装的升级：重跑安装命令加 `--force`：

```bash
cargo install --git https://github.com/raystyle/reader_rs --force
```

手动升级：到 [GitHub Releases](https://github.com/raystyle/reader_rs/releases) 取新平台资产，`.sha256` 校验后解压覆盖（资产内含 `reader` 与 `rr` 双名）。

## 配置与管理

环境变量（均有缺省，不配即用）：

| 变量 | 作用 | 缺省 |
| --- | --- | --- |
| `GH_TOKEN` | `self update` 回退 GitHub API 时的认证令牌（提高配额）；默认匿名即可用，撞限流自动回退 `gh api` | 匿名（有配额） |
| `READER_MIRROR` | 分发镜像基址（模型下载与 self update 查新） | `https://reader.ohmygh.com` |
| `READER_OCR_CACHE_DIR` | 覆盖 OCR 模型缓存目录 | 平台缓存目录（见下表） |
| `READER_OCR_MODEL_SIZE` | OCR 模型档位临时覆盖（A/B 对比用），优先于 `ocr switch` 设置 | 未设（取 `ocr switch` 设置，再缺省 `tiny`） |

**OCR 模型档位**（`READER_OCR_MODEL_SIZE`，缺省 `tiny`）：

| 档位 | 速度 | 质量 |
| --- | --- | --- |
| `tiny` | 快（多核约 1-5 秒/页） | 缺省档；混排文本偶有掉字 |
| `small` | 慢（约 3 秒/页量级） | 输出更净更全（实测行召回更高、掉字更少） |

切换即用，两档模型独立缓存：

```bash
reader ocr switch small                 # 切档并持久化（写入缓存目录旁 model-size 文件）
reader ocr init --size small            # 预下载该档模型进缓存
READER_OCR_MODEL_SIZE=small reader extract ./scan.pdf --ocr   # 环境变量临时覆盖（优先于 switch 设置）
```

### 模型来源与手动部署

来源三级回退（国内机器首用不再卡 HF）：镜像 `reader.ohmygh.com`（R2 自定义域）到 HuggingFace 直连（[PaddlePaddle/PP-OCRv6_tiny_det_safetensors](https://huggingface.co/PaddlePaddle/PP-OCRv6_tiny_det_safetensors) 与 [PP-OCRv6_tiny_rec_safetensors](https://huggingface.co/PaddlePaddle/PP-OCRv6_tiny_rec_safetensors)，small 档同系两仓；ppocr-rs 钉 revision 与逐件 sha256）到 GitHub Releases `models-v6` 资产。首用 `--ocr` 或 `ocr init` 时按链下载约 6.2 MB（small 档约 31 MB），逐件 sha256 校验落缓存，ppocr-rs 内嵌钉死值全量校验兜底。

模型管理三件套（`reader ocr --help` 看全）：

```bash
reader ocr init [--size tiny|small] [--offline]   # 显式下载/修复模型进缓存（有效件跳过；--offline 只校验）
reader ocr doctor                                  # 只读诊断两档就位情况（退出码 0 为当前档双包完整）
reader ocr switch <tiny|small>                     # 切换档位并持久化
```

缓存结构（档位 × det / rec 两包，各 4 件加隐藏完成标记 `.ppocr-rs.complete`）：

```text
<缓存目录>/
  tiny-det/   model.safetensors、config.json、inference.yml、preprocessor_config.json、.ppocr-rs.complete
  tiny-rec/   同上五件
```

三平台缺省缓存目录：

| 平台 | 缓存目录 |
| --- | --- |
| Windows | `%LOCALAPPDATA%\reader\models`（无则 `%APPDATA%` 下同路径） |
| macOS | `~/Library/Caches/reader/models` |
| Linux | `$XDG_CACHE_HOME/reader/models`（未设则 `~/.cache/reader/models`） |

手动部署（网络不可达机器，如内网服务器；模型件字节跨平台一致）：

1. 可达机器跑 `reader ocr init`（small 档加 `--size small`），缓存自动落地；
2. 整目录拷贝到目标机器同位置（必须含隐藏标记文件）：

```bash
scp -r <可达机>/reader/models/tiny-det <可达机>/reader/models/tiny-rec <用户>@<目标机>:~/.cache/reader/models/
```

3. 目标机验证（`ocr doctor` 的 `verdict ok` 即当前档双包完整；或 `--offline` 实测零下载）：

```bash
reader ocr doctor
reader extract ./scan.pdf --ocr --offline --pages 1
```

注意：`.ppocr-rs.complete` 内容是模型清单指纹而非空文件，`ocr init` 与官方下载链会自动补写；缓存目录可随时删除，下次按需重新下载。档位设置文件在缓存目录旁（`model-size`），删缓存目录不影响档位偏好。

卸载：预编译安装删除 `reader` 与 `rr` 两个二进制与缓存目录即可；cargo 安装用 `cargo uninstall reader_rs` 一并移除双名。

## 快速开始

装好后先跑三条（bash 与 pwsh 同形，文件换成自己的）：

```bash
reader extract ./doc.pdf --pages 1-3   # PDF 按页读文本
rr search ./report.docx "摘要" -C 2    # Word 按节搜词，带上下文
reader skill > SKILL.md                # 生成 agent 自述，放项目根即被发现
```

## Agent 发现与 SKILL 安装

面向编码 agent 的自省接口：

```bash
reader --llms     # 紧凑命令索引（省 token 形态）
reader skill      # 输出 SKILL.md 全文（frontmatter + 命令 + 输出契约 + 退出码 + 示例）
```

把 SKILL 装给 agent（项目根目录放一份，多数编码 agent 会自动发现）：

```bash
reader skill > SKILL.md
```

仓根已提交一份 [SKILL.md](SKILL.md)，与运行时输出逐字节一致（集成测试做漂移守卫）；升级版本后用上面命令刷新即可。

## 命令

文档子命令四个：`search`（搜）、`extract`（取）、`query`（mq 结构化提取）与 `figures`（图片本体导出）；外加发现接口 `skill` 子命令与 `--llms` 旗标（见上节）、`self update` 自升级（见「升级」节）。输入文件按扩展名分派：`.pdf` 按页，markdown（`.md` / `.markdown`）与 anydoc 家族（`.doc` / `.docx` / `.epub` / `.odt` / `.rtf` / `.ppt(x)` / `.xls(x)` / `.ods` / `.odp` / `.csv`）按 GFM markdown 顶层标题分节，图片（`.png` / `.jpg` / `.jpeg` / `.bmp` / `.gif` / `.webp` / `.tiff` / `.tif`）单图即单页（D43）。`search` 也接受目录：递归批量搜支持格式，命中行带路径前缀（P0012）。

### search 搜索

```text
reader search <文件|目录> <关键词> [--regex] [-i] [-C N] [--pages 范围]
```

| 参数 | 说明 |
| --- | --- |
| `<文件>` | 文档路径（.pdf 及 anydoc 家族，见「支持格式」） |
| `<目录>` | 递归批量搜支持格式：text 命中行 `路径:单元:行号:文本`；json `hits[]` 带 `file` 字段加 `files.scanned / files.skipped`；坏文件 stderr 跳过后继续；`--pages` 不可用 |
| `<关键词>` | 字面匹配串；`--regex` 时按正则解释 |
| `--regex` | 按正则匹配（regex crate 语法） |
| `-i`, `--ignore-case` | 忽略大小写 |
| `-C N`, `--context N` | 命中行前后各带 N 行上下文 |
| `--pages 范围` | 限定页/节（1 起），写法 `1-3,5`；仅单文件模式 |
| `--format 形态` | `text`（行式，缺省）或 `json`（包膜） |
| `--filter 路径` | 裁剪 JSON `data` 的点路径（如 `hits[].text`）；仅 `--format json` 下可用 |
| `--ocr` | PDF 与图片单文件 needs_ocr 页走 OCR 兜底（PP-OCRv6 tiny 首用下载约 6.2MB 模型进缓存目录；多核并行约 1-5 秒/页；可与 `--pages` 组合，仅对范围内 needs_ocr 页兜底；目录模式不可用） |
| `--offline` | 禁模型下载（须与 `--ocr` 同用；模型未就位时报错） |

输出格式（仿 grep，稳定可解析）：

```text
单元:行号:命中行文本        命中行（PDF 单元为页，其余格式为节）
单元-行号-上下文文本        上下文行（-C 时）
路径:单元:行号:命中行文本   目录批量模式（路径前缀；Windows 盘符冒号从右取三段解析）
```

文档含文本层不可靠页（扫描件、编码问题，及图片文件整页）时，stderr 额外出一条 `needs_ocr` 警示并列出页码；stdout 不混入提示。退出码：命中 0；无命中 1；出错（缺文件、坏参数、不支持的格式、目录无支持格式文件）2。

示例（bash 与 pwsh 同形，Windows 路径换成反斜杠形态即可）：

```bash
reader search ./doc.pdf "error" -i -C 1
reader search ./doc.pdf "err(or|code)" --regex --pages 2-10
rr search ./report.docx "配置"
rr search ./材料 "代理" --format json --filter 'hits[].file'
```

### extract 提取

```text
reader extract <文件> [--pages 范围] [-o 输出文件]
```

| 参数 | 说明 |
| --- | --- |
| `<文件>` | 文档路径（.pdf 及 anydoc 家族，见「支持格式」） |
| `--pages 范围` | 限定页/节（1 起），写法 `1-3,5`；缺省全部 |
| `-o`, `--out 文件` | 写入文件；缺省输出到 stdout |
| `--format 形态` | `text`（行式，缺省）或 `json`（包膜） |
| `--filter 路径` | 裁剪 JSON `data` 的点路径（如 `units[].no`）；仅 `--format json` 下可用 |
| `--offset N` | 跳过前 N 个单元（0 起；两形态同用），大文档分页读 |
| `--limit M` | 最多输出 M 个单元；JSON 形态有剩余时 meta 带 `next_offset` 与 `cta` |
| `--ocr` / `--offline` | 同 search：PDF 与图片的 needs_ocr 单元 OCR 兜底回填正文（`needs_ocr` 标记保留） |

输出格式：按单元分节，节头为 `== page N ==`（PDF 与图片）、`== section N ==`（标题节）或 `== part N ==`（超过 200 行的单元按行分片：无标题整篇或超长节；P0010/P0011），随后为该单元的文本行；输出行为 markdown 形态（PDF 走 pdf-inspector 布局管线，anydoc 家族为 GFM：标题、表格、列表、代码块）。文本层不可靠页（扫描件、编码问题、乱码、空提取，及图片文件）在节头后第一行给 `[needs_ocr: 原因]` 提示。退出码：成功 0，出错 2。

示例：

```bash
reader extract ./doc.pdf
reader extract ./doc.pdf --pages 1-3,5
rr extract ./report.docx -o report.txt
```

### query 结构化提取

```text
reader query <文件> <mq表达式> [--format text|json] [--filter 路径]
```

用 mq 表达式（jq 风格，学习自 [harehare/mq](https://github.com/harehare/mq)，完整语法见 [mqlang.org](https://mqlang.org)）对文档做结构化提取：`.h1`..`.h6` 标题、`.code` 代码块、`.link` 链接、`.table` 表格、`.list` 列表，管道组合如 `.[] | select(contains("关键词"))`。输入面：`.md` / `.markdown` 原文直查，`.pdf` 与 anydoc 家族先转 markdown 再查；不支持目录。text 形态逐命中输出 markdown 片段原文；json 形态 `data` 为 `results[]` 加 `count`。退出码同 search：命中 0、无命中 1、出错 2。

```bash
reader query ./README.md ".h2"
reader query ./notes.md ".[] | select(contains(\"配置\"))" --format json --filter 'results[]'
```

## JSON 输出

`--format json` 给 Agent 结构化包膜（compact 单行）：

```text
成功：{"ok":true,"data":<数据>,"meta":{"command":...,"duration_ms":...}}
失败：{"ok":false,"error":"<原因>","meta":{...}}   # stdout 出包膜，stderr 仍出人读行，退出码不变
```

- search 的 `data`：`hits[]`（`unit` / `line` / `text` / `before[]` / `after[]`）加 `needs_ocr_units[]`（不可靠页序号）。无命中是 `ok:true` 加空 `hits`，退出码仍 1（`ok` 表执行成败，与命中有无分轨）。
- extract 的 `data`：`units[]`（`kind` / `no` / `needs_ocr` / `lines[]`）。
- query 的 `data`：`results[]`（mq 命中的 markdown 片段）加 `count`。
- figures 的 `data`：`figures[]`（`kind` / `anchor` / `caption` / `context[]` / `file` / `bytes` / `format`）加 `count`。
- 分页：`--offset/--limit` 后 meta 有剩余时附 `next_offset` 与 `cta`（下一条可直接执行的命令）。
- `--filter` 点路径裁剪 `data`（包膜保留）：`hits[].text`（数组映射）、`units[0].lines`（下标）、`hits[].unit` 等键访问链；非法路径报错退出 2，不静默。

```bash
reader search ./doc.pdf "error" --format json
reader search ./doc.pdf "error" --format json --filter 'hits[].unit'
reader extract ./doc.pdf --format json --offset 0 --limit 20
```

### figures：图片本体导出与元数据对齐（D47）

把文档里的图片本体落盘，并与文本元数据对齐（定位回文档的锚、图题候选、图题后上下文行）；图表理解交给调用方的多模态模型（reader 不载模型，S010 定界）。

| 路径 | 行为 |
| --- | --- |
| PDF | 优先内嵌位图直抽（DCT 原字节 jpg、Flate 按色彩空间解码 png；论文插图的正确本体）；页无内嵌图且是扫描页才回退整页渲染 PNG；图题与上下文从页文本层对齐 |
| markdown | 解析 `![alt](path)` 引用并复制（悬空跳过；远程引用不抓取）；alt 即图题 |
| anydoc 家族 | zip 直读内嵌图片部件原字节（锚为部件路径；EPUB 封面与插图即此路；legacy 二进制容器族 v1 无图可导） |
| 图片文件 | 本体即自身，原字节复制 |

输出行式 `figure: kind | 锚 | 图题或- | 落盘路径 | 字节数B`；退出码：有图 0 / 无图 1 / 出错 2。缺省输出目录 `<文件名>-figures/`。

```bash
reader figures ./scan.pdf --pages 12-32
reader figures ./report.docx --format json --filter 'figures[].anchor'
```

### export：一键完整提取（D47）

`reader export <文件> [--pages] [--out DIR] [--ocr] [--offline]`：文本、图片与对齐元数据一次落一个目录

```text
<文件名>-export/
  manifest.json   对齐索引：units（no/kind/needs_ocr/行数/page_file）与 figures（锚/图题/文件）
  text.md         连续全文（extract text 同形态）        text.json  units 全量（机器用）
  pages/p0001.md  逐单元文本（markdown，支持格式）
  images/         图本体（figures 同源：PDF 内嵌图 / 扫描页渲染 / Office 内嵌件）
```

导出目录可直接 `search` 二次复用（pages/ 是 markdown，命中行带 `pages/p0012.md` 页锚即回原文档定位）：

```bash
reader export ./paper.pdf --ocr
reader export ./book.epub --out ./book-everything
reader search ./paper-export/ "certificate" -i
# paper-export\pages\p0009.md:1:3:certificates
```

## 支持格式

| 格式 | 单元 | 引擎与说明 |
| --- | --- | --- |
| markdown（.md / .markdown） | 标题节 | 原文直读进分节管线（P0016）；与 anydoc 家族同口径 |
| PDF（.pdf） | 页 | pdf-inspector markdown 布局管线：多栏阅读序、表格成形、needs_ocr 检出 |
| Word（.doc / .docx / .docm） | 标题节 | anydoc 统一引擎出 GFM；legacy .doc 直读 |
| EPUB（.epub） | 标题节 | anydoc；节界来自正文标题（spine 章序保留于阅读序） |
| ODT / RTF | 标题节 | anydoc |
| PowerPoint（.ppt(x) 等） | 标题节 | anydoc（含备注） |
| Excel（.xls / .xlsx / .xlsm / .xlsb） | 标题节 | anydoc（表格通道） |
| ODF 表格与演示（.ods / .odp） | 标题节 | anydoc |
| CSV（.csv） | part 分片（200 行） | anydoc；无签名格式按扩展名识别；无标题格式天然走分片 |
| 图片（.png / .jpg / .jpeg / .bmp / .gif / .webp / .tiff / .tif） | 页（单图即 page 1） | image crate 解码（内容嗅探、首帧、EXIF 方向、透明底合成白底）；无文本层恒标 `[needs_ocr: image]`，`--ocr` 识别（D43）；多帧动图取首帧 |

选型：anydoc 0.2.4（firecrawl，MIT），双通道核实与保真实测见 [S004-Word文档读取选型](docs/research/S004-Word文档读取选型-docx自解与doc直读双路线实测.md)，重构方案见 [P0009-anydoc统一文档引擎大重构](docs/proven/P0009-anydoc统一文档引擎大重构.md)；图片支持零新依赖（image 0.25 已随 OCR 管线在依赖树），研究见 S009。

边界：只读、不渲染、不编辑；扫描件与编码问题页检出后以 `[needs_ocr]` 提示，PDF 与图片单文件可加 `--ocr` 兜底识别（PP-OCRv6 tiny，首用下载约 6.2MB 模型，多核约 1-5 秒/页，仍标 needs_ocr；P0014/P0018、D43）。文本质量承诺面向英文与中文内容。分节口径：超过 200 行的单元（无标题整篇或超长节）按行分片为 part，单元号全局连续（P0010/P0011）。

## 文档导航

项目协作文档（贡献者向）：[AGENTS.md](AGENTS.md) 协作规则最高约束；[INDEX.md](INDEX.md) 全仓索引；[PRD.md](PRD.md) / [GOAL.md](GOAL.md) / [PLAN.md](PLAN.md) / [TODO.md](TODO.md) 四原语；[CHANGELOG.md](CHANGELOG.md) 与 [ROADMAP.md](ROADMAP.md) 版本与路线；深文档六目录（proven / diary / research / references / guide / mistakes）经 [INDEX.md](INDEX.md) 进。

## 贡献与支持

- 提问、报错与功能建议：开 [GitHub Issue](https://github.com/raystyle/reader_rs/issues)。
- 协作规则（四原语、工作规则、文档体系）：见 [AGENTS.md](AGENTS.md)。
- 接受 PR：一次提交只做一件事；改动须过门禁（cargo 三件加文档四件，清单见 [AGENTS.md](AGENTS.md) 二节）。

## 致谢

核心能力站在这些项目上：

- [pdf-inspector](https://github.com/firecrawl/pdf-inspector)（firecrawl，MIT）：PDF 布局管线，多栏阅读序与 needs_ocr 检出
- [anydoc](https://crates.io/crates/anydoc)（firecrawl，MIT）：Word / EPUB / ODT / RTF / Office / CSV 家族统一文档引擎
- [mq](https://github.com/harehare/mq)（harehare，MIT）：jq 风格 markdown 查询，`query` 子命令内核
- [ppocr-rs](https://github.com/weidix/ppocr-rs) 与 [hayro](https://crates.io/crates/hayro)：OCR 兜底的原生 CPU 内核与 PDF 渲染；模型来自 [PaddlePaddle PP-OCRv6](https://huggingface.co/PaddlePaddle)
- [clap](https://github.com/clap-rs/clap) / [serde](https://serde.rs) / [ureq](https://github.com/algesten/ureq)：CLI、序列化与下载基建

## License

MIT（SPDX 标识 `MIT`），全文见 [LICENSE](LICENSE)。
