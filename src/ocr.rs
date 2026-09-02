//! OCR 兜底管线（P0014；选型与实证见 S006）：needs_ocr 页经 hayro 渲染为位图，
//! vendored pure-onnx-ocr（tract 跑 PP-OCRv5 mobile）出行级文本。
//! 模型三件首用从 ModelScope RapidAI/RapidOCR 下载进缓存目录，SHA-256 钉死校验；
//! 原件经进程内 strip value_info（tract 符号维度冲突规避，S006 踩坑 1）后落 stripped 件。

use hayro::hayro_syntax::Pdf;
use hayro::{render, RenderCache, RenderSettings};
use prost::Message;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const BASE: &str = "https://modelscope.cn/models/RapidAI/RapidOCR/resolve/master";

/// 模型件登记：缓存文件名、源 URL 路径、SHA-256。原件哈希钉 ModelScope 官方件；
/// stripped 件哈希钉本进程 strip_value_info 的确定性输出（prost 编码与 Python onnx
/// 序列化字节不同，故与 S006 PoC 件哈希不同；功能等价以 tract 加载加真样本 OCR 验证）。
const DET: ModelFile = ModelFile {
    name: "det-dyn.onnx",
    url_path: "/onnx/PP-OCRv5/det/ch_PP-OCRv5_det_mobile.onnx",
    source_sha: "4d97c44a20d30a81aad087d6a396b08f786c4635742afc391f6621f5c6ae78ae",
    cached_sha: "a4a307dbf6d7a18f3b021abdfecea6bf8a0b4124e707380b6f2918425fa5a30c",
    strip: true,
};
const REC: ModelFile = ModelFile {
    name: "rec-dyn.onnx",
    url_path: "/onnx/PP-OCRv5/rec/ch_PP-OCRv5_rec_mobile.onnx",
    source_sha: "5825fc7ebf84ae7a412be049820b4d86d77620f204a041697b0494669b1742c5",
    cached_sha: "0b5e82dc8cb0e28e66c541848f4392d5da73e5b0d0afb0b559dce872ee656f3c",
    strip: true,
};
const DICT: ModelFile = ModelFile {
    name: "ppocrv5_dict.txt",
    url_path: "/paddle/PP-OCRv5/rec/ch_PP-OCRv5_rec_mobile/ppocrv5_dict.txt",
    source_sha: "d1979e9f794c464c0d2e0b70a7fe14dd978e9dc644c0e71f14158cdf8342af1b",
    cached_sha: "d1979e9f794c464c0d2e0b70a7fe14dd978e9dc644c0e71f14158cdf8342af1b",
    strip: false,
};

struct ModelFile {
    name: &'static str,
    url_path: &'static str,
    source_sha: &'static str,
    cached_sha: &'static str,
    strip: bool,
}

/// 三件模型在缓存目录中的就位路径。
pub struct ModelPaths {
    det: PathBuf,
    rec: PathBuf,
    dict: PathBuf,
}

/// 对 `page_nos`（1 起）做 OCR 兜底，返回页号与行级文本。
/// 模型缺失且 `offline` 为真时报错不下载。
pub fn ocr_pages(
    path: &Path,
    page_nos: &[u32],
    offline: bool,
) -> Result<Vec<(u32, Vec<String>)>, String> {
    let models = ensure_models(offline)?;
    // OcrEngine 内含 RefCell 计划缓存、非 Send/Sync，不进静态；构建仅约 29ms（S006 实测），
    // 相对 19-42 秒/页的推理可忽略，每次调用现建。
    let engine = pure_onnx_ocr::OcrEngineBuilder::new()
        .det_model_path(&models.det)
        .rec_model_path(&models.rec)
        .dictionary_path(&models.dict)
        .build()
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
        eprintln!("reader: OCR 兜底第 {no} 页（mobile 模型约 19-42 秒/页，掉字率见 S006）…");
        let pixmap = render(page, &RenderCache::new(), &Default::default(), &settings);
        let png = pixmap
            .into_png()
            .map_err(|e| format!("页 {no} 渲染编码失败: {e:?}"))?;
        let image =
            image::load_from_memory(&png).map_err(|e| format!("页 {no} 位图解码失败: {e}"))?;
        let results = engine
            .run_from_image(&image)
            .map_err(|e| format!("页 {no} OCR 失败: {e}"))?;
        out.push((no, results.into_iter().map(|r| r.text).collect::<Vec<_>>()));
    }
    Ok(out)
}

/// 确保三件模型在缓存目录就位（哈希错或缺失时按 `offline` 决定下载或报错）。
fn ensure_models(offline: bool) -> Result<ModelPaths, String> {
    let dir = cache_dir()?;
    for file in [&DET, &REC, &DICT] {
        let path = dir.join(file.name);
        if path.metadata().is_ok_and(|m| m.is_file())
            && sha256_file(&path).is_ok_and(|sha| sha == file.cached_sha)
        {
            continue;
        }
        if offline {
            return Err(format!(
                "OCR 模型未就位（{} 缺失或校验不符）且 --offline 禁下载；去掉 --offline 让 reader 从 ModelScope 下载约 20.5MB 模型进 {}",
                file.name,
                dir.display()
            ));
        }
        eprintln!(
            "reader: 首用下载 OCR 模型 {} 进 {} …",
            file.name,
            dir.display()
        );
        let raw = fetch(&format!("{BASE}{}", file.url_path))?;
        let sha = sha256_hex(&raw);
        if sha != file.source_sha {
            return Err(format!(
                "OCR 模型源件校验失败 {}: 期望 {} 实得 {sha}",
                file.url_path, file.source_sha
            ));
        }
        let cooked = if file.strip {
            strip_value_info(&raw)?
        } else {
            raw
        };
        let sha = sha256_hex(&cooked);
        if sha != file.cached_sha {
            return Err(format!(
                "OCR 模型处理后校验失败 {}: 期望 {} 实得 {sha}",
                file.name, file.cached_sha
            ));
        }
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("无法创建模型缓存目录 {}: {e}", dir.display()))?;
        std::fs::write(&path, &cooked)
            .map_err(|e| format!("无法写入模型缓存 {}: {e}", path.display()))?;
    }
    Ok(ModelPaths {
        det: dir.join(DET.name),
        rec: dir.join(REC.name),
        dict: dir.join(DICT.name),
    })
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

/// 下载为内存字节（模型最大 16MB；ureq 默认响应上限 10MB，显式放宽到 64MB）。
fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let mut resp = ureq::get(url)
        .call()
        .map_err(|e| format!("下载 OCR 模型失败 {url}: {e}"))?;
    resp.body_mut()
        .with_config()
        .limit(64 * 1024 * 1024)
        .read_to_vec()
        .map_err(|e| format!("读取 OCR 模型响应失败 {url}: {e}"))
}

/// 剥离 ONNX 中间 value_info 静态形状并把输出 shape 清为动态
/// （tract 符号维度推断与静态元数据冲突，S006 踩坑 1；对齐 PoC strip_value_info.py）。
fn strip_value_info(model: &[u8]) -> Result<Vec<u8>, String> {
    use tract_onnx::pb;
    let mut m = pb::ModelProto::decode(model).map_err(|e| format!("ONNX 解析失败: {e}"))?;
    let graph = m.graph.as_mut().ok_or("ONNX 无 graph")?;
    graph.value_info.clear();
    for out in &mut graph.output {
        let Some(shape) =
            out.r#type
                .as_mut()
                .and_then(|t| t.value.as_mut())
                .and_then(|v| match v {
                    pb::type_proto::Value::TensorType(t) => t.shape.as_mut(),
                })
        else {
            continue;
        };
        for dim in &mut shape.dim {
            dim.value = Some(pb::tensor_shape_proto::dimension::Value::DimParam(
                "dyn".to_string(),
            ));
        }
    }
    Ok(m.encode_to_vec())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("无法读取 {}: {e}", path.display()))?;
    Ok(sha256_hex(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_value_info_clears_intermediate_shapes() {
        // 最小 ONNX：graph 带一条 value_info 与一个静态输出 shape
        use tract_onnx::pb;
        let dim = pb::tensor_shape_proto::Dimension {
            value: Some(pb::tensor_shape_proto::dimension::Value::DimValue(960)),
            denotation: String::new(),
        };
        let shape = pb::TensorShapeProto { dim: vec![dim] };
        let tensor = pb::type_proto::Tensor {
            elem_type: 1,
            shape: Some(shape),
        };
        let vi = pb::ValueInfoProto {
            name: "x".to_string(),
            r#type: Some(pb::TypeProto {
                value: Some(pb::type_proto::Value::TensorType(tensor)),
                denotation: String::new(),
            }),
            ..Default::default()
        };
        let model = pb::ModelProto {
            graph: Some(pb::GraphProto {
                value_info: vec![vi.clone()],
                output: vec![vi],
                ..Default::default()
            }),
            ..Default::default()
        };
        let stripped = strip_value_info(&model.encode_to_vec()).unwrap();
        let back = pb::ModelProto::decode(stripped.as_slice()).unwrap();
        let graph = back.graph.unwrap();
        assert!(graph.value_info.is_empty());
        let out_dim = &graph.output[0]
            .r#type
            .as_ref()
            .unwrap()
            .value
            .as_ref()
            .unwrap();
        let pb::type_proto::Value::TensorType(t) = out_dim;
        let dims = &t.shape.as_ref().unwrap().dim;
        assert!(matches!(
            dims[0].value,
            Some(pb::tensor_shape_proto::dimension::Value::DimParam(_))
        ));
    }
}
