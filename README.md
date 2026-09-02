# Reader

Agent 原生文档阅读、搜索和提取工具。Rust 单二进制 CLI，从本地 PDF、markdown 与 Word / EPUB / ODT / RTF / Office / CSV 文档读文本层：PDF 按页读、其余格式按标题节读，按词或正则搜、按单元取、按 mq 表达式结构化提取；PDF 扫描件可 `--ocr` 兜底识别。输出稳定可解析（行式标记、grep 语义退出码），单调用无交互。仓库 `reader_rs`，CLI `reader`（缩写 `rr`，同一二进制两个名字）。远端 <https://github.com/raystyle/reader_rs>。

## 安装

前置：Rust 工具链（1.88+，推荐 <https://rustup.rs>）。支持 Windows / macOS / Linux，CI 三系统门禁见 `.github\workflows\ci.yml`。

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

仓根已提交一份 `SKILL.md`，与运行时输出逐字节一致（集成测试做漂移守卫）；升级版本后用上面命令刷新即可。

## 命令

文档子命令三个：`search`（搜）、`extract`（取）与 `query`（mq 结构化提取）；外加发现接口 `skill` 子命令与 `--llms` 旗标（见上节）、`self update` 自升级。输入文件按扩展名分派：`.pdf` 按页，markdown（`.md` / `.markdown`）与 anydoc 家族（`.doc` / `.docx` / `.epub` / `.odt` / `.rtf` / `.ppt(x)` / `.xls(x)` / `.ods` / `.odp` / `.csv`）按 GFM markdown 顶层标题分节。`search` 也接受目录：递归批量搜支持格式，命中行带路径前缀（P0012）。

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
| `--ocr` | PDF 单文件 needs_ocr 页走 OCR 兜底（PP-OCRv6 tiny 首用下载约 6.2MB 模型进缓存目录；多核并行约 1-5 秒/页；目录模式不可用） |
| `--offline` | 禁模型下载（须与 `--ocr` 同用；模型未就位时报错） |

输出格式（仿 grep，稳定可解析）：

```text
单元:行号:命中行文本        命中行（PDF 单元为页，其余格式为节）
单元-行号-上下文文本        上下文行（-C 时）
路径:单元:行号:命中行文本   目录批量模式（路径前缀；Windows 盘符冒号从右取三段解析）
```

文档含文本层不可靠页（扫描件、编码问题，仅 PDF）时，stderr 额外出一条 `needs_ocr` 警示并列出页码；stdout 不混入提示。退出码：命中 0；无命中 1；出错（缺文件、坏参数、不支持的格式、目录无支持格式文件）2。

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
| `--ocr` / `--offline` | 同 search：PDF needs_ocr 页 OCR 兜底回填正文（`needs_ocr` 标记保留） |

输出格式：按单元分节，节头为 `== page N ==`（PDF）、`== section N ==`（标题节）或 `== part N ==`（超过 200 行的单元按行分片：无标题整篇或超长节；P0010/P0011），随后为该单元的文本行；输出行为 markdown 形态（PDF 走 pdf-inspector 布局管线，anydoc 家族为 GFM：标题、表格、列表、代码块）。文本层不可靠页（扫描件、编码问题、乱码、空提取，仅 PDF）在节头后第一行给 `[needs_ocr: 原因]` 提示。退出码：成功 0，出错 2。

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

### self update 自升级

```text
reader self update [--force]
```

从 GitHub Releases 最新正式版下载本平台资产，sha256 digest 校验后替换当前运行的二进制与同目录兄弟（`reader` / `rr` 双名）；已最新时明示。`--force` 同版本重装；`GH_TOKEN` 环境变量注入认证（匿名限流 403 时自动回退 `gh api`）。只走 stable 通道，不做自动更新。

## JSON 输出

`--format json` 给 Agent 结构化包膜（compact 单行）：

```text
成功：{"ok":true,"data":<数据>,"meta":{"command":...,"duration_ms":...}}
失败：{"ok":false,"error":"<原因>","meta":{...}}   # stdout 出包膜，stderr 仍出人读行，退出码不变
```

- search 的 `data`：`hits[]`（`unit` / `line` / `text` / `before[]` / `after[]`）加 `needs_ocr_units[]`（不可靠页序号）。无命中是 `ok:true` 加空 `hits`，退出码仍 1（`ok` 表执行成败，与命中有无分轨）。
- extract 的 `data`：`units[]`（`kind` / `no` / `needs_ocr` / `lines[]`）。
- query 的 `data`：`results[]`（mq 命中的 markdown 片段）加 `count`。
- 分页：`--offset/--limit` 后 meta 有剩余时附 `next_offset` 与 `cta`（下一条可直接执行的命令）。
- `--filter` 点路径裁剪 `data`（包膜保留）：`hits[].text`（数组映射）、`units[0].lines`（下标）、`hits[].unit` 等键访问链；非法路径报错退出 2，不静默。

```bash
reader search ./doc.pdf "error" --format json
reader search ./doc.pdf "error" --format json --filter 'hits[].unit'
reader extract ./doc.pdf --format json --offset 0 --limit 20
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

选型：anydoc 0.2.4（firecrawl，MIT），双通道核实与保真实测见 `docs\research\S004-Word文档读取选型-docx自解与doc直读双路线实测.md`，重构方案 `docs\proven\P0009-anydoc统一文档引擎大重构.md`。

边界：只读、不渲染、不编辑；扫描件与编码问题页检出后以 `[needs_ocr]` 提示，PDF 单文件可加 `--ocr` 兜底识别（PP-OCRv6 tiny，首用下载约 6.2MB 模型，多核约 1-5 秒/页，仍标 needs_ocr；P0014/P0018）。文本质量承诺面向英文与中文内容。分节口径：超过 200 行的单元（无标题整篇或超长节）按行分片为 part，单元号全局连续（P0010/P0011）。

## 文档导航

项目协作文档（贡献者向）：`AGENTS.md` 最高约束；`INDEX.md` 全量索引；`GOAL/PLAN/TODO` 三原语；`docs\` 下 proven/diary/research/references/guide/mistakes 六目录。
