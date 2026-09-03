//! CLI 集成测试：assert_cmd 整跑 `reader` 二进制。
//! 测试 PDF 由 lopdf 现造、EPUB 由 rbook builder 现造、docx 由 zip 现造最小 OOXML（P0009），
//! legacy .doc 用仓内二进制资产（Word COM 现造，CI 无 Word 不能现造）；
//! 期望值来自写入的内容本身，独立于被测实现；
//! 行级断言锚稳定字段（`单元:行:` 前缀加同行文本），不锚定 markdown 装饰前缀。

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

/// 造两章测试 EPUB：章正文自带 h1 标题（anydoc 通道按正文标题分节，head/title 元数据不进正文；
/// 书名元数据省略，避免 anydoc 渲染成首个标题节挤占单元序号）。章 1 为 EPUB_CH1_TEXT，章 2 为 EPUB_CH2_TEXT。
fn make_test_epub(path: &Path) -> TestResult {
    use rbook::epub::{Epub, EpubChapter};
    Epub::builder()
        .identifier("urn:reader-rs-test")
        .language("en")
        .chapter([
            EpubChapter::new("One").xhtml_body(format!("<h1>One</h1><p>{EPUB_CH1_TEXT}</p>")),
            EpubChapter::new("Two").xhtml_body(format!("<h1>Two</h1><p>{EPUB_CH2_TEXT}</p>")),
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

// ---------- OCR 兜底（P0014） ----------

/// 目录搜索加 --ocr 报错退出 2（边界：不做批量目录加 OCR 组合）。
#[test]
fn dies_ocr_on_directory() -> TestResult {
    let dir = TestDir::make("ocr_dir")?;
    reader()?
        .args(["search"])
        .arg(&dir.0)
        .args(["DOCX", "--ocr"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("--ocr 不适用于目录搜索"));
    Ok(())
}

/// --offline 单挂（无 --ocr）属旗标误用，两个子命令都报错退出 2。
#[test]
fn dies_offline_without_ocr() -> TestResult {
    let pdf = TestPdf::make("offline_alone")?;
    reader()?
        .args(["extract"])
        .arg(&pdf.0)
        .arg("--offline")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("--offline 须与 --ocr 同用"));
    reader()?
        .args(["search"])
        .arg(&pdf.0)
        .args(["Reader", "--offline"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("--offline 须与 --ocr 同用"));
    Ok(())
}

/// --ocr 端到端冒烟：仅当 READER_OCR_CACHE_DIR 指向三件齐备的模型缓存时跑（CI 无模型自动跳过）。
/// 无文本页 OCR 后单元结构不变（节头与 needs_ocr 标记保留），管线全链路跑通即验收。
#[test]
fn ocr_fallback_runs_when_models_cached() -> TestResult {
    let Some(cache) = std::env::var_os("READER_OCR_CACHE_DIR").map(PathBuf::from) else {
        eprintln!("skip: READER_OCR_CACHE_DIR 未设（无本地模型缓存）");
        return Ok(());
    };
    for name in ["tiny-det/model.safetensors", "tiny-rec/model.safetensors"] {
        if !cache.join(name).is_file() {
            eprintln!("skip: 模型缓存 {} 缺 {name}", cache.display());
            return Ok(());
        }
    }
    let path = pdf_path("ocr_smoke");
    make_pdf_with(&path, &[vec![line(PAGE1_TEXT, 72, 720)], vec![]])?;
    let out = stdout_of(
        reader()?
            .args(["extract", "--ocr", "--offline"])
            .arg(&path)
            .env("READER_OCR_CACHE_DIR", &cache),
    )?;
    assert!(
        out.contains("== page 2 ==") && out.contains("[needs_ocr"),
        "OCR 兜底后页 2 节头与 needs_ocr 标记应保留:\n{out}"
    );
    assert!(
        out.contains("== page 1 ==") && out.contains(PAGE1_TEXT),
        "OCR 兜底不应影响正常页文本:\n{out}"
    );
    std::fs::remove_file(&path)?;
    Ok(())
}

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
fn epub_search_finds_keyword_with_section() -> TestResult {
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

/// EPUB 章标题成为 GFM 标题行，anydoc 通道按标题分节（P0009；原章单元改节单元）。
#[test]
fn epub_extract_outputs_heading_sections() -> TestResult {
    let epub = TestEpub::make("epub_extract")?;
    reader()?
        .args(["extract"])
        .arg(&epub.0)
        .assert()
        .success()
        .stdout(predicate::str::contains("== section 1 =="))
        .stdout(predicate::str::contains("== section 2 =="))
        .stdout(predicate::str::contains(EPUB_CH1_TEXT))
        .stdout(predicate::str::contains(EPUB_CH2_TEXT));
    Ok(())
}

#[test]
fn epub_pages_filter_selects_section() -> TestResult {
    let epub = TestEpub::make("epub_pages")?;
    reader()?
        .args(["extract"])
        .arg(&epub.0)
        .args(["--pages", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("== section 2 =="))
        .stdout(predicate::str::contains(EPUB_CH2_TEXT))
        .stdout(predicate::str::contains("== section 1 ==").not())
        .stdout(predicate::str::contains(EPUB_CH1_TEXT).not());
    Ok(())
}

// ---------- anydoc 家族：docx / csv / legacy doc（P0009） ----------

const DOCX_P1_TEXT: &str = "Hello DOCX Reader";
const DOCX_P2_TEXT: &str = "Second section rust search";

const DOCX_CONTENT_TYPES: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/><Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/></Types>"#;

const DOCX_RELS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#;

const DOCX_DOC_RELS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/></Relationships>"#;

const DOCX_STYLES: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/><w:pPr><w:outlineLvl w:val="0"/></w:pPr></w:style></w:styles>"#;

const DOCX_BODY: &str = r#"<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Reader docx 夹具首节</w:t></w:r></w:p><w:p><w:r><w:t>Hello DOCX Reader</w:t></w:r></w:p><w:p><w:r><w:t>a &amp; b 中文</w:t></w:r></w:p><w:tbl><w:tr><w:tc><w:p><w:r><w:t>表头A</w:t></w:r></w:p></w:tc><w:tc><w:p><w:r><w:t>表头B</w:t></w:r></w:p></w:tc></w:tr></w:tbl><w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>第二节</w:t></w:r></w:p><w:p><w:r><w:t>Second section rust search</w:t></w:r></w:p>"#;

fn docx_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("reader_rs_cli_{}_{name}.docx", std::process::id()))
}

/// 造 docx：按给定 `w:body` 正文写入最小 OOXML（styles.xml 定义 Heading1）。
fn make_docx_with_body(path: &Path, body: &str) -> TestResult {
    use std::io::Write as _;
    let document = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>{body}</w:body></w:document>"#
    );
    let file = std::fs::File::create(path)?;
    let mut w = zip::ZipWriter::new(file);
    let opts = zip::write::SimpleFileOptions::default();
    for (name, content) in [
        ("[Content_Types].xml", DOCX_CONTENT_TYPES),
        ("_rels/.rels", DOCX_RELS),
        ("word/_rels/document.xml.rels", DOCX_DOC_RELS),
        ("word/styles.xml", DOCX_STYLES),
        ("word/document.xml", &document),
    ] {
        w.start_file(name, opts)?;
        w.write_all(content.as_bytes())?;
    }
    w.finish()?;
    Ok(())
}

/// 造测试 docx：两节标题、实体段、一行表格。
fn make_test_docx(path: &Path) -> TestResult {
    make_docx_with_body(path, DOCX_BODY)
}

struct TestDocx(PathBuf);

impl TestDocx {
    fn make(name: &str) -> TestResult<Self> {
        let path = docx_path(name);
        make_test_docx(&path)?;
        Ok(Self(path))
    }
}

impl Drop for TestDocx {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn docx_search_finds_keyword_with_section() -> TestResult {
    let docx = TestDocx::make("docx_search")?;
    let out = stdout_of(reader()?.args(["search"]).arg(&docx.0).arg("DOCX"))?;
    assert_hit_line(&out, "1:2:", DOCX_P1_TEXT);
    Ok(())
}

/// 实体保真回归（S004：office_oxide 同题丢实体）加 GFM 表格形态。
#[test]
fn docx_extract_sections_with_entity_and_table() -> TestResult {
    let docx = TestDocx::make("docx_extract")?;
    reader()?
        .args(["extract"])
        .arg(&docx.0)
        .assert()
        .success()
        .stdout(predicate::str::contains("== section 1 =="))
        .stdout(predicate::str::contains("== section 2 =="))
        .stdout(predicate::str::contains("a & b 中文"))
        .stdout(predicate::str::contains("| 表头A |"))
        .stdout(predicate::str::contains(DOCX_P1_TEXT))
        .stdout(predicate::str::contains(DOCX_P2_TEXT));
    Ok(())
}

#[test]
fn docx_pages_filter_selects_section() -> TestResult {
    let docx = TestDocx::make("docx_pages")?;
    reader()?
        .args(["extract"])
        .arg(&docx.0)
        .args(["--pages", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("== section 2 =="))
        .stdout(predicate::str::contains(DOCX_P2_TEXT))
        .stdout(predicate::str::contains("== section 1 ==").not())
        .stdout(predicate::str::contains(DOCX_P1_TEXT).not());
    Ok(())
}

/// 超长节再分片（P0011）：短节 section、451 行节切 3 个 part，单元号全局连续。
#[test]
fn overlong_section_chunks_into_parts_between_sections() -> TestResult {
    let path = docx_path("docx_mixed");
    let heading = |t: &str| {
        format!(
            r#"<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>{t}</w:t></w:r></w:p>"#
        )
    };
    let para = |t: &str| format!(r#"<w:p><w:r><w:t>{t}</w:t></w:r></w:p>"#);
    let mut body = String::new();
    body.push_str(&heading("首节"));
    body.push_str(&para(DOCX_P1_TEXT));
    body.push_str(&heading("长节"));
    for i in 0..450 {
        body.push_str(&para(&format!("bulk-{i:03}-载荷")));
    }
    body.push_str(&heading("尾节"));
    body.push_str(&para(DOCX_P2_TEXT));
    make_docx_with_body(&path, &body)?;
    let out = stdout_of(reader()?.args(["extract"]).arg(&path))?;
    for want in [
        "== section 1 ==",
        "== part 2 ==",
        "== part 3 ==",
        "== part 4 ==",
        "== section 5 ==",
    ] {
        assert!(out.contains(want), "缺 {want}:\n{out}");
    }
    assert!(out.contains("bulk-449-载荷"));
    let p2 = stdout_of(
        reader()?
            .args(["extract"])
            .arg(&path)
            .args(["--pages", "2"]),
    )?;
    assert!(
        p2.contains("bulk-000-载荷") && !p2.contains("首节"),
        "--pages 2 应只出长节首个 part:\n{p2}"
    );
    std::fs::remove_file(&path)?;
    Ok(())
}

/// CSV 无签名格式：靠扩展名命名；无标题小文档整篇一个 part（P0010 起 section 只给有标题文档）。
#[test]
fn csv_extracts_single_part() -> TestResult {
    let path = std::env::temp_dir().join(format!("reader_rs_cli_{}_note.csv", std::process::id()));
    std::fs::write(&path, "name,note\nReader,rust 搜索\n")?;
    reader()?
        .args(["extract"])
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("== part 1 =="))
        .stdout(predicate::str::contains("Reader"));
    let _ = std::fs::remove_file(&path);
    Ok(())
}

/// 无标题长文档按 200 行预算分片为 part：分页与 --pages 恢复可用（P0010）。
#[test]
fn headingless_long_csv_chunks_into_parts() -> TestResult {
    let path = std::env::temp_dir().join(format!("reader_rs_cli_{}_long.csv", std::process::id()));
    let mut csv = String::from("idx,payload\n");
    for i in 0..300 {
        csv.push_str(&format!("{i},row-{i}-数据\n"));
    }
    std::fs::write(&path, csv)?;
    let out = stdout_of(reader()?.args(["extract"]).arg(&path))?;
    assert!(
        out.contains("== part 1 ==") && out.contains("== part 2 =="),
        "302 行应分 2 片:\n{out}"
    );
    assert!(!out.contains("== part 3 =="));
    assert!(out.contains("row-299-数据"));
    let p2 = stdout_of(
        reader()?
            .args(["extract"])
            .arg(&path)
            .args(["--pages", "2"]),
    )?;
    assert!(p2.contains("row-299-数据"));
    assert!(
        !p2.contains("row-0-数据"),
        "--pages 2 不应含 part 1 行:\n{p2}"
    );
    let v = json_stdout(
        reader()?
            .args(["extract"])
            .arg(&path)
            .args(["--format", "json", "--limit", "1"]),
    )?;
    assert_eq!(v["data"]["units"][0]["kind"], serde_json::json!("part"));
    let _ = std::fs::remove_file(&path);
    Ok(())
}

/// legacy .doc（Word 97-2003 二进制）：仓内资产（Word COM 现造），中文与 & 保真。
/// 仓内路径用正斜杠分段（M005：反斜杠 join 在 CI linux/macOS 上不是分隔符）。
#[test]
fn legacy_doc_asset_extracts() -> TestResult {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/assets/legacy.doc");
    let out = stdout_of(reader()?.args(["extract"]).arg(&path))?;
    assert!(
        out.contains("alpha&beta") && out.contains("中文校验行"),
        "legacy .doc 应保真提出 authored 文本:\n{out}"
    );
    Ok(())
}

// ---------- 批量目录搜索（P0012） ----------

/// 临时目录夹具：好 pdf、好 docx、坏 pdf 各一；Drop 清理整目录。
struct TestDir(PathBuf);

impl TestDir {
    fn make(name: &str) -> TestResult<Self> {
        let dir = std::env::temp_dir().join(format!("reader_rs_cli_{}_{name}", std::process::id()));
        std::fs::create_dir_all(&dir)?;
        make_test_pdf(&dir.join("a.pdf"))?;
        make_test_docx(&dir.join("b.docx"))?;
        std::fs::write(dir.join("broken.pdf"), "not a pdf")?;
        Ok(Self(dir))
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn batch_search_hits_carry_file_paths_and_skip_broken() -> TestResult {
    let dir = TestDir::make("batch_hit")?;
    let out = reader()?
        .args(["search"])
        .arg(&dir.0)
        .arg("DOCX")
        .assert()
        .success()
        .get_output()
        .clone();
    let stdout = String::from_utf8(out.stdout.clone())?;
    assert!(
        stdout.contains("b.docx:1:2:"),
        "目录模式命中行应带路径前缀:\n{stdout}"
    );
    assert!(!stdout.contains("a.pdf:"), "不含关键词的文件不应出命中行");
    let stderr = String::from_utf8(out.stderr.clone())?;
    assert!(
        stderr.contains("跳过") && stderr.contains("broken.pdf"),
        "坏文件应 stderr 跳过后继续:\n{stderr}"
    );
    Ok(())
}

#[test]
fn batch_search_no_match_exits_1() -> TestResult {
    let dir = TestDir::make("batch_miss")?;
    reader()?
        .args(["search"])
        .arg(&dir.0)
        .arg("zzz-no-such-word")
        .assert()
        .code(1);
    Ok(())
}

#[test]
fn dies_pages_on_directory() -> TestResult {
    let dir = TestDir::make("batch_pages")?;
    reader()?
        .args(["search"])
        .arg(&dir.0)
        .args(["DOCX", "--pages", "1"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("--pages 不适用于目录"));
    Ok(())
}

/// json 形态：hits 带 file 字段，files 统计扫描与跳过；--filter 可裁出文件清单。
#[test]
fn batch_search_json_carries_file_and_counts() -> TestResult {
    let dir = TestDir::make("batch_json")?;
    let v = json_stdout(
        reader()?
            .args(["search"])
            .arg(&dir.0)
            .args(["DOCX", "--format", "json"]),
    )?;
    assert!(
        v["data"]["hits"][0]["file"]
            .as_str()
            .is_some_and(|f| f.ends_with("b.docx")),
        "hits[0].file 应为命中文件路径: {v}"
    );
    assert_eq!(v["data"]["files"]["scanned"], serde_json::json!(3));
    assert_eq!(v["data"]["files"]["skipped"], serde_json::json!(1));
    let files = json_stdout(reader()?.args(["search"]).arg(&dir.0).args([
        "DOCX",
        "--format",
        "json",
        "--filter",
        "hits[].file",
    ]))?;
    let list = files["data"].as_array().cloned().unwrap_or_default();
    assert!(
        list.iter()
            .any(|f| f.as_str().is_some_and(|s| s.ends_with("b.docx"))),
        "filter hits[].file 应给文件清单: {files}"
    );
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

// ---------- Agent 自省与发现（P0007） ----------

#[test]
fn llms_outputs_compact_index() -> TestResult {
    reader()?
        .arg("--llms")
        .assert()
        .success()
        .stdout(predicate::str::contains("reader search"))
        .stdout(predicate::str::contains("reader extract"))
        .stdout(predicate::str::contains("reader skill"))
        .stdout(predicate::str::contains("退出码"));
    Ok(())
}

#[test]
fn skill_outputs_skill_md() -> TestResult {
    // 结构锚定 2026-09-03 重构后的三节式（常用例子加输出契约加渐进引导）
    reader()?
        .arg("skill")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("---\nname: reader"))
        .stdout(predicate::str::contains("## 常用例子"))
        .stdout(predicate::str::contains("## 输出契约"))
        .stdout(predicate::str::contains("## 渐进深入"))
        .stdout(predicate::str::contains("--offset"));
    Ok(())
}

/// 漂移守卫一：clap 命令树的每个 long 旗标（含组子命令二层，如 ocr init --size、
/// self update --force）都必须出现在 --llms 与 skill 输出里（期望值来自 clap 命令树
/// 本身，独立于 curated 文本；新增参数漏登记会当场红）。
#[test]
fn introspection_texts_cover_all_clap_flags() -> TestResult {
    let cmd = reader_rs::command_tree();
    let llms = reader_rs::introspect::llms_text();
    let skill = reader_rs::introspect::skill_md();
    let mut missing = Vec::new();
    let mut check = |long: &str, scope: &str| {
        if long == "help" || long == "version" {
            return;
        }
        for (name, text) in [("--llms", &llms), ("skill", &skill)] {
            if !text.contains(&format!("--{long}")) {
                missing.push(format!("{scope} --{long} 未见于 {name}"));
            }
        }
    };
    for arg in cmd.get_arguments() {
        if let Some(long) = arg.get_long() {
            check(long, "顶层");
        }
    }
    for sub in cmd.get_subcommands() {
        for arg in sub.get_arguments() {
            if let Some(long) = arg.get_long() {
                check(long, sub.get_name());
            }
        }
        for nested in sub.get_subcommands() {
            for arg in nested.get_arguments() {
                if let Some(long) = arg.get_long() {
                    check(long, &format!("{} {}", sub.get_name(), nested.get_name()));
                }
            }
        }
    }
    assert!(missing.is_empty(), "旗标漂移:\n{}", missing.join("\n"));
    Ok(())
}

/// 漂移守卫二：仓根 SKILL.md 与 `reader skill` 运行时输出逐字节一致。
#[test]
fn committed_skill_md_matches_runtime_output() -> TestResult {
    let committed =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("SKILL.md"))?;
    assert_eq!(committed, reader_rs::introspect::skill_md());
    Ok(())
}

/// help 的 examples 节（S002 结论 7：examples 是 agent 读帮助的关键节）。
#[test]
fn search_help_contains_examples() -> TestResult {
    reader()?
        .args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("示例:"))
        .stdout(predicate::str::contains("reader search ./doc.pdf"));
    Ok(())
}

/// M007：管道读者早退（`| head` 同型）时按 Unix 惯例死于 SIGPIPE（信号 13），
/// 不 panic 不喷 stderr。大输出夹具必须超过管道缓冲（Linux 默认 64KB），
/// 保证关闭管道前 writer 仍在写、死因确定是 SIGPIPE 而非自然写完。
#[cfg(unix)]
#[test]
fn sigpipe_on_closed_stdout_kills_quietly() -> TestResult {
    use std::io::Read;
    use std::os::unix::process::ExitStatusExt;
    use std::process::Stdio;

    let path = pdf_path("sigpipe");
    let filler = "filler ".repeat(40); // 约 280B/页 × 400 页 ≈ 110KB
    let pages: Vec<Vec<TextLine>> = (0..400).map(|_| vec![line(&filler, 72, 720)]).collect();
    make_pdf_with(&path, &pages)?;

    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_reader"))
        .args(["extract", path.to_str().expect("临时路径应可转 str")])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdout = child.stdout.take().expect("piped stdout");
    let mut first = [0u8; 16];
    stdout.read_exact(&mut first)?; // 读首块确认子进程已开写
    drop(stdout); // 模拟 head 早退关闭管道
    let status = child.wait()?;
    let _ = std::fs::remove_file(&path);
    assert_eq!(
        status.signal(),
        Some(13),
        "应死于 SIGPIPE（信号 13），实际 {status:?}"
    );
    Ok(())
}

// ---------- self update（P0015） ----------

/// self update 帮助面：`--help` 出 --force；裸 `self` 缺子命令按 clap 惯例退出 2。
/// 真更新路径不进集成测试（下载加替换自身不可在 CI 跑），端到端实测记 P0015 验收。
#[test]
fn self_update_help_surface() -> TestResult {
    reader()?
        .args(["self", "update", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--force"));
    reader()?.args(["self"]).assert().code(2);
    Ok(())
}

// ---------- ocr 子命令组（D42：init / doctor / switch） ----------

/// 真下载路径不进集成测试（三通道出网不可在 CI 跑）：init 以 `--offline`
/// 只校验形态作零网络代理；镜像真实下载的端到端验收记 D42 大陆侧回执。
/// 各用例缓存目录一律 `<临时>/<用例名>/models`——档位设置文件落在兄弟位
/// `<临时>/<用例名>/model-size`，与开发者真机设置互不污染（hermetic 不变量）。
fn ocr_case_dir(case: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("reader-ocr-{case}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("models")).expect("建用例目录");
    dir
}

/// ocr 组帮助面：init 出 --size 与 --offline；裸 `ocr` 缺子命令按 clap 惯例退出 2。
#[test]
fn ocr_subcommands_help_surface() -> TestResult {
    reader()?
        .args(["ocr", "init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--size"))
        .stdout(predicate::str::contains("--offline"));
    reader()?
        .args(["ocr", "doctor", "--help"])
        .assert()
        .success();
    reader()?
        .args(["ocr", "switch", "--help"])
        .assert()
        .success();
    reader()?.args(["ocr"]).assert().code(2);
    Ok(())
}

/// doctor 对空缓存目录：两档四包全 missing、镜像探活不可达（信息行）、退出 1；
/// 全程零网络（镜像指向本机 discard 端口，连接拒绝即失败）。
#[test]
fn ocr_doctor_reports_missing_on_empty_cache() -> TestResult {
    let case = ocr_case_dir("doctor-empty");
    reader()?
        .args(["ocr", "doctor"])
        .env("READER_OCR_CACHE_DIR", case.join("models"))
        .env("READER_MIRROR", "http://127.0.0.1:9")
        .assert()
        .code(1)
        .stdout(predicate::str::contains("ocr_doctor: cache"))
        .stdout(predicate::str::contains("ocr_doctor: settings"))
        .stdout(predicate::str::contains("ocr_doctor: size tiny（默认）"))
        .stdout(predicate::str::contains("ocr_doctor: tiny-det missing"))
        .stdout(predicate::str::contains("ocr_doctor: small-rec missing"))
        .stdout(predicate::str::contains("ocr_doctor: mirror unreachable"))
        .stdout(predicate::str::contains("ocr_doctor: verdict failed"));
    let _ = std::fs::remove_dir_all(&case);
    Ok(())
}

/// doctor 对伪缓存（垃圾字节）报 corrupt 而非 missing：字节与钉死值不符即损件。
#[test]
fn ocr_doctor_flags_corrupt_files() -> TestResult {
    let case = ocr_case_dir("doctor-corrupt");
    let pkg = case.join("models").join("tiny-det");
    std::fs::create_dir_all(&pkg).expect("建包目录");
    for name in [
        "model.safetensors",
        "config.json",
        "inference.yml",
        "preprocessor_config.json",
    ] {
        std::fs::write(pkg.join(name), b"garbage bytes").expect("写伪件");
    }
    reader()?
        .args(["ocr", "doctor"])
        .env("READER_OCR_CACHE_DIR", case.join("models"))
        .env("READER_MIRROR", "http://127.0.0.1:9")
        .assert()
        .code(1)
        .stdout(predicate::str::contains(
            "ocr_doctor: tiny-det corrupt（model.safetensors 校验不符）",
        ))
        .stdout(predicate::str::contains("ocr_doctor: verdict failed"));
    let _ = std::fs::remove_dir_all(&case);
    Ok(())
}

/// switch 写档位设置文件并即时生效（doctor 反映）；env 优先于设置且 switch 时告警；
/// 非法档位按旗标误用退出 2。
#[test]
fn ocr_switch_persists_setting_and_env_wins() -> TestResult {
    let case = ocr_case_dir("switch");
    let cache = case.join("models");
    let settings = case.join("model-size");
    reader()?
        .args(["ocr", "switch", "small"])
        .env("READER_OCR_CACHE_DIR", &cache)
        .assert()
        .success()
        .stdout(predicate::str::contains("ocr_switch: tiny -> small"))
        .stdout(predicate::str::contains("ocr_switch: small 未就位"));
    let saved = std::fs::read_to_string(&settings).expect("设置文件应已写入");
    assert!(saved.trim() == "small", "设置文件内容应为 small: {saved}");
    // doctor 反映新档位：settings small、size 来源设置、small 未就位 verdict failed
    reader()?
        .args(["ocr", "doctor"])
        .env("READER_OCR_CACHE_DIR", &cache)
        .env("READER_MIRROR", "http://127.0.0.1:9")
        .assert()
        .code(1)
        .stdout(predicate::str::contains("ocr_doctor: size small（设置）"))
        .stdout(predicate::str::contains("ocr_doctor: verdict failed"));
    // env 优先：READER_OCR_MODEL_SIZE 覆盖设置文件
    reader()?
        .args(["ocr", "doctor"])
        .env("READER_OCR_CACHE_DIR", &cache)
        .env("READER_OCR_MODEL_SIZE", "tiny")
        .env("READER_MIRROR", "http://127.0.0.1:9")
        .assert()
        .stdout(predicate::str::contains("ocr_doctor: size tiny（env）"));
    // env 导出时 switch 走 stderr 告警（stdout 契约行照出）
    reader()?
        .args(["ocr", "switch", "tiny"])
        .env("READER_OCR_CACHE_DIR", &cache)
        .env("READER_OCR_MODEL_SIZE", "small")
        .assert()
        .success()
        .stdout(predicate::str::contains("ocr_switch: small -> tiny"))
        .stderr(predicate::str::contains("环境变量优先"));
    // 非法档位
    reader()?
        .args(["ocr", "switch", "medium"])
        .env("READER_OCR_CACHE_DIR", &cache)
        .assert()
        .code(2)
        .stderr(predicate::str::contains("tiny / small"));
    let _ = std::fs::remove_dir_all(&case);
    Ok(())
}

/// `ocr init --offline` 空缓存：只校验不下载，件无效即失败退出 2（零网络可测形态）。
#[test]
fn dies_ocr_init_offline_with_empty_cache() -> TestResult {
    let case = ocr_case_dir("init-offline");
    reader()?
        .args(["ocr", "init", "--offline"])
        .env("READER_OCR_CACHE_DIR", case.join("models"))
        .env("READER_MIRROR", "http://127.0.0.1:9")
        .assert()
        .code(2)
        .stdout(predicate::str::contains("ocr_init: size tiny"))
        .stdout(predicate::str::contains(
            "ocr_init: tiny-det model.safetensors failed（--offline 禁下载且缓存件无效）",
        ))
        .stdout(predicate::str::contains("ocr_init: verdict failed"));
    let _ = std::fs::remove_dir_all(&case);
    Ok(())
}

/// `ocr init --size` 非法值按旗标误用退出 2。
#[test]
fn dies_ocr_init_rejects_unknown_size() -> TestResult {
    let case = ocr_case_dir("init-bad-size");
    reader()?
        .args(["ocr", "init", "--size", "medium"])
        .env("READER_OCR_CACHE_DIR", case.join("models"))
        .assert()
        .code(2)
        .stderr(predicate::str::contains("tiny / small"));
    let _ = std::fs::remove_dir_all(&case);
    Ok(())
}

/// doctor 对真缓存（门控：READER_OCR_CACHE_DIR 指向完整 tiny 缓存才跑，CI 自动跳过）
/// 且镜像不可达：本地健康不依赖镜像（内网机离线可用即健康），verdict ok 退出 0。
#[test]
fn ocr_doctor_healthy_with_real_cache_and_dead_mirror() -> TestResult {
    let Some(cache) = std::env::var_os("READER_OCR_CACHE_DIR").map(PathBuf::from) else {
        eprintln!("skip: READER_OCR_CACHE_DIR 未设（无本地模型缓存）");
        return Ok(());
    };
    for name in ["tiny-det/model.safetensors", "tiny-rec/model.safetensors"] {
        if !cache.join(name).is_file() {
            eprintln!("skip: 缓存缺 {name}");
            return Ok(());
        }
    }
    // 门控机不设 READER_OCR_MODEL_SIZE 时若本机档位非 tiny 会误判，显式钉 tiny
    reader()?
        .args(["ocr", "doctor"])
        .env("READER_OCR_CACHE_DIR", &cache)
        .env("READER_OCR_MODEL_SIZE", "tiny")
        .env("READER_MIRROR", "http://127.0.0.1:9")
        .assert()
        .code(0)
        .stdout(predicate::str::contains("ocr_doctor: tiny-det ok"))
        .stdout(predicate::str::contains("ocr_doctor: tiny-rec ok"))
        .stdout(predicate::str::contains("ocr_doctor: mirror unreachable"))
        .stdout(predicate::str::contains("ocr_doctor: verdict ok"));
    Ok(())
}

// ---------- markdown 支持与 mq 结构化提取（P0016） ----------

const MD_DOC: &str = "\
# 指南标题

前言段落含 keyword-alpha。

## 第一节 工具

- item one
- item keyword-beta

```rust
fn main() {}
```

## 第二节 流程

表格行 | 列
";

fn md_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("reader_rs_cli_{}_{name}.md", std::process::id()))
}

struct TestMd(PathBuf);

impl TestMd {
    fn make(name: &str) -> TestResult<Self> {
        let path = md_path(name);
        std::fs::write(&path, MD_DOC)?;
        Ok(Self(path))
    }
}

impl Drop for TestMd {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// .md 进格式面：按标题分节（3 节），搜索命中带节号与行号。
#[test]
fn markdown_extract_sections_and_search() -> TestResult {
    let md = TestMd::make("basic")?;
    let out = stdout_of(reader()?.args(["extract"]).arg(&md.0))?;
    assert!(
        out.contains("== section 1 ==") && out.contains("== section 3 =="),
        "md 应按顶层标题分 3 节:\n{out}"
    );
    reader()?
        .args(["search"])
        .arg(&md.0)
        .arg("keyword-beta")
        .assert()
        .success()
        .stdout(predicate::str::contains("2:3:"));
    Ok(())
}

/// 无标题长 md 按 200 行预算切 part（与无标题 anydoc 文档同口径，P0010 继承）。
#[test]
fn markdown_headingless_chunks_into_parts() -> TestResult {
    let path = md_path("parts");
    let body = (0..250)
        .map(|i| format!("row-{i:03}"))
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&path, body)?;
    let out = stdout_of(reader()?.args(["extract"]).arg(&path))?;
    assert!(
        out.contains("== part 1 ==") && out.contains("== part 2 =="),
        "250 行无标题 md 应切 2 个 part:\n{}",
        &out[..out.len().min(200)]
    );
    std::fs::remove_file(&path)?;
    Ok(())
}

/// query：mq 表达式结构化提取——`.h2` 出二级标题，select 管道筛内容。
#[test]
fn query_extracts_structures() -> TestResult {
    let md = TestMd::make("query")?;
    let out = stdout_of(reader()?.args(["query"]).arg(&md.0).arg(".h2"))?;
    assert!(
        out.contains("## 第一节 工具") && out.contains("## 第二节 流程"),
        ".h2 应出两个二级标题:\n{out}"
    );
    reader()?
        .args(["query"])
        .arg(&md.0)
        .arg(".[] | select(contains(\"keyword-beta\"))")
        .assert()
        .success()
        .stdout(predicate::str::contains("keyword-beta"));
    Ok(())
}

/// query 无命中退出 1；坏表达式退出 2；json 形态 results 加 count。
#[test]
fn query_exit_codes_and_json() -> TestResult {
    let md = TestMd::make("query2")?;
    reader()?
        .args(["query"])
        .arg(&md.0)
        .arg(".h5")
        .assert()
        .code(1);
    reader()?
        .args(["query"])
        .arg(&md.0)
        .arg("bad syntax here")
        .assert()
        .code(2);
    let v = json_stdout(
        reader()?
            .args(["query"])
            .arg(&md.0)
            .args([".h2", "--format", "json"]),
    )?;
    assert_eq!(v["data"]["count"], serde_json::json!(2));
    assert!(
        v["data"]["results"][0]
            .as_str()
            .is_some_and(|s| s.contains("第一节")),
        "results[0] 应为首个二级标题: {v}"
    );
    Ok(())
}

/// query 拒绝目录输入。
#[test]
fn dies_query_on_directory() -> TestResult {
    let dir = TestDir::make("query_dir")?;
    reader()?
        .args(["query"])
        .arg(&dir.0)
        .arg(".h")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("query 不支持目录"));
    Ok(())
}
