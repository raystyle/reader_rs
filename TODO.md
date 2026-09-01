# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

anydoc 统一文档引擎大重构（对应 `GOAL.md`，方案 P0009，登记日 2026-09-01）：PDF 保留 pdf-inspector 直连页契约，其余十三格式统一 anydoc 按标题分节。选型与补测见 `docs\research\S004-Word文档读取选型-docx自解与doc直读双路线实测.md`（含决策变更记录：用户裁定推翻 docx 自解初判）。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| S004 选型研究（docx 双路线） | 已完成 | 自解加 office_oxide 对照；anydoc 补测全绿；决策变更记录 | 2026-09-01 |
| 立项 P0009 | 已完成 | 方案、三原语、INDEX 登记 | 2026-09-01 |
| 引擎切换 | 进行中 | Cargo 依赖手术、`src\anydoc.rs`、`document.rs` 分派、删 `epub.rs` | 2026-09-01 |
| 测试与夹具 | 未开始 | docx/csv 夹具用例、legacy.doc 资产、EPUB 断言改 section | 2026-09-01 |
| 文案与收官 | 未开始 | lib/introspect/SKILL 再生成、门禁、真样本回归、CHANGELOG/README/AGENTS/INDEX/ROADMAP/diary | 2026-09-01 |
