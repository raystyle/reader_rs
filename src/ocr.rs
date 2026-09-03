//! OCR 兜底管线（P0014 落地、P0018 换引擎）：needs_ocr 页经 hayro 渲染为位图，
//! ppocr-rs 原生 CPU 内核跑 PP-OCRv6（S008 裁决 tiny 质量与速度双优；0.8 秒/页量级、
//! S006 掉字点全修）。模型由 ppocr ModelStore 管理（HuggingFace 钉 rev 加 sha256、
//! 缓存目录、offline 语义与 P0014 一致）。档位默认 tiny，`READER_OCR_MODEL_SIZE=small`
//! 切 small（D29 A/B 对比与 D25 质量档评估用，未定 CLI 面）。

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
    let size = model_size()?;
    // 线程自适应（用户裁定核数自适应，P0017）：ppocr CPU 内核 rayon 并行，全核数交给引擎。
    let options = OcrOptions {
        threads: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1),
        detector_size: size,
        recognizer_size: size,
        ..OcrOptions::default()
    };
    let engine = if offline {
        let paths = store
            .resolve_pair(size, size, ModelAccess::Offline)
            .map_err(|e| {
                format!(
                    "OCR 模型未就位且 --offline 禁下载；去掉 --offline 让 reader 下载 PP-OCRv6 {} 进 {}: {e}",
                    size.as_str(),
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
            .paths(ppocr_rs::ModelKind::Detector, size)
            .is_ok_and(|p| !p.weights.is_file())
        {
            eprintln!(
                "reader: 首用下载 OCR 模型（PP-OCRv6 {}）进 {} …",
                size.as_str(),
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
        eprintln!(
            "reader: OCR 兜底第 {no} 页（PP-OCRv6 {}，多核并行）…",
            size.as_str()
        );
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

/// 模型档位：`READER_OCR_MODEL_SIZE` 环境变量（tiny / small），默认 tiny（D29 A/B 用）。
fn model_size() -> Result<ModelSize, String> {
    match std::env::var("READER_OCR_MODEL_SIZE") {
        Ok(v) => parse_model_size(&v),
        Err(_) => Ok(ModelSize::Tiny),
    }
}

fn parse_model_size(v: &str) -> Result<ModelSize, String> {
    match v.trim().to_ascii_lowercase().as_str() {
        "tiny" => Ok(ModelSize::Tiny),
        "small" => Ok(ModelSize::Small),
        other => Err(format!(
            "READER_OCR_MODEL_SIZE 只认 tiny / small，收到 `{other}`"
        )),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_model_size_accepts_tiny_and_small() {
        assert!(matches!(parse_model_size("tiny"), Ok(ModelSize::Tiny)));
        assert!(matches!(parse_model_size(" SMALL "), Ok(ModelSize::Small)));
    }

    #[test]
    fn dies_parse_model_size_rejects_unknown() {
        let err = parse_model_size("medium").unwrap_err();
        assert!(err.contains("tiny / small"), "错误应提示合法档位: {err}");
    }
}
