# S008-OCR质量升级-ppocr-rs的PP-OCRv6原生内核双优胜出现管线换引擎

> 2026-09-03。触发点：用户点名「关键是 OCR 质量」（mobile 掉字问题，S006 待办 2）。流程按 R002；六态标准见 G002。PoC：server 模型走 `target\poc-ocr`（现有管线换件），v6 走 `target\poc-v6`（clone weidix/ppocr-rs，rev d07857c）。

## 背景

现管线（P0014/P0017）：hayro 渲染加 vendored pure-onnx-ocr 跑 PP-OCRv5 mobile，系统性掉字（S006 关键结论 4）。S006 备查路线两条：server 模型（更大）与 weidix/ppocr-rs（PP-OCRv6 safetensors 原生内核）。本研究给两路线实测裁决。

## 关键结论

1. **PP-OCRv6 tiny（ppocr-rs 原生 CPU 内核）双优胜出**：0.8 秒/页（12 线程）且 S006 全部已知掉字点修复（「人工完成」「采用」「显示」「高达 287 天」「工作」），37 行召回高于 mobile 的 30。比 mobile 快 4 到 6 倍且质量更好。[实证: 2026-09-03 poc-v6 真样本第 10 页热跑]
2. **PP-OCRv5 server 出局**：质量好（34 行、正文全对）但 125.6 秒/页（tract CPU，det 20s 加 rec 合计 358s），比 mobile 慢 25 倍以上，不值。[实证: 2026-09-03 poc-ocr 换 server 件实测]
3. **v6 small 档可选**：3.2 秒/页（热跑）、51 行召回、水印区更干净（「能力无法沉淀」完整）；默认仍取 tiny（速度优先），small 留作质量档候选。[实证: 同上 small 档实测]
4. **ppocr-rs 工程面合格**：纯 Rust CPU 路径（anyhow/image/rayon/safetensors，无 C）、MIT 或 Apache-2.0、lib API 完整（ModelStore.ensure/ensure_offline 钉哈希缓存、OcrEngine.load_from_store、recognize 出阅读序行）；模型 tiny 套件仅 6.2MB（HuggingFace 钉 rev 加 sha256）。默认 GPU（wgpu）为可选 feature 不进默认面。[实证: 2026-09-03 读仓 Cargo.toml 与 src]
5. **风险认账**：ppocr-rs 是 0.1.0 社区新项目（S006 已标注未实测，本研究补上实证）；未上 crates.io（ppocr-rs 名被它物占用），以 git rev 钉死引入；上游若断更，vendor fork 兜底成本低（无补丁需求）。[实证: gh 仓查；推断: 维护连续性]

## 现状或实测

### 双通道核实

| 候选 | crates.io | GitHub | 裁决 |
| --- | --- | --- | --- |
| weidix/ppocr-rs | 未发布（ppocr-rs 名撞它物） | 0.1.0，MIT 或 Apache-2.0，2026-09 活跃 | 选中：git rev 钉 d07857c 引入 |
| PP-OCRv5 server（ONNX） | ModelScope RapidOCR 仓 det 84MB 加 rec 80MB（sha256 已验 `0f8846b1…`/`e0938540…`） | 同上仓 | 出局：tract CPU 125.6s/页 |

[实证: 2026-09-03 ModelScope API 加 gh api]

### 同页四配置对比

样本：安全牛 PDF 第 10 页渲染件（1190x1683），本机 32 核；v6 用 12 线程。

| 配置 | wall | 行数 | 已知掉字点 |
| --- | --- | --- | --- |
| v5 mobile（现管线，batch 2 并行） | 约 3.0s | 30 | 全中（「人工」误识为「人」等） |
| v5 server（现管线换件） | 125.6s | 34 | 全修复 |
| v6 tiny（ppocr-rs） | 0.8s | 37 | 全修复，水印区少量噪声（「y」） |
| v6 small（ppocr-rs） | 3.2s | 51 | 全修复且更干净 |

[实证: 2026-09-03 上述 PoC 输出原文]

## 踩坑沉淀

| 现象 | 根因 | 正确处理 |
| --- | --- | --- |
| 并行 curl 下载两件模型只落了一件 | Git Bash 里 `cd X && curl a & curl b & wait` 的第二条 curl 在仓根执行 | 并行下载别混 cd；或逐条串行 |
| server 模型 tract 直接 load 必炸 | 同 mobile：静态 value_info 冲突（det 596 条、rec 512 条） | 同 S006 踩坑 1，strip 后可跑（strip 件哈希 `77a62413…`/`3f37a37c…`） |

## 待办

1. 转 P0018 落地：引擎换 ppocr-rs（git rev 钉死），hayro 渲染保留；pure-onnx-ocr 加 tract-onnx 加 prost 出树、vendor 目录退役；模型管理换 ModelStore（缓存目录与 --offline 语义保留）。
2. v6 small 质量档：`--ocr-quality` 之类旗标留候选，有真实需求再提。
3. 若 ppocr-rs 上游发布 crates.io 正式版，转 registry 依赖。
