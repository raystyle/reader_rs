# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

P0014 OCR 落地（登记日 2026-09-03）。**2026-09-03 已达成**：`--ocr`/`--offline` 进 extract 与 search（仅 PDF 单文件 needs_ocr 页）；模型三件 ModelScope 下载加双套 SHA-256 钉死、进程内 prost strip；门禁与真样本回归全绿。过程与经验回填 `docs\proven\P0014-OCR兜底落地.md`。下一目标待用户点名（候选：阶段 3 余项 MCP/token 计数/pager/completions、分发面 crates.io/brew/scoop、OCR 质量升级 S006 待办 2）。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| 立项登记 | 已完成 | GOAL/TODO/PLAN 三原语；模型 URL 与双套 SHA-256 钉死 | 2026-09-03 |
| vendor pure-onnx-ocr | 已完成 | `vendor\pure-onnx-ocr\`（max_width 2560 加 println 转 eprintln 补丁）；patch 接入 | 2026-09-03 |
| src\ocr.rs | 已完成 | 缓存目录加环境覆盖、三件下载校验、prost strip、hayro 渲染、引擎现建 | 2026-09-03 |
| --ocr 接线 | 已完成 | extract 与 search 加 `--ocr`/`--offline`；目录加 `--ocr` 报错；OcrOpts 穿参 | 2026-09-03 |
| 测试 | 已完成 | dies_ 负例两件加模型缓存门控冒烟；16 单元加 46 集成全绿 | 2026-09-03 |
| 真样本回归 | 已完成 | 安全牛 PDF：无 `--ocr` 行为不变；`--ocr` 两页出正文；首用下载全流程实测 | 2026-09-03 |
| 门禁与回填 | 已完成 | fmt/clippy/test 加 rumdl 三件全绿；AGENTS/INDEX/SKILL/diary/P0014/M009-M011/CHANGELOG 收口 | 2026-09-03 |
