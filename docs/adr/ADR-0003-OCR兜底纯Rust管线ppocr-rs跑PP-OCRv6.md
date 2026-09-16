---
id: ADR-0003
title: OCR兜底纯Rust管线ppocr-rs跑PP-OCRv6
status: accepted
date: 2026-09-16
deciders:
  - ray
supersedes: []
superseded_by: null
tags: [OCR, 依赖]
---

# ADR-0003:OCR兜底纯Rust管线ppocr-rs跑PP-OCRv6

## Context

扫描件与图片文件无文本层，需要 OCR 兜底。约束：纯 Rust 单二进制、musl 静态可发、不外挂运行时。选型与换引擎全过程见 `../research/S006-内嵌OCR选型-纯Rust管线hayro加pure-onnx-ocr实测可行.md` 与 `../research/S008-OCR质量升级-ppocr-rs的PP-OCRv6原生内核双优胜出现管线换引擎.md`，方案全文见 `../proven/P0014-OCR兜底落地.md`、`../proven/P0017-OCR性能优化-宽度分组分批加组间并行.md`、`../proven/P0018-OCR换引擎ppocr-rs.md`。

## Decision

OCR 兜底走纯 Rust 管线：hayro 渲染 needs_ocr 页与图片文件为位图，ppocr-rs 原生 CPU 内核跑 PP-OCRv6（缺省 tiny 档，small 档候选 REQ-025）；不绑 ort / ONNX 运行时（RapidOCR 系全绑 ort，出局）。OCR 仅以 `--ocr` opt-in 兜底，默认只提示不识别。

## Consequences

- 好：musl 静态边界与单二进制保住；tiny 档 0.8 秒/页量级加掉字全修（S008 同页四配置对比双优）；模型套件 6.2MB
- 坏：ppocr-rs 未上 crates.io，Cargo.toml 钉 git rev，换版要同步 mirror pin 表（src/mirror.rs 单测闸）；水印区噪声行与封面大字读散属已知残差（needs_ocr 域内）
