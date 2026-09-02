//! OCR 兜底管线（P0014 落地、P0018 换引擎）：needs_ocr 页经 hayro 渲染为位图，
//! ppocr-rs 原生 CPU 内核跑 PP-OCRv6 tiny（S008 裁决：质量与速度双优；0.8 秒/页量级、
//! S006 掉字点全修）。模型由 ppocr ModelStore 管理（HuggingFace 钉 rev 加 sha256、
//! 缓存目录、offline 语义与 P0014 一致）。

use hayro::hayro_syntax::Pdf;
use hayro::{render, RenderCache, RenderSettings};
use ppocr_rs::{ModelAccess, ModelSize, ModelStore, OcrEngine, OcrOptions, RgbImage};
use std::path::{Path, PathBuf};

/// 对 `page_nos`（1 起）做 OCR 兜底，返回页号与行级文本（阅读序，空行滤除）。
/// 模型缺失且 `offline` 为真时报错不下载。
pub fn ocr_pages(
    path: &Path,
    page_nos: &[u32],
    offline: bool,
) -> Result<Vec<(u32, Vec<String>)>, String> {
    let dir = cache_dir()?;
    let store = ModelStore::new(&dir);
    // 线程自适应（用户裁定核数自适应，P0017）：ppocr CPU 内核 rayon 并行，全核数交给引擎。
    let options = OcrOptions {
        threads: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1),
        ..OcrOptions::default()
    };
    let engine = if offline {
        let paths = store
            .resolve_pair(ModelSize::Tiny, ModelSize::Tiny, ModelAccess::Offline)
            .map_err(|e| {
                format!(
                    "OCR 模型未就位且 --offline 禁下载；去掉 --offline 让 reader 下载 PP-OCRv6 tiny（约 6.2MB）进 {}: {e}",
                    dir.display()
                )
            })?;
        OcrEngine::load(
            &paths.detector.weights,
            &paths.recognizer.weights,
            &paths.recognizer.inference,
            options,
        )
    } else {
        if store
            .paths(ppocr_rs::ModelKind::Detector, ModelSize::Tiny)
            .is_ok_and(|p| !p.weights.is_file())
        {
            eprintln!(
                "reader: 首用下载 OCR 模型（PP-OCRv6 tiny 约 6.2MB）进 {} …",
                dir.display()
            );
        }
        OcrEngine::load_from_store(&store, options)
    }
    .map_err(|e| format!("OCR 引擎构建失败: {e}"))?;

    let file = std::fs::read(path).map_err(|e| format!("无法读取 PDF {}: {e}", path.display()))?;
    let pdf = Pdf::new(file).map_err(|e| format!("无法解析 PDF {}: {e:?}", path.display()))?;
    let pages = pdf.pages();
    let settings = RenderSettings {
        x_scale: 2.0,
        y_scale: 2.0,
        // hayro 默认透明底会染黑 OCR 输入（S006 踩坑 3），显式白底
        bg_color: hayro::vello_cpu::color::palette::css::WHITE,
        ..Default::default()
    };
    let mut out = Vec::new();
    for &no in page_nos {
        let Some(page) = pages.get((no - 1) as usize) else {
            continue;
        };
        eprintln!("reader: OCR 兜底第 {no} 页（PP-OCRv6 tiny，多核并行约 1-5 秒/页）…");
        let pixmap = render(page, &RenderCache::new(), &Default::default(), &settings);
        let png = pixmap
            .into_png()
            .map_err(|e| format!("页 {no} 渲染编码失败: {e:?}"))?;
        let rgb = image::load_from_memory(&png)
            .map_err(|e| format!("页 {no} 位图解码失败: {e}"))?
            .to_rgb8();
        let (w, h) = rgb.dimensions();
        let image = RgbImage::new(w, h, rgb.into_raw())
            .map_err(|e| format!("页 {no} 位图转换失败: {e}"))?;
        let result = engine
            .recognize(&image)
            .map_err(|e| format!("页 {no} OCR 失败: {e}"))?;
        let lines = result
            .lines
            .into_iter()
            .map(|l| l.text)
            .filter(|t| !t.trim().is_empty())
            .collect::<Vec<_>>();
        out.push((no, lines));
    }
    Ok(out)
}

/// 缓存目录：`READER_OCR_CACHE_DIR` 环境变量优先（测试门控用），否则平台缓存目录下 `reader\models`。
fn cache_dir() -> Result<PathBuf, String> {
    if let Some(dir) = std::env::var_os("READER_OCR_CACHE_DIR") {
        return Ok(PathBuf::from(dir));
    }
    let base = platform_cache_dir()
        .ok_or_else(|| "无法定位缓存目录（可设 READER_OCR_CACHE_DIR 指定）".to_string())?;
    Ok(base.join("reader").join("models"))
}

#[cfg(target_os = "windows")]
fn platform_cache_dir() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA")
        .or_else(|| std::env::var_os("APPDATA"))
        .map(PathBuf::from)
}

#[cfg(target_os = "macos")]
fn platform_cache_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library").join("Caches"))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn platform_cache_dir() -> Option<PathBuf> {
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
}
