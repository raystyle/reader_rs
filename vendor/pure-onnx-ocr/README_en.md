# `pure-onnx-ocr`

Author: Shion Watanabe  
Date: 2025-11-09  
Repository: http://github.com/siska-tech/pure-onnx-ocr

Pure Rust OCR pipeline that re-implements the PaddleOCR (DBNet + SVTR\_HGNet) models without relying on C/C++ runtimes. The crate provides a high-level `OcrEngine` facade that hides detection and recognition stages behind a builder-style configuration API.

## Highlights

- **Pure Rust runtime** – no native shared libraries or FFI bindings; `cargo build` is enough.
- **DBNet + SVTR pipeline** – mirrors the official PaddleOCR ONNX export while staying within the Rust ecosystem.
- **Extensible architecture** – detection, recognition, and geometry utilities are separated so you can swap or extend individual stages.
- **Portable** – designed to run in environments where shipping C++ runtimes is difficult (embedded, serverless, WASM).

## Prerequisites

- Rust 1.75 or newer (stable channel)
- CPU inference on x86\_64 or aarch64
- ONNX models (`det.onnx`, `rec.onnx`) and the PaddleOCR dictionary (`ppocrv5_dict.txt`)

## Installation

```toml
[dependencies]
pure_onnx_ocr = "0.1.0"
image = "0.25"       # recommended for image I/O
geo-types = "0.7"    # recommended for working with polygon results
```

Download the `PP-OCRv5_Server-ONNX` (or Mobile) bundle from PaddleOCR. Place the files under `models/ppocrv5/` (or any path of your choice) and pass the paths into the builder.

## Quick Start

```rust
use pure_onnx_ocr::{OcrEngineBuilder, OcrResult};

fn main() -> Result<(), pure_onnx_ocr::OcrError> {
    let engine = OcrEngineBuilder::new()
        .det_model_path("models/ppocrv5/det.onnx")
        .rec_model_path("models/ppocrv5/rec.onnx")
        .dictionary_path("models/ppocrv5/ppocrv5_dict.txt")
        .det_limit_side_len(960)
        .det_unclip_ratio(1.5)
        .rec_batch_size(8)
        .build()?;

    let results: Vec<OcrResult> = engine.run_from_path("examples/demo.jpg")?;
    for (idx, result) in results.iter().enumerate() {
        println!(
            "#{} text={} confidence={:.4} polygon={:?}",
            idx,
            result.text,
            result.confidence,
            result.bounding_box.exterior().points()
        );
    }

    Ok(())
}
```

## Smoke Testing with `ocr_smoke`

If you want to replicate the behaviour of the original `test_ocr.py` without leaving the Rust ecosystem, you can use the bundled `ocr_smoke` binary.

- By default it points to `models/ppocrv5/det.onnx`, `models/ppocrv5/rec.onnx`, and `models/ppocrv5/ppocrv5_dict.txt`.
- Example usage:

```bash
cargo run --bin ocr_smoke -- path/to/image.jpg

# Override model paths and runtime options
cargo run --bin ocr_smoke -- path/to/image.jpg \
  --det-model models/ppocrv5/det.onnx \
  --rec-model models/ppocrv5/rec.onnx \
  --dictionary models/ppocrv5/ppocrv5_dict.txt \
  --det-limit-side-len 960 \
  --det-unclip-ratio 1.5 \
  --rec-batch-size 8
```

The CLI prints inference timing, recognised texts with confidences, and polygon coordinates. It exits with a descriptive error when the image or models are missing.

Internally, the detection pre-processing stage now zero-pads resized tensors so their height/width are multiples of 32, matching DBNet’s input requirements.

> **Current limitation:** Although the pipeline loads and runs, the OCR results are still noisy and often incorrect. Root-cause analysis and debugging remain open tasks.

### Troubleshooting

- `ModelLoad`: `tract` rejected an operator that the ONNX graph requires (e.g., `LayerNormalization`, `Scan`). Try a simplified model or file an issue with model details.
- `Dictionary`: ensure the dictionary file is encoded in UTF-8 without BOM.

## API Overview

| Symbol             | Description                                                                                                   |
| ------------------ | ------------------------------------------------------------------------------------------------------------- |
| `OcrEngineBuilder` | Configures model paths and runtime parameters. Produces an `OcrEngine`.                                       |
| `OcrEngine`        | Facade that executes detection + recognition. Provides `run_from_path` and `run_from_image`.                  |
| `OcrResult`        | Holds the text, confidence score, and `Polygon` bounding box for a single region.                             |
| `OcrError`         | Enumerates all errors emitted by the library (I/O, model loading, preprocessing, inference, post-processing). |
| `Polygon`          | Re-export of `geo-types::Polygon`. Useful for downstream geometry processing.                                 |

For detailed behavior and error semantics, see `docs/interface_design_en.md`.

## Documentation Set

- Architecture: `docs/architecture_en.md`
- Detailed design: `docs/detail_design_en.md`
- Interface design: `docs/interface_design_en.md`
- Requirements: `docs/requirements_en.md`
- References: `docs/references_en.md`
- Test specification: `docs/test_specification_en.md`

Each English document mirrors the Japanese source to help international contributors understand the project.

## Project Status

- 2025-11-09: Completed PoC for `det.onnx` (DBNet) loading via `tract-onnx`.
- 2025-11-09: Validated `rec.onnx` (SVTR\_HGNet) dummy inference; confirmed output shape `[1, 40, 18385]`.
- 2025-11-09: Implemented detection preprocessing (`DetPreProcessor`) with resizing, normalization, and NCHW transforms.
- 2025-11-09: Implemented detection inference session with runnable caching per input resolution.
- 2025-11-09: Implemented detection post-processing (contour extraction and filtering).
- 2025-11-09: Implemented polygon unclipping via `i_overlay` buffering.
- 2025-11-09: Implemented polygon scaling back to original coordinates.
- 2025-11-09: Implemented recognition preprocessing with cropping, force resize, normalization, and batching.
- 2025-11-09: Implemented recognition inference session with batch execution.
- 2025-11-09: Implemented dictionary loader with dedupe and bidirectional mapping.
- 2025-11-09: Implemented Pure Rust CTC greedy decoder with duplicate suppression and blank removal.
- 2025-11-09: Implemented recognition post-processor that combines logits, CTC decoding, and dictionary lookup.
- 2025-11-09: Implemented `OcrEngineBuilder`, `OcrEngine`, and public error surface.
- 2025-11-09: Refreshed README and added bilingual documentation set (`task-doc-001`).
- 2025-11-09: Enhanced public Rustdoc coverage (`task-doc-002`) and validated `cargo doc` output.
- 2025-11-09: Completed Cargo metadata (`task-doc-003`) and `cargo package --no-verify` validation.
- 2025-11-09: Added integration tests (`task-doc-004`) with fixture strategy and CI guidance.

## Contributing

Issues and pull requests are welcome. Please:

- Run `cargo fmt` and `cargo clippy` before submitting patches.
- Add unit tests where possible.
- Update the corresponding task file in `docs/devlog/` when documentation or feature work progresses.

## License

Licensed under `Apache-2.0`, aligning with PaddleOCR, OnnxOCR, and tract licensing.

## Testing

- Unit tests: `cargo test`
- Integration tests: provide PP-OCRv5 models and a demo image via the `PURE_ONNX_OCR_FIXTURE_DIR` environment variable or `tests/fixtures/`. See `tests/fixtures/README.md` for the expected directory structure. Tests skip automatically when fixtures are missing.

