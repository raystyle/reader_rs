//! Agent 自省与发现（P0007）：`--llms` 紧凑索引与 `skill` 子命令的 SKILL.md 生成。
//! 文本为 curated 内容（含退出码、输出契约等 clap 不知道的语义）；
//! 漂移由 tests\cli.rs 双守卫兜底：clap 命令树旗标全覆盖断言 + 仓根 SKILL.md 逐字节一致断言。

/// `reader --llms`：紧凑命令索引（agent 发现用，单行一句、稳定可解析）。
pub fn llms_text() -> String {
    let v = env!("CARGO_PKG_VERSION");
    format!(
        "\
reader v{v} — Agent 原生文档阅读、搜索和提取工具（PDF 按页；Word/EPUB/ODT/RTF/Office/CSV 按标题节，只读文本层；缩写 rr 同入口）
reader search <文件|目录> <关键词> [--regex] [-i|--ignore-case] [-C|--context N] [--pages 范围] [--format text|json] [--filter 路径] [--ocr] [--offline]
reader extract <文件> [--pages 范围] [-o|--out 文件] [--format text|json] [--filter 路径] [--offset N] [--limit M] [--ocr] [--offline]
reader skill — 输出 SKILL.md（本索引的长形态，含输出契约与示例）
reader self update [--force] — 自升级（GitHub Releases 最新正式版，资产 sha256 digest 校验后替换自身与兄弟二进制；GH_TOKEN 注入认证，限流回退 gh api）
reader --llms — 本索引
退出码: 0 成功或命中 / 1 无命中（仅 search） / 2 出错（stderr 人读行；--format json 时 stdout 另出错误包膜）
输出 text: 命中行 单元:行号:文本；上下文 单元-行号-文本；extract 节头 == page N ==、== section N == 或 == part N ==（超 200 行单元按行分片）；目录批量模式命中行前缀 路径:
输出 json: {{\"ok\":bool,\"data\":...,\"meta\":{{command,duration_ms[,next_offset,cta]}}}}；--filter 点路径裁剪 data（如 hits[].text）
不可靠页: 扫描件或编码问题页以 needs_ocr 提示（extract 节头后提示行，search 走 stderr）；--ocr 对 PDF 单文件兜底识别（首用下载约 20.5MB 模型，19-42 秒/页，mobile 模型有掉字；--offline 禁下载）
"
    )
}

/// `reader skill`：生成 SKILL.md（仓根提交同名文件，漂移由测试守卫）。
pub fn skill_md() -> String {
    let v = env!("CARGO_PKG_VERSION");
    format!(
        "\
---
name: reader
description: Agent 原生文档阅读、搜索和提取工具。从本地 PDF 与 Word / EPUB / ODT / RTF / Office / CSV 文档读文本层——按页或节读、按词或正则搜、按单元取。输出稳定可解析，grep 语义退出码。
---

# Reader

Rust 单二进制 CLI（v{v}；命令 `reader`，缩写 `rr` 同入口）。只读本地文档文本层：PDF 按页；Word（.doc / .docx）、EPUB、ODT、RTF、PowerPoint、Excel、ODF、CSV 按标题节。无交互、无守护进程；机器可读优先，错误走 stderr。

## 何时使用

- 在本地文档中定位关键词或正则命中（行式 `单元:行号:文本`，直接可解析）。
- 在目录里找哪些文档提到某词：`search` 直接给目录，递归批量搜，命中行带路径前缀。
- 把文档文本层喂给 LLM 上下文：大文档用 `--offset` / `--limit` 分页，按 meta 的 `next_offset` 与 `cta` 链式推进。
- 要结构化结果：`--format json` 包膜，`--filter` 点路径裁剪只取所需字段。

不适用：渲染、编辑、支持列表以外格式。扫描件与乱码层 PDF 以 needs_ocr 检出提示；PDF 单文件可加 `--ocr` 兜底识别（mobile 模型有系统性掉字，仍标 needs_ocr）。

## 命令

### search 搜索

```text
reader search <文件|目录> <关键词> [--regex] [-i|--ignore-case] [-C|--context N] [--pages 范围] [--format text|json] [--filter 路径] [--ocr] [--offline]
```

- 文件或目录：目录递归批量搜支持格式（顺序遍历，路径排序稳定）；text 命中行 `路径:单元:行号:文本`，json `hits[]` 带 `file` 字段加 `files.scanned / files.skipped` 统计；坏文件 stderr 跳过后继续；`--pages` 与 `--ocr` 目录下不可用。
- `--regex`：关键词按正则解释（regex crate 语法）。
- `-i`, `--ignore-case`：忽略大小写。
- `-C N`, `--context N`：命中行前后各带 N 行上下文（`单元-行号-文本` 形态）。
- `--pages 范围`：限定页或节（1 起），写法 `1-3,5`；仅单文件模式。
- `--ocr`：对 needs_ocr 页走 OCR 兜底（仅 PDF 单文件；首用从 ModelScope 下载约 20.5MB 模型进缓存目录，SHA-256 钉死校验；约 19-42 秒/页；`needs_ocr` 标记保留）。`--offline` 禁下载（模型未就位时报错）。
- `--format json`：data 为 `hits[]`（unit / line / text / before / after；批量另有 file）加 `needs_ocr_units[]`（仅单文件）。无命中是 `ok:true` 加空 hits，退出码仍 1。
- `--filter 路径`：裁剪 json 的 data，如 `hits[].text`、批量 `hits[].file`；仅 json 形态可用。

### extract 提取

```text
reader extract <文件> [--pages 范围] [-o|--out 文件] [--format text|json] [--filter 路径] [--offset N] [--limit M] [--ocr] [--offline]
```

- `--pages 范围`：限定页或节（1 起）；缺省全部。
- `-o`, `--out 文件`：写入文件；缺省 stdout。
- `--offset N` / `--limit M`：按单元分页（0 起）；json 形态有剩余时 meta 带 `next_offset` 与 `cta`（下一条可直接执行的命令）。
- `--ocr` / `--offline`：同 search，对 needs_ocr 页 OCR 兜底回填正文（仅 PDF）。
- `--format json`：data 为 `units[]`（kind / no / needs_ocr / lines）。
- `--filter 路径`：裁剪 json 的 data，如 `units[].no`；仅 json 形态可用。

### skill 与 --llms

- `reader skill`：输出本文件（重定向可写回仓根 SKILL.md）。
- `reader --llms`：紧凑命令索引（本文件的省 token 形态）。

### self update 自升级

```text
reader self update [--force]
```

- 从 GitHub Releases 最新正式版下载本平台资产，sha256 digest 校验后替换当前运行的二进制与同目录兄弟（reader / rr 双名）；已最新时输出 `self_update: current <版本>`。
- `--force`：版本相同也强制重装。
- 环境变量 `GH_TOKEN` 注入 GitHub 认证（匿名限流 403 时自动回退 `gh api`）。
- 输出行：`self_update: updated <旧> -> <新>` 加每条 `path: <替换路径>`；出错退出 2，stderr 出人读原因。
- 只走 stable 正式版通道；不做自动更新，仅显式执行。

## 输出契约

text 形态（缺省）：

- search 命中行 `单元:行号:文本`；上下文行 `单元-行号-文本`。PDF 单元是页，其余格式单元是标题节；超过 200 行的单元（无标题整篇或超长节）按行分片为 part。
- extract 按单元分节，节头 `== page N ==`、`== section N ==` 或 `== part N ==`；输出行为 markdown 形态（标题、链接、表格语法；anydoc 家族为 GFM）。
- 文本层不可靠页（扫描件、编码问题，仅 PDF）：extract 在节头后出 `[needs_ocr: 原因]` 提示行；search 走 stderr 警示，stdout 不混入。

json 形态（`--format json`，compact 单行）：

```text
成功: {{\"ok\":true,\"data\":<数据>,\"meta\":{{\"command\":...,\"duration_ms\":...}}}}
失败: {{\"ok\":false,\"error\":\"<原因>\",\"meta\":{{...}}}}
```

失败时 stdout 出包膜、stderr 仍出人读行，退出码不变。

## 退出码

- 0：成功；search 为有命中。
- 1：search 无命中（执行本身成功）。
- 2：出错（缺文件、坏参数、不支持的格式）。

## 示例

```bash
reader search ./doc.pdf \"error\" -i -C 1
reader search ./doc.pdf \"err(or|code)\" --regex --pages 2-10
reader search ./report.docx \"配置\" --format json --filter 'hits[].unit'
reader extract ./doc.pdf --pages 1-3
reader search ./docs \"配置\" --format json --filter 'hits[].file'
reader extract ./report.docx --format json --offset 0 --limit 5
```
"
    )
}
