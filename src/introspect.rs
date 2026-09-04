//! Agent 自省与发现（P0007）：`--llms` 紧凑索引与 `skill` 子命令的 SKILL.md 生成。
//! 文本为 curated 内容（含退出码、输出契约等 clap 不知道的语义）；
//! 漂移由 tests\cli.rs 双守卫兜底：clap 命令树旗标全覆盖断言 + 仓根 SKILL.md 逐字节一致断言。

/// `reader --llms`：紧凑命令索引（agent 发现用，单行一句、稳定可解析）。
pub fn llms_text() -> String {
    let v = env!("CARGO_PKG_VERSION");
    format!(
        "\
reader v{v} — Agent 原生文档阅读、搜索和提取工具（PDF 按页；markdown(.md) 与 Word/EPUB/ODT/RTF/Office/CSV 按标题节；图片 png/jpg/jpeg/bmp/gif/webp/tiff/tif 单图即单页；只读文本层；缩写 rr 同入口）
reader search <文件|目录> <关键词> [--regex] [-i|--ignore-case] [-C|--context N] [--pages 范围] [--format text|json] [--filter 路径] [--ocr] [--offline]
reader extract <文件> [--pages 范围] [-o|--out 文件] [--format text|json] [--filter 路径] [--offset N] [--limit M] [--ocr] [--offline]
reader query <文件> <mq表达式> [--format text|json] [--filter 路径] — mq 结构化提取（.h2/.code/.link/select 管道；全格式面转 markdown 后查询）
reader skill — 输出 SKILL.md（本索引的长形态，含输出契约与示例）
reader self update [--force] — 自升级（镜像 latest.json 优先、回退 GitHub Releases；资产 sha256 校验后替换自身与兄弟二进制；GH_TOKEN 注入认证，限流回退 gh api）
reader ocr init [--size tiny|small] [--offline] — 下载/修复 OCR 模型进缓存（镜像 到 HF 到 GitHub Releases 三级回退；--offline 只校验不下载）
reader ocr doctor — 诊断本地 OCR 模型就位情况（只读；退出码 0 为当前档双包完整 / 1 为有缺损；镜像探活为信息行）
reader ocr switch <tiny|small> — 切换模型档位并持久化（env READER_OCR_MODEL_SIZE 优先于本设置）
reader --llms — 本索引
退出码: 0 成功或命中 / 1 无命中（仅 search） / 2 出错（stderr 人读行；--format json 时 stdout 另出错误包膜）
输出 text: 命中行 单元:行号:文本；上下文 单元-行号-文本；extract 节头 == page N ==、== section N == 或 == part N ==（超 200 行单元按行分片）；目录批量模式命中行前缀 路径:
输出 json: {{\"ok\":bool,\"data\":...,\"meta\":{{command,duration_ms[,next_offset,cta]}}}}；--filter 点路径裁剪 data（如 hits[].text）
不可靠页: 扫描件、编码问题页与图片文件以 needs_ocr 提示（extract 节头后提示行，search 走 stderr）；--ocr 对 PDF 与图片单文件兜底识别（首用下载约 6.2MB 模型，多核并行约 1-5 秒/页（PP-OCRv6 tiny，P0018）；--offline 禁下载）
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
description: Agent 原生文档阅读、搜索和提取工具。从本地 PDF、markdown、图片与 Word / EPUB / ODT / RTF / Office / CSV 文档读文本层：按页或节读、按词或正则搜、按单元取、按 mq 表达式结构化提取、图片 OCR 识别。输出稳定可解析，grep 语义退出码。
---

# Reader

Rust 单二进制 CLI（v{v}；命令 `reader`，缩写 `rr` 同入口）。只读本地文档文本层：PDF 按页；markdown（.md / .markdown）与 Word（.doc / .docx）、EPUB、ODT、RTF、PowerPoint、Excel、ODF、CSV 按标题节；图片（.png / .jpg / .jpeg / .bmp / .gif / .webp / .tiff / .tif）单图即单页，`--ocr` 识别。无交互、无守护进程；机器可读优先，错误走 stderr。

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
# 扫描件/乱码层 PDF 与图片文件：OCR 兜底（PDF 与图片单文件；首用下载约 6.2MB 模型）
reader extract ./scan.pdf --ocr
reader extract ./photo.jpg --ocr
# OCR 模型管理：显式下载（镜像优先）、诊断本地就位、切换档位 tiny / small
reader ocr init
reader ocr doctor
reader ocr switch small
# 自升级（镜像 latest.json 优先、回退 GitHub Releases，校验后替换自身）
reader self update
```

## 输出契约

- 退出码：0 成功或命中；1 无命中（search/query）；2 出错（stderr 人读行，json 形态 stdout 另出错误包膜）。
- text 形态：search 命中行 `单元:行号:文本`（上下文 `单元-行号-文本`，目录模式前缀 `路径:`）；extract 节头 `== page N ==` / `== section N ==` / `== part N ==`（超 200 行单元按行分片），不可靠页节头后 `[needs_ocr: 原因]` 提示行；query 逐命中输出 markdown 片段原文。
- json 形态（`--format json`，compact 单行）：`{{\"ok\":bool,\"data\":...,\"meta\":{{command,duration_ms[,next_offset,cta]}}}}`；`--filter` 点路径裁剪 data（如 `hits[].text`、`results[]`）。
- needs_ocr 页（扫描件/乱码 PDF、图片文件）OCR 后仍保留标记：OCR 文本仍可能有误。
- ocr 子命令输出行式：`ocr_init:` / `ocr_doctor:` / `ocr_switch:` 前缀、ASCII token 前置（ok / missing / corrupt / download mirror|huggingface|github / verdict）；doctor 退出码 0 为当前档双包完整、1 为有缺损；init / switch 失败为 2。

## 渐进深入

- `reader --llms`：紧凑命令索引（省 token）。
- `reader <子命令> --help`：该命令的全部参数与示例（如 `reader query --help`）。
- 参数速查：search `[--regex] [-i|--ignore-case] [-C|--context N] [--pages] [--format] [--filter] [--ocr] [--offline]`；extract `[--pages] [-o|--out] [--format] [--filter] [--offset] [--limit] [--ocr] [--offline]`；query `[--format] [--filter]`；self update `[--force]`；ocr init `[--size tiny|small] [--offline]`；ocr doctor 无参；ocr switch `<tiny|small>`。
- 完整说明见 README.md。
"
    )
}
