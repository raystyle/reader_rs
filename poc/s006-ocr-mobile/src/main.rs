use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};
use std::time::Instant;

fn main() {
    // 用法: poc-ocr render <pdf> <page> [scale]  —— hayro 渲染一页为 PNG
    //      poc-ocr ocr <png>                    —— pure-onnx-ocr 识别
    let mode = std::env::args().nth(1).expect("render|ocr");
    match mode.as_str() {
        "render" => {
            let path = std::env::args().nth(2).unwrap();
            let want: usize = std::env::args().nth(3).unwrap().parse().unwrap();
            let scale: f32 = std::env::args()
                .nth(4)
                .and_then(|s| s.parse().ok())
                .unwrap_or(2.0);
            let file = std::fs::read(&path).unwrap();
            let pdf = Pdf::new(file).unwrap();
            let page = &pdf.pages()[want - 1];
            let settings = RenderSettings {
                x_scale: scale,
                y_scale: scale,
                bg_color: hayro::vello_cpu::color::palette::css::WHITE,
                ..Default::default()
            };
            let t0 = Instant::now();
            let pixmap = render(
                page,
                &RenderCache::new(),
                &Default::default(),
                &settings,
            );
            let png = pixmap.into_png().unwrap();
            let out = format!("out/render_p{want}.png");
            std::fs::create_dir_all("out").unwrap();
            std::fs::write(&out, &png).unwrap();
            println!(
                "rendered page {want} scale {scale} -> {out} ({} bytes, {:?})",
                png.len(),
                t0.elapsed()
            );
        }
        "ocr" => {
            let img = std::env::args().nth(2).unwrap();
            let t0 = Instant::now();
            let engine = pure_onnx_ocr::OcrEngineBuilder::new()
                .det_model_path("models/ppocrv5/det-dyn.onnx")
                .rec_model_path("models/ppocrv5/rec-dyn.onnx")
                .dictionary_path("models/ppocrv5/ppocrv5_dict.txt")
                .det_limit_side_len(1600)
                .build()
                .expect("build engine");
            println!("engine built in {:?}", t0.elapsed());
            let t1 = Instant::now();
            let results = engine.run_from_path(&img).expect("ocr run");
            println!("ocr of {img}: {} lines in {:?}", results.len(), t1.elapsed());
            for (i, r) in results.iter().enumerate() {
                println!("#{i} conf={:.3} {}", r.confidence, r.text);
            }
        }
        _ => eprintln!("unknown mode"),
    }
}
