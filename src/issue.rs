//! issue 域（总台统一入口 issues.ohmygh.com，ohmycloud REQ-057 契约对齐）：
//! `new` 一键提交（自动带 tool=reader 与版本、平台、主机名）、`list` 集中列表（新到旧）、
//! `show` 单条详情。提交面每 IP 10 条/时（HTTP 429），读面匿名 GET；基址可由
//! `READER_ISSUES_API` 覆盖（测试与灰度）。HTTP 走 ureq（全局超时 20 秒，4xx/5xx
//! 不转传输错误、按状态码分流），JSON 走 serde_json；不新增依赖。

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// 统一 issue 入口缺省基址（`READER_ISSUES_API` 可覆盖）。
const DEFAULT_API: &str = "https://issues.ohmygh.com";

/// 响应体读取上限（1MB：列表至多 100 行加正文，远小于此）。
const BODY_LIMIT: u64 = 1024 * 1024;

/// 提交体（字段序即 POST JSON 序，契约 {tool,title,body,version,platform,host}）。
#[derive(Serialize, Debug, PartialEq)]
pub struct IssueReport {
    /// 工具名（reader 侧恒 `reader`）
    pub tool: String,
    /// 标题（trim 后 1 至 200 字）
    pub title: String,
    /// 正文（至多 20000 字，缺省空串）
    pub body: String,
    /// 版本（Cargo.toml，截断 40）
    pub version: String,
    /// 平台（编译目标三元组，截断 64）
    pub platform: String,
    /// 主机名（HOSTNAME 或 COMPUTERNAME，缺 unknown；截断 64）
    pub host: String,
}

/// 提交回执（201 {ok,id,url} 的有效载荷）。
#[derive(Deserialize, Debug, PartialEq)]
pub struct FiledReceipt {
    /// 入库编号
    pub id: i64,
    /// 详情页地址（/i/<id>）
    pub url: String,
}

/// 一条 issue（list 无 body、show 有；ip 属服务端审计面，不透出到输出）。
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IssueRow {
    /// 入库编号
    pub id: i64,
    /// 工具名
    pub tool: String,
    /// 标题
    pub title: String,
    /// 正文（仅 show 返回）
    #[serde(default)]
    pub body: String,
    /// 报告版本
    pub version: String,
    /// 报告平台
    pub platform: String,
    /// 报告主机名
    pub host: String,
    /// 状态（open / closed）
    pub status: String,
    /// 提交端 IP（服务端审计字段，只读不透出）
    #[serde(skip_serializing)]
    pub ip: String,
    /// 提交时间（ISO 形）
    pub created_at: String,
}

/// 取 API 基址：`READER_ISSUES_API` 覆盖缺省（测试与灰度用）。
fn api_base() -> String {
    std::env::var("READER_ISSUES_API").unwrap_or_else(|_| DEFAULT_API.to_string())
}

/// 构建共享 agent：全局超时 20 秒；4xx/5xx 不当传输错误（按状态码分流读体）。
fn agent() -> ureq::Agent {
    ureq::Agent::new_with_config(
        ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(20)))
            .http_status_as_error(false)
            .build(),
    )
}

/// 校验工具名形 `^[a-z][a-z0-9_-]{0,31}$`（纯 ASCII，字节判定等价）。
fn valid_tool(tool: &str) -> bool {
    let b = tool.as_bytes();
    match b.first() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    b.len() <= 32
        && b[1..]
            .iter()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'_' || *c == b'-')
}

/// 按 Unicode 字符数截断（契约的"客户端先截断"）。
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        s.chars().take(max).collect()
    }
}

/// 自报平台：运行时 arch 加 os（如 x86_64-linux；截断 64 内天然满足）。
fn self_platform() -> String {
    format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS)
}

/// 自报主机名：HOSTNAME 优先、COMPUTERNAME 次之，都缺即 unknown；截断 64。
fn self_host() -> String {
    // HOSTNAME 多为 shell 变量未导出，COMPUTERNAME 在 Windows；unix 再兜 /etc/hostname
    let from_env = ["HOSTNAME", "COMPUTERNAME"]
        .iter()
        .find_map(|k| std::env::var(k).ok().filter(|v| !v.trim().is_empty()));
    let raw = from_env
        .or_else(|| {
            std::fs::read_to_string("/etc/hostname")
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| "unknown".to_string());
    truncate(&raw, 64)
}

/// 构建提交体并做客户端先校验：tool 形 `^[a-z][a-z0-9_-]{0,31}$`、title trim 后
/// 1 至 200 字、body 至多 20000 字；version 截断 40、platform 与 host 截断 64。
/// 版本、平台、主机名自动取自报值（tool 保留入参以便他仓复用）。
///
/// # Errors
///
/// tool 不合形、标题 trim 后为空或超 200 字、正文超 20000 字时返回人读错误（调用方退出 2）。
pub fn build_report(tool: &str, title: &str, body: &str) -> Result<IssueReport, String> {
    if !valid_tool(tool) {
        return Err(format!("tool 形不符（^[a-z][a-z0-9_-]{{0,31}}$）: {tool}"));
    }
    let title = title.trim();
    if title.is_empty() {
        return Err("标题不能为空（trim 后 1 至 200 字）".to_string());
    }
    if title.chars().count() > 200 {
        return Err(format!("标题 {} 字，超上限 200", title.chars().count()));
    }
    if body.chars().count() > 20000 {
        return Err(format!("正文 {} 字，超上限 20000", body.chars().count()));
    }
    Ok(IssueReport {
        tool: tool.to_string(),
        title: title.to_string(),
        body: body.to_string(),
        version: truncate(env!("CARGO_PKG_VERSION"), 40),
        platform: self_platform(),
        host: self_host(),
    })
}

/// 读响应体（限 1MB）并解析 JSON。
fn body_json(body: &mut ureq::Body) -> Result<serde_json::Value, String> {
    let bytes = body
        .with_config()
        .limit(BODY_LIMIT)
        .read_to_vec()
        .map_err(|e| format!("读取响应失败: {e}"))?;
    serde_json::from_slice(&bytes).map_err(|e| format!("回执解析失败: {e}"))
}

/// 从错误回执取 error 文案（缺即"回执不识别"）。
fn err_text(v: &serde_json::Value) -> String {
    v.get("error")
        .and_then(|e| e.as_str())
        .unwrap_or("回执不识别")
        .to_string()
}

/// 把非 2xx 状态转人读错误（429 限速、400 校验、其余带状态码）。
fn http_error(status: u16, v: &serde_json::Value) -> String {
    match status {
        429 => format!("提交限速（每 IP 10 条/时），稍后再试: {}", err_text(v)),
        400 => format!("校验不过: {}", err_text(v)),
        _ => format!("issues.ohmygh.com 回错（HTTP {status}）: {}", err_text(v)),
    }
}

/// 提交一条 issue（tool=reader、上下文自动）：POST /api/issues，期望 201 {ok,id,url}。
///
/// # Errors
///
/// 网络不可达、HTTP 4xx/5xx（429 限速、400 校验不过等）或回执形状不对时返回人读错误（退出 2）。
pub fn file_new(title: &str, body: &str) -> Result<FiledReceipt, String> {
    let report = build_report("reader", title, body)?;
    let mut resp = agent()
        .post(format!("{}/api/issues", api_base()))
        .send_json(&report)
        .map_err(|e| format!("issues.ohmygh.com 不可达: {e}"))?;
    let status = resp.status().as_u16();
    let v = body_json(resp.body_mut())?;
    if status != 201 {
        return Err(http_error(status, &v));
    }
    let ok = v.get("ok").and_then(|o| o.as_bool()).unwrap_or(false);
    if !ok {
        return Err(format!("回执 ok 不为真（HTTP {status}）: {}", err_text(&v)));
    }
    serde_json::from_value::<FiledReceipt>(v).map_err(|e| format!("回执形状不对: {e}"))
}

/// 列表（新到旧）：GET /api/issues?tool=&status=&limit=（limit 夹取 1 至 100）。
/// 空列表是合法结果（调用方退出 1）。
///
/// # Errors
///
/// 网络不可达、HTTP 错误或回执形状不对时返回人读错误（退出 2）。
pub fn list(tool: Option<&str>, status: Option<&str>, limit: u32) -> Result<Vec<IssueRow>, String> {
    let mut req = agent()
        .get(format!("{}/api/issues", api_base()))
        .query("limit", limit.clamp(1, 100).to_string());
    if let Some(t) = tool {
        req = req.query("tool", t);
    }
    if let Some(s) = status {
        req = req.query("status", s);
    }
    let mut resp = req
        .call()
        .map_err(|e| format!("issues.ohmygh.com 不可达: {e}"))?;
    let status_code = resp.status().as_u16();
    let v = body_json(resp.body_mut())?;
    if status_code != 200 {
        return Err(http_error(status_code, &v));
    }
    let issues = v
        .get("issues")
        .cloned()
        .ok_or_else(|| "回执缺 issues 数组".to_string())?;
    serde_json::from_value::<Vec<IssueRow>>(issues).map_err(|e| format!("回执形状不对: {e}"))
}

/// 单条详情：GET /api/issues/<id>；404 归 `Ok(None)`（调用方退出 1）。
///
/// # Errors
///
/// 网络不可达、404 以外的 HTTP 错误或回执形状不对时返回人读错误（退出 2）。
pub fn show(id: u64) -> Result<Option<IssueRow>, String> {
    let mut resp = agent()
        .get(format!("{}/api/issues/{id}", api_base()))
        .call()
        .map_err(|e| format!("issues.ohmygh.com 不可达: {e}"))?;
    let status_code = resp.status().as_u16();
    let v = body_json(resp.body_mut())?;
    if status_code == 404 {
        return Ok(None);
    }
    if status_code != 200 {
        return Err(http_error(status_code, &v));
    }
    match v.get("issue") {
        Some(issue) => serde_json::from_value::<IssueRow>(issue.clone())
            .map(Some)
            .map_err(|e| format!("回执形状不对: {e}")),
        None => Err("回执缺 issue 对象".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_shape() {
        assert!(valid_tool("reader"));
        assert!(valid_tool("a"));
        assert!(valid_tool("hst-rs_2"));
        assert!(!valid_tool("Reader"), "大写不开");
        assert!(!valid_tool("1abc"), "数字不开头");
        assert!(!valid_tool(""), "空不开");
        assert!(!valid_tool("a".repeat(33).as_str()), "超 32");
        assert!(valid_tool(&"a".repeat(32)), "恰 32 开");
        assert!(!valid_tool("a b"), "空格不收");
    }

    #[test]
    fn build_report_validates_and_truncates() {
        let r = build_report("reader", "  标题  ", "正文").expect("应过");
        assert_eq!(r.title, "标题", "标题 trim");
        assert_eq!(r.tool, "reader");
        assert_eq!(r.version, env!("CARGO_PKG_VERSION"));
        assert!(!r.platform.is_empty());

        assert!(build_report("reader", "   ", "").is_err(), "空标题拒");
        let long_title = build_report("reader", &"题".repeat(201), "").unwrap_err();
        assert!(long_title.contains("200"), "超长标题报上限: {long_title}");
        let long_body = "字".repeat(20001);
        let err = build_report("reader", "t", &long_body).unwrap_err();
        assert!(err.contains("20000"), "超长正文报上限: {err}");
        let long_host = truncate(&"主".repeat(100), 64);
        assert_eq!(long_host.chars().count(), 64, "截断按字符数");
    }

    #[test]
    fn row_json_roundtrip_drops_ip() {
        let raw = r#"{"id":12,"tool":"reader","title":"t","body":"b","version":"0.7.0","platform":"p","host":"h","status":"open","ip":"1.2.3.4","created_at":"2026-09-17T00:00:00Z"}"#;
        let row: IssueRow = serde_json::from_str(raw).expect("应解析");
        assert_eq!(row.ip, "1.2.3.4");
        let out = serde_json::to_value(&row).expect("应序列化");
        assert!(out.get("ip").is_none(), "ip 不透出");
        assert_eq!(out["id"], 12);
    }
}
