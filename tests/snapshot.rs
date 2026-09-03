//! 回归层快照防线（D34，G006）：公开输出的全量快照，兜底「断言没写到的格式漂移」。
//! 纪律：快照变更必须人工审 diff，禁止盲目 `cargo insta accept`（G006 演进表注记）。
//! `cargo test --test snapshot` 单独调度；快照文件落 `tests\snapshots\` 入仓。

use assert_cmd::Command;
use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};
use std::path::{Path, PathBuf};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

const PAGE1_TEXT: &str = "Hello Snapshot World";
const PAGE2_TEXT: &str = "Second page rust regress";

fn reader() -> TestResult<Command> {
    Ok(Command::cargo_bin("reader")?)
}

/// 造两页测试 PDF（与 cli.rs 同款最小 lopdf 夹具，本 target 独立 crate 需自带）。
fn make_pdf(path: &Path) -> TestResult {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
        "Encoding" => "WinAnsiEncoding",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! { "F1" => font_id },
    });
    let mut page_ids = Vec::new();
    for text in [PAGE1_TEXT, PAGE2_TEXT] {
        let operations = vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), 12.into()]),
            Operation::new("Td", vec![72.into(), 720.into()]),
            Operation::new("Tj", vec![Object::string_literal(text)]),
            Operation::new("ET", vec![]),
        ];
        let content = Content { operations };
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode()?));
        page_ids.push(doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
            "Resources" => resources_id,
            "Contents" => content_id,
        }));
    }
    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => page_ids.iter().copied().map(Object::from).collect::<Vec<_>>(),
            "Count" => page_ids.len() as i64,
        }),
    );
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc.save(path)?;
    Ok(())
}

fn stdout_of(out: &assert_cmd::assert::Assert) -> TestResult<String> {
    Ok(String::from_utf8(out.get_output().stdout.clone())?)
}

fn snap_pdf(tag: &str) -> PathBuf {
    // 用例各自独立文件名：同进程同 pid 下并行用例会互删共享文件（2026-09-03 首跑踩坑）
    std::env::temp_dir().join(format!("reader_rs_snap_{tag}_{}.pdf", std::process::id()))
}

#[test]
fn snapshot_extract_two_page_pdf() -> TestResult {
    let pdf = snap_pdf("extract");
    make_pdf(&pdf)?;
    let out = reader()?.args(["extract"]).arg(&pdf).assert().success();
    insta::assert_snapshot!("extract_two_page_pdf", stdout_of(&out)?);
    std::fs::remove_file(&pdf)?;
    Ok(())
}

#[test]
fn snapshot_search_hits_format() -> TestResult {
    let pdf = snap_pdf("search");
    make_pdf(&pdf)?;
    let out = reader()?
        .args(["search"])
        .arg(&pdf)
        .arg("rust")
        .assert()
        .success();
    insta::assert_snapshot!("search_hits_format", stdout_of(&out)?);
    std::fs::remove_file(&pdf)?;
    Ok(())
}

#[test]
fn snapshot_llms_index() -> TestResult {
    let out = reader()?.arg("--llms").assert().success();
    insta::assert_snapshot!("llms_index", stdout_of(&out)?);
    Ok(())
}
