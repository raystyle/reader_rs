# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**：基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`PRD.md` = 要什么；`GOAL.md` = 要达成什么；`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：D28 / D29 / D30 测试体系与 PoC 目录

> 2026-09-03 已达成：R006 六层规范、tests\ab\ A/B 层加首跑报告、poc\ 目录约定与迁移。下一目标待用户点名后在此立项（候选见 `PRD.md` D23 至 D26）。

## 完成的定义

> 本目标验收口径。

- D28：`docs\references\R006-测试体系细则-六层分层与各层标准.md` 落盘，六层各定「测什么、落点、什么叫过、何时跑」，真样本行为基线登记。[实证: 2026-09-03 已达成]
- D29：`tests\ab\` 建成（README 协议、manifest 对象资源、expectations 独立来源检查点、合成样本 scan-cjk.pdf 入仓、external 真样本钉 sha256）；`.tools\ab_run.py` 跑批器出质量与性能报告；tiny vs small 首跑报告落 reports。[实证: 2026-09-03 已达成]
- D30：`poc\` 目录约定（poc\README.md 登记表、gitignore 钉产物与模型）、G002 增八节 PoC 产物约定、S006 PoC 源码迁入 poc\s006-ocr-mobile\。[实证: 2026-09-03 已达成]
- 支撑改动：`READER_OCR_MODEL_SIZE` 档位开关（tiny 默认、small 可选）进 src\ocr.rs 带单元测试；门禁三件加 rumdl 四件全绿。[实证: 2026-09-03 已达成]
