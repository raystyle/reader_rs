# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

EPUB 支持（对应 `GOAL.md`，方案 P0003，登记日 2026-08-31）。**2026-08-31 达成：cargo test 20 例全绿，真实样本 37 章回归正确。** 下一刀候选：ROADMAP 阶段 2/3（提取质量、Agent 原生输出形态），待立项。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| 选型研究 S003 | 已完成 | `epub` crate GPL-3.0 出局；选 rbook 加 quick-xml | 2026-08-31 |
| 立项 P0003 | 已完成 | `docs\proven\P0003-EPUB支持与格式分派.md` | 2026-08-31 |
| 提取层格式分派 | 已完成 | `src\document.rs` TextUnit 统一页/章；`src\epub.rs` rbook 加 quick-xml 文本化（pre 保换行） | 2026-08-31 |
| CLI 与测试 | 已完成 | 扩展名自动分派；EPUB 用例 4 个（builder 现造）；真实样本《Powershell For Sysadmins》37 章回归 | 2026-08-31 |
| 门禁与登记 | 已完成 | 门禁三件与 rumdl 三件套全绿；INDEX/CHANGELOG/diary 登记 | 2026-08-31 |

（P0003 已完成；过程与经验在方案文档「实施过程与经验」节。）
