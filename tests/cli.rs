//! CLI 集成测试：assert_cmd 整跑 `reader` 二进制。
//! 测试 PDF 由 lopdf 现造（已知文本与坐标），期望值来自写入的内容本身，独立于被测实现；
//! 行级断言锚稳定字段（`页:行:` 前缀加同行文本），不锚定 markdown 装饰前缀。

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

/// 一条已知坐标的文本行（x 左边距，y 基线；页高 792，y 大者在页面上方）。
struct TextLine {
    text: String,
    x: i64,
    y: i64,
}

fn line(text: impl Into<String>, x: i64, y: i64) -> TextLine {
    TextLine {
        text: text.into(),
        x,
        y,
    }
}

/// 造测试 PDF：每页一组文本行；空组即无文本页（扫描件形态）。
fn make_pdf_with(path: &Path, pages: &[Vec<TextLine>]) -> TestResult {
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
    for lines in pages {
        let mut operations = Vec::new();
        for TextLine { text, x, y } in lines {
            operations.push(Operation::new("BT", vec![]));
            operations.push(Operation::new(
                "Tf",
                vec![Object::Name(b"F1".to_vec()), 12.into()],
            ));
            operations.push(Operation::new("Td", vec![(*x).into(), (*y).into()]));
            operations.push(Operation::new(
                "Tj",
                vec![Object::string_literal(text.as_str())],
            ));
            operations.push(Operation::new("ET", vec![]));
        }
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

/// 造两页测试 PDF：页 1 为 PAGE1_TEXT，页 2 为 PAGE2_TEXT。
fn make_test_pdf(path: &Path) -> TestResult {
    make_pdf_with(
        path,
        &[
            vec![line(PAGE1_TEXT, 72, 720)],
            vec![line(PAGE2_TEXT, 72, 720)],
        ],
    )
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

const EPUB_CH1_TEXT: &str = "Hello EPUB Reader";
const EPUB_CH2_TEXT: &str = "Second chapter powershell search";

fn epub_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("reader_rs_cli_{}_{name}.epub", std::process::id()))
}

/// 造两章测试 EPUB：章 1 为 EPUB_CH1_TEXT，章 2 为 EPUB_CH2_TEXT。
fn make_test_epub(path: &Path) -> TestResult {
    use rbook::epub::{Epub, EpubChapter};
    Epub::builder()
        .identifier("urn:reader-rs-test")
        .title("Reader RS Test Book")
        .language("en")
        .chapter([
            EpubChapter::new("One").xhtml_body(format!("<p>{EPUB_CH1_TEXT}</p>")),
            EpubChapter::new("Two").xhtml_body(format!("<p>{EPUB_CH2_TEXT}</p>")),
        ])
        .write()
        .save(path)?;
    Ok(())
}

struct TestEpub(PathBuf);

impl TestEpub {
    fn make(name: &str) -> TestResult<Self> {
        let path = epub_path(name);
        make_test_epub(&path)?;
        Ok(Self(path))
    }
}

impl Drop for TestEpub {
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
    let out = stdout_of(reader()?.args(["search"]).arg(&pdf.0).arg("Reader"))?;
    assert_hit_line(&out, "1:1:", PAGE1_TEXT);
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
    let out = stdout_of(
        reader()?
            .args(["search"])
            .arg(&pdf.0)
            .args(["R.ST", "--regex", "-i"]),
    )?;
    assert_hit_line(&out, "2:1:", PAGE2_TEXT);
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
    let out = stdout_of(
        reader()?
            .args(["search"])
            .arg(&pdf.0)
            .args(["Reader", "-C", "1"]),
    )?;
    assert_hit_line(&out, "1:1:", PAGE1_TEXT);
    Ok(())
}

/// 取子命令 stdout 为字符串（exit 0 前提由本函数默认 success 断言）。
fn stdout_of(cmd: &mut Command) -> TestResult<String> {
    let out = cmd.assert().success().get_output().stdout.clone();
    Ok(String::from_utf8(out)?)
}

/// 断言存在以 `unit_line`（如 `1:1:`）起始的命中行，且该行含 `text`（不锚定 markdown 装饰）。
fn assert_hit_line(stdout: &str, unit_line: &str, text: &str) {
    let hit = stdout
        .lines()
        .find(|l| l.starts_with(unit_line))
        .unwrap_or_else(|| panic!("stdout 无 {unit_line} 前缀行:\n{stdout}"));
    assert!(
        hit.contains(text),
        "命中行不含期望文本 {text:?}，实际: {hit}"
    );
}

/// 两栏页阅读序 oracle：左右两栏各 22 行变宽散文（左栏 x=72 行尾不越 250，右栏 x=340，
/// 栏间沟约 90pt；两栏 y 网格错开 5pt 免误判表格）。正确阅读序为左栏读完再右栏，
/// 朴素 y 扫描会全页逐行交错。行数与沟宽按 pdf-inspector 栏检测阈值设计（items≥40、沟≥30pt）。
#[test]
fn extract_orders_two_columns_by_reading_order() -> TestResult {
    let filler = [
        "alpha bravo",
        "charlie delta echo",
        "foxtrot",
        "golf hotel india juliet",
        "kilo lima",
    ];
    let mut page = Vec::new();
    for i in 0..22 {
        let text = format!("L{:02} {}", i + 1, filler[i % filler.len()]);
        page.push(line(&text, 72, 750 - (i as i64) * 14));
    }
    for i in 0..22 {
        let text = format!("R{:02} {}", i + 1, filler[(i + 2) % filler.len()]);
        page.push(line(&text, 340, 745 - (i as i64) * 14));
    }
    let path = pdf_path("two_col");
    make_pdf_with(&path, &[page])?;
    let out = stdout_of(reader()?.args(["extract"]).arg(&path))?;
    let anchors = ["L01", "L22", "R01", "R22"];
    let positions: Vec<usize> = anchors
        .iter()
        .map(|w| out.find(w).unwrap_or_else(|| panic!("输出缺 {w}:\n{out}")))
        .collect();
    let mut sorted = positions.clone();
    sorted.sort_unstable();
    assert_eq!(
        positions, sorted,
        "两栏阅读序不符（应为左栏首行到末行后右栏首行到末行）:\n{out}"
    );
    std::fs::remove_file(&path)?;
    Ok(())
}

/// 无文本页（扫描件形态）：extract 给 needs_ocr 提示行，search 给 stderr 警示且 stdout 保持纯命中。
#[test]
fn needs_ocr_page_hinted_in_extract_and_search() -> TestResult {
    let path = pdf_path("textless");
    make_pdf_with(&path, &[vec![line(PAGE1_TEXT, 72, 720)], vec![]])?;
    let out = stdout_of(reader()?.args(["extract"]).arg(&path))?;
    assert!(
        out.contains("[needs_ocr"),
        "无文本页应给 [needs_ocr: ...] 提示行:\n{out}"
    );
    let search = reader()?
        .args(["search"])
        .arg(&path)
        .arg("Reader")
        .assert()
        .success()
        .stdout(predicate::str::contains("[needs_ocr").not())
        .get_output()
        .clone();
    assert_hit_line(
        &String::from_utf8(search.stdout.clone())?,
        "1:1:",
        PAGE1_TEXT,
    );
    let stderr = String::from_utf8(search.stderr)?;
    assert!(
        stderr.contains("needs_ocr") && stderr.contains("page 2"),
        "search 应对不可靠页 stderr 警示，实际 stderr:\n{stderr}"
    );
    std::fs::remove_file(&path)?;
    Ok(())
}

// ---------- JSON 包膜与分页裁剪（P0006） ----------

/// JSON stdout 解析为 Value（断言可解析本身就是验收点）。
fn json_stdout(cmd: &mut Command) -> TestResult<serde_json::Value> {
    let out = stdout_of(cmd)?;
    Ok(
        serde_json::from_str(out.trim())
            .map_err(|e| format!("stdout 不是合法 JSON: {e}\n{out}"))?,
    )
}

#[test]
fn search_json_envelope_wraps_hits() -> TestResult {
    let pdf = TestPdf::make("json_search")?;
    let v = json_stdout(
        reader()?
            .args(["search"])
            .arg(&pdf.0)
            .args(["Reader", "--format", "json"]),
    )?;
    assert_eq!(v["ok"], serde_json::json!(true));
    assert_eq!(v["data"]["hits"][0]["unit"], serde_json::json!(1));
    assert_eq!(v["data"]["hits"][0]["line"], serde_json::json!(1));
    assert!(
        v["data"]["hits"][0]["text"]
            .as_str()
            .is_some_and(|t| t.contains(PAGE1_TEXT)),
        "hits[0].text 应含写入文本: {v}"
    );
    assert_eq!(v["data"]["needs_ocr_units"], serde_json::json!([]));
    assert_eq!(v["meta"]["command"], serde_json::json!("search"));
    assert!(v["meta"]["duration_ms"].is_u64());
    Ok(())
}

/// 无命中：ok 仍为 true（执行成功），退出码 1（grep 语义）。
#[test]
fn search_json_no_hit_ok_true_exit_1() -> TestResult {
    let pdf = TestPdf::make("json_miss")?;
    let out = reader()?
        .args(["search"])
        .arg(&pdf.0)
        .args(["zzz-no-such", "--format", "json"])
        .assert()
        .code(1)
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_str(String::from_utf8(out)?.trim())?;
    assert_eq!(v["ok"], serde_json::json!(true));
    assert_eq!(v["data"]["hits"], serde_json::json!([]));
    Ok(())
}

/// 错误路径：stdout 出 {ok:false,error,meta}，stderr 保留人读行，退出 2。
#[test]
fn json_error_envelope_on_stdout_exit_2() -> TestResult {
    let out = reader()?
        .args([
            "search",
            "no-such-file-reader-rs.pdf",
            "x",
            "--format",
            "json",
        ])
        .assert()
        .code(2)
        .get_output()
        .clone();
    let v: serde_json::Value = serde_json::from_str(String::from_utf8(out.stdout.clone())?.trim())?;
    assert_eq!(v["ok"], serde_json::json!(false));
    assert!(v["error"].as_str().is_some_and(|e| e.contains("无法读取")));
    assert_eq!(v["meta"]["command"], serde_json::json!("search"));
    assert!(
        String::from_utf8(out.stderr)?.contains("reader:"),
        "stderr 人读行应保留"
    );
    Ok(())
}

#[test]
fn extract_json_units_carry_needs_ocr() -> TestResult {
    let path = pdf_path("json_textless");
    make_pdf_with(&path, &[vec![line(PAGE1_TEXT, 72, 720)], vec![]])?;
    let v = json_stdout(
        reader()?
            .args(["extract"])
            .arg(&path)
            .args(["--format", "json"]),
    )?;
    let units = v["data"]["units"]
        .as_array()
        .ok_or("units 应为数组")?
        .clone();
    assert_eq!(units.len(), 2);
    assert_eq!(units[0]["kind"], serde_json::json!("page"));
    assert_eq!(units[0]["needs_ocr"], serde_json::json!(null));
    assert!(
        units[1]["needs_ocr"].as_str().is_some(),
        "无文本页 needs_ocr 应为原因串: {units:?}"
    );
    assert_eq!(v["meta"]["command"], serde_json::json!("extract"));
    std::fs::remove_file(&path)?;
    Ok(())
}

/// 分页：offset/limit 取单元切片；有剩余时 meta 带 next_offset 与 cta，末页不带。
#[test]
fn extract_json_pagination_meta() -> TestResult {
    let pdf = TestPdf::make("json_page")?;
    let v = json_stdout(
        reader()?
            .args(["extract"])
            .arg(&pdf.0)
            .args(["--format", "json", "--limit", "1"]),
    )?;
    assert_eq!(v["data"]["units"].as_array().unwrap().len(), 1);
    assert_eq!(v["meta"]["next_offset"], serde_json::json!(1));
    let cta = v["meta"]["cta"].as_str().unwrap_or_default().to_string();
    assert!(
        cta.contains("--offset 1") && cta.contains("--format json"),
        "cta 应给下一页命令: {cta}"
    );
    let last = json_stdout(
        reader()?
            .args(["extract"])
            .arg(&pdf.0)
            .args(["--format", "json", "--offset", "1"]),
    )?;
    assert_eq!(last["data"]["units"].as_array().unwrap().len(), 1);
    assert_eq!(
        last["data"]["units"][0]["no"],
        serde_json::json!(2),
        "offset 1 应取第 2 页"
    );
    assert!(
        last["meta"].get("next_offset").is_none() && last["meta"].get("cta").is_none(),
        "末页不应带 next_offset/cta: {last}"
    );
    Ok(())
}

/// 文本形态同享分页：--offset 1 只出第 2 页。
#[test]
fn extract_text_mode_offset_skips_first_unit() -> TestResult {
    let pdf = TestPdf::make("text_offset")?;
    let out = stdout_of(
        reader()?
            .args(["extract"])
            .arg(&pdf.0)
            .args(["--offset", "1"]),
    )?;
    assert!(out.contains("== page 2 ==") && !out.contains("== page 1 =="));
    Ok(())
}

#[test]
fn search_json_filter_trims_data() -> TestResult {
    let pdf = TestPdf::make("json_filter")?;
    let v = json_stdout(reader()?.args(["search"]).arg(&pdf.0).args([
        "Reader",
        "--format",
        "json",
        "--filter",
        "hits[].text",
    ]))?;
    let texts = v["data"].as_array().ok_or("filter 后 data 应为数组")?;
    assert!(
        texts
            .iter()
            .any(|t| t.as_str().is_some_and(|s| s.contains(PAGE1_TEXT))),
        "裁剪后应只留命中文本: {v}"
    );
    Ok(())
}

#[test]
fn dies_filter_without_json() -> TestResult {
    let pdf = TestPdf::make("filter_no_json")?;
    reader()?
        .args(["extract"])
        .arg(&pdf.0)
        .args(["--filter", "units"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains(
            "--filter 仅在 --format json 下可用",
        ));
    Ok(())
}

/// 非法 filter 路径：错误包膜加退出 2，不静默空值。
#[test]
fn json_filter_bad_path_error_envelope() -> TestResult {
    let pdf = TestPdf::make("filter_bad")?;
    let out = reader()?
        .args(["search"])
        .arg(&pdf.0)
        .args(["Reader", "--format", "json", "--filter", "nope"])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_str(String::from_utf8(out)?.trim())?;
    assert_eq!(v["ok"], serde_json::json!(false));
    assert!(v["error"].as_str().is_some_and(|e| e.contains("无键 nope")));
    Ok(())
}

#[test]
fn dies_zero_limit() -> TestResult {
    let pdf = TestPdf::make("zero_limit")?;
    reader()?
        .args(["extract"])
        .arg(&pdf.0)
        .args(["--limit", "0"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("无效 --limit"));
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

#[test]
fn epub_search_finds_keyword_with_chapter() -> TestResult {
    let epub = TestEpub::make("epub_search")?;
    reader()?
        .args(["search"])
        .arg(&epub.0)
        .arg("EPUB")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("1:2:{EPUB_CH1_TEXT}")));
    Ok(())
}

#[test]
fn epub_extract_outputs_chapter_sections() -> TestResult {
    let epub = TestEpub::make("epub_extract")?;
    reader()?
        .args(["extract"])
        .arg(&epub.0)
        .assert()
        .success()
        .stdout(predicate::str::contains("== chapter 1 =="))
        .stdout(predicate::str::contains("== chapter 2 =="))
        .stdout(predicate::str::contains(EPUB_CH1_TEXT))
        .stdout(predicate::str::contains(EPUB_CH2_TEXT));
    Ok(())
}

#[test]
fn epub_pages_filter_selects_chapter() -> TestResult {
    let epub = TestEpub::make("epub_pages")?;
    reader()?
        .args(["extract"])
        .arg(&epub.0)
        .args(["--pages", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("== chapter 2 =="))
        .stdout(predicate::str::contains(EPUB_CH2_TEXT))
        .stdout(predicate::str::contains("== chapter 1 ==").not())
        .stdout(predicate::str::contains(EPUB_CH1_TEXT).not());
    Ok(())
}

#[test]
fn dies_unsupported_format() -> TestResult {
    let path = std::env::temp_dir().join(format!("reader_rs_cli_{}_note.txt", std::process::id()));
    std::fs::write(&path, "plain text")?;
    reader()?
        .args(["search"])
        .arg(&path)
        .arg("plain")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("不支持的格式"));
    let _ = std::fs::remove_file(&path);
    Ok(())
}
