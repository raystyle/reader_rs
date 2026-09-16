---
id: REQ-039
title: 工程效率件edition2024-nextest-llvm-cov-deny 原D39
status: draft
priority: should
trace: null
---

# REQ-039:工程效率件edition2024-nextest-llvm-cov-deny 原D39

## Scenario

CI 门禁扩展轮到来时，一批工程效率件候选启用：edition 2024 加 rust-toolchain.toml 钉死、nextest 运行器、llvm-cov 覆盖率、cargo deny / audit 供应链闸。裁定表见 `../guide/G007-RustCLI工程基线-供稿要点逐项裁定.md` 一与三节。

## Criteria

- [ ] 触发条件：CI 门禁扩展轮立项；未触发不动
- [ ] edition 2024 迁移单独走一轮（全量编译与测试回归）
- [ ] 供应链闸（deny / audit）采纳时定策略：失败堵门还是报告（ppocr-rs git 依赖的 advisory 口径先核）
