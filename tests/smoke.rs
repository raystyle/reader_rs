//! 冒烟测试层（G006）：只断「活着」，秒级完成。
//! `cargo test --test smoke` 单独调度；质量判断不归本层（归验收与 A/B）。
//! 前身 tests\smoke.py（D31 第 2 轮裁定：冒烟归 cargo 体系，uv Python 只留 A/B）。

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
fn version_and_help_alive() -> TestResult {
    reader()?
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
    reader()?
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("search"));
    Ok(())
}

#[test]
fn search_exit_codes_grep_semantics() -> TestResult {
    reader()?
        .args(["search"])
        .arg(repo_file("README.md"))
        .arg("reader")
        .assert()
        .success();
    reader()?
        .args(["search"])
        .arg(repo_file("README.md"))
        .arg("zz-绝不存在的词-zz")
        .assert()
        .code(1);
    Ok(())
}

#[test]
fn scanned_sample_hinted_needs_ocr() -> TestResult {
    reader()?
        .args(["extract"])
        .arg(repo_file("tests/ab/assets/scan-cjk.pdf"))
        .assert()
        .success()
        .stdout(predicate::str::contains("[needs_ocr"));
    Ok(())
}
