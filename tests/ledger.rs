//! ledger 命令面回归（契约 = ohmycloud REQ-063 加总台修正令 2026-09-20 收口）：
//! 各仓 CLI 只增不关不删（close 与 promote/demote/supersede 面退役，指路 omc
//! 工位）；attest 验证类三型客户端先拒；签名道与请求形状面归统一 crate
//! ledger-client 自测，本 target 只测 CLI 面（零网络）。

use predicates::prelude::*;

fn reader() -> Result<assert_cmd::Command, Box<dyn std::error::Error>> {
    Ok(assert_cmd::Command::cargo_bin("reader")?)
}

/// 收口面：issue close 退役（unrecognized 子命令）。
#[test]
fn issue_close_face_retired() -> Result<(), Box<dyn std::error::Error>> {
    reader()?
        .args(["issue", "close", "1", "--digest", "sha256:0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
    Ok(())
}

/// 收口面：artifact promote 退役（unrecognized 子命令）。
#[test]
fn artifact_promote_face_retired() -> Result<(), Box<dyn std::error::Error>> {
    reader()?
        .args(["artifact", "promote", "some-id"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
    Ok(())
}

/// attest 类型收口：验证类三型之外（promote/demote/supersede 等）客户端先拒
/// 并指路 omc 工位（不挂网络）。
#[test]
fn attest_type_restricted_to_verification_kinds() -> Result<(), Box<dyn std::error::Error>> {
    for t in ["promote", "demote", "supersede", "status", "claim"] {
        reader()?
            .args(["artifact", "attest", "x", "--attest-type", t])
            .assert()
            .failure()
            .stderr(predicate::str::contains("attest --attest-type 仅"))
            .stderr(predicate::str::contains("omc 工位"));
    }
    // --checks 非 JSON 对象客户端先拒（不挂网络）
    reader()?
        .args([
            "artifact",
            "attest",
            "x",
            "--attest-type",
            "attest_dev",
            "--checks",
            "not-json",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--checks 须为 JSON 对象"));
    Ok(())
}

/// 帮助面：issue 与 artifact 组说明含只增口径与 omc 指路；attest 帮助列三型。
#[test]
fn help_faces_note_add_only_scope() -> Result<(), Box<dyn std::error::Error>> {
    reader()?
        .args(["issue", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("只增不关不删"))
        .stdout(predicate::str::contains("omc 工位"));
    reader()?
        .args(["artifact", "attest", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("attest_dev"));
    Ok(())
}

/// REQ-062 过滤面：`issue list` 帮助含 --status 与 --kind；缺值即拒（零网络）。
#[test]
fn issue_list_filter_flags_on_help_and_reject_missing_value(
) -> Result<(), Box<dyn std::error::Error>> {
    reader()?
        .args(["issue", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--status"))
        .stdout(predicate::str::contains("--kind"));
    reader()?
        .args(["issue", "list", "--status"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("a value is required"));
    Ok(())
}

/// REQ-063 新标面：--outcome 收口 success|failure 客户端先拒（不挂网络）；
/// --summary 必填缺失即 clap 拒；帮助面列新标旗标。
#[test]
fn artifact_publish_new_standard_flags() -> Result<(), Box<dyn std::error::Error>> {
    reader()?
        .args([
            "artifact",
            "publish",
            "t",
            "--kind",
            "lesson",
            "--digest",
            "sha256:0",
            "--summary",
            "s",
            "--outcome",
            "bogus",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--outcome 仅 success|failure"));
    reader()?
        .args([
            "artifact",
            "publish",
            "t",
            "--kind",
            "lesson",
            "--digest",
            "sha256:0",
            "--outcome",
            "success",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--summary"));
    reader()?
        .args(["artifact", "publish", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--summary"))
        .stdout(predicate::str::contains("--outcome"))
        .stdout(predicate::str::contains("--git-sha"));
    Ok(())
}
