//! ledger 域回归（G006 回归层独立 target，契约 = ohmycloud REQ-063）：
//! 签名道（五头、签名基逐字、Ed25519 可回核）、issue 与 artifact 请求形状、
//! 家族翻页面（limit 缺省 100 加 before 游标加 has_more 加饱和提示）。
//! 独立 target 原因同旧 issue 面：`READER_LEDGER` 只能进程内改 env，混编
//! target 会串染并行子进程继承 env；target 内串行锁防并行线程互染端口。

use base64::Engine;
use ed25519_dalek::Verifier;
use predicates::prelude::*;
use reader_rs::ledger;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Mutex, MutexGuard, OnceLock};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

/// 本 target 内串行锁（env 与端口同源坑，同旧 issue 面教训）。
fn env_serial_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

/// 一次性 HTTP 服务（字节面捕获）：回固定状态与 JSON 体，返回原始请求字节
/// （头加体，验签回核须逐字节）。
fn serve_once_bytes(
    listener: TcpListener,
    status: u16,
    reason: &str,
    body: &str,
) -> TestResult<Vec<u8>> {
    let (mut stream, _) = listener.accept()?;
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10)))?;
    let mut seen = Vec::new();
    let mut buf = [0u8; 4096];
    // 读到头完
    while !seen.windows(4).any(|w| w == b"\r\n\r\n") {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        seen.extend_from_slice(&buf[..n]);
    }
    let head_end = seen
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or("请求无头体分隔")?;
    let head = String::from_utf8_lossy(&seen[..head_end]).to_string();
    let clen = head
        .lines()
        .find_map(|l| {
            let (k, v) = l.split_once(':')?;
            k.eq_ignore_ascii_case("content-length")
                .then(|| v.trim().parse::<usize>().ok())?
        })
        .unwrap_or(0);
    while seen.len() - head_end - 4 < clen {
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
    Ok(seen)
}

/// 请求头集合（键小写）。
type Headers = Vec<(String, String)>;

/// 从捕获的原始请求字节解析：请求行 path、指定头、体字节。
fn parse_request(raw: &[u8]) -> TestResult<(String, Headers, Vec<u8>)> {
    let split = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or("无分隔")?;
    let head = String::from_utf8_lossy(&raw[..split]).to_string();
    let mut lines = head.lines();
    let request_line = lines.next().unwrap_or("").to_string();
    let path = request_line.split(' ').nth(1).unwrap_or("").to_string();
    let headers = lines
        .filter_map(|l| {
            let (k, v) = l.split_once(':')?;
            Some((k.trim().to_ascii_lowercase(), v.trim().to_string()))
        })
        .collect();
    Ok((path, headers, raw[split + 4..].to_vec()))
}

fn header<'a>(headers: &'a [(String, String)], name: &str) -> &'a str {
    headers
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
        .unwrap_or("")
}

/// 测试密钥（与生产密钥无关；仅验签名道自洽）。
fn test_seed_b64() -> String {
    let seed = [9u8; 32];
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(seed)
}

/// 签名道全链：五头在、时间戳在窗、签名基可由捕获请求逐字重建并用测试公钥回核。
#[test]
fn signed_post_five_headers_and_verifiable_signature() -> TestResult {
    let _serial = env_serial_lock();
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    std::env::set_var("READER_LEDGER", format!("http://127.0.0.1:{port}"));
    std::env::set_var("READER_LEDGER_KEY", test_seed_b64());
    let handle = std::thread::spawn(move || {
        serve_once_bytes(
            listener,
            201,
            "Created",
            r#"{"ok":true,"issue":7,"event":{"seq":11}}"#,
        )
        .expect("服务线程应过")
    });
    let opened = ledger::issue_new("sig round trip", "bug", "accept", "note").expect("开单应过");
    let raw = handle.join().expect("join");
    std::env::remove_var("READER_LEDGER");
    std::env::remove_var("READER_LEDGER_KEY");

    assert_eq!((opened.issue, opened.seq), (7, 11));
    let (path, headers, body) = parse_request(&raw)?;
    assert_eq!(path, format!("/repos/{}/issues", ledger::REPO_ID));
    for h in [
        "idempotency-key",
        "x-key-id",
        "x-timestamp",
        "x-nonce",
        "x-signature",
    ] {
        assert!(!header(&headers, h).is_empty(), "五头缺 {h}");
    }
    assert_eq!(header(&headers, "x-key-id"), ledger::KEY_ID);
    let ts: u64 = header(&headers, "x-timestamp").parse()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    assert!(
        now.abs_diff(ts) <= 60,
        "时间戳须在正负 60 秒窗: {ts} vs {now}"
    );
    // 签名基逐字重建并回核（与服务端 verifyEd25519 同构）
    let base = ledger::signature_base(
        &path,
        ts,
        header(&headers, "x-nonce"),
        header(&headers, "idempotency-key"),
        &String::from_utf8_lossy(&body),
    );
    let sig_bytes =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(header(&headers, "x-signature"))?;
    let sig = ed25519_dalek::Signature::from_slice(&sig_bytes)?;
    // 公钥从测试 seed 派生（seed 字节本身不是公钥）
    let signing = ed25519_dalek::SigningKey::from_bytes(&[9u8; 32]);
    let verifying = ed25519_dalek::VerifyingKey::from(&signing);
    if verifying.verify(base.as_bytes(), &sig).is_err() {
        eprintln!("BASE=[{base}]");
        eprintln!("BODY=[{}]", String::from_utf8_lossy(&body));
        eprintln!("PATH=[{path}]");
    }
    verifying
        .verify(base.as_bytes(), &sig)
        .expect("签名应可回核");
    // 体即签名基所覆内容（同键不同内容分道的根）
    assert_eq!(
        format!("{:x}", Sha256::digest(&body)),
        base.split('\n').next_back().unwrap_or("")
    );
    Ok(())
}

/// 家族翻页面：more=1 恒带、before 透传、has_more 解析、饱和提示 CLI 面。
#[test]
fn issue_list_family_pagination_and_hints() -> TestResult {
    let _serial = env_serial_lock();
    let page = r#"{"ok":true,"count":2,"has_more":true,"issues":[
        {"issue_n":5,"title":"t5","status":"open","kind":"bug","assignee":null,"hasResult":false},
        {"issue_n":4,"title":"t4","status":"done","kind":"improvement","assignee":"ray","hasResult":true}]}"#;
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    std::env::set_var("READER_LEDGER", format!("http://127.0.0.1:{port}"));
    let owned = page.to_string();
    let handle = std::thread::spawn(move || {
        serve_once_bytes(listener, 200, "OK", &owned).expect("服务线程应过")
    });
    let out = ledger::issue_list(3, Some(6)).expect("列表应过");
    let raw = handle.join().expect("join");
    std::env::remove_var("READER_LEDGER");

    let (path_q, _, _) = parse_request(&raw)?;
    assert!(
        path_q.starts_with(&format!("/repos/{}/issues?", ledger::REPO_ID)),
        "路径面: {path_q}"
    );
    assert!(path_q.contains("limit=3"), "limit 透传: {path_q}");
    assert!(path_q.contains("before=6"), "before 透传: {path_q}");
    assert!(path_q.contains("more=1"), "more=1 恒带: {path_q}");
    assert_eq!(out.rows.len(), 2);
    assert_eq!(out.rows[0].issue_n, 5);
    assert_eq!(out.rows[1].kind, "improvement");
    assert!(out.rows[1].has_result);
    assert_eq!(out.has_more, Some(true));

    // CLI 面：has_more 精确判定出翻页提示行
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    std::env::set_var("READER_LEDGER", format!("http://127.0.0.1:{port}"));
    let owned = page.to_string();
    let handle = std::thread::spawn(move || {
        serve_once_bytes(listener, 200, "OK", &owned).expect("服务线程应过")
    });
    assert_cmd::Command::cargo_bin("reader")?
        .args(["issue", "list", "--limit", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#5 open bug - t5"))
        .stdout(predicate::str::contains("#4 done improvement ray t4"))
        .stderr(predicate::str::contains("更早仍有条目（has_more）"))
        .stderr(predicate::str::contains("--before <id>"));
    let _ = handle.join();
    std::env::remove_var("READER_LEDGER");
    Ok(())
}

/// issue 面请求形状与错误道：kind 校验客户端先拒；401 归因文案。
#[test]
fn issue_faces_validate_and_attribute_errors() {
    let _serial = env_serial_lock();
    // kind 客户端先拒（不挂服务）
    let err = ledger::issue_new("t", "feature", "a", "").unwrap_err();
    assert!(err.contains("kind 仅"), "{err}");
    // title 校验
    let err = ledger::issue_new("   ", "bug", "a", "").unwrap_err();
    assert!(err.contains("title"), "{err}");

    // 401 归因：本地服务回 401
    let listener = TcpListener::bind("127.0.0.1:0").expect("绑定");
    let port = listener.local_addr().expect("端口").port();
    std::env::set_var("READER_LEDGER", format!("http://127.0.0.1:{port}"));
    std::env::set_var("READER_LEDGER_KEY", test_seed_b64());
    let handle = std::thread::spawn(move || {
        serve_once_bytes(
            listener,
            401,
            "Unauthorized",
            r#"{"ok":false,"error":"X-Key-Id 不在册、已吊销或不绑定本仓"}"#,
        )
        .expect("服务线程应过")
    });
    let err = ledger::issue_new("t", "bug", "a", "").unwrap_err();
    let _ = handle.join();
    std::env::remove_var("READER_LEDGER");
    std::env::remove_var("READER_LEDGER_KEY");
    assert!(err.contains("验签或密钥不过"), "{err}");
    assert!(err.contains("不在册"), "{err}");
}

/// artifact 面：publish 请求形状（kind/digest 校验加 deps 透传）与回执解析。
#[test]
fn artifact_publish_shape_and_validation() -> TestResult {
    let _serial = env_serial_lock();
    let digest = format!("sha256:{}", "b".repeat(64));
    // 客户端先校验（不挂服务）
    assert!(ledger::artifact_publish("n", "research", "sha256:abc", None, None, &[], "").is_err());
    assert!(ledger::artifact_publish("n", "diary", &digest, None, None, &[], "").is_err());

    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    std::env::set_var("READER_LEDGER", format!("http://127.0.0.1:{port}"));
    std::env::set_var("READER_LEDGER_KEY", test_seed_b64());
    let handle = std::thread::spawn(move || {
        serve_once_bytes(
            listener,
            201,
            "Created",
            r#"{"ok":true,"artifact_id":"art-1","digest":"d","event":{"seq":42}}"#,
        )
        .expect("服务线程应过")
    });
    let deps = vec!["art-0".to_string()];
    let out = ledger::artifact_publish(
        "S010 定界研究",
        "research",
        &digest,
        Some("0.10.0"),
        Some("v0.9.1..v0.10.0"),
        &deps,
        "研究落地产物",
    )
    .expect("publish 应过");
    let raw = handle.join().expect("join");
    std::env::remove_var("READER_LEDGER");
    std::env::remove_var("READER_LEDGER_KEY");

    assert_eq!((out.artifact_id.as_str(), out.seq), ("art-1", 42));
    let (path, _, body) = parse_request(&raw)?;
    assert_eq!(path, format!("/repos/{}/artifacts", ledger::REPO_ID));
    let v: serde_json::Value = serde_json::from_slice(&body)?;
    assert_eq!(v["kind"], "research");
    assert_eq!(v["version"], "0.10.0");
    assert_eq!(v["git_range"], "v0.9.1..v0.10.0");
    assert_eq!(v["deps"][0], "art-0");
    assert_eq!(v["digest"], digest);
    Ok(())
}

/// 关单链请求形状：result（digest）加 status done 两连发。
#[test]
fn issue_close_posts_result_then_done() -> TestResult {
    let _serial = env_serial_lock();
    let digest = format!("sha256:{}", "c".repeat(64));
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    std::env::set_var("READER_LEDGER", format!("http://127.0.0.1:{port}"));
    std::env::set_var("READER_LEDGER_KEY", test_seed_b64());
    let owned_digest = digest.clone();
    let handle = std::thread::spawn(move || {
        // 两条顺序请求（同一服务进程 accept 两次）
        let l = listener;
        let r1 = serve_once_bytes(
            l.try_clone().expect("clone"),
            201,
            "Created",
            r#"{"ok":true,"event":{"seq":100}}"#,
        )
        .expect("第一条应过");
        let r2 = serve_once_bytes(l, 201, "Created", r#"{"ok":true,"event":{"seq":101}}"#)
            .expect("第二条应过");
        (r1, r2, owned_digest)
    });
    let (s1, s2) = ledger::issue_close(3, &digest, "修复随版").expect("关单链应过");
    let (raw1, raw2, _) = handle.join().expect("join");
    std::env::remove_var("READER_LEDGER");
    std::env::remove_var("READER_LEDGER_KEY");

    assert_eq!((s1, s2), (100, 101));
    let (p1, _, b1) = parse_request(&raw1)?;
    let (p2, _, b2) = parse_request(&raw2)?;
    assert_eq!(p1, format!("/repos/{}/issues/3/events", ledger::REPO_ID));
    assert_eq!(p2, p1);
    let v1: serde_json::Value = serde_json::from_slice(&b1)?;
    let v2: serde_json::Value = serde_json::from_slice(&b2)?;
    assert_eq!(v1["type"], "result");
    assert_eq!(v1["payload"]["digest"], digest);
    assert_eq!(v2["type"], "status");
    assert_eq!(v2["payload"]["to"], "done");
    Ok(())
}
