# Reader

一句话定位：**Agent 原生文档阅读、搜索和提取工具**。为 Agent 管线设计的 Rust CLI，从本地文档读文本层：按页读、按词/正则搜、按页取。输出稳定可解析（行式标记、grep 语义退出码），单调用无交互，机器可读优先于人类美观。显示名 Reader，仓库 `reader_rs`，CLI `reader`（缩写 `rr`，同一二进制两个名字）。远端 <https://github.com/raystyle/reader_rs>。首个支持格式为 PDF，提取引擎 firecrawl/pdf-inspector（选型见 `docs\research\S001-PDF文本提取crate选型-pdf-inspector双通道核实.md`）。

## 快速开始

先读：

```text
AGENTS.md          最高约束
GOAL.md            当前目标
PLAN.md            怎么做
TODO.md            做到哪
docs\proven\P0001-PDF文本搜索与提取CLI最小闭环.md
```

构建与使用：

```powershell
cargo build
cargo run --bin reader -- search .\document.pdf "关键词"
cargo run --bin reader -- search .\document.pdf "r.st" --regex -i -C 2
cargo run --bin reader -- extract .\document.pdf --pages 1-3,5
cargo run --bin reader -- search .\book.epub "Get-Process" -C 1
cargo run --bin rr -- extract .\book.epub -o out.txt
```

## 目录结构

核心布局（明细见 `INDEX.md`）：

```text
reader_rs/
  INDEX.md           文档总索引（P/S/R/G/M 编号定位）
  GOAL / PLAN / TODO 三原语
  src\               reader CLI（reader/rr 双 bin）
  tests\             集成测试（assert_cmd）
  docs\proven\       P 编号，已完成 plan 归档
  docs\diary\        项目日记（一天一篇总结自省）
  docs\research\     S 编号，研究原型过程（六态）
  docs\guide\        G 编号，元规范
  docs\references\   R 编号，开发测试参考
  docs\mistakes\     M 编号，错误速查
```

## 核心概念

- **按页提取**：文本项带页码与坐标，行按 y 聚类、行内按 x 排序重建
- **grep 语义**：search 命中退出 0、无命中退出 1、出错退出 2；`-C` 上下文、`-i` 忽略大小写、`--regex` 正则
- **六态标记**：研究与测试文档的事实性断言标实证 / 推断 / 经验 / 记忆 / 假设 / 直觉，标准见 `docs\guide\G002-研究标准细则-结构与六态标记.md`

## 常用命令

```powershell
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
rumdl check .
```

## 文档导航

- `AGENTS.md`：定位 / 操作规则 / 意图路由 / 资源索引
- `docs\guide\G001-文档标准细则-命名写作规范与rumdl检查.md`：命名与写作
- `docs\guide\G002-研究标准细则-结构与六态标记.md`：研究规范与六态标记
- `docs\references\R003-测试标准细则-分层断言与门禁流程.md`：测试分层与门禁
- `INDEX.md`：全量索引

## 环境前提

本机 2026-08-31：

- Windows，pwsh
- rustc / cargo 1.97.1（环境由 `D:\ohmyenv`（ohmypwsh 部署）提供）
