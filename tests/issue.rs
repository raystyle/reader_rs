//! issue 域公开 API 回归（G006 回归层独立 target，契约 = ohmycloud REQ-057）：
//! `new` 提交体自动上下文与回执、`list` 过滤参数与行序、`show` 详情与 404 归 None，
//! 错误面 429/400 转人读文案。独立 target 的原因同 tests/mirror.rs：
//! `READER_ISSUES_API` 只能进程内改 env，混编 target 会串染并行子进程继承 env。

use reader_rs::issue::{file_new, list, show};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Mutex, MutexGuard, OnceLock};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

/// 本 target 内串行锁：`READER_ISSUES_API` 是进程级 env，`#[test]` 并行线程会
/// 互染端口（后设者覆盖先设者，先设者的一次性服务 `accept` 永等即挂死；
/// 2026-09-18 lan-mac 实机验收首跑实证，本机与 CI 过往绿属调度侥幸）。
/// 动 env 的测试都经 [`with_api`]，取锁后全段串行。
fn env_serial_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

/// 一次性 HTTP 服务：读完请求（头加 Content-Length 体）后回固定状态与 JSON 体，
/// 返回捕获的原始请求（断言提交体与查询串用）。
fn serve_once(
    listener: TcpListener,
    status: u16,
    reason: &str,
    body: String,
) -> TestResult<String> {
    let (mut stream, _) = listener.accept()?;
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10)))?;
    let mut seen = Vec::new();
    let mut buf = [0u8; 1024];
    while !seen.windows(4).any(|w| w == b"\r\n\r\n") {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        seen.extend_from_slice(&buf[..n]);
    }
    let head = String::from_utf8_lossy(&seen).to_string();
    let clen = head
        .lines()
        .find_map(|l| {
            let (k, v) = l.split_once(':')?;
            k.eq_ignore_ascii_case("content-length")
                .then(|| v.trim().parse::<usize>().ok())?
        })
        .unwrap_or(0);
    while seen.len() - head.find("\r\n\r\n").unwrap() - 4 < clen {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        seen.extend_from_slice(&buf[..n]);
    }
    let resp = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(resp.as_bytes())?;
    Ok(String::from_utf8_lossy(&seen).to_string())
}

fn with_api<T>(
    status: u16,
    reason: &'static str,
    body: &str,
    f: impl FnOnce() -> T,
) -> (String, T) {
    let _serial = env_serial_lock();
    let listener = TcpListener::bind("127.0.0.1:0").expect("绑定");
    let port = listener.local_addr().expect("端口").port();
    std::env::set_var("READER_ISSUES_API", format!("http://127.0.0.1:{port}"));
    let owned = body.to_string();
    let handle = std::thread::spawn(move || {
        serve_once(listener, status, reason, owned).expect("服务线程应过")
    });
    let out = f();
    std::env::remove_var("READER_ISSUES_API");
    let captured = handle.join().expect("join");
    (captured, out)
}

#[test]
fn file_new_posts_auto_context_and_parses_receipt() -> TestResult {
    let (captured, out) = with_api(
        201,
        "Created",
        r#"{"ok":true,"id":12,"url":"https://issues.ohmygh.com/i/12"}"#,
        || file_new("中文标题", "正文一行"),
    );
    assert!(
        captured.starts_with("POST /api/issues "),
        "路径与方法: {captured}"
    );
    assert!(captured.contains("application/json"), "JSON 头: {captured}");
    // send_json 序列化为 pretty 形,断言前剥白屏
    let flat: String = captured.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(flat.contains(r#""tool":"reader""#), "tool 自动: {flat}");
    assert!(flat.contains(r#""title":"中文标题""#), "标题: {flat}");
    let expect_platform = format!(
        r#""platform":"{}-{}""#,
        std::env::consts::ARCH,
        std::env::consts::OS
    );
    assert!(flat.contains(&expect_platform), "平台随构建宿主: {flat}");
    assert!(flat.contains(r#""version":""#), "版本在场: {flat}");
    let r = out.expect("应成功");
    assert_eq!(r.id, 12);
    assert_eq!(r.url, "https://issues.ohmygh.com/i/12");
    Ok(())
}

#[test]
fn file_new_maps_429_and_400() {
    let (_, out) = with_api(
        429,
        "Too Many Requests",
        r#"{"ok":false,"error":"rate"}"#,
        || file_new("t", ""),
    );
    let err = out.expect_err("429 应错");
    assert!(err.contains("限速"), "429 文案: {err}");

    let (_, out) = with_api(
        400,
        "Bad Request",
        r#"{"ok":false,"error":"title 过长"}"#,
        || file_new("t", ""),
    );
    let err = out.expect_err("400 应错");
    assert!(
        err.contains("校验不过") && err.contains("title 过长"),
        "400 文案: {err}"
    );
}

#[test]
fn file_new_rejects_bad_title_before_network() {
    // 客户端先校验：不挂服务（挂了反证没发请求）
    let out = file_new("   ", "");
    let err = out.expect_err("空标题应在客户端拒");
    assert!(err.contains("标题"), "文案: {err}");
}

#[test]
fn list_passes_filters_and_parses_rows() -> TestResult {
    let (captured, out) = with_api(
        200,
        "OK",
        r#"{"ok":true,"count":1,"issues":[{"id":12,"tool":"reader","title":"误报","version":"0.7.0","platform":"x86_64-linux","host":"h","status":"open","ip":"1.2.3.4","created_at":"2026-09-17T00:00:00Z"}]}"#,
        || list(Some("reader"), Some("open"), 5, None),
    );
    assert!(captured.starts_with("GET /api/issues"), "路径: {captured}");
    assert!(captured.contains("tool=reader"), "tool 过滤: {captured}");
    assert!(captured.contains("status=open"), "status 过滤: {captured}");
    assert!(captured.contains("limit=5"), "limit 透传: {captured}");
    assert!(
        !captured.contains("before="),
        "无游标不带 before: {captured}"
    );
    let page = out.expect("应成功");
    assert_eq!(page.rows.len(), 1);
    assert_eq!(page.rows[0].title, "误报");
    assert_eq!(
        page.rows[0].ip, "1.2.3.4",
        "ip 可读（仅服务端审计，不透出序列化）"
    );
    assert_eq!(page.has_more, None, "旧形回执无 has_more 归 None");
    Ok(())
}

/// #53 keyset 游标：before 透传进查询串；带 before 的回执 has_more 被解析。
#[test]
fn list_before_cursor_passthrough_and_has_more() -> TestResult {
    let (captured, out) = with_api(
        200,
        "OK",
        r#"{"ok":true,"count":1,"has_more":true,"issues":[{"id":50,"tool":"reader","title":"翻页","version":"0.9.0","platform":"x86_64-linux","host":"h","status":"open","ip":"1.2.3.4","created_at":"2026-09-19T00:00:00Z"}]}"#,
        || list(None, None, 3, Some(51)),
    );
    assert!(captured.contains("before=51"), "游标透传: {captured}");
    assert!(captured.contains("limit=3"), "limit 透传: {captured}");
    let page = out.expect("应成功");
    assert_eq!(page.rows[0].id, 50);
    assert_eq!(page.has_more, Some(true), "has_more 解析");
    Ok(())
}

#[test]
fn list_empty_is_ok_not_error() {
    let (_, out) = with_api(200, "OK", r#"{"ok":true,"count":0,"issues":[]}"#, || {
        list(None, None, 50, None)
    });
    assert!(out.expect("空列表应成功").rows.is_empty());
}

#[test]
fn show_found_and_missing() {
    let (_, out) = with_api(
        200,
        "OK",
        r#"{"ok":true,"issue":{"id":12,"tool":"reader","title":"误报","body":"复现步骤","version":"0.7.0","platform":"x86_64-linux","host":"h","status":"open","ip":"1.2.3.4","created_at":"2026-09-17T00:00:00Z"}}"#,
        || show(12),
    );
    let row = out.expect("应成功").expect("应找到");
    assert_eq!(row.body, "复现步骤");
    assert_eq!(row.status, "open");

    let (_, out) = with_api(
        404,
        "Not Found",
        r#"{"ok":false,"error":"issue 未找到"}"#,
        || show(999),
    );
    assert!(out.expect("404 应成功归 None").is_none());
}

/// 饱和截断提示（#52 修复随 #53 家族统一，CLI 面）：旧形回执（无 before）打满
/// 上限出双出口提示行（过滤收窄加 --before 翻页）；带 before 的回执走 has_more
/// 精确判定（true 出「更早仍有条目」行且 json 面透出，false 无提示）；旧形不满
/// 上限不出提示。
#[test]
fn list_saturation_hint_cli_face() {
    use predicates::prelude::*; // .not() 布尔扩展
    let _serial = env_serial_lock();
    let one_row = r#"{"ok":true,"count":1,"issues":[{"id":12,"tool":"reader","title":"误报","version":"0.9.0","platform":"x86_64-linux","host":"h","status":"open","ip":"1.2.3.4","created_at":"2026-09-19T00:00:00Z"}]}"#;
    // 旧形打满：limit 1 对 1 条，双出口提示（含 --before 出口）
    let listener = TcpListener::bind("127.0.0.1:0").expect("绑定");
    let port = listener.local_addr().expect("端口").port();
    std::env::set_var("READER_ISSUES_API", format!("http://127.0.0.1:{port}"));
    let owned = one_row.to_string();
    let handle =
        std::thread::spawn(move || serve_once(listener, 200, "OK", owned).expect("服务线程应过"));
    assert_cmd::Command::cargo_bin("reader")
        .expect("bin 在位")
        .args(["issue", "list", "--limit", "1"])
        .assert()
        .success()
        .stdout(predicates::str::contains("#12"))
        .stderr(predicates::str::contains("返回条数已达上限 1（可能截断）"))
        .stderr(predicates::str::contains("--before <id> 翻更早一页"));
    let _ = handle.join();
    std::env::remove_var("READER_ISSUES_API");

    // 旧形不满：limit 5 对 1 条，无提示
    let listener = TcpListener::bind("127.0.0.1:0").expect("绑定");
    let port = listener.local_addr().expect("端口").port();
    std::env::set_var("READER_ISSUES_API", format!("http://127.0.0.1:{port}"));
    let owned = one_row.to_string();
    let handle =
        std::thread::spawn(move || serve_once(listener, 200, "OK", owned).expect("服务线程应过"));
    assert_cmd::Command::cargo_bin("reader")
        .expect("bin 在位")
        .args(["issue", "list", "--limit", "5"])
        .assert()
        .success()
        .stderr(predicates::str::contains("可能截断").not());
    let _ = handle.join();
    std::env::remove_var("READER_ISSUES_API");

    // 带 before：has_more 精确判定（true 出翻页行；json 面透出 has_more）
    let more_row = r#"{"ok":true,"count":1,"has_more":true,"issues":[{"id":50,"tool":"reader","title":"翻页","version":"0.9.0","platform":"x86_64-linux","host":"h","status":"open","ip":"1.2.3.4","created_at":"2026-09-19T00:00:00Z"}]}"#;
    let listener = TcpListener::bind("127.0.0.1:0").expect("绑定");
    let port = listener.local_addr().expect("端口").port();
    std::env::set_var("READER_ISSUES_API", format!("http://127.0.0.1:{port}"));
    let owned = more_row.to_string();
    let handle =
        std::thread::spawn(move || serve_once(listener, 200, "OK", owned).expect("服务线程应过"));
    assert_cmd::Command::cargo_bin("reader")
        .expect("bin 在位")
        .args([
            "issue", "list", "--limit", "3", "--before", "51", "--format", "json",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("\"has_more\":true"))
        .stderr(predicates::str::contains("更早仍有条目（has_more）"));
    let _ = handle.join();
    std::env::remove_var("READER_ISSUES_API");
}
