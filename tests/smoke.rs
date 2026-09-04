//! 冒烟测试层（G006）：只断「活着」，秒级完成。
//! `cargo test --test smoke` 单独调度；质量判断不归本层（归验收与 A/B）。
//! 前身 tests\smoke.py（D31 第 2 轮裁定：冒烟归 cargo 体系，uv Python 只留 A/B）。

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::{Path, PathBuf};

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

// ---------- 全格式最小活体（D44）：支持面各族逐族 extract 活体 ----------
// 覆盖矩阵缺口实证（2026-09-04）：odt / rtf / pptx 族 / xlsx 族 / ods / odp / ppt 族
// 在 smoke 与 cli 两层均零覆盖。夹具来源（用户裁定：取 anydoc 官方测试用例）：
// tests\assets\anydoc\ 官方 fixtures（firecrawl/anydoc@261fc25，MIT，sha256 见其 README），
// 覆盖现造不出的 ppt 二进制族与 xls / xlsb 变体；pdf / md / csv / epub 仍现造（lopdf / rbook），
// legacy .doc 用仓内资产。断「活着」（退出 0 加已知针），质量与负例归 cli 集成层。

/// 单用例临时目录（按用例名分目录，互不污染；结尾 remove 兜底清理）。
fn case_dir(case: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("reader_rs_smoke_{}_{case}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("建用例目录");
    dir
}

/// 一页一行的文本层 PDF（lopdf 现造，cli.rs 同手法压缩版）。
fn make_smoke_pdf(path: &Path) -> TestResult {
    use lopdf::content::{Content, Operation};
    use lopdf::{dictionary, Object, Stream};
    let mut doc = lopdf::Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
        "Encoding" => "WinAnsiEncoding",
    });
    let resources_id = doc.add_object(dictionary! { "Font" => dictionary! { "F1" => font_id } });
    let operations = vec![
        Operation::new("BT", vec![]),
        Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), 12.into()]),
        Operation::new("Td", vec![72.into(), 720.into()]),
        Operation::new("Tj", vec![Object::string_literal("smoke pdf text")]),
        Operation::new("ET", vec![]),
    ];
    let content_id = doc.add_object(Stream::new(
        dictionary! {},
        Content { operations }.encode()?,
    ));
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Resources" => resources_id,
        "Contents" => content_id,
    });
    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        }),
    );
    let catalog_id = doc.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
    doc.trailer.set("Root", catalog_id);
    doc.save(path)?;
    Ok(())
}

/// 单章 EPUB（rbook 现造；章正文自带 h1，元数据省略防挤占单元序号）。
fn make_smoke_epub(path: &Path) -> TestResult {
    use rbook::epub::{Epub, EpubChapter};
    Epub::builder()
        .identifier("urn:reader-rs-smoke")
        .language("en")
        .chapter([
            EpubChapter::new("One").xhtml_body("<h1>One</h1><p>smoke epub text</p>".to_string())
        ])
        .write()
        .save(path)?;
    Ok(())
}

/// 全格式 extract 活体：支持面 16 扩展名族逐族一件夹具，断退出 0 加 stdout 已知针。
/// anydoc 官方件（tests\assets\anydoc\）覆盖 odt / ods / odp / pptx / ppt / xlsx / xls /
/// xlsb / rtf 九族（ppt 二进制族与 xls / xlsb 变体现造不出，官方语料直接补齐）；
/// pps / pot / pptm 等其余变体与主格式同一解析通道，主格式活体即通道活体。
#[test]
fn all_supported_formats_extract_alive() -> TestResult {
    let case = case_dir("formats");
    make_smoke_pdf(&case.join("text.pdf"))?;
    std::fs::write(case.join("note.md"), "# Smoke\n\nsmoke md text\n")?;
    std::fs::write(case.join("sheet.csv"), "a,b\nsmoke,csv\n")?;
    make_smoke_epub(&case.join("book.epub"))?;
    for (name, needle) in [
        ("text.pdf", "== page 1 =="),
        ("note.md", "== section 1 =="),
        ("sheet.csv", "| smoke | csv |"),
        ("book.epub", "smoke epub text"),
        // anydoc 官方 fixtures（针为实测提取输出的稳定字段）
        ("anydoc/odt/text.odt", "Fixture Document"),
        ("anydoc/rtf/text.rtf", "Fixture Document"),
        ("anydoc/ods/sheet.ods", "## Values"),
        ("anydoc/xlsx/sheet.xlsx", "## Values"),
        ("anydoc/xls/sheet.xls", "## Values"),
        ("anydoc/xlsb/handmade-sheet.xlsb", "| Region |"),
        ("anydoc/odp/pres.odp", "Deck Title Slide"),
        ("anydoc/pptx/pres.pptx", "Deck Title Slide"),
        ("anydoc/ppt/pres.ppt", "Deck Title Slide"),
    ] {
        let path = if name.starts_with("anydoc/") {
            repo_file(&format!("tests/assets/{name}"))
        } else {
            case.join(name)
        };
        reader()?
            .args(["extract"])
            .arg(&path)
            .assert()
            .success()
            .stdout(predicate::str::contains(needle));
    }
    let legacy_doc = repo_file("tests/assets/legacy.doc");
    if legacy_doc.is_file() {
        // 仓内资产（Word COM 现造）：活体只断非空（内容保真断言归 cli 层）
        reader()?
            .args(["extract"])
            .arg(&legacy_doc)
            .assert()
            .success()
            .stdout(predicate::str::is_empty().not());
    } else {
        eprintln!("skip: tests/assets/legacy.doc 缺失（资产另造）");
    }
    let _ = std::fs::remove_dir_all(&case);
    Ok(())
}

/// 图片文件活体（D43）：合成 PNG 走 needs_ocr 提示路径（识别质量归门控与 A/B 层）。
#[test]
fn image_file_extract_hints_alive() -> TestResult {
    let case = case_dir("image");
    use image::{ImageBuffer, Rgb};
    let img = ImageBuffer::from_fn(32u32, 16u32, |x, _| {
        if x < 16 {
            Rgb([0u8, 0, 0])
        } else {
            Rgb([255u8, 255, 255])
        }
    });
    img.save(case.join("shot.png"))?;
    reader()?
        .args(["extract"])
        .arg(case.join("shot.png"))
        .assert()
        .success()
        .stdout(predicate::str::contains("== page 1 =="))
        .stdout(predicate::str::contains("[needs_ocr: image]"));
    let _ = std::fs::remove_dir_all(&case);
    Ok(())
}
