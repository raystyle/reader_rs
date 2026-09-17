---
id: ADR-0001
title: 产品定位Agent原生文档阅读搜索和提取工具
status: accepted
date: 2026-09-16
deciders:
  - ray
supersedes: []
superseded_by: null
tags: [定位]
---

# ADR-0001:产品定位Agent原生文档阅读搜索和提取工具

## Context

首版只有 PDF search / extract 最小闭环，定位未定型。备选：面向人的通用文档阅读器、面向 Agent 管线的只读文本层工具。定位展开与边界全文见 `../references/R001-项目定位-Agent原生文档阅读搜索和提取工具.md`（原 P0002 方案档案已清退，见 ADR-0006）。

## Decision

服务对象先 Agent 后人：为 Agent 管线设计的 Rust 单二进制 CLI，从本地文档读文本层（读、搜、提取），不做渲染与编辑。输出稳定可解析：行式标记、grep 语义退出码 0/1/2、JSON 包膜；机器可读优先于人类美观，单调用完成一件事。

## Consequences

- 好：命令面与输出契约全部围绕可解析性设计（`--llms` 索引、SKILL 生成、JSON 包膜由此生长）；质量承诺可以收窄到英文与中文
- 坏：人类美观体验不是目标；不做编辑与渲染划出能力边界，相关需求一律拒绝防复问
