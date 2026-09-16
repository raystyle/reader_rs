---
id: REQ-036
title: 错误与诊断体系thiserror-anyhow-tracing 原D36
status: draft
priority: should
trace: null
---

# REQ-036:错误与诊断体系thiserror-anyhow-tracing 原D36

## Scenario

当前错误面以 anyhow 与 eprintln 为主。当公开库 API 或错误分类需求出现时，需要分层错误体系：库内 thiserror、边界 anyhow 加 cause chain、tracing 结构化诊断。裁定表在册见 `../guide/G007-RustCLI工程基线-供稿要点逐项裁定.md` 一节。

## Criteria

- [ ] 触发条件：公开库 API 面确立（如 REQ-024 crates.io 发布采纳）或错误分类需求实名化；未触发不动
- [ ] 若立：错误分类枚举先澄清（IO、格式、OCR、镜像、参数五族是否够用）
- [ ] 诊断输出不破行式契约：tracing 面 stderr only，stdout 契约零变化
