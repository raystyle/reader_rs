---
id: ADR-0002
title: anydoc统一文档引擎与PDF直连双轨
status: accepted
date: 2026-09-16
deciders:
  - ray
supersedes: []
superseded_by: null
tags: [架构, 格式面]
---

# ADR-0002:anydoc统一文档引擎与PDF直连双轨

## Context

格式面从 2 种扩到 14 种时，备选是逐格式各接一个 crate 或统一引擎。选型双路线实测（docx 自解丢实体 vs anydoc 保真含 legacy .doc 直读）见 `../research/S004-Word文档读取选型-docx自解与doc直读双路线实测.md`；重构决策择要即本篇（原 P0009 方案档案已清退，见 ADR-0006）。

## Decision

Word（含 legacy .doc）/ EPUB / ODT / RTF / PowerPoint / Excel / ODF / CSV 统一走 anydoc 0.2.4 出 GFM markdown 按标题分节；PDF 保持 pdf-inspector 直连不走 anydoc，保页契约（页级单元与 needs_ocr 检出）。

## Consequences

- 好：一族格式一条管线一份分节口径，测试面可镜像 anydoc 官方语料（D44 71 件入仓）；PDF 页契约不被 GFM 分节吞掉
- 坏：EPUB 单元由章改节是破坏性变更（v0.2.0 已发）；anydoc 版本升级牵一族格式需全量回归；两轨分派逻辑要维护（src/document.rs 为真源）
