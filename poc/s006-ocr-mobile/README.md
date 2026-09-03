# s006-ocr-mobile：S006 内嵌 OCR 选型 PoC

> S006（`docs\research\S006-内嵌OCR选型-纯Rust管线hayro加pure-onnx-ocr实测可行.md`）的原型产物：hayro 渲染 PDF 页为位图，pure-onnx-ocr（vendor-poon）跑 PP-OCRv5 mobile，真样本端到端验证纯 Rust 内嵌 OCR 可行。

## 复现

1. 恢复 vendor：`vendor-poon\` 为 pure-onnx-ocr 0.1 的 vendored 源码（未入仓；可从仓史 P0014 前的 `vendor\` 找回，或重新 vendor）。
2. 模型：PP-OCRv5 mobile 三件落 `models\`（未入仓，ModelScope 下载，SHA-256 见 S006）。
3. `cargo run --release -- <样本.pdf> <页码>`；`src\bin\profile.rs` 为三段计时剖析入口（P0017 用）。

## 退役去向

P0014 转正进 `src\ocr.rs`；P0018 换 ppocr-rs PP-OCRv6 tiny 后本管线退役（2026-09-03）。剖析数据见 `docs\proven\P0017-OCR性能优化-宽度分组分批加组间并行.md`。
