# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

P0018 OCR 换引擎 ppocr-rs（登记日 2026-09-03；依据 `docsesearch\S008`）。**2026-09-03 已达成**：v6 tiny 落地（git rev 钉 d07857c），真样本双路（离线缓存与在线首用）exit 0、掉字点全修；tract/pure-onnx-ocr/prost 出树、vendor 退役、二进制回落 28.3MB；门禁全绿。过程与经验回填 `docs\proven\P0018-OCR换引擎ppocr-rs.md`。下一目标待用户点名。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| S008 研究与裁决 | 已完成 | 四配置同页实测；研究文档落盘 | 2026-09-03 |
| 依赖切换 | 已完成 | ppocr-rs git rev 钉入；旧引擎与 vendor 出树 | 2026-09-03 |
| src\ocr.rs 重写 | 已完成 | ModelStore 加 OcrOptions（threads 全核）；hayro 转 RgbImage；offline 映射 | 2026-09-03 |
| 文案与测试 | 已完成 | mobile 掉字表述改 v6；冒烟门控改 tiny 目录；SKILL 重生 | 2026-09-03 |
| 真样本回归 | 已完成 | 双路 exit 0；掉字点全修；水印噪声与封面读散入档 | 2026-09-03 |
| 门禁与回填 | 已完成 | 门禁三件加 rumdl 四件全绿；P0018 归档加 diary 加 INDEX | 2026-09-03 |
