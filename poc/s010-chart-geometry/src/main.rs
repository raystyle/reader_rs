//! S010 T2 PoC:ppocr-rs polygon 盒子的几何配对,验证「带数据标签的柱状图可还原成表」。
//! 样本:安全相似系统学.pdf 图 1-1(PDF 页 13,柱状图,数据标签齐全但 OCR 出来是散片)。
//! 算法:盒子按 y 聚行;年份串最多的那行 = x 轴列;数值标签(轴行上方)按 x 中心就近
//! 配列,同列按 y 升序保上下序;其余文本盒列为图例候选。不追色不追形,纯文本几何。

use hayro::hayro_syntax::Pdf;
use hayro::{render, RenderCache, RenderSettings};
use ppocr_rs::{ModelAccess, ModelSize, ModelStore, OcrEngine, OcrOptions, RgbImage};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let pdf_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "E:/研究资料/书籍作品/安全相似系统学.pdf".to_string());
    let page_no: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(13);

    // 1. 渲染页(与主仓 figures/ocr 同口径:2 倍白底)
    let bytes = std::fs::read(&pdf_path).map_err(|e| format!("读 PDF 失败: {e}"))?;
    let pdf = Pdf::new(bytes).map_err(|e| format!("解析失败: {e:?}"))?;
    let pages = pdf.pages();
    let page = pages
        .get(page_no - 1)
        .ok_or_else(|| format!("页 {page_no} 超范围"))?
        .clone();
    let settings = RenderSettings {
        x_scale: 2.0,
        y_scale: 2.0,
        bg_color: hayro::vello_cpu::color::palette::css::WHITE,
        ..Default::default()
    };
    let pixmap = render(&page, &RenderCache::new(), &Default::default(), &settings);
    let png = pixmap.into_png().map_err(|e| format!("编码失败: {e:?}"))?;
    let rgb = image::load_from_memory(&png)
        .map_err(|e| format!("解码失败: {e}"))?
        .to_rgb8();
    let (w, h) = rgb.dimensions();
    let img = RgbImage::new(w, h, rgb.into_raw()).map_err(|e| format!("转换失败: {e}"))?;

    // 2. OCR(离线,须先 reader ocr init;盒子全保留)
    let cache = std::env::var_os("READER_OCR_CACHE_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(default_cache);
    let store = ModelStore::new(&cache);
    let size = ModelSize::Tiny;
    let paths = store
        .resolve_pair(size, size, ModelAccess::Offline)
        .map_err(|e| format!("模型未就位(先 reader ocr init): {e}"))?;
    let engine = OcrEngine::load(
        &paths.detector.weights,
        &paths.recognizer.weights,
        &paths.recognizer.inference,
        OcrOptions {
            threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
            detector_size: size,
            recognizer_size: size,
            ..OcrOptions::default()
        },
    )
    .map_err(|e| format!("引擎构建失败: {e}"))?;
    let result = engine.recognize(&img).map_err(|e| format!("识别失败: {e}"))?;

    // 3. 盒子几何
    struct BoxLine {
        text: String,
        x0: f64,
        x1: f64,
        cx: f64,
        cy: f64,
    }
    let boxes: Vec<BoxLine> = result
        .lines
        .iter()
        .map(|l| {
            let xs: Vec<f64> = l.polygon.iter().map(|p| p.0 as f64).collect();
            let ys: Vec<f64> = l.polygon.iter().map(|p| p.1 as f64).collect();
            let (x0, x1) = xs.iter().cloned().fold((f64::MAX, f64::MIN), |(a, b), x| {
                (a.min(x), b.max(x))
            });
            BoxLine {
                text: l.text.trim().to_string(),
                x0,
                x1,
                cx: xs.iter().sum::<f64>() / xs.len() as f64,
                cy: ys.iter().sum::<f64>() / ys.len() as f64,
            }
        })
        .filter(|b| !b.text.is_empty())
        .collect();
    println!("OCR 盒子 {} 个", boxes.len());

    let all_digits = |t: &str| !t.is_empty() && t.chars().all(|c| c.is_ascii_digit());
    let is_number = |t: &str| {
        !t.is_empty()
            && t.chars().all(|c| c.is_ascii_digit() || c == '.')
            && t.parse::<f64>().is_ok()
    };

    // x 轴年份列:两种形态——独立 4 位年份盒,与 OCR 融合的长数字串盒(实测本样本
    // 年份轴融成「20042005…2015」单盒,按 4 位切块在盒宽内插值列坐标)。
    // 取含年份数最多的 y 簇(容差 30px)为轴行。
    #[derive(Clone)]
    struct Col {
        year: String,
        cx: f64,
    }
    let mut candidates: Vec<(f64, Vec<Col>)> = vec![]; // (cy, 该 y 簇的年份列)
    for b in &boxes {
        let mut cols: Vec<Col> = vec![];
        if all_digits(&b.text) && b.text.len() == 4 && b.text.starts_with('2') {
            cols.push(Col {
                year: b.text.clone(),
                cx: b.cx,
            });
        } else if all_digits(&b.text) && b.text.len() >= 8 && b.text.len() % 4 == 0 && b.text.starts_with("20") {
            let n = b.text.len() / 4;
            for (i, chunk) in b.text.as_bytes().chunks(4).enumerate() {
                let year = String::from_utf8_lossy(chunk).to_string();
                if !year.starts_with('2') {
                    continue;
                }
                cols.push(Col {
                    year,
                    cx: b.x0 + (i as f64 + 0.5) / n as f64 * (b.x1 - b.x0),
                });
            }
        }
        if cols.is_empty() {
            continue;
        }
        if let Some(slot) = candidates
            .iter_mut()
            .find(|(cy, _)| (cy - b.cy).abs() < 30.0)
        {
            slot.1.extend(cols);
        } else {
            candidates.push((b.cy, cols));
        }
    }
    let Some((axis_cy, mut columns_axis)) = candidates.into_iter().max_by_key(|(_, c)| c.len()) else {
        return Err("未找到年份轴行(无 2xxx 年份形态盒)".to_string());
    };
    if columns_axis.len() < 3 {
        return Err(format!("年份列过少({})", columns_axis.len()));
    }
    columns_axis.sort_by(|a, b| a.cx.partial_cmp(&b.cx).unwrap());
    println!(
        "年份轴行 {} 列: {}",
        columns_axis.len(),
        columns_axis
            .iter()
            .map(|c| c.year.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    );

    // 数值标签配列:轴行上方,按 x 中心就近取年份列;同列 y 升序保上下序
    let mut columns: Vec<(String, f64, Vec<(f64, String)>)> = columns_axis
        .iter()
        .map(|c| (c.year.clone(), c.cx, vec![]))
        .collect();
    let axis_digits_cy = axis_cy;
    let mut legend: Vec<String> = vec![];
    for b in &boxes {
        // 轴行自身的盒(独立年份数字)跳过
        if all_digits(&b.text)
            && b.text.len() == 4
            && (b.cy - axis_digits_cy).abs() < 30.0
        {
            continue;
        }
        if all_digits(&b.text) && b.text.len() >= 8 {
            continue; // 融合轴串已消费
        }
        if is_number(&b.text) && b.cy < axis_digits_cy {
            if let Some(col) = columns
                .iter_mut()
                .min_by(|(_, xc, _), (_, yc, _)| {
                    (xc - b.cx)
                        .abs()
                        .partial_cmp(&(yc - b.cx).abs())
                        .unwrap()
                })
            {
                col.2.push((b.cy, b.text.clone()));
            }
        } else if !is_number(&b.text) {
            legend.push(format!("{} @({:.0},{:.0})", b.text, b.cx, b.cy));
        }
    }
    for col in columns.iter_mut() {
        col.2.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    println!("图例与文本盒:");
    for l in &legend {
        println!("  {l}");
    }
    println!("还原表(年份: 值按上到下):");
    for (year, _, vals) in &columns {
        let vs: Vec<&str> = vals.iter().map(|(_, t)| t.as_str()).collect();
        println!("  {year}: {}", vs.join(", "));
    }
    println!(
        "覆盖率: {}/{} 年份列有标签",
        columns.iter().filter(|(_, _, v)| !v.is_empty()).count(),
        columns.len()
    );
    Ok(())
}

fn default_cache() -> std::path::PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .or_else(|| std::env::var_os("APPDATA"))
        .map(std::path::PathBuf::from)
        .expect("缓存目录不可定位");
    base.join("reader").join("models")
}
