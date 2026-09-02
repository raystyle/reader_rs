# S006-内嵌OCR选型-纯Rust管线hayro加pure-onnx-ocr实测可行

> 2026-09-02。触发点：用户用 reader 抽 `D:\安全牛《新一代自动化渗透测试工具与应用实践指南》--水印.pdf`，81 页全部 needs_ocr（第 1 页 scanned，其余疑似乱码文本层），当场点名研究「全平台支持的小型 OCR 内嵌进程序」。流程按 R002；六态标准见 G002。PoC 工程 `target\poc-ocr\`（gitignored，证据以本文记录为准）。

## 背景

Reader 边界此前明确不做 OCR（扫描件检出后提示，不识别）。用户要求研究把小型 OCR 内嵌进单二进制，约束：全平台（Windows 优先，依赖均跨平台）、编译级内嵌（非外部进程或服务）、尽量守住「纯 Rust 单二进制、不外挂二进制运行时」边界；文本质量承诺英文与中文。

## 关键结论

1. **纯 Rust 内嵌中文 OCR 可行，已端到端实证**。管线：hayro 0.7.1（PDF 页渲染为 PNG）加 pure-onnx-ocr 0.1.0（tract-onnx 0.20 跑 PP-OCRv5 mobile ONNX）。模型三件 det 4.6MB 加 rec 15.8MB 加 dict 74KB 共约 20.5MB，符合「小型」。真实乱码层样本一页跑通，正文行置信 0.9 以上。[实证: 2026-09-02 target\poc-ocr 安全牛 PDF 第 10 页全链路]
2. **ocrs（rten 系，1.9k 星，最成熟纯 Rust OCR）只认拉丁字母，中文出局**。README 明示「recognizes the Latin alphabet only」，CJK 支持 issue 8 仍 OPEN。[实证: 2026-09-02 gh 查 robertknight/ocrs README 与 issue 8]
3. **RapidOCR 系 Rust 库全部外挂原生运行时，破纯 Rust 边界**：oar-ocr 0.9.2、paddle-ocr-rs 0.6.1、rapidocr-core 0.2.2 均依赖 ort（ONNX Runtime C++ 二进制，默认 download-binaries）；rusto-rs 0.2.5 绑 MNN（C++）。若用户放宽「不外挂二进制运行时」，oar-ocr 最成熟（162 星、执行提供器矩阵全、模型 ModelScope 下载带 SHA-256、且同样用 hayro 渲染 PDF）。[实证: 2026-09-02 各仓 Cargo.toml]
4. **性能与质量代价要认账**：CPU release 下 det 限边 960 约 19 秒/页、1600 约 42 秒/页；mobile 模型有系统性掉字（用、示、至、长、高等单字丢失），粗体标题检测偏弱，水印覆盖行置信塌到 0.34 到 0.74。[实证: 同上 PoC 双配置实测]
5. **模型分发不建议 include_bytes 内嵌**：当前 reader.exe release 7.3MB，内嵌后约 28MB；建议首次运行从 ModelScope RapidOCR 仓下载进缓存目录并 SHA-256 钉死校验（oar-ocr 同模式；RapidOCR 仓官方文件自带 SHA-256）。[实证: 2026-09-02 本机二进制与 ModelScope API 元数据]
6. 备查路线两条：平台原生 OCR API（Windows.Media.Ocr 加 macOS Vision，零模型体积，但 Linux 无等价物）[推断: 平台 API 常识，未实测]；weidix/ppocr-rs（PP-OCRv6 safetensors 原生 CPU/WGPU 内核，MIT 或 Apache-2.0，0.1.0 社区新项目）[实证: 2026-09-02 gh 查仓，未实测质量]。

## 现状或实测

### 双通道核实

| 候选 | crates.io | GitHub | 裁决 |
| --- | --- | --- | --- |
| pure-onnx-ocr | 0.1.0，Apache-2.0，tract-onnx 0.20 纯 Rust | siska-tech/pure-onnx-ocr，17 星，2025-11-22 推 | 选中实证；库新、有坑（见踩坑节） |
| hayro | 0.7.1，MIT 或 Apache-2.0 | LaurenzV/hayro，758 星，2026-08-24 推 | 选中实证（纯 Rust PDF 渲染） |
| ocrs | 0.13.0，MIT 或 Apache-2.0 | robertknight/ocrs，1879 星，活跃 | 出局：拉丁字母限定 |
| oar-ocr | 0.9.2，Apache-2.0 | greatv/oar-ocr，162 星 | 边界外：ort 原生运行时 |
| paddle-ocr-rs | 0.6.1，Apache-2.0 | mg-chao/paddle-ocr-rs | 边界外：ort 加可选 opencv |
| rapidocr-core | 0.2.2，Apache-2.0 | White-NX/rapidocr-rs | 边界外：ort |
| rusto-rs | 0.2.5，MIT | byrizki/rusto-rs，12 星 | 边界外：MNN C++（自称 Pure Rust 名不副实） |
| franken_ocr | 0.9.0 | Dicklesworthstone/franken_ocr，315 星 | 出局：3B MoE VLM，非小型 |
| leptess | 0.14.0，MIT | houqp/leptess | 出局：tesseract C 库 FFI |

[实证: 2026-09-02 cargo info --registry crates-io 加 gh repo view 逐条]

### PoC 实测

样本：`D:\安全牛《新一代自动化渗透测试工具与应用实践指南》--水印.pdf` 第 10 页（乱码文本层型，页内无嵌入图像）。

提取侧结构性发现：该 PDF 仅第 1 页是整页扫描图（FlateDecode 1241x1754 RGB），乱码页无图像可抽，**必须走渲染不能走抽图**；lopdf `get_page_images` 可承担真扫描件抽图路径。[实证: 2026-09-02 poc-ocr 探针输出]

| 步骤 | 配置 | 结果 |
| --- | --- | --- |
| hayro 渲染 | scale 2 出 1190x1683 PNG | 588ms/页，白底后清晰 [实证] |
| pure-onnx-ocr 加载模型 | det 加 rec 加 dict | 29ms [实证] |
| OCR | det_limit 960（默认） | 30 行 18.9s；长行空白（见踩坑 2） [实证] |
| OCR | det_limit 960 加 vendor 改 max_width 2560 | 30 行 18.9s，正文 0.87 到 0.99 [实证] |
| OCR | det_limit 1600 加 max_width 2560 | 31 行 42.3s，正文 0.97 到 0.996 但粗体标题更差 [实证] |

质量摘录（1600 配置）：正文「传统渗透测试全流程依赖安全专家工完成…」错 1 字（人工识别为人）；系统性掉字例「采用识别为采」「显示识别为显」「2 至 4 识别为 24」「高达识别为达」；水印覆盖行如「失，企」conf 0.579。[实证: 2026-09-02 PoC 输出原文]

模型来源与校验：ModelScope `RapidAI/RapidOCR` 仓（det `4d97c44a…`、rec `5825fc7e…`、dict `d1979e9f…`，均 sha256sum -c 过）。PP-OCR 模型随 PaddleOCR 仓 Apache-2.0 发布。[实证: 下载校验过；推断: 许可随仓]

## 踩坑沉淀

| 现象 | 根因 | 正确处理 |
| --- | --- | --- |
| tract 分析阶段 panic：`Impossible to unify Sym(DynamicDimension.0) with Val(1)`（Conv.0） | RapidOCR 及第三方转换的 ONNX 自带静态 value_info 形状元数据，与 tract 符号维度推断冲突 | 剥离 `graph.value_info` 并把输出 shape 清为动态（PoC `strip_value_info.py`，`uv run --script` 跑）；两份模型源（RapidOCR 官方与第三方 Liyulingyue 转换）同病 |
| 正文长行识别为空白且 conf=1.000 | pure-onnx-ocr 0.1.0 识别预处理硬编码 `max_width: 320`（preprocessing.rs RecPreProcessorConfig 默认值），长行压扁 6 倍以上致 CTC 全 blank | vendor 后一行改 2560 即恢复；上游可提 issue 或 PR |
| hayro 渲染出黑底 | RenderSettings 默认 `bg_color: TRANSPARENT` | 显式设 WHITE（hayro 自家 example 亦如此） |
| crates.io `ppocr-rs`（0.7.3）与 GitHub `weidix/ppocr-rs` 同名不同物 | 名称撞车（前者为 dariofinardi/PaddleOCR-OCR-rs，走 ort） | 选型以 repository 字段为准，不认名 |

## 待办

1. 若立项落地：`extract` 对 needs_ocr 页加 `--ocr` 兜底管线（hayro 渲染加 tract 推理）；模型首用下载进缓存目录（SHA-256 钉死，源 RapidOCR ModelScope 仓）；pure-onnx-ocr 以 vendor 或 fork 方式修 max_width 并观察上游。
2. 质量升级备查：PP-OCRv5 server 模型（体积更大，超出「小型」口径）或 weidix/ppocr-rs PP-OCRv6 原生内核对比实测。[假设: server 或 v6 模型掉字率显著低于 v5 mobile]
3. 若只做 Windows 加 macOS 双平台，平台原生 OCR API 可零模型体积，Linux 无等价物需另行兜底。[推断: 未实测]
