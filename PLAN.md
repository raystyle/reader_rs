# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**——基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：S006 内嵌 OCR 选型研究

> 已达成。用户点名研究「全平台支持的小型 OCR 内嵌」，登记日 2026-09-02；依据 `docs\research\S006-内嵌OCR选型-纯Rust管线hayro加pure-onnx-ocr实测可行.md`。

研究已收官：双通道普查九候选、PoC 端到端实证、坑位沉淀、INDEX/GOAL/TODO/diary 登记与文档门禁全绿。本文件保留下一目标（OCR 落地）的预案，待用户点名立项后转正式 plan。

## 下一目标预案：OCR 落地

> 待立项，编号拟 P0014。以下为预案草稿，立项时按实际点名范围裁剪。

### 1. 闸门

S006 已实证全链路可行：hayro 0.7 渲染加 pure-onnx-ocr 0.1（tract 跑 PP-OCRv5 mobile，模型 20.5MB）真样本跑通；两坑（tract value_info 剥离、rec max_width 320 硬编码）均有解。无新选型。[依据: S006 关键结论 1 与踩坑节]

### 2. 实施件与依据

| 件 | 要做什么 | 依据 |
| --- | --- | --- |
| `src\ocr.rs`（新） | hayro 渲染 needs_ocr 页为位图；tract 推理 det 加 rec；出行级文本与置信 | S006 PoC 实证路径 |
| 模型管理 | 首用从 ModelScope RapidOCR 仓下载三件进缓存目录，SHA-256 钉死校验（钉 `4d97c44a…` / `5825fc7e…` / `d1979e9f…`）；离线旗标报错不下载 | S006 关键结论 5 |
| `src\document.rs` / `src\pdf.rs` | needs_ocr 页接 `--ocr` 兜底；默认行为不变（只提示） | 边界：不破坏现有契约 |
| vendored pure-onnx-ocr | max_width 320 改 2560；观察上游或提 issue | S006 踩坑 2 |
| `tests\cli.rs` | needs_ocr 夹具加 `--ocr` 冒烟（模型走测试缓存或 mock 下载） | R003 |
| Cargo.toml | hayro、tract-onnx、pure-onnx-ocr（patch 或 vendor）进依赖；注意二进制体积变化 | S006 关键结论 5 |

### 3. 每件验收

门禁三件加 rumdl 三件套全绿；既有用例零改动；真样本回归（安全牛 PDF `--ocr` 出正文，无 `--ocr` 时行为与现状一致）；性能口径按异步兜底设计，不进热路径（19 到 42 秒/页量级，S006 实测）。

### 4. 边界

不做 OCR 质量调优（server 模型、PP-OCRv6 对比留 S006 待办 2）；不做平台原生 API 路线（S006 待办 3）；不做批量目录加 OCR 组合；模型不 include_bytes 内嵌。

## 完成的定义

> 本目标验收口径。

- S006 研究已达成并归档（本目标实际已完成）
- P0014 预案待用户点名立项后细化转正式 plan
