# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

D28 / D29 / D30 测试体系与 PoC 目录（登记日 2026-09-03；依据 `PRD.md`）。**2026-09-03 已达成**：G006 六层测试规范落盘；tests\ab\ A/B 层建成并首跑 tiny vs small（报告 tests\ab\reports\2026-09-03-tiny-vs-small.md）；`READER_OCR_MODEL_SIZE` 档位开关进 src\ocr.rs；poc\ 目录约定落地（G002 八节）加 S006 PoC 迁移。下一目标待用户点名（候选见 `PRD.md` D23 至 D26）。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| PRD 登记 D28-D30 | 已完成 | 六层规范、A/B 目录、PoC 目录三条 | 2026-09-03 |
| OCR 模型档位开关 | 已完成 | `READER_OCR_MODEL_SIZE` tiny/small；parse 单元测试两枚 | 2026-09-03 |
| poc 目录约定与迁移 | 已完成 | poc\README.md 登记表；S006 源码迁入；gitignore 钉产物与模型；G002 八节 | 2026-09-03 |
| tests\ab 目录与对象资源 | 已完成 | manifest 双样本（合成 scan-cjk 入仓加 external 真样本钉 sha256）；检查点独立来源 | 2026-09-03 |
| ab_run.py 跑批器 | 已完成 | 热跑计时、去空白匹配、报告落 reports；首跑暴露冷跑计时失真已修 | 2026-09-03 |
| 真样本 A/B 首跑 | 已完成 | tiny vs small：真样本 5/5 对 5/5、37 对 51 行、1.99s 对 5.13s/页；合成 1/5 对 4/5 | 2026-09-03 |
| G006 六层规范 | 已完成 | 六层定义、落点、口径、时机；真样本基线登记 | 2026-09-03 |
| 同步与门禁 | 已完成 | INDEX、AGENTS、.tools README、GOAL/PLAN/diary；门禁全绿 | 2026-09-03 |
