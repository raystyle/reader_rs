//! OCR 兜底管线（P0014 落地、P0018 换引擎、D42 镜像源链、D43 图片文件）：needs_ocr 页经
//! hayro 渲染为位图，图片文件直接解码（image crate，首帧语义），ppocr-rs 原生 CPU 内核跑
//! PP-OCRv6（S008 裁决 tiny 质量与速度双优；0.8 秒/页量级、S006 掉字点全修）。模型由
//! ppocr ModelStore 管理（缓存目录与 offline 语义与 P0014 一致）；
//! D42 后首用下载走三级回退（镜像 到 HF 直连 到 GitHub Releases 模型 tag，`mirror` 模块），
//! ppocr-rs 内嵌钉死值全量 sha256 校验是终检闸。档位 tiny / small：env
//! `READER_OCR_MODEL_SIZE`（A/B 跑批器用，最高）> `ocr switch` 设置文件 > 默认 tiny；
//! `ocr init` / `ocr doctor` / `ocr switch` 三子命令实现在本模块（D42 用户点名）。

use crate::mirror::{self, FileState};
use hayro::hayro_syntax::Pdf;
use hayro::{render, RenderCache, RenderSettings};
use ppocr_rs::{ModelAccess, ModelKind, ModelSize, ModelStore, OcrEngine, OcrOptions, RgbImage};
use std::path::{Path, PathBuf};

/// 对 `page_nos`（1 起）做 OCR 兜底，返回页号与行级文本（阅读序，空行滤除）。
/// 模型缺失且 `offline` 为真时报错不下载。
pub fn ocr_pages(
    path: &Path,
    page_nos: &[u32],
    offline: bool,
) -> Result<Vec<(u32, Vec<String>)>, String> {
    let engine = build_engine(offline)?;
    let size = model_size()?;

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

/// OCR 引擎构建（PDF 页与图片文件共用，D43 抽出）。offline 走 Offline 解析（模型未就位
/// 即报错并提示去掉 --offline）；在线先零网络 Offline 探测（标记有效走快路径，标记缺走
/// 全量校验补标记），未就位才三级回退预取（镜像 到 HF 到 GitHub，D42），终检仍交回
/// Offline 解析；链败回退 ppocr-rs 原生 HF 下载（自带 3 次重试，兜 pin 表漂移）。
fn build_engine(offline: bool) -> Result<OcrEngine, String> {
    let dir = cache_dir()?;
    let store = ModelStore::new(&dir);
    let size = model_size()?;
    let options = ocr_options(size);
    if offline {
        let paths = store
            .resolve_pair(size, size, ModelAccess::Offline)
            .map_err(|e| {
                format!(
                    "OCR 模型未就位且 --offline 禁下载；去掉 --offline 让 reader 下载 PP-OCRv6 {} 进 {}: {e}",
                    size.as_str(),
                    dir.display()
                )
            })?;
        return OcrEngine::load(
            &paths.detector.weights,
            &paths.recognizer.weights,
            &paths.recognizer.inference,
            options,
        )
        .map_err(|e| format!("OCR 引擎构建失败: {e}"));
    }
    match store.resolve_pair(size, size, ModelAccess::Offline) {
        Ok(paths) => OcrEngine::load(
            &paths.detector.weights,
            &paths.recognizer.weights,
            &paths.recognizer.inference,
            options,
        ),
        Err(_) => {
            eprintln!(
                "reader: 下载 OCR 模型（PP-OCRv6 {}，镜像优先）进 {} …",
                size.as_str(),
                dir.display()
            );
            let prefetched = prefetch_pair(&dir, size);
            let resolved = if prefetched {
                store.resolve_pair(size, size, ModelAccess::Offline).ok()
            } else {
                None
            };
            match resolved {
                Some(paths) => OcrEngine::load(
                    &paths.detector.weights,
                    &paths.recognizer.weights,
                    &paths.recognizer.inference,
                    options,
                ),
                None => {
                    eprintln!("reader: 镜像链未凑齐模型，回退 HuggingFace 直连（ppocr-rs 原生）");
                    OcrEngine::load_from_store(&store, options)
                }
            }
        }
    }
    .map_err(|e| format!("OCR 引擎构建失败: {e}"))
}

/// 引擎参数：线程自适应（用户裁定核数自适应，P0017），档位两侧同参。
fn ocr_options(size: ModelSize) -> OcrOptions {
    OcrOptions {
        threads: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1),
        detector_size: size,
        recognizer_size: size,
        ..OcrOptions::default()
    }
}

/// 图片文件 OCR（D43）：ImageReader 内容嗅探解码（多帧动图只取首帧，用户裁定 YAGNI；
/// 默认 Limits 512MB 防解压炸弹）、EXIF 方向应用（jpeg / tiff / webp 携带时）、alpha
/// 合成白底后 recognize，返回行级文本（阅读序，空行滤除）。
pub fn ocr_image(path: &Path, offline: bool) -> Result<Vec<String>, String> {
    use image::ImageDecoder;
    let reader = image::ImageReader::open(path)
        .map_err(|e| format!("无法读取图片 {}: {e}", path.display()))?
        .with_guessed_format()
        .map_err(|e| format!("无法嗅探图片格式 {}: {e}", path.display()))?;
    let mut decoder = reader
        .into_decoder()
        .map_err(|e| format!("无法解码图片 {}: {e}", path.display()))?;
    let orientation = decoder
        .orientation()
        .unwrap_or(image::metadata::Orientation::NoTransforms);
    let mut img = image::DynamicImage::from_decoder(decoder)
        .map_err(|e| format!("无法解码图片 {}: {e}", path.display()))?;
    img.apply_orientation(orientation);
    let rgb = blend_alpha_on_white(&img)?;
    let engine = build_engine(offline)?;
    let result = engine
        .recognize(&rgb)
        .map_err(|e| format!("图片 OCR 失败: {e}"))?;
    Ok(result
        .lines
        .into_iter()
        .map(|l| l.text)
        .filter(|t| !t.trim().is_empty())
        .collect())
}

/// alpha 合成白底（out = (c * a + 255 * (255 - a)) / 255）：透明像素直接丢 alpha 会按
/// 0 处理染黑（hayro 透明底同款坑 S006）；不透明像素 a=255 恒等，无 alpha 格式走同路径。
fn blend_alpha_on_white(img: &image::DynamicImage) -> Result<RgbImage, String> {
    use image::GenericImageView;
    let (w, h) = img.dimensions();
    let rgba = img.to_rgba8();
    let mut raw = Vec::with_capacity(w as usize * h as usize * 3);
    for pixel in rgba.pixels() {
        let a = pixel.0[3] as u32;
        for &c in &pixel.0[..3] {
            raw.push(((c as u32 * a + 255 * (255 - a)) / 255) as u8);
        }
    }
    RgbImage::new(w, h, raw).map_err(|e| format!("位图尺寸非法: {e}"))
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

/// 档位设置文件：缓存目录的**兄弟位** `model-size`（缺省 `%LOCALAPPDATA%\reader\model-size`）。
/// 锚在兄弟位是两个不变量：`READER_OCR_CACHE_DIR` 指临时目录的测试，设置文件也随临时
/// 目录（不泄开发者真机档位，测试 hermetic）；`models\` 缓存可随时整删而档位偏好不丢。
fn settings_path() -> Result<PathBuf, String> {
    Ok(settings_path_for(&cache_dir()?))
}

/// `settings_path` 的纯函数核(单测用):缓存目录的兄弟位文件名。
fn settings_path_for(cache: &Path) -> PathBuf {
    cache.with_file_name("model-size")
}

/// 档位解析的三级合成（纯函数，单测用）：env（A/B 跑批器通道，最高）> 设置文件 > Tiny；
/// 设置文件内容非法时回退 Tiny（读侧 caller 负责提示）。
fn resolve_size(
    env: Option<&str>,
    file: Option<&str>,
) -> Result<(ModelSize, &'static str), String> {
    if let Some(v) = env {
        return parse_model_size(v).map(|s| (s, "env"));
    }
    match file.map(str::trim).filter(|t| !t.is_empty()) {
        Some(t) => match parse_model_size(t) {
            Ok(s) => Ok((s, "设置")),
            Err(_) => Ok((ModelSize::Tiny, "默认")),
        },
        None => Ok((ModelSize::Tiny, "默认")),
    }
}

/// 模型档位（带来源标签，doctor 报告用）。
fn model_size_with_source() -> Result<(ModelSize, &'static str), String> {
    let env = std::env::var("READER_OCR_MODEL_SIZE").ok();
    let file = std::fs::read_to_string(settings_path()?)
        .ok()
        .filter(|_| env.is_none());
    if env.is_none() {
        if let Some(text) = &file {
            if parse_model_size(text.trim()).is_err() {
                eprintln!(
                    "reader: 档位设置文件 {} 内容非法，本次按默认 tiny",
                    settings_path()?.display()
                );
            }
        }
    }
    resolve_size(env.as_deref(), file.as_deref())
}

/// 模型档位。
fn model_size() -> Result<ModelSize, String> {
    model_size_with_source().map(|(s, _)| s)
}

/// 设置文件状态（doctor 报告行）。
fn settings_state() -> Result<(&'static str, PathBuf), String> {
    let path = settings_path()?;
    let state = match std::fs::read_to_string(&path) {
        Ok(text) => match parse_model_size(text.trim()) {
            Ok(ModelSize::Tiny) => "tiny",
            Ok(ModelSize::Small) => "small",
            Ok(ModelSize::Medium) => "invalid",
            Err(_) => "invalid",
        },
        Err(_) => "absent",
    };
    Ok((state, path))
}

/// 三级回退预取双包（镜像 到 HF 到 GitHub）：逐件只补无效件（有效跳过），
/// 全部就位返回真。下载细节、目录自建与校验在 `mirror` 模块（M017）。
fn prefetch_pair(dir: &Path, size: ModelSize) -> bool {
    let mut all_ok = true;
    for kind in [ModelKind::Detector, ModelKind::Recognizer] {
        let Ok(pin) = mirror::package_pin(size, kind) else {
            return false;
        };
        let pkg_dir = mirror::package_dir(dir, pin);
        for file in pin.files {
            if matches!(mirror::assess_file(&pkg_dir, file), FileState::Ok) {
                continue;
            }
            if mirror::download_file(pin, file, &pkg_dir.join(file.name)).is_err() {
                all_ok = false;
            }
        }
    }
    all_ok
}

/// ocr 三子命令的输出结果：`lines` 是 stdout 稳定行（ASCII token 前置，lib.rs 逐行打出），
/// `healthy` 决定退出码（doctor：当前档双包完整；init：全包就位；switch：恒真）。
pub struct OcrOutcome {
    pub lines: Vec<String>,
    pub healthy: bool,
}

/// `ocr init`：显式下载 / 修复档位双包进缓存。逐件有效跳过、缺损重下（三级回退），
/// 末尾交 ppocr-rs `verify()` 全量校验并补缓存标记（表漂移当场红）。
/// `--offline` 只校验不下载（语义对齐 `--ocr --offline`，零网络可测）。
pub fn init_models(size_arg: Option<ModelSize>, offline: bool) -> Result<OcrOutcome, String> {
    let dir = cache_dir()?;
    let store = ModelStore::new(&dir);
    let size = match size_arg {
        Some(s) => s,
        None => model_size()?,
    };
    let mut lines = vec![format!("ocr_init: size {}", size.as_str())];
    let mut ok = true;
    for kind in [ModelKind::Detector, ModelKind::Recognizer] {
        let pin = mirror::package_pin(size, kind)?;
        let pkg_dir = mirror::package_dir(&dir, pin);
        let mut pkg_ok = true;
        for file in pin.files {
            if matches!(mirror::assess_file(&pkg_dir, file), FileState::Ok) {
                lines.push(format!("ocr_init: {} {} ok", pin.name, file.name));
                continue;
            }
            if offline {
                lines.push(format!(
                    "ocr_init: {} {} failed（--offline 禁下载且缓存件无效）",
                    pin.name, file.name
                ));
                pkg_ok = false;
                continue;
            }
            match mirror::download_file(pin, file, &pkg_dir.join(file.name)) {
                Ok(source) => lines.push(format!(
                    "ocr_init: {} {} download {}",
                    pin.name,
                    file.name,
                    source.as_str()
                )),
                Err(e) => {
                    lines.push(format!(
                        "ocr_init: {} {} failed（{e}）",
                        pin.name, file.name
                    ));
                    pkg_ok = false;
                }
            }
        }
        if pkg_ok {
            // 终检闸：ppocr-rs 按内嵌钉死值全量校验并补 .ppocr-rs.complete
            if let Err(e) = store.verify(kind, size) {
                lines.push(format!("ocr_init: {} failed（终检: {e}）", pin.name));
                ok = false;
            } else {
                lines.push(format!("ocr_init: {} complete", pin.name));
            }
        } else {
            lines.push(format!("ocr_init: {} incomplete", pin.name));
            ok = false;
        }
    }
    lines.push(format!(
        "ocr_init: verdict {}",
        if ok { "ok" } else { "failed" }
    ));
    Ok(OcrOutcome { lines, healthy: ok })
}

/// `ocr doctor`：只读诊断（不建目录、不写文件、不下载）。两档四包逐包判定 +
/// 设置文件与档位来源 + 镜像探活（信息行，不可达不影响判定：内网机离线可用即健康）。
/// healthy = 当前档双包完整。
pub fn doctor_models() -> Result<OcrOutcome, String> {
    let dir = cache_dir()?;
    let (size, source) = model_size_with_source()?;
    let (settings, settings_file) = settings_state()?;
    let mut lines = vec![
        format!("ocr_doctor: cache {}", dir.display()),
        format!(
            "ocr_doctor: settings {} {}",
            settings_file.display(),
            settings
        ),
        format!("ocr_doctor: size {}（{}）", size.as_str(), source),
    ];
    let current_prefix = format!("{}-", size.as_str());
    let mut healthy = true;
    for pin in mirror::PACKAGES {
        let line = match mirror::assess_package(&dir, pin) {
            mirror::PackageVerdict::Ok => format!("ocr_doctor: {} ok", pin.name),
            mirror::PackageVerdict::Missing(f) => {
                format!("ocr_doctor: {} missing（缺 {f}）", pin.name)
            }
            mirror::PackageVerdict::Corrupt(f) => {
                format!("ocr_doctor: {} corrupt（{f} 校验不符）", pin.name)
            }
        };
        if pin.name.starts_with(&current_prefix) && !matches!(line.rsplit(' ').next(), Some("ok")) {
            healthy = false;
        }
        lines.push(line);
    }
    // 镜像探活：顺手报 latest.json 广告的版本；仅信息，不改 healthy
    match mirror::fetch_latest_manifest() {
        Ok(m) => lines.push(format!(
            "ocr_doctor: mirror ok {} latest {}",
            mirror::latest_json_url(),
            m.version
        )),
        Err(e) => lines.push(format!("ocr_doctor: mirror unreachable（{e}）")),
    }
    lines.push(format!(
        "ocr_doctor: verdict {}",
        if healthy { "ok" } else { "failed" }
    ));
    Ok(OcrOutcome { lines, healthy })
}

/// `ocr switch <tiny|small>`：写档位设置文件并提示。只切换不自动下载（单调用完成
/// 一件事）；env `READER_OCR_MODEL_SIZE` 已导出时警告本设置不生效（env 优先）。
pub fn switch_model(target: ModelSize) -> Result<OcrOutcome, String> {
    let (current, _) = model_size_with_source()?;
    let path = settings_path()?;
    if std::env::var_os("READER_OCR_MODEL_SIZE").is_some() {
        eprintln!(
            "reader: READER_OCR_MODEL_SIZE 已导出，环境变量优先，本次 switch 对当前会话不生效"
        );
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("建设置目录失败: {e}"))?;
    }
    std::fs::write(&path, format!("{}\n", target.as_str()))
        .map_err(|e| format!("写档位设置失败 {}: {e}", path.display()))?;
    let mut lines = vec![format!(
        "ocr_switch: {} -> {}（写入 {}）",
        current.as_str(),
        target.as_str(),
        path.display()
    )];
    // 目标档未就位则提示 init（不改退出码）
    let dir = cache_dir()?;
    let ready = [ModelKind::Detector, ModelKind::Recognizer]
        .iter()
        .all(|&kind| {
            mirror::package_pin(target, kind)
                .map(|pin| {
                    matches!(
                        mirror::assess_package(&dir, pin),
                        mirror::PackageVerdict::Ok
                    )
                })
                .unwrap_or(false)
        });
    if !ready {
        lines.push(format!(
            "ocr_switch: {} 未就位，先跑 reader ocr init 下载",
            target.as_str()
        ));
    }
    Ok(OcrOutcome {
        lines,
        healthy: true,
    })
}

pub(crate) fn parse_model_size(v: &str) -> Result<ModelSize, String> {
    match v.trim().to_ascii_lowercase().as_str() {
        "tiny" => Ok(ModelSize::Tiny),
        "small" => Ok(ModelSize::Small),
        other => Err(format!("模型档位只认 tiny / small，收到 `{other}`")),
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

    /// D42 档位三级:env(A/B 跑批器通道)> 设置文件 > Tiny;设置非法回退默认;env 非法报错。
    #[test]
    fn resolve_size_env_over_settings_over_default() {
        assert_eq!(
            resolve_size(Some("small"), Some("tiny")).unwrap(),
            (ModelSize::Small, "env")
        );
        assert_eq!(
            resolve_size(Some("  Tiny "), None).unwrap(),
            (ModelSize::Tiny, "env")
        );
        assert_eq!(
            resolve_size(None, Some("small\n")).unwrap(),
            (ModelSize::Small, "设置")
        );
        assert_eq!(
            resolve_size(None, Some(" 垃圾 ")).unwrap(),
            (ModelSize::Tiny, "默认")
        );
        assert_eq!(resolve_size(None, None).unwrap(), (ModelSize::Tiny, "默认"));
        assert!(resolve_size(Some("medium"), None).is_err());
    }

    /// 设置文件恒在缓存目录兄弟位(hermetic 不变量:缓存指到哪,设置跟到哪的上级)。
    /// 斜杠两平台都是合法分隔符,反斜杠字面量在 unix 不是分隔符会假失败(首踩 mac/linux 实机)。
    #[test]
    fn settings_path_is_sibling_of_cache_dir() {
        assert_eq!(
            settings_path_for(Path::new("/tmp/reader-test-1/models")),
            PathBuf::from("/tmp/reader-test-1/model-size")
        );
        #[cfg(windows)]
        assert_eq!(
            settings_path_for(Path::new(r"C:\u\ray\AppData\Local\reader\models")),
            PathBuf::from(r"C:\u\ray\AppData\Local\reader\model-size")
        );
    }

    /// alpha 合成白底(D43):不透明恒等、全透明纯白、半透明按 (c*a+255*(255-a))/255;
    /// 无 alpha 格式(如 jpg)to_rgba8 补 a=255 走同路径结果不变。
    #[test]
    fn blend_alpha_composites_transparent_onto_white() {
        use image::{DynamicImage, Rgba, RgbaImage};
        let mut rgba = RgbaImage::new(3, 1);
        rgba.put_pixel(0, 0, Rgba([10, 20, 30, 255])); // 不透明:恒等
        rgba.put_pixel(1, 0, Rgba([10, 20, 30, 0])); // 全透明:纯白
        rgba.put_pixel(2, 0, Rgba([0, 0, 0, 128])); // 半透明黑:(0*128+255*127)/255=127
        let blended = blend_alpha_on_white(&DynamicImage::ImageRgba8(rgba)).expect("合成");
        assert_eq!(blended.pixels().len(), 9, "3x1x3 字节");
        let px = |i: u32| blended.pixel(i, 0);
        assert_eq!(px(0), [10, 20, 30], "不透明像素应恒等");
        assert_eq!(px(1), [255, 255, 255], "全透明像素应落白底(S006 染黑坑)");
        assert_eq!(px(2), [127, 127, 127], "半透明黑对白底应折半");
    }
}
