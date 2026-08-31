# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

PDF 提取质量：markdown 管线与 needs_ocr 提示（对应 `GOAL.md`，方案 P0005，登记日 2026-08-31）。上一目标 P0004 已收官（CI 三系统绿，WSL 本地门禁实证绿）。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| 立项 P0005 | 已完成 | `docs\proven\P0005-PDF提取质量-markdown管线与needs_ocr提示.md`；三原语与 ROADMAP/INDEX 登记 | 2026-08-31 |
| pdf.rs 切 markdown 管线 | 进行中 | `extract_pages_markdown` 接管；0/1 基页码换算；朴素行重建删除 | 2026-08-31 |
| TextUnit 扩展 needs_ocr | 进行中 | `Option<String>` 装原因；EPUB 通道恒 None（与上一项同批落地） | 2026-08-31 |
| 输出提示两路径 | 未开始 | extract 页节后 `[needs_ocr: 原因]` 行；search stderr 汇总警示 | 2026-08-31 |
| 测试补强与回归 | 未开始 | 两栏阅读序正例、无文本页检出例、search 警示例；既有 20 测回归 | 2026-08-31 |
| 真实样本回归与性能观察 | 未开始 | 390 页 PDF 多栏抽查对比旧输出；release 计时对比 S001 基线 0.57s | 2026-08-31 |
| 收官登记 | 未开始 | P0005 回填、CHANGELOG 破坏性变更行、diary、门禁与提交 | 2026-08-31 |
