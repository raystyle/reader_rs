//! Agent 自省与发现（P0007）：`--llms` 紧凑索引与 `skill` 子命令的 SKILL.md 生成。
//! 文本为 curated 内容（含退出码、输出契约等 clap 不知道的语义）；
//! 漂移由 tests\cli.rs 双守卫兜底：clap 命令树旗标全覆盖断言 + 仓根 SKILL.md 逐字节一致断言。

/// `reader --llms`：紧凑命令索引（agent 发现用，单行一句、稳定可解析）。
pub fn llms_text() -> String {
    let v = env!("CARGO_PKG_VERSION");
    format!(
        "\
reader v{v} — Agent 原生文档阅读、搜索和提取工具（PDF 按页；markdown(.md) 与 Word/EPUB/ODT/RTF/Office/CSV 按标题节，只读文本层；缩写 rr 同入口）
reader search <文件|目录> <关键词> [--regex] [-i|--ignore-case] [-C|--context N] [--pages 范围] [--format text|json] [--filter 路径] [--ocr] [--offline]
reader extract <文件> [--pages 范围] [-o|--out 文件] [--format text|json] [--filter 路径] [--offset N] [--limit M] [--ocr] [--offline]
reader query <文件> <mq表达式> [--format text|json] [--filter 路径] — mq 结构化提取（.h2/.code/.link/select 管道；全格式面转 markdown 后查询）
reader skill — 输出 SKILL.md（本索引的长形态，含输出契约与示例）
reader self update [--force] — 自升级（GitHub Releases 最新正式版，资产 sha256 digest 校验后替换自身与兄弟二进制；GH_TOKEN 注入认证，限流回退 gh api）
reader --llms — 本索引
退出码: 0 成功或命中 / 1 无命中（仅 search） / 2 出错（stderr 人读行；--format json 时 stdout 另出错误包膜）
输出 text: 命中行 单元:行号:文本；上下文 单元-行号-文本；extract 节头 == page N ==、== section N == 或 == part N ==（超 200 行单元按行分片）；目录批量模式命中行前缀 路径:
输出 json: {{\"ok\":bool,\"data\":...,\"meta\":{{command,duration_ms[,next_offset,cta]}}}}；--filter 点路径裁剪 data（如 hits[].text）
不可靠页: 扫描件或编码问题页以 needs_ocr 提示（extract 节头后提示行，search 走 stderr）；--ocr 对 PDF 单文件兜底识别（首用下载约 20.5MB 模型，多核并行约 3-10 秒/页（P0017），mobile 模型有掉字；--offline 禁下载）
"
    )
}

/// `reader skill`：生成 SKILL.md（仓根提交同名文件，漂移由测试守卫）。
/// 风格（2026-09-03 用户裁定）：常用例子为主体，细节渐进引导到 `--help` 与 `--llms`，
/// 不做全量参数文档（全量面在 `--help` 与 README）。
pub fn skill_md() -> String {
    let v = env!("CARGO_PKG_VERSION");
    format!(
        "\
---
name: reader
description: Agent 原生文档阅读、搜索和提取工具。从本地 PDF、markdown 与 Word / EPUB / ODT / RTF / Office / CSV 文档读文本层：按页或节读、按词或正则搜、按单元取、按 mq 表达式结构化提取。输出稳定可解析，grep 语义退出码。
---

# Reader

Rust 单二进制 CLI（v{v}；命令 `reader`，缩写 `rr` 同入口）。只读本地文档文本层：PDF 按页；markdown（.md / .markdown）与 Word（.doc / .docx）、EPUB、ODT、RTF、PowerPoint、Excel、ODF、CSV 按标题节。无交互、无守护进程；机器可读优先，错误走 stderr。

## 常用例子

```bash
# 搜：单文件命中行 单元:行号:文本；目录递归批量搜（命中行带路径前缀）
reader search ./doc.pdf \"error\" -i -C 1
reader search ./docs \"配置\" --format json --filter 'hits[].file'
# 取：按页/节提取，大文档分页
reader extract ./doc.pdf --pages 1-3
reader extract ./report.docx --format json --offset 0 --limit 5
# 结构化提取：mq 表达式（jq 风格，语法见 mqlang.org）
reader query ./README.md \".h2\"
reader query ./notes.md \".[] | select(contains(\\\"配置\\\"))\" --format json
# 扫描件/乱码层 PDF：OCR 兜底（仅 PDF 单文件；首用下载约 20.5MB 模型，慢）
reader extract ./scan.pdf --ocr
# 自升级（GitHub Releases 最新正式版，校验后替换自身）
reader self update
```

## 输出契约

- 退出码：0 成功或命中；1 无命中（search/query）；2 出错（stderr 人读行，json 形态 stdout 另出错误包膜）。
- text 形态：search 命中行 `单元:行号:文本`（上下文 `单元-行号-文本`，目录模式前缀 `路径:`）；extract 节头 `== page N ==` / `== section N ==` / `== part N ==`（超 200 行单元按行分片），不可靠页节头后 `[needs_ocr: 原因]` 提示行；query 逐命中输出 markdown 片段原文。
- json 形态（`--format json`，compact 单行）：`{{\"ok\":bool,\"data\":...,\"meta\":{{command,duration_ms[,next_offset,cta]}}}}`；`--filter` 点路径裁剪 data（如 `hits[].text`、`results[]`）。
- needs_ocr 页（扫描件/乱码，仅 PDF）OCR 后仍保留标记：mobile 模型有系统性掉字。

## 渐进深入

- `reader --llms`：紧凑命令索引（省 token）。
- `reader <子命令> --help`：该命令的全部参数与示例（如 `reader query --help`）。
- 参数速查：search `[--regex] [-i|--ignore-case] [-C|--context N] [--pages] [--format] [--filter] [--ocr] [--offline]`；extract `[--pages] [-o|--out] [--format] [--filter] [--offset] [--limit] [--ocr] [--offline]`；query `[--format] [--filter]`；self update `[--force]`。
- 完整说明见 README.md。
"
    )
}
