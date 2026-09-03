//! 回归测试层（G006）：真样本行为基线对照，兜底「测试没写到的真实行为」。
//! `cargo test --test regress` 单独调度；external 样本缺失或 sha256 不符时跳过（不算失败）。
//! 前身 tests\regress.py（D31 第 2 轮裁定归 cargo）。
//!
//! 基线独立来源：
//! - CLR 书：search `assert_cmd` 25 命中行（S001/P0006 实证序列），extract 页标记 399
//!   （2026-09-03 本机实测；S001 记 390 系旧管线口径）
//! - 安全牛水印 PDF：extract 页标记 81（2026-09-03 本机实测）
//!
//! 基线变化必须有意为之并同步 G006 基线表。

use assert_cmd::Command;
use sha2::{Digest, Sha256};
use std::path::Path;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

struct ExternalSample {
    name: &'static str,
    path: &'static str,
    sha256: &'static str,
}

const CLR: ExternalSample = ExternalSample {
    name: "clr-book",
    path: "D:/Command-Line Rust_ A Project-Based Primer for Writing Rust CLIs 1 (2022, O'Reilly Media).pdf",
    sha256: "05fc14c57ee757355621988315978280bbf41158646be3fc7f25ced25ac78de9",
};

const ANNIU: ExternalSample = ExternalSample {
    name: "anniu-watermark",
    path: "D:/安全牛《新一代自动化渗透测试工具与应用实践指南》--水印.pdf",
    sha256: "edc1ea37c8e75489ffe91e3c8acd50ab139fd5eb9a65f502137649aedafd3dff",
};

fn reader() -> TestResult<Command> {
    Ok(Command::cargo_bin("reader")?)
}

/// external 样本可用性：缺失或哈希不符返回 false（跳过不算失败）。
fn available(s: &ExternalSample) -> TestResult<bool> {
    let path = Path::new(s.path);
    if !path.is_file() {
        eprintln!(
            "SKIP {}：样本不存在（{}），external 样本换机需调路径",
            s.name, s.path
        );
        return Ok(false);
    }
    let digest = format!("{:x}", Sha256::digest(std::fs::read(path)?));
    if digest != s.sha256 {
        eprintln!("SKIP {}：sha256 不符，样本被换过", s.name);
        return Ok(false);
    }
    Ok(true)
}

fn page_markers(stdout: &str) -> usize {
    stdout.lines().filter(|l| l.starts_with("== page ")).count()
}

#[test]
fn clr_search_assert_cmd_25_hits() -> TestResult {
    if !available(&CLR)? {
        return Ok(());
    }
    let out = reader()?
        .args(["search", CLR.path, "assert_cmd"])
        .assert()
        .success();
    let hits = String::from_utf8(out.get_output().stdout.clone())?;
    let n = hits.lines().filter(|l| !l.trim().is_empty()).count();
    assert_eq!(n, 25, "CLR 书 search assert_cmd 基线 25 命中行，实得 {n}");
    Ok(())
}

#[test]
fn clr_extract_399_page_markers() -> TestResult {
    if !available(&CLR)? {
        return Ok(());
    }
    let out = reader()?.args(["extract", CLR.path]).assert().success();
    let n = page_markers(&String::from_utf8(out.get_output().stdout.clone())?);
    assert_eq!(n, 399, "CLR 书 extract 页标记基线 399，实得 {n}");
    Ok(())
}

#[test]
fn anniu_extract_81_page_markers() -> TestResult {
    if !available(&ANNIU)? {
        return Ok(());
    }
    let out = reader()?.args(["extract", ANNIU.path]).assert().success();
    let n = page_markers(&String::from_utf8(out.get_output().stdout.clone())?);
    assert_eq!(n, 81, "安全牛 extract 页标记基线 81，实得 {n}");
    Ok(())
}

#[test]
fn scan_cjk_reports_needs_ocr() -> TestResult {
    let sample = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ab/assets/scan-cjk.pdf");
    reader()?
        .args(["extract"])
        .arg(sample)
        .assert()
        .success()
        .stdout(predicates::str::contains("[needs_ocr: scanned]"));
    Ok(())
}
