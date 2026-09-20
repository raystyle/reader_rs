//! ledger.ohmygh.com 客户端（REQ-063 Phase 3 仓内集成，替代 issues.ohmygh.com
//! 旧面，REQ-056 的 issue 功能迁入本账本）：仓级公共账本双层 append-only，
//! issue 流管义务（开单加事件加投影）、artifact 流即共享库本体（publish 加
//! attest 加 promote）。写入五头必签：Idempotency-Key 加 X-Key-Id 加
//! X-Timestamp（正负 60 秒窗）加 X-Nonce（10 分钟不重）加 X-Signature；签名基
//! = v1/POST/路径/时间戳/nonce/幂等键/body sha256 各行换行连，Ed25519 私钥签，
//! 签名 base64url。读面沿 REQ-057 家族形：limit 缺省 100 加 before 游标加
//! has_more 加 count 语义（本次返回条数）。私钥运行时从环境
//! `READER_LEDGER_KEY`（base64url seed）或本地密档读（`READER_LEDGER_KEY_FILE`
//! 覆写路径，缺省 `~/.config/reader/ledger-key`），不进仓不进 argv。

use base64::Engine;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::time::Duration;

/// 账本仓标识（规范化 remote；URL 路径原样嵌入，含斜杠）。
pub const REPO_ID: &str = "github.com/raystyle/reader_rs";
/// 账本缺省基址（`READER_LEDGER` 覆盖，测试与灰度用）。
pub const DEFAULT_BASE: &str = "https://ledger.ohmygh.com";
/// 本仓公钥 JWK（身份分发面：CLI 自带验签材料，总台账本 pubkeys 表同形登记；
/// 2026-09-20 生成，对应私钥在提交侧密档）。
pub const PUBLIC_JWK: &str =
    r#"{"crv":"Ed25519","kty":"OKP","x":"8zNA74bNFEFg3mL_3pw8AdPUZeudhmlqGlFoVs6tHJE"}"#;
/// 本仓 key_id（kid）= sha256hex(规范化 JWK：键序字母的紧凑 JSON)。
pub const KEY_ID: &str = "07f7b09b4314de37ed4ac73d3084093b9ce59bcff50282e79c54309b0036c523";

/// 响应体读取上限（1MB：投影列表与时间线远小于此）。
const BODY_LIMIT: u64 = 1024 * 1024;

/// issue kind 面（服务端契约同值）。
pub const ISSUE_KINDS: [&str; 2] = ["bug", "improvement"];
/// artifact kind 面（服务端契约 15 值同集）。
pub const ARTIFACT_KINDS: [&str; 15] = [
    "experience",
    "lesson",
    "research",
    "prototype",
    "binary",
    "image",
    "wasm",
    "sbom",
    "schema",
    "openapi",
    "eval-set",
    "benchmark",
    "runbook",
    "decision",
    "attested-report",
];
/// artifact 事件面（promote 单列命令，其余走 attest --type）。
pub const ATTEST_TYPES: [&str; 6] = [
    "attest_dev",
    "attest_prod",
    "verification_failed",
    "promote",
    "demote",
    "supersede",
];

/// 从 JWK 文本推 kid：取 kty/crv/x 三键按字母序紧凑序列化后 sha256hex。
/// （键序 crv < kty < x 恰为字母序；对账 [`KEY_ID`] 用，单测断言一致。）
///
/// # Errors
///
/// JWK 文本非合法 JSON 或缺 kty/crv/x 任一键时返回人读错误。
pub fn kid_of_jwk(jwk_text: &str) -> Result<String, String> {
    let v: Value = serde_json::from_str(jwk_text).map_err(|e| format!("JWK 解析失败: {e}"))?;
    let pick = |k: &str| {
        v.get(k)
            .and_then(Value::as_str)
            .map(|s| s.to_string())
            .ok_or_else(|| format!("JWK 缺 {k}"))
    };
    let crv = pick("crv")?;
    let kty = pick("kty")?;
    let x = pick("x")?;
    let canon = format!(r#"{{"crv":"{crv}","kty":"{kty}","x":"{x}"}}"#);
    Ok(format!("{:x}", Sha256::digest(canon.as_bytes())))
}

/// 读 Ed25519 私钥（32 字节 seed）：环境 `READER_LEDGER_KEY`（base64url）优先，
/// 次走本地密档（`READER_LEDGER_KEY_FILE` 路径，缺省 `~/.config/reader/ledger-key`，
/// 首行 base64url seed）。密档不进仓不进 argv。
///
/// # Errors
///
/// 环境与密档都取不到、内容非 32 字节 seed 时返回人读错误。
pub fn load_signing_key() -> Result<SigningKey, String> {
    if let Ok(seed_b64) = std::env::var("READER_LEDGER_KEY") {
        return seed_from_base64url(&seed_b64, "READER_LEDGER_KEY");
    }
    let path = std::env::var("READER_LEDGER_KEY_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var_os("HOME")
                .or_else(|| std::env::var_os("USERPROFILE"))
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            home.join(".config").join("reader").join("ledger-key")
        });
    let text = std::fs::read_to_string(&path).map_err(|e| {
        format!(
            "读私钥密档 {} 失败（READER_LEDGER_KEY 环境或该密档须在位）: {e}",
            path.display()
        )
    })?;
    let first = text.lines().next().unwrap_or("").trim();
    seed_from_base64url(first, &path.display().to_string())
}

fn seed_from_base64url(text: &str, from: &str) -> Result<SigningKey, String> {
    let raw = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(text.trim())
        .map_err(|e| format!("私钥（{from}）非合法 base64url: {e}"))?;
    let seed: [u8; 32] = raw
        .try_into()
        .map_err(|_| format!("私钥（{from}）须为 32 字节 seed 的 base64url"))?;
    Ok(SigningKey::from_bytes(&seed))
}

fn api_base() -> String {
    std::env::var("READER_LEDGER")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_BASE.to_string())
        .trim_end_matches('/')
        .to_string()
}

/// 签名基（服务端 verifyEd25519 逐字同构）：七行换行连。
pub fn signature_base(path: &str, ts: u64, nonce: &str, idem: &str, body: &str) -> String {
    let body_sha = format!("{:x}", Sha256::digest(body.as_bytes()));
    format!("v1\nPOST\n{path}\n{ts}\n{nonce}\n{idem}\n{body_sha}")
}

/// 随机 16 字节 hex（幂等键与 nonce 用；getrandom 系统熵源）。
fn rand_hex() -> Result<String, String> {
    let mut buf = [0u8; 16];
    getrandom::fill(&mut buf).map_err(|e| format!("取系统熵失败: {e}"))?;
    Ok(buf.iter().map(|b| format!("{b:02x}")).collect())
}

/// 签名一条 POST：返回五头。签名基走 [`signature_base`]，Ed25519 签名 base64url。
fn signed_headers(
    path: &str,
    body: &str,
    key: &SigningKey,
) -> Result<Vec<(String, String)>, String> {
    let idem = rand_hex()?;
    let nonce = rand_hex()?;
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let sig = key.sign(signature_base(path, ts, &nonce, &idem, body).as_bytes());
    let sig_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(sig.to_bytes());
    Ok(vec![
        ("Idempotency-Key".to_string(), idem),
        ("X-Key-Id".to_string(), KEY_ID.to_string()),
        ("X-Timestamp".to_string(), ts.to_string()),
        ("X-Nonce".to_string(), nonce),
        ("X-Signature".to_string(), sig_b64),
    ])
}

/// 共享 agent：全局超时 20 秒；4xx/5xx 不当传输错误（按状态码分流读体）。
fn agent() -> ureq::Agent {
    ureq::Agent::new_with_config(
        ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(20)))
            .http_status_as_error(false)
            .build(),
    )
}

fn body_json(body: &mut ureq::Body) -> Result<Value, String> {
    let bytes = body
        .with_config()
        .limit(BODY_LIMIT)
        .read_to_vec()
        .map_err(|e| format!("读取响应失败: {e}"))?;
    serde_json::from_slice(&bytes).map_err(|e| format!("回执解析失败: {e}"))
}

/// 错误文案（回执 error 字段缺则带状态码归因）。
fn err_text(v: &Value, status: u16) -> String {
    let msg = v
        .get("error")
        .and_then(Value::as_str)
        .unwrap_or("回执不识别");
    match status {
        401 => format!("验签或密钥不过（HTTP 401）: {msg}"),
        409 => format!("幂等冲突（HTTP 409）: {msg}"),
        429 => format!("配额满（HTTP 429）: {msg}"),
        _ => format!("ledger.ohmygh.com 回错（HTTP {status}）: {msg}"),
    }
}

/// 签名 POST：五头全带；幂等键撞车（409）自动换键重试一次。
///
/// # Errors
///
/// 私钥不可得、网络不可达、验签不过（401，含密钥未注册与时间窗）、幂等冲突
/// 重试后仍 409、配额满（429）或服务端 4xx/5xx 时返回人读错误。
fn post_signed(path: &str, payload: &Value) -> Result<Value, String> {
    let key = load_signing_key()?;
    let body = serde_json::to_string(payload).map_err(|e| format!("序列化请求体失败: {e}"))?;
    let url = format!("{}{path}", api_base());
    for attempt in 0..2 {
        let headers = signed_headers(path, &body, &key)?;
        let mut req = agent().post(&url);
        for (k, v) in &headers {
            req = req.header(k, v);
        }
        // 发送体即签名体（ureq send_json 出 pretty 形，与 to_string 不同字节；
        // 签名基必须逐字覆盖线上体，故按字符串原样发送）
        let mut resp = req
            .header("Content-Type", "application/json")
            .send(body.clone())
            .map_err(|e| format!("ledger.ohmygh.com 不可达: {e}"))?;
        let status = resp.status().as_u16();
        let v = body_json(resp.body_mut())?;
        if (200..300).contains(&status) {
            return Ok(v);
        }
        // 409 = 幂等键撞车（随机键理论不撞；撞即换键一次）
        if status == 409 && attempt == 0 {
            eprintln!("reader: 幂等键冲突，换键重试一次");
            continue;
        }
        return Err(err_text(&v, status));
    }
    Err("幂等冲突换键重试仍 409".to_string())
}

/// 无签名 GET（读面不受配额限制）。
fn get(path_and_query: &str) -> Result<Value, String> {
    let mut resp = agent()
        .get(&format!("{}{path_and_query}", api_base()))
        .call()
        .map_err(|e| format!("ledger.ohmygh.com 不可达: {e}"))?;
    let status = resp.status().as_u16();
    let v = body_json(resp.body_mut())?;
    if status == 404 {
        return Err("__not_found__".to_string());
    }
    if status != 200 {
        return Err(err_text(&v, status));
    }
    Ok(v)
}

/// digest 形校验：`sha256:` 加 64 位小写 hex（服务端契约同则 `[0-9a-f]{64}`）。
pub fn valid_digest(s: &str) -> bool {
    s.len() == 71
        && s.starts_with("sha256:")
        && s[7..]
            .chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}

// ---------- issue 面 ----------

/// 开单回执（issue 号与首事件 seq）。
#[derive(Debug)]
pub struct IssueOpened {
    /// 仓内单调 issue 号。
    pub issue: u64,
    /// issue_open 事件 seq。
    pub seq: u64,
}

/// 开单：POST /repos/:repo/issues {title, kind, acceptance, body}。
/// 客户端先校验（title trim 后 1 至 200 字、kind 面），服务端同则兜底。
///
/// # Errors
///
/// 校验不过、私钥不可得、网络或服务端错误（401/429/5xx 等）时返回人读错误。
pub fn issue_new(
    title: &str,
    kind: &str,
    acceptance: &str,
    body: &str,
) -> Result<IssueOpened, String> {
    let title = title.trim();
    if title.is_empty() || title.chars().count() > 200 {
        return Err("title 必填且至多 200 字".to_string());
    }
    if !ISSUE_KINDS.contains(&kind) {
        return Err(format!(
            "kind 仅 {}（BUG 错误任务或改进优化任务）",
            ISSUE_KINDS.join("|")
        ));
    }
    let v = post_signed(
        &format!("/repos/{REPO_ID}/issues"),
        &json!({ "title": title, "kind": kind, "acceptance": acceptance, "body": body }),
    )?;
    let issue = v
        .get("issue")
        .and_then(Value::as_u64)
        .ok_or("回执缺 issue 号")?;
    let seq = event_seq(&v);
    Ok(IssueOpened { issue, seq })
}

/// 追加 issue 事件：POST /repos/:repo/issues/:n/events {type, payload, body}。
/// 服务端约束：status 的 payload.to 须在状态面；done 须先有 result 事件。
///
/// # Errors
///
/// type 不在面、issue 不存在（404）、验签/配额/网络错误时返回人读错误。
pub fn issue_event(n: u64, event_type: &str, payload: Value, body: &str) -> Result<u64, String> {
    let v = post_signed(
        &format!("/repos/{REPO_ID}/issues/{n}/events"),
        &json!({ "type": event_type, "payload": payload, "body": body }),
    )?;
    Ok(event_seq(&v))
}

/// 关单链：先 result 事件（引用 digest，关单判据）再 status done；返回两事件 seq。
///
/// # Errors
///
/// 同 [`issue_event`] 两步各自错误面。
pub fn issue_close(n: u64, digest: &str, note: &str) -> Result<(u64, u64), String> {
    if !valid_digest(digest) {
        return Err("digest 须 sha256:<64hex>（result 引用的产物或正文哈希）".to_string());
    }
    let result_seq = issue_event(n, "result", json!({ "digest": digest }), note)?;
    let status_seq = issue_event(n, "status", json!({ "to": "done" }), "")?;
    Ok((result_seq, status_seq))
}

/// 列表行（服务端投影字段同形；`hasResult` 服务端为驼峰）。
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IssueRow {
    /// 仓内 issue 号。
    #[serde(rename = "issue_n")]
    pub issue_n: u64,
    /// 标题。
    pub title: String,
    /// 投影状态（open/claimed/in_progress/blocked/done）。
    pub status: String,
    /// 任务性质（bug/improvement）。
    pub kind: String,
    /// 认领者（无则 null）。
    pub assignee: Option<String>,
    /// 是否已有 result 事件（关单判据面）。
    #[serde(rename = "hasResult")]
    pub has_result: bool,
}

/// 列表页（家族翻页形）。
#[derive(Debug)]
pub struct IssuePage {
    /// 本次返回行（新到旧）。
    pub rows: Vec<IssueRow>,
    /// 更早是否仍有条目（more=1 请求恒携带）。
    pub has_more: Option<bool>,
}

/// 列表（新到旧）：GET /repos/:repo/issues?limit=&before=&more=1。
/// limit 夹取 1 至 100（缺省 100）；恒带 more=1 索取 has_more。
///
/// # Errors
///
/// 网络不可达或回执形状不对时返回人读错误。
pub fn issue_list(limit: u32, before: Option<u64>) -> Result<IssuePage, String> {
    let mut q = format!(
        "/repos/{REPO_ID}/issues?limit={}&more=1",
        limit.clamp(1, 100)
    );
    if let Some(b) = before {
        q.push_str(&format!("&before={b}"));
    }
    let v = get(&q)?;
    let rows: Vec<IssueRow> =
        serde_json::from_value(v.get("issues").cloned().ok_or("回执缺 issues 数组")?)
            .map_err(|e| format!("回执形状不对: {e}"))?;
    Ok(IssuePage {
        rows,
        has_more: v.get("has_more").and_then(Value::as_bool),
    })
}

/// 单条详情：GET /repos/:repo/issues/:n（投影加时间线）；404 归 `Ok(None)`。
///
/// # Errors
///
/// 网络不可达或回执形状不对时返回人读错误。
pub fn issue_show(n: u64) -> Result<Option<IssueDetail>, String> {
    let v = match get(&format!("/repos/{REPO_ID}/issues/{n}")) {
        Ok(v) => v,
        Err(e) if e == "__not_found__" => return Ok(None),
        Err(e) => return Err(e),
    };
    let projection = v.get("projection").cloned().ok_or("回执缺 projection")?;
    let timeline = v
        .get("timeline")
        .and_then(Value::as_array)
        .cloned()
        .ok_or("回执缺 timeline")?;
    // acceptance 与开单标题藏在首事件（issue_open）payload 字符串里
    let open_payload = timeline
        .iter()
        .find(|e| e.get("type").and_then(Value::as_str) == Some("issue_open"))
        .and_then(|e| e.get("payload").and_then(Value::as_str))
        .and_then(|s| serde_json::from_str::<Value>(s).ok())
        .unwrap_or(Value::Null);
    Ok(Some(IssueDetail {
        issue: n,
        status: projection
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        kind: projection
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        assignee: projection
            .get("assignee")
            .and_then(Value::as_str)
            .map(|s| s.to_string()),
        acceptance: open_payload
            .get("acceptance")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        timeline,
    }))
}

/// issue 详情（投影加开单验收面加原始时间线）。
#[derive(Debug)]
pub struct IssueDetail {
    /// issue 号。
    pub issue: u64,
    /// 投影状态。
    pub status: String,
    /// 任务性质。
    pub kind: String,
    /// 认领者。
    pub assignee: Option<String>,
    /// 开单时登记的验收条件。
    pub acceptance: String,
    /// 原始事件时间线（seq/type/payload/…，payload 为 JSON 字符串）。
    pub timeline: Vec<Value>,
}

// ---------- artifact 面 ----------

/// publish 回执。
#[derive(Debug)]
pub struct ArtifactPublished {
    /// artifact 标识（服务端 uuid）。
    pub artifact_id: String,
    /// 内容寻址 digest（与提交值同）。
    pub digest: String,
    /// publish 事件 seq。
    pub seq: u64,
}

/// 登记产物：POST /repos/:repo/artifacts {name, kind, digest, version?, git_range?, deps[]}。
/// 库一律收信息体（digest 恒为正文或记录哈希）；同 digest 重复登记服务端 409。
///
/// # Errors
///
/// name/kind/digest 校验不过、私钥不可得、同 digest 已登记（409）、验签/配额/
/// 网络错误时返回人读错误。
pub fn artifact_publish(
    name: &str,
    kind: &str,
    digest: &str,
    version: Option<&str>,
    git_range: Option<&str>,
    deps: &[String],
    body: &str,
) -> Result<ArtifactPublished, String> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 200 {
        return Err("name 必填且至多 200 字".to_string());
    }
    if !ARTIFACT_KINDS.contains(&kind) {
        return Err(format!("kind 仅 {}", ARTIFACT_KINDS.join("|")));
    }
    if !valid_digest(digest) {
        return Err("digest 须 sha256:<64hex>（一律正文或记录哈希为身份）".to_string());
    }
    let mut payload =
        json!({ "name": name, "kind": kind, "digest": digest, "deps": deps, "body": body });
    if let Some(v) = version {
        payload["version"] = json!(v);
    }
    if let Some(g) = git_range {
        payload["git_range"] = json!(g);
    }
    let v = post_signed(&format!("/repos/{REPO_ID}/artifacts"), &payload)?;
    Ok(ArtifactPublished {
        artifact_id: v
            .get("artifact_id")
            .and_then(Value::as_str)
            .ok_or("回执缺 artifact_id")?
            .to_string(),
        digest: digest.to_string(),
        seq: event_seq(&v),
    })
}

/// 产物事件（attest/promote 等）：POST /repos/:repo/artifacts/:id/attestations。
///
/// # Errors
///
/// type 不在面、artifact 不存在（404）、验签/配额/网络错误时返回人读错误。
pub fn artifact_attest(artifact_id: &str, attest_type: &str, note: &str) -> Result<u64, String> {
    if !ATTEST_TYPES.contains(&attest_type) {
        return Err(format!("type 仅 {}", ATTEST_TYPES.join("|")));
    }
    let mut payload = json!({ "type": attest_type, "payload": {}, "body": note });
    if attest_type == "attest_prod" {
        payload["payload"] = json!({ "env": "prod" });
    }
    let v = post_signed(
        &format!("/repos/{REPO_ID}/artifacts/{artifact_id}/attestations"),
        &payload,
    )?;
    Ok(event_seq(&v))
}

/// 产物列表行。
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ArtifactRow {
    /// artifact 标识。
    pub artifact_id: String,
    /// 名称。
    pub name: String,
    /// kind。
    pub kind: String,
    /// digest。
    pub digest: String,
    /// dev 验证与否。
    pub dev_verified: bool,
    /// prod 验证与否。
    pub prod_verified: bool,
    /// 是否该 name 的当前持有者（最新 promote 且未退役）。
    pub current: bool,
}

/// 产物列表：GET /repos/:repo/artifacts?current=&env=（可按 name/kind 过滤）。
///
/// # Errors
///
/// 网络不可达或回执形状不对时返回人读错误。
pub fn artifact_list(
    name: Option<&str>,
    kind: Option<&str>,
    env: Option<&str>,
    current: bool,
) -> Result<Vec<ArtifactRow>, String> {
    let mut q = format!("/repos/{REPO_ID}/artifacts?");
    if let Some(n) = name {
        q.push_str(&format!("name={}&", n));
    }
    if let Some(k) = kind {
        q.push_str(&format!("kind={}&", k));
    }
    if let Some(e) = env {
        q.push_str(&format!("env={}&", e));
    }
    if current {
        q.push_str("current=1&");
    }
    let v = get(q.trim_end_matches('&'))?;
    serde_json::from_value(v.get("artifacts").cloned().ok_or("回执缺 artifacts 数组")?)
        .map_err(|e| format!("回执形状不对: {e}"))
}

fn event_seq(v: &Value) -> u64 {
    v.get("event")
        .and_then(|e| e.get("seq"))
        .and_then(Value::as_u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// kid 推导与内置常量对账（JWK 改动须同步 KEY_ID）。
    #[test]
    fn kid_matches_embedded_jwk() {
        assert_eq!(kid_of_jwk(PUBLIC_JWK).unwrap(), KEY_ID);
    }

    /// 签名基逐字（七行换行连，body sha256 为 hex）。
    #[test]
    fn signature_base_exact_form() {
        let body = r#"{"title":"t"}"#;
        let base = signature_base(
            &format!("/repos/{REPO_ID}/issues"),
            1700000000,
            "nonce1",
            "idem2",
            body,
        );
        let body_sha = format!("{:x}", Sha256::digest(body.as_bytes()));
        let want =
            format!("v1\nPOST\n/repos/{REPO_ID}/issues\n1700000000\nnonce1\nidem2\n{body_sha}");
        assert_eq!(base, want);
        assert_eq!(want.split('\n').count(), 7);
        // body 变化即基变化（幂等键同内容分道的根）
        assert_ne!(
            signature_base("/p", 1, "n", "i", "a"),
            signature_base("/p", 1, "n", "i", "b")
        );
    }

    /// 幂等键与 nonce：连取多次互不重（16 字节熵面）。
    #[test]
    fn idem_and_nonce_unique() {
        let mut seen = std::collections::HashSet::new();
        for _ in 0..64 {
            seen.insert(rand_hex().unwrap());
        }
        assert_eq!(seen.len(), 64, "64 次取样零重");
    }

    /// digest 与 kind 校验面。
    #[test]
    fn validates_digest_and_kinds() {
        assert!(valid_digest(&format!("sha256:{}", "a".repeat(64))));
        assert!(
            !valid_digest(&format!("sha256:{}", "A".repeat(64))),
            "大写拒"
        );
        assert!(!valid_digest("sha256:abc"));
        assert!(!valid_digest(&format!("sha1:{}", "a".repeat(64))));
        assert!(ISSUE_KINDS.contains(&"bug"));
        assert_eq!(ARTIFACT_KINDS.len(), 15);
        assert!(ARTIFACT_KINDS.contains(&"research") && ARTIFACT_KINDS.contains(&"experience"));
        assert!(!ATTEST_TYPES.contains(&"publish"));
    }

    /// 私钥加载环境道：合法 32 字节 seed 过并派生公钥；非 32 字节拒。
    /// （单测合一串行：env 是进程级，并行测试互踩 remove_var。）
    #[test]
    fn loads_seed_from_env_and_rejects_bad_shape() {
        let seed = [7u8; 32];
        std::env::set_var(
            "READER_LEDGER_KEY",
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(seed),
        );
        let key = load_signing_key().expect("env 道应过");
        // 公钥与 seed 对应（验证通道可从签名反推持有者）
        let verifying = ed25519_dalek::VerifyingKey::from(&key);
        let x = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(verifying.to_bytes());
        assert_eq!(
            kid_of_jwk(&format!(r#"{{"crv":"Ed25519","kty":"OKP","x":"{x}"}}"#))
                .unwrap()
                .len(),
            64
        );
        // 非 32 字节 seed 拒
        std::env::set_var(
            "READER_LEDGER_KEY",
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode([1u8; 16]),
        );
        let err = load_signing_key().unwrap_err();
        assert!(err.contains("32 字节"), "{err}");
        std::env::remove_var("READER_LEDGER_KEY");
    }
}
