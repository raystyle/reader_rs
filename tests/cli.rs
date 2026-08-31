//! CLI 集成测试：assert_cmd 整跑 `reader` 二进制。
//! 测试 PDF 由 lopdf 现造（两页已知文本），期望值来自写入的文本本身，独立于被测实现。

use assert_cmd::Command;
use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};
use predicates::prelude::*;
use std::path::{Path, PathBuf};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

const PAGE1_TEXT: &str = "Hello Reader World";
const PAGE2_TEXT: &str = "Second page rust search";

fn reader() -> TestResult<Command> {
    Ok(Command::cargo_bin("reader")?)
}

fn pdf_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("reader_rs_cli_{}_{name}.pdf", std::process::id()))
}

/// 造两页测试 PDF：页 1 为 PAGE1_TEXT，页 2 为 PAGE2_TEXT。
fn make_test_pdf(path: &Path) -> TestResult {
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
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), 12.into()]),
                Operation::new("Td", vec![72.into(), 720.into()]),
                Operation::new("Tj", vec![Object::string_literal(text)]),
                Operation::new("ET", vec![]),
            ],
        };
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

struct TestPdf(PathBuf);

impl TestPdf {
    fn make(name: &str) -> TestResult<Self> {
        let path = pdf_path(name);
        make_test_pdf(&path)?;
        Ok(Self(path))
    }
}

impl Drop for TestPdf {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn runs_help() -> TestResult {
    reader()?
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("search"))
        .stdout(predicate::str::contains("extract"));
    Ok(())
}

#[test]
fn search_finds_keyword_with_page() -> TestResult {
    let pdf = TestPdf::make("search_hit")?;
    reader()?
        .args(["search"])
        .arg(&pdf.0)
        .arg("Reader")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("1:1:{PAGE1_TEXT}")));
    Ok(())
}

#[test]
fn search_no_match_exits_1() -> TestResult {
    let pdf = TestPdf::make("search_miss")?;
    reader()?
        .args(["search"])
        .arg(&pdf.0)
        .arg("zzz-no-such-word")
        .assert()
        .code(1);
    Ok(())
}

#[test]
fn search_regex_ignore_case_hits_page2() -> TestResult {
    let pdf = TestPdf::make("search_regex")?;
    reader()?
        .args(["search"])
        .arg(&pdf.0)
        .args(["R.ST", "--regex", "-i"])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("2:1:{PAGE2_TEXT}")));
    Ok(())
}

#[test]
fn search_pages_filter_excludes_hit() -> TestResult {
    let pdf = TestPdf::make("search_pages")?;
    reader()?
        .args(["search"])
        .arg(&pdf.0)
        .args(["Reader", "--pages", "2"])
        .assert()
        .code(1);
    Ok(())
}

#[test]
fn search_with_context_prints_neighbor_lines() -> TestResult {
    let pdf = TestPdf::make("search_ctx")?;
    reader()?
        .args(["search"])
        .arg(&pdf.0)
        .args(["Reader", "-C", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("1:1:{PAGE1_TEXT}")));
    Ok(())
}

#[test]
fn extract_outputs_page_sections() -> TestResult {
    let pdf = TestPdf::make("extract_all")?;
    reader()?
        .args(["extract"])
        .arg(&pdf.0)
        .assert()
        .success()
        .stdout(predicate::str::contains("== page 1 =="))
        .stdout(predicate::str::contains("== page 2 =="))
        .stdout(predicate::str::contains(PAGE1_TEXT))
        .stdout(predicate::str::contains(PAGE2_TEXT));
    Ok(())
}

#[test]
fn extract_pages_filter_keeps_selected() -> TestResult {
    let pdf = TestPdf::make("extract_pages")?;
    reader()?
        .args(["extract"])
        .arg(&pdf.0)
        .args(["--pages", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("== page 2 =="))
        .stdout(predicate::str::contains(PAGE2_TEXT))
        .stdout(predicate::str::contains("== page 1 ==").not())
        .stdout(predicate::str::contains(PAGE1_TEXT).not());
    Ok(())
}

#[test]
fn dies_missing_file() -> TestResult {
    reader()?
        .args(["search", "no-such-file-reader-rs.pdf", "x"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("reader:"));
    Ok(())
}

#[test]
fn dies_bad_page_spec() -> TestResult {
    let pdf = TestPdf::make("bad_pages")?;
    reader()?
        .args(["search"])
        .arg(&pdf.0)
        .args(["Reader", "--pages", "3-1"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("无效页范围"));
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    reader()?.assert().failure();
    Ok(())
}
