# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

S006 内嵌 OCR 选型研究（登记日 2026-09-02，触发：安全牛扫描 PDF 81 页全 needs_ocr）。**2026-09-02 已达成**：纯 Rust 管线（hayro 加 pure-onnx-ocr 跑 PP-OCRv5 mobile）真样本端到端实证可行；研究文档 `docs\research\S006-内嵌OCR选型-纯Rust管线hayro加pure-onnx-ocr实测可行.md` 落盘。OCR 落地实现（`--ocr` 兜底、模型下载缓存）待立项。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| 双通道候选普查 | 已完成 | crates.io 加 gh 核实九候选（ocrs、oar-ocr、paddle-ocr-rs、rapidocr-core、rusto-rs、franken_ocr、leptess、pure-onnx-ocr、hayro） | 2026-09-02 |
| 边界裁决 | 已完成 | ocrs 拉丁限定出局；RapidOCR 系全绑 ort 或 MNN 破纯 Rust 边界 | 2026-09-02 |
| PoC 实证 | 已完成 | hayro 渲染（588ms/页）加 tract 推理 PP-OCRv5 mobile（19 到 42s/页）；中文正文置信 0.9 以上 | 2026-09-02 |
| 坑位沉淀 | 已完成 | tract value_info 剥离、rec max_width 320 硬编码、hayro 默认透明底、ppocr-rs 同名撞车 | 2026-09-02 |
| S006 落盘与登记 | 已完成 | research 落文档；INDEX/GOAL/TODO/diary 同步；文档门禁待跑 | 2026-09-02 |
