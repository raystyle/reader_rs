//! 验收测试层（G006）可机检部分：cucumber BDD（D33）。
//! 场景在 `tests/features/accept.feature`（业务语言），本文件是步骤绑定。
//! `cargo test --test accept` 调度（harness = false，cucumber 自管输出与退出码）。
//! 实机清单与发版资产验收仍走 R004/R005 与发版流程。

use cucumber::{then, when, World};
use std::path::PathBuf;

#[derive(Debug, Default, World)]
pub struct AcceptWorld {
    code: i32,
    stdout: String,
    stderr: String,
}

fn repo_file(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

#[when(expr = "执行 {string}")]
fn run_reader(w: &mut AcceptWorld, args: String) {
    let argv: Vec<&str> = args.split_whitespace().collect();
    let out = assert_cmd::Command::cargo_bin("reader")
        .expect("reader 二进制应已由 cargo 构建")
        .args(&argv)
        .output()
        .expect("reader 应可执行");
    w.code = out.status.code().unwrap_or(-1);
    w.stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    w.stderr = String::from_utf8_lossy(&out.stderr).into_owned();
}

#[then(expr = "退出码为 {int}")]
fn check_code(w: &mut AcceptWorld, want: i32) {
    assert_eq!(w.code, want, "退出码不符；stderr: {}", w.stderr);
}

#[then(expr = "标准输出包含 {string}")]
fn stdout_contains(w: &mut AcceptWorld, needle: String) {
    assert!(
        w.stdout.contains(&needle),
        "stdout 应包含 `{needle}`，实得前 200 字符: {}",
        &w.stdout[..w.stdout.len().min(200)]
    );
}

#[then(expr = "标准输出非空")]
fn stdout_non_empty(w: &mut AcceptWorld) {
    assert!(
        !w.stdout.trim().is_empty(),
        "stdout 不应为空；stderr: {}",
        w.stderr
    );
}

#[then(expr = "包膜字段齐备")]
fn envelope_fields(w: &mut AcceptWorld) {
    let v: serde_json::Value =
        serde_json::from_str(&w.stdout).unwrap_or_else(|e| panic!("stdout 应为 JSON: {e}"));
    assert_eq!(
        v.get("ok").and_then(|b| b.as_bool()),
        Some(true),
        "包膜 ok 应为 true"
    );
    assert!(v.get("data").is_some(), "包膜缺 data 字段");
    assert!(v.get("meta").is_some(), "包膜缺 meta 字段");
}

#[then(expr = "版本与清单一致")]
fn version_matches(w: &mut AcceptWorld) {
    let manifest = std::fs::read_to_string(repo_file("Cargo.toml")).expect("Cargo.toml 应可读");
    let want = manifest
        .lines()
        .find_map(|l| {
            l.trim()
                .strip_prefix("version")
                .and_then(|r| r.trim().strip_prefix('='))
        })
        .map(|v| v.trim().trim_matches('"').to_string())
        .expect("Cargo.toml 应有 version");
    assert!(
        w.stdout.contains(&want),
        "版本输出应含 {want}，实得: {}",
        w.stdout.trim()
    );
}

fn main() {
    futures::executor::block_on(AcceptWorld::run("tests/features"));
}
