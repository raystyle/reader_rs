# S009-图片文件支持-image直依赖零新增与OCR管线复用

> 2026-09-04。触发点：用户点名「研究下支持图片文件的分析」（PRD D43 第 1 轮）。流程按 R002；六态标准见 G002。无 PoC 目录：核心事实全部来自依赖源码与仓内实测（本机 `target/ocr-text.png` 端到端，2026-09-04）。

## 背景

OCR 管线（P0014 / P0018 / D42）只吃 PDF：hayro 渲染 needs_ocr 页为位图，ppocr-rs recognize。位图文件（png / jpg 等）没有文本层，天然是 OCR 的输入，但 extract / search 面按扩展名分派不认图片。本研究裁定接入形态。

## 关键结论

1. **零新依赖可落地**：`image` 0.25.10 已是直依赖（hayro 位图到引擎的桥，Cargo.toml），默认 features 已编入 15 种解码器（png / jpeg / bmp / gif / webp / tiff / ico / avif / hdr / pnm / qoi / tga / dds / exr / ff），图片文件比 PDF 更近：decode 到 rgb 直接 `recognize`，hayro 渲染整段省掉。[实证: 2026-09-04 读 image-0.25.10 Cargo.toml features 表与仓 Cargo.toml]
2. **ppocr-rs 直吃位图**：`OcrEngine::recognize(&RgbImage)` 即入口，`RgbImage::new(w, h, Vec<u8>)` 纯数据构造（返回 Result，尺寸零拒绝），仓内 ocr.rs 已在用同款。[实证: ppocr-rs rev d07857c src/pixels.rs]
3. **两个必带坑有现成解**：透明底 alpha 合成白底（直接丢 alpha 染黑，S006 hayro 同款；自写 `(c*a+255*(255-a))/255` 三行）；EXIF 方向（`ImageDecoder::orientation()` 加 `DynamicImage::apply_orientation`，jpeg / tiff / webp 实现，其余格式默认 NoTransforms）。[实证: image-0.25.10 src/metadata.rs 与 codecs 覆盖表]
4. **解压炸弹有内建防线**：`ImageReader` 解码走默认 `Limits { max_alloc: 512MB }`，恶意大图按 Limits 错误拒，无需自设。[实证: image-0.25.10 src/io/limits.rs:49]
5. **四裁（用户第 1 轮，2026-09-04）**：格式集收常用八种（png / jpg / jpeg / bmp / gif / webp / tiff / tif，avif 解码链路重与冷门专业格式不承诺）；默认行为 `--ocr` opt-in 同 PDF 契约（无文本层恒标 `[needs_ocr: image]`，OCR 后标记保留）；多帧动图只取首帧（YAGNI）；单图即 page 1（`--pages` 过滤只认 1）。[实证: 本轮追问链裁定]
6. **query 面拒图片**：无文本层无 markdown 可转，专属错误指路 `extract --ocr` / `search --ocr`（不落通用「不支持的格式」误导）。[实证: 2026-09-04 裁定；落地 query.rs]

## 现状或实测

### 端到端（本机，2026-09-04）

- GDI+ 现造 480x140 文字图 `READER SMOKE 12345`：`reader extract target/ocr-text.png --ocr` 完整识别出该行，`[needs_ocr: image]` 标记保留，退出 0。
- 无 `--ocr`：`== page 1 ==` 加 `[needs_ocr: image]` 提示；`search` 无命中退出 1 加 stderr 提示；`--pages 2` 空结果；批量目录含图片进扫描面不报错、目录 `--ocr` 仍按契约拒绝。

### 实现面

`ocr.rs` 抽 `build_engine`（PDF 页与图片共用引擎构建，行为零变化）；新增 `ocr_image`（ImageReader 内容嗅探到 EXIF 到白底到 recognize）与 `blend_alpha_on_white`（纯函数单测覆盖不透明恒等 / 全透明纯白 / 半透明折半）。`document.rs` 八扩展名分派加 `is_supported` 真源；`query.rs` 专属错误。

## 待办

1. 冷门格式（avif / ico / tga / pnm / qoi / dds / exr / hdr / ff）有真实需求再议（解码器已在二进制里，差分派表与质量承诺）。
2. 多帧逐帧成页（多页 TIFF 扫描册场景）留候选，有需求再立项。
3. 大图缩放（det 模型输入上限）暂不做：默认 Limits 512MB 内存防线够用，超大图走 A/B 层实证后再裁。
