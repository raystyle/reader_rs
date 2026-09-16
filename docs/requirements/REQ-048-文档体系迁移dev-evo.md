---
id: REQ-048
title: 文档体系迁移dev-evo
status: implemented
priority: must
trace: uv run /mnt/d/ProjectEvo/plugins/project-evo/skills/dev-evo/scripts/check.py . 退出码 0（PE 全项）
---

# REQ-048:文档体系迁移dev-evo

## Scenario

reader_rs 文档体系要从旧根原语（PRD / GOAL / PLAN / TODO / INDEX 加 docs 六目录）迁移到 dev-evo 文档即代码体系（AGENTS 五节合同加 ADR 加 REQ），不推倒重来，逐件迁移逐件对账。

## Criteria

- [x] dev-evo 骨架生成（init.py 幂等，存量件不覆盖）
- [x] AGENTS 重写为五节合同（旧三节全文留档 docs/guides/agents-legacy-three-sections.md）
- [x] 定位与架构级决策择要转 ADR 四件（指针承接不搬运，回指 proven 与 research）
- [x] 活跃队列按 D 号即 REQ 号转 REQ 八件、PRD 历史留档加迁移注记（历史不回填）
- [x] docs/README 地图承接 INDEX 职责、四原语与 INDEX 顶部迁移注记、research 与 diary 登记入册
- [x] 门禁接入：check.py PE 全项退出码 0（存量禁字为零，豁免机制在册；标题括号存量清偿）
- [x] Rust 三栈面：AGENTS Commands 含 fmt 与 clippy 与 test；cargo test --doc 与 missing_docs 一句裁定明示不适用
