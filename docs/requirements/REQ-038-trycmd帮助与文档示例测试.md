---
id: REQ-038
title: trycmd帮助与文档示例测试 原D38
status: draft
priority: should
trace: null
---

# REQ-038:trycmd帮助与文档示例测试 原D38

## Scenario

README 与 SKILL 的示例命令数量上来后，示例漂移风险变大；trycmd 可把 markdown 里的示例变成可执行断言。裁定表见 `../guide/G007-RustCLI工程基线-供稿要点逐项裁定.md` 三节。

## Criteria

- [ ] 触发条件：README 示例数量明显增长或示例失准事故首例；未触发不动
- [ ] 若立：示例夹具口径对齐 G005（现造夹具，不引外部样本）
- [ ] 与既有漂移守卫（SKILL 快照、introspect curated 测试）分工划清不重叠
