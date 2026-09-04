//! 语料快照回归层（G006 回归族独立 target）：anydoc 官方 corpus 逐件经 reader 真 CLI
//! 出全量快照与负例断言，守「anydoc 升级或分派层改动引起的输出漂移」（D44 第 3 轮，
//! 用户裁定扩充：测试文档与对应用例从每族一件活体扩为全量语料逐件快照）。
//! 期望行为编码同上游文件名 `--<outcome>` 后缀：`errors` 断退出 2；`recovers` / `skips`
//! 断退出 0（容错出部分内容）。abuse 族不进快照（资源滥用件的语义是「拒得快不挂不死」，
//! 大输出无基线价值）：断退出码属 {0, 2} 且无 panic 形迹。
//! 纪律：快照首跑 `.snap.new` 人工审后收录；漂移必须能回指有意变更（anydoc 版本或 P 编号）。
//! `cargo test --test corpus` 单独调度；语料来源与 sha256 见 `tests\assets\anydoc\README.md`。

use assert_cmd::Command;
use std::path::{Path, PathBuf};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn reader() -> TestResult<Command> {
    Ok(Command::cargo_bin("reader")?)
}

fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/assets/anydoc")
        .join(rel)
}

/// 一次 extract 的可快照面：退出码加 stdout 加 stderr（三者均确定性）。
/// stderr 里的夹具路径整串归一为 `<fixture>`（只换仓根前缀会残留 OS 分隔符差异，
/// 跨机快照必炸：Windows `\` 对 Unix `/`，CI 首跑实证）。
fn extract_snapshot(rel: &str) -> TestResult<(i32, String)> {
    let path = fixture(rel);
    let out = reader()?.args(["extract"]).arg(&path).assert();
    let output = out.get_output();
    let code = output.status.code().unwrap_or(-1);
    let text = format!(
        "exit: {code}\n--- stdout ---\n{}--- stderr ---\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
    .replace(&path.display().to_string(), "<fixture>");
    Ok((code, text))
}

/// 正例与容错例（期望退出 0）：全量快照。
macro_rules! corpus_ok {
    ($($fn_name:ident => $rel:expr),* $(,)?) => {$(
        #[test]
        fn $fn_name() -> TestResult {
            let (code, snap) = extract_snapshot($rel)?;
            assert_eq!(code, 0, "应退出 0（正例或容错例）:\n{snap}");
            insta::assert_snapshot!(snap);
            Ok(())
        }
    )*};
}

/// 负例（期望退出 2，上游 `--errors` 标签）：快照含 stderr 错误面。
macro_rules! corpus_errors {
    ($($fn_name:ident => $rel:expr),* $(,)?) => {$(
        #[test]
        fn $fn_name() -> TestResult {
            let (code, snap) = extract_snapshot($rel)?;
            assert_eq!(code, 2, "应退出 2（errors 标签）:\n{snap}");
            insta::assert_snapshot!(snap);
            Ok(())
        }
    )*};
}

corpus_ok! {
    csv_handmade_quoted => "csv/handmade-quoted.csv",
    csv_handmade_semicolon => "csv/handmade-semicolon.csv",
    csv_handmade_utf16 => "csv/handmade-utf16.csv",
    csv_sheet => "csv/sheet.csv",
    doc_handmade_blockstyle => "doc/handmade-blockstyle.doc",
    doc_handmade_cyrillic => "doc/handmade-cyrillic.doc",
    doc_handmade_shiftjis => "doc/handmade-shiftjis.doc",
    doc_text => "doc/text.doc",
    docx_handmade_altpath => "docx/handmade-altpath.docx",
    docx_handmade_blockstyle => "docx/handmade-blockstyle.docx",
    docx_handmade_manyrefs => "docx/handmade-manyrefs.docx",
    docx_handmade_math => "docx/handmade-math.docx",
    docx_handmade_numbering => "docx/handmade-numbering.docx",
    docx_handmade_ole => "docx/handmade-ole.docx",
    docx_handmade_outline => "docx/handmade-outline.docx",
    docx_handmade_rich => "docx/handmade-rich.docx",
    docx_handmade_strict => "docx/handmade-strict.docx",
    docx_handmade_tables => "docx/handmade-tables.docx",
    docx_text => "docx/text.docx",
    epub_book => "epub/book.epub",
    epub_handmade_css_links => "epub/handmade-css-links.epub",
    epub_handmade_features => "epub/handmade-features.epub",
    epub_handmade_math => "epub/handmade-math.epub",
    ods_handmade_durations => "ods/handmade-durations.ods",
    ods_handmade_gaps => "ods/handmade-gaps.ods",
    ods_sheet => "ods/sheet.ods",
    odp_pres => "odp/pres.odp",
    odt_handmade_blockstyle => "odt/handmade-blockstyle.odt",
    odt_handmade_defaults => "odt/handmade-defaults.odt",
    odt_handmade_lists => "odt/handmade-lists.odt",
    odt_handmade_manifestcomment => "odt/handmade-manifestcomment.odt",
    odt_handmade_math => "odt/handmade-math.odt",
    odt_text => "odt/text.odt",
    ppt_handmade_multimaster => "ppt/handmade-multimaster.ppt",
    ppt_handmade_sparsenotes => "ppt/handmade-sparsenotes.ppt",
    ppt_pres => "ppt/pres.ppt",
    pptx_handmade_altpath => "pptx/handmade-altpath.pptx",
    pptx_handmade_inherit => "pptx/handmade-inherit.pptx",
    pptx_handmade_links => "pptx/handmade-links.pptx",
    pptx_handmade_math => "pptx/handmade-math.pptx",
    pptx_handmade_order => "pptx/handmade-order.pptx",
    pptx_handmade_strict => "pptx/handmade-strict.pptx",
    pptx_pres => "pptx/pres.pptx",
    rtf_handmade_bin => "rtf/handmade-bin.rtf",
    rtf_handmade_blockstyle => "rtf/handmade-blockstyle.rtf",
    rtf_handmade_cocoa => "rtf/handmade-cocoa.rtf",
    rtf_handmade_math => "rtf/handmade-math.rtf",
    rtf_handmade_merge => "rtf/handmade-merge.rtf",
    rtf_text => "rtf/text.rtf",
    xls_sheet => "xls/sheet.xls",
    xlsb_handmade_sheet => "xlsb/handmade-sheet.xlsb",
    xlsx_handmade_merged => "xlsx/handmade-merged.xlsx",
    xlsx_sheet => "xlsx/sheet.xlsx",
    // malformed 容错例（上游 --recovers / --skips 标签，退出 0 出部分内容）
    malformed_brokenpersist_recovers => "malformed/brokenpersist--recovers.ppt",
    malformed_corrupt_styles_skips => "malformed/corrupt-styles--skips.docx",
    malformed_mismatched_recovers => "malformed/mismatched--recovers.docx",
    malformed_missing_styles_skips => "malformed/missing-styles--skips.docx",
    malformed_unbalanced_recovers => "malformed/unbalanced--recovers.rtf",
    malformed_unclosed_recovers => "malformed/unclosed--recovers.docx",
}

corpus_errors! {
    malformed_empty_errors => "malformed/empty--errors.docx",
    malformed_encrypted_errors => "malformed/encrypted--errors.odt",
    malformed_truncated_errors_doc => "malformed/truncated--errors.doc",
    malformed_truncated_errors_docx => "malformed/truncated--errors.docx",
}

/// abuse 族（资源滥用件）：语义是「拒得快、不挂不死、不 panic」——断退出码属
/// {0, 2} 且输出无 panic 形迹；不做快照（大输出无基线价值）。计时归 perf 报告不进断言。
#[test]
fn abuse_fixtures_reject_fast_without_panic() -> TestResult {
    for rel in [
        "abuse/deepnest--errors.ppt",
        "abuse/deepxml--errors.docx",
        "abuse/emptyrowrepeat--errors.ods",
        "abuse/hugerepeat--errors.ods",
        "abuse/hugespan--errors.ods",
        "abuse/hugespan--errors.pptx",
        "abuse/imagebomb--errors.docx",
        "abuse/zipbomb--errors.docx",
    ] {
        let (code, snap) = extract_snapshot(rel)?;
        assert!(
            code == 0 || code == 2,
            "{rel} 退出码应属 {{0,2}}（拒绝或容错），实际 {code}:\n{snap}"
        );
        assert!(!snap.contains("panicked"), "{rel} 不应 panic:\n{snap}");
    }
    Ok(())
}
