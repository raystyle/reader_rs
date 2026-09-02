---
name: reader
description: Agent 原生文档阅读、搜索和提取工具。从本地 PDF、markdown 与 Word / EPUB / ODT / RTF / Office / CSV 文档读文本层：按页或节读、按词或正则搜、按单元取、按 mq 表达式结构化提取。输出稳定可解析，grep 语义退出码。
---

# Reader

Rust 单二进制 CLI（v0.3.0；命令 `reader`，缩写 `rr` 同入口）。只读本地文档文本层：PDF 按页；markdown（.md / .markdown）与 Word（.doc / .docx）、EPUB、ODT、RTF、PowerPoint、Excel、ODF、CSV 按标题节。无交互、无守护进程；机器可读优先，错误走 stderr。

## 常用例子

```bash
# 搜：单文件命中行 单元:行号:文本；目录递归批量搜（命中行带路径前缀）
reader search ./doc.pdf "error" -i -C 1
reader search ./docs "配置" --format json --filter 'hits[].file'
# 取：按页/节提取，大文档分页
reader extract ./doc.pdf --pages 1-3
reader extract ./report.docx --format json --offset 0 --limit 5
# 结构化提取：mq 表达式（jq 风格，语法见 mqlang.org）
reader query ./README.md ".h2"
reader query ./notes.md ".[] | select(contains(\"配置\"))" --format json
# 扫描件/乱码层 PDF：OCR 兜底（仅 PDF 单文件；首用下载约 6.2MB 模型）
reader extract ./scan.pdf --ocr
# 自升级（GitHub Releases 最新正式版，校验后替换自身）
reader self update
```

## 输出契约

- 退出码：0 成功或命中；1 无命中（search/query）；2 出错（stderr 人读行，json 形态 stdout 另出错误包膜）。
- text 形态：search 命中行 `单元:行号:文本`（上下文 `单元-行号-文本`，目录模式前缀 `路径:`）；extract 节头 `== page N ==` / `== section N ==` / `== part N ==`（超 200 行单元按行分片），不可靠页节头后 `[needs_ocr: 原因]` 提示行；query 逐命中输出 markdown 片段原文。
- json 形态（`--format json`，compact 单行）：`{"ok":bool,"data":...,"meta":{command,duration_ms[,next_offset,cta]}}`；`--filter` 点路径裁剪 data（如 `hits[].text`、`results[]`）。
- needs_ocr 页（扫描件/乱码，仅 PDF）OCR 后仍保留标记：OCR 文本仍可能有误。

## 渐进深入

- `reader --llms`：紧凑命令索引（省 token）。
- `reader <子命令> --help`：该命令的全部参数与示例（如 `reader query --help`）。
- 参数速查：search `[--regex] [-i|--ignore-case] [-C|--context N] [--pages] [--format] [--filter] [--ocr] [--offline]`；extract `[--pages] [-o|--out] [--format] [--filter] [--offset] [--limit] [--ocr] [--offline]`；query `[--format] [--filter]`；self update `[--force]`。
- 完整说明见 README.md。
