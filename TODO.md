# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

anydoc 统一文档引擎大重构（对应 `GOAL.md`，方案 P0009，登记日 2026-09-01）。**2026-09-01 达成**：格式面 2 到 14 种，37 集成加 9 单元测试全绿，真样本四路回归（docx / 大 docx / legacy .doc / PDF 页契约）。EPUB 单元由章改节（破坏性变更记 CHANGELOG）。下版候选（MCP、TOON、musl、包管理器分发、无标题长文档分节策略等）待立项。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| S004 选型研究（docx 双路线） | 已完成 | 自解加 office_oxide 对照；anydoc 补测全绿；决策变更记录 | 2026-09-01 |
| 立项 P0009 | 已完成 | 方案、三原语、INDEX 登记 | 2026-09-01 |
| 引擎切换 | 已完成 | Cargo 依赖手术、`src\anydoc.rs`、`document.rs` 分派、删 `epub.rs`；feat 提交 e110e71 | 2026-09-01 |
| 测试与夹具 | 已完成 | docx/csv 夹具用例、legacy.doc 仓内资产、EPUB 断言改 section（含章界坑修正） | 2026-09-01 |
| 文案与收官 | 已完成 | lib/introspect/SKILL 再生成、门禁全绿、真样本回归、CHANGELOG/README/AGENTS/INDEX/ROADMAP/diary、P0009 回填 | 2026-09-01 |
