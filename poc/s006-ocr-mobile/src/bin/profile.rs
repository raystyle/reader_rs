use std::time::Instant;

// 性能与质量剖析：profile [png] [det_limit] [rec_batch] [det.onnx rec.onnx dict.txt]
fn main() {
    let img_path = std::env::args().nth(1).unwrap_or_else(|| "out/render_p10.png".into());
    let det_limit: u32 = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(960);
    let det = std::env::args().nth(4).unwrap_or_else(|| "models/ppocrv5/det-dyn.onnx".into());
    let rec = std::env::args().nth(5).unwrap_or_else(|| "models/ppocrv5/rec-dyn.onnx".into());
    let dict = std::env::args().nth(6).unwrap_or_else(|| "models/ppocrv5/ppocrv5_dict.txt".into());
    let image = image::open(&img_path).unwrap();
    let engine = pure_onnx_ocr::OcrEngineBuilder::new()
        .det_model_path(&det)
        .rec_model_path(&rec)
        .dictionary_path(&dict)
        .det_limit_side_len(det_limit)
        .build()
        .unwrap();
    let t0 = Instant::now();
    let run = engine.run_with_metrics_from_image(&image).unwrap();
    let t = &run.timings;
    println!("lines={} wall={:?} det_infer={:?} rec_infer={:?}", run.results.len(), t.total, t.detection.inference, t.recognition.inference);
    for (i, r) in run.results.iter().enumerate() {
        println!("#{i} conf={:.3} {}", r.confidence, r.text);
    }
}
