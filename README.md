# Reader

Agent 原生文档阅读、搜索和提取工具。Rust 单二进制 CLI，从本地 PDF / EPUB 读文本层：按页读、按词或正则搜、按页取。输出稳定可解析（行式标记、grep 语义退出码），单调用无交互。仓库 `reader_rs`，CLI `reader`（缩写 `rr`，同一二进制两个名字）。远端 <https://github.com/raystyle/reader_rs>。

## 安装

前置：Rust 工具链（1.88+，推荐 <https://rustup.rs>）。支持 Windows / macOS / Linux，CI 三系统门禁见 `.github\workflows\ci.yml`。

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

## 命令

两个子命令：`search`（搜）与 `extract`（取）。输入文件按扩展名分派：`.pdf` 按页、`.epub` 按章（spine 阅读序）。

### search 搜索

```text
reader search <文件> <关键词> [--regex] [-i] [-C N] [--pages 范围]
```

| 参数 | 说明 |
| --- | --- |
| `<文件>` | 文档路径（.pdf / .epub） |
| `<关键词>` | 字面匹配串；`--regex` 时按正则解释 |
| `--regex` | 按正则匹配（regex crate 语法） |
| `-i`, `--ignore-case` | 忽略大小写 |
| `-C N`, `--context N` | 命中行前后各带 N 行上下文 |
| `--pages 范围` | 限定页/章（1 起），写法 `1-3,5` |

输出格式（仿 grep，稳定可解析）：

```text
页:行号:命中行文本        命中行
页-行号-上下文文本        上下文行（-C 时）
```

退出码：命中 0；无命中 1；出错（缺文件、坏参数、不支持的格式）2。

示例（bash 与 pwsh 同形，Windows 路径换成反斜杠形态即可）：

```bash
reader search ./doc.pdf "error" -i -C 1
reader search ./doc.pdf "err(or|code)" --regex --pages 2-10
rr search ./book.epub "Get-Process"
```

### extract 提取

```text
reader extract <文件> [--pages 范围] [-o 输出文件]
```

| 参数 | 说明 |
| --- | --- |
| `<文件>` | 文档路径（.pdf / .epub） |
| `--pages 范围` | 限定页/章（1 起），写法 `1-3,5`；缺省全部 |
| `-o`, `--out 文件` | 写入文件；缺省输出到 stdout |

输出格式：按单元分节，节头为 `== page N ==`（PDF）或 `== chapter N ==`（EPUB），随后为该页/章的文本行。退出码：成功 0，出错 2。

示例：

```bash
reader extract ./doc.pdf
reader extract ./doc.pdf --pages 1-3,5
rr extract ./book.epub -o book.txt
```

## 支持格式

| 格式 | 单元 | 说明 |
| --- | --- | --- |
| PDF | 页 | pdf-inspector 位置感知提取，行按坐标重建 |
| EPUB | 章（spine 序） | rbook 解容器，XHTML 文本化；pre 代码块保留换行 |

边界：只读、不渲染、不编辑、不做 OCR；扫描件 PDF 无文本层时提不出内容。

## 文档导航

项目协作文档（贡献者向）：`AGENTS.md` 最高约束；`INDEX.md` 全量索引；`GOAL/PLAN/TODO` 三原语；`docs\` 下 proven/diary/research/references/guide/mistakes 六目录。
