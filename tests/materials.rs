//! 本地研究资料库语料回归层（G006 真样本族 gated target，D46 第 2 轮）：E:\研究资料
//! （用户裁定弃 E:\ebook 改此，主性能与质量测试面）逐件对照 manifest 质量基线（status / units / needs_ocr 全确定性），
//! 守「pdf-inspector / anydoc / 分派层升级引起的输出漂移」——CLR 书页标记 399 那类
//! 手工基线的语料化全自动版。外部真样本不入仓：manifest 钉 sha256，盘或 manifest
//! 缺失即整体跳过不算失败（CI 免跑）。性能口径不进断言（G005 计时禁令）：
//! 全语料计时归 `.tools/materials-corpus.py --perf` 报告（tests\materials\reports\）。
//! 基线重钉：`uv run --script .tools/materials-corpus.py --baseline`（有意变更须人工审）。
//! 大书较重（5.4GB 语料全量一遍），按回归层纪律跑：动提取/搜索/OCR 管线后或发版前。

use assert_cmd::Command;
use serde_json::Value;
use std::path::{Path, PathBuf};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn manifest_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/materials/manifest.json")
}

/// 语料跑批用二进制：env READER_MATERIALS_BIN 指定（推荐 release，口径同 --verify），
/// 缺省 release 件优先（先 `cargo build --release`），再回退 cargo_bin 的 debug 件。
fn corpus_reader() -> TestResult<Command> {
    if let Some(bin) = std::env::var_os("READER_MATERIALS_BIN") {
        return Ok(Command::new(bin));
    }
    let release = Path::new(env!("CARGO_MANIFEST_DIR")).join(if cfg!(windows) {
        "target/release/reader.exe"
    } else {
        "target/release/reader"
    });
    if release.is_file() {
        Ok(Command::new(release))
    } else {
        Ok(Command::cargo_bin("reader")?)
    }
}

/// 一次 extract 的结构面（json 加 filter 取 units[].needs_ocr，免大书全文进内存）。
fn probe(bin: &mut Command, file: &Path) -> TestResult<(i32, Option<usize>, Option<usize>)> {
    let out = bin
        .args(["extract"])
        .arg(file)
        .args(["--format", "json", "--filter", "units[].needs_ocr"])
        .assert()
        .get_output()
        .clone();
    let code = out.status.code().unwrap_or(-1);
    if code != 0 {
        return Ok((code, None, None));
    }
    let v: Value = serde_json::from_slice(&out.stdout)?;
    let needs = v["data"].as_array().cloned();
    Ok((
        code,
        needs.as_ref().map(Vec::len),
        needs.map(|l| l.iter().filter(|x| !x.is_null()).count()),
    ))
}

/// manifest 质量基线逐件核验：与 `.tools/materials-corpus.py --verify` 同口径。
#[test]
fn materials_corpus_baseline_match() -> TestResult {
    let Some((root, entries)) = read_manifest() else {
        eprintln!(
            "skip: tests/materials/manifest.json 或语料盘缺失（E:\\研究资料 本机语料，CI 免跑）"
        );
        return Ok(());
    };
    let mut drift: Vec<String> = Vec::new();
    let total = entries.len();
    for entry in &entries {
        let rel = entry["rel"].as_str().unwrap_or_default().to_string();
        let want_status = entry["status"].as_i64();
        let want_units = entry["units"].as_i64();
        let want_needs = entry["needs_ocr"].as_i64();
        let (status, units, needs) = probe(&mut corpus_reader()?, &root.join(&rel))?;
        for (field, want, got) in [
            ("status", want_status, Some(status as i64)),
            ("units", want_units, units.map(|n| n as i64)),
            ("needs_ocr", want_needs, needs.map(|n| n as i64)),
        ] {
            if let Some(w) = want {
                if got != Some(w) {
                    drift.push(format!("{rel}: {field} 基线 {w} 实测 {got:?}"));
                }
            }
        }
    }
    assert!(
        drift.is_empty(),
        "{} 处漂移(共 {total} 件;有意变更须 materials-corpus.py --baseline 重钉并人工审):\n{}",
        drift.len(),
        drift.join("\n")
    );
    eprintln!("materials corpus: {total} 件全数一致");
    Ok(())
}

/// 门控读 manifest:文件在仓且其 root 盘也在才跑;条目缺基线字段(status 缺)也跳过。
fn read_manifest() -> Option<(PathBuf, Vec<Value>)> {
    let path = manifest_path();
    if !path.is_file() {
        return None;
    }
    let v: Value = serde_json::from_str(&std::fs::read_to_string(&path).ok()?).ok()?;
    let root = PathBuf::from(v["root"].as_str()?);
    if !root.is_dir() {
        return None;
    }
    let entries: Vec<Value> = v["entries"].as_array()?.clone();
    (entries.iter().all(|e| e["status"].is_i64())).then_some((root, entries))
}
