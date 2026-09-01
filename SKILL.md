---
name: reader
description: Agent 原生文档阅读、搜索和提取工具。从本地 PDF 与 Word / EPUB / ODT / RTF / Office / CSV 文档读文本层——按页或节读、按词或正则搜、按单元取。输出稳定可解析，grep 语义退出码。
---

# Reader

Rust 单二进制 CLI（v0.1.0；命令 `reader`，缩写 `rr` 同入口）。只读本地文档文本层：PDF 按页；Word（.doc / .docx）、EPUB、ODT、RTF、PowerPoint、Excel、ODF、CSV 按标题节。无交互、无守护进程；机器可读优先，错误走 stderr。

## 何时使用

- 在本地文档中定位关键词或正则命中（行式 `单元:行号:文本`，直接可解析）。
- 把文档文本层喂给 LLM 上下文：大文档用 `--offset` / `--limit` 分页，按 meta 的 `next_offset` 与 `cta` 链式推进。
- 要结构化结果：`--format json` 包膜，`--filter` 点路径裁剪只取所需字段。

不适用：扫描件 OCR（只检出并以 needs_ocr 提示，不识别）、渲染、编辑、支持列表以外格式。

## 命令

### search 搜索

```text
reader search <文件> <关键词> [--regex] [-i|--ignore-case] [-C|--context N] [--pages 范围] [--format text|json] [--filter 路径]
```

- `--regex`：关键词按正则解释（regex crate 语法）。
- `-i`, `--ignore-case`：忽略大小写。
- `-C N`, `--context N`：命中行前后各带 N 行上下文（`单元-行号-文本` 形态）。
- `--pages 范围`：限定页或节（1 起），写法 `1-3,5`。
- `--format json`：data 为 `hits[]`（unit / line / text / before / after）加 `needs_ocr_units[]`。无命中是 `ok:true` 加空 hits，退出码仍 1。
- `--filter 路径`：裁剪 json 的 data，如 `hits[].text`；仅 json 形态可用。

### extract 提取

```text
reader extract <文件> [--pages 范围] [-o|--out 文件] [--format text|json] [--filter 路径] [--offset N] [--limit M]
```

- `--pages 范围`：限定页或节（1 起）；缺省全部。
- `-o`, `--out 文件`：写入文件；缺省 stdout。
- `--offset N` / `--limit M`：按单元分页（0 起）；json 形态有剩余时 meta 带 `next_offset` 与 `cta`（下一条可直接执行的命令）。
- `--format json`：data 为 `units[]`（kind / no / needs_ocr / lines）。
- `--filter 路径`：裁剪 json 的 data，如 `units[].no`；仅 json 形态可用。

### skill 与 --llms

- `reader skill`：输出本文件（重定向可写回仓根 SKILL.md）。
- `reader --llms`：紧凑命令索引（本文件的省 token 形态）。

## 输出契约

text 形态（缺省）：

- search 命中行 `单元:行号:文本`；上下文行 `单元-行号-文本`。PDF 单元是页，其余格式单元是标题节。
- extract 按单元分节，节头 `== page N ==` 或 `== section N ==`；输出行为 markdown 形态（标题、链接、表格语法；anydoc 家族为 GFM）。
- 文本层不可靠页（扫描件、编码问题，仅 PDF）：extract 在节头后出 `[needs_ocr: 原因]` 提示行；search 走 stderr 警示，stdout 不混入。

json 形态（`--format json`，compact 单行）：

```text
成功: {"ok":true,"data":<数据>,"meta":{"command":...,"duration_ms":...}}
失败: {"ok":false,"error":"<原因>","meta":{...}}
```

失败时 stdout 出包膜、stderr 仍出人读行，退出码不变。

## 退出码

- 0：成功；search 为有命中。
- 1：search 无命中（执行本身成功）。
- 2：出错（缺文件、坏参数、不支持的格式）。

## 示例

```bash
reader search ./doc.pdf "error" -i -C 1
reader search ./doc.pdf "err(or|code)" --regex --pages 2-10
reader search ./report.docx "配置" --format json --filter 'hits[].unit'
reader extract ./doc.pdf --pages 1-3
reader extract ./report.docx --format json --offset 0 --limit 5
```
