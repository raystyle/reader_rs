# reader_rs::ocr

OCR 兜底管线（P0014 落地、P0018 换引擎、D42 镜像源链、D43 图片文件）：needs_ocr 页经
hayro 渲染为位图，图片文件直接解码（image crate，首帧语义），ppocr-rs 原生 CPU 内核跑
PP-OCRv6（S008 裁决 tiny 质量与速度双优；0.8 秒/页量级、S006 掉字点全修）。模型由
ppocr ModelStore 管理（缓存目录与 offline 语义与 P0014 一致）；
D42 后首用下载走三级回退（镜像 到 HF 直连 到 GitHub Releases 模型 tag，`mirror` 模块），
ppocr-rs 内嵌钉死值全量 sha256 校验是终检闸。档位 tiny / small：env
`READER_OCR_MODEL_SIZE`（A/B 跑批器用，最高）> `ocr switch` 设置文件 > 默认 tiny；
`ocr init` / `ocr doctor` / `ocr switch` 三子命令实现在本模块（D42 用户点名）。

## Functions

- `doctor_models` — `ocr doctor`：只读诊断（不建目录、不写文件、不下载）。
- `init_models` — `ocr init`：显式下载 / 修复档位双包进缓存。
- `ocr_image` — 对图片文件整图 OCR,返回行级文本(阅读序,空行滤除;D43)。
- `ocr_pages` — 对指定页做 OCR 兜底,返回页号与行级文本(阅读序,空行滤除)。
- `switch_model` — `ocr switch <tiny|small>`：写档位设置文件并提示，只切换不自动下载（单调用完成

## Types

- `OcrOutcome` — ocr 三子命令的输出结果：`lines` 是 stdout 稳定行（ASCII token 前置，lib.rs 逐行打出），

