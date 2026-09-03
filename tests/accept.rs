//! 验收测试层（G006）可机检部分：对外契约与需求口径。
//! `cargo test --test accept` 单独调度；实机清单与发版资产验收仍走 R004/R005 与发版流程。
//! 前身 tests\accept.py（D31 第 2 轮裁定归 cargo）。

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn reader() -> TestResult<Command> {
    Ok(Command::cargo_bin("reader")?)
}

fn repo_file(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

#[test]
fn version_matches_cargo_toml() -> TestResult {
    let manifest = std::fs::read_to_string(repo_file("Cargo.toml"))?;
    let want = manifest
        .lines()
        .find_map(|l| {
            l.trim()
                .strip_prefix("version")
                .and_then(|r| r.trim().strip_prefix('='))
        })
        .map(|v| v.trim().trim_matches('"').to_string())
        .ok_or("Cargo.toml 找不到 version")?;
    reader()?
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(want));
    Ok(())
}

#[test]
fn llms_index_non_empty() -> TestResult {
    reader()?
        .arg("--llms")
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
    Ok(())
}

#[test]
fn json_envelope_has_ok_data_meta() -> TestResult {
    let out = reader()?
        .args(["search"])
        .arg(repo_file("README.md"))
        .args(["reader", "--format", "json"])
        .assert()
        .success();
    let v: serde_json::Value = serde_json::from_slice(&out.get_output().stdout)?;
    assert_eq!(
        v.get("ok").and_then(|b| b.as_bool()),
        Some(true),
        "包膜 ok 应为 true"
    );
    assert!(v.get("data").is_some(), "包膜缺 data 字段");
    assert!(v.get("meta").is_some(), "包膜缺 meta 字段");
    Ok(())
}

#[test]
fn dies_missing_file_exit_2() -> TestResult {
    reader()?
        .args(["search", "不存在的文件.pdf", "x"])
        .assert()
        .code(2);
    Ok(())
}

#[test]
fn query_h1_on_markdown() -> TestResult {
    reader()?
        .args(["query"])
        .arg(repo_file("README.md"))
        .arg(".h1")
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
    Ok(())
}
