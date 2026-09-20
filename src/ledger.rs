//! ledger 胶水层：统一客户端 crate ledger-client（REQ-063 全舰队唯一实现）
//! 的仓内接线（总台修正令 2026-09-20 收口：签名道加只增面全在 crate，本仓
//! 不再自研副本；关闭与删除唯一道 = 开发工作台经 herdr 委托 omc 工位）。
//! 本模块只做三件：私钥装载（env `READER_LEDGER_KEY` 的 32 字节 hex，或本地
//! 密档 `READER_LEDGER_KEY_FILE` 路径，缺省 `~/.config/reader/ledger-key`，
//! 不进仓不进 argv）、在册 kid 对账（换对须同步 [`KEY_ID`] 与总台登记）、
//! [`ledger_client::Ledger`] 装配。命令面与行式在 lib.rs。

use ledger_client::{KeyPair, Ledger};
use std::path::PathBuf;

/// 账本仓标识（规范化 remote；URL 路径原样嵌入，含斜杠）。
pub const REPO_ID: &str = "github.com/raystyle/reader_rs";
/// 在册 kid（2026-09-20 总台 pubkeys 登记值；换密钥对须同步本常量与登记）。
pub const KEY_ID: &str = "07f7b09b4314de37ed4ac73d3084093b9ce59bcff50282e79c54309b0036c523";

/// 读私钥 hex（32 字节 seed 的 64 位 hex）：环境 `READER_LEDGER_KEY` 优先，
/// 次走本地密档首行（`READER_LEDGER_KEY_FILE` 路径，缺省
/// `~/.config/reader/ledger-key`）。密档不进仓不进 argv。
fn secret_hex() -> Result<String, String> {
    if let Ok(hex) = std::env::var("READER_LEDGER_KEY") {
        if !hex.trim().is_empty() {
            return Ok(hex.trim().to_string());
        }
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
    Ok(text.lines().next().unwrap_or("").trim().to_string())
}

/// 装配 Ledger 客户端：装载私钥、对账在册 kid（不符即拒，防换对后写面静默 401）。
///
/// # Errors
///
/// 私钥不可得或非 32 字节 hex、派生 kid 与 [`KEY_ID`] 不符（换对须同步登记与
/// 本常量）时返回人读错误。
pub fn connect() -> Result<Ledger, String> {
    let hex = secret_hex()?;
    let key = KeyPair::load_secret_hex(&hex)
        .map_err(|e| format!("账本私钥装载失败（READER_LEDGER_KEY 或密档，32 字节 hex）: {e}"))?;
    if key.key_id != KEY_ID {
        return Err(format!(
            "私钥派生 kid {} 与在册 {} 不符（密钥对已换或密档错件；换对须同步总台登记与本仓 KEY_ID 常量）",
            key.key_id, KEY_ID
        ));
    }
    Ok(Ledger::new(REPO_ID, key))
}

/// 把 crate 错误转 CLI 人读行（按状态码归因）。
pub fn err_line(e: ledger_client::LedgerError) -> String {
    match &e {
        ledger_client::LedgerError::Api { status, message } => match status {
            401 => format!("验签或密钥不过（HTTP 401）: {message}"),
            409 => format!("幂等或同 digest 冲突（HTTP 409）: {message}"),
            429 => format!("配额满（HTTP 429）: {message}"),
            _ => format!("ledger.ohmygh.com 回错（HTTP {status}）: {message}"),
        },
        ledger_client::LedgerError::Key(k) => format!("密钥面错误: {k}"),
        ledger_client::LedgerError::Http(h) => format!("ledger.ohmygh.com 不可达: {h}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 密档形状道：hex 首行读取与路径回退（临时密档，不碰真档）。
    #[test]
    fn secret_hex_reads_first_line_of_key_file() {
        let dir = std::env::temp_dir().join(format!("reader-ledger-key-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let kf = dir.join("k");
        std::fs::write(&kf, format!("{}\n# 注释行\n", "ab".repeat(32))).unwrap();
        std::env::set_var("READER_LEDGER_KEY", "");
        std::env::set_var("READER_LEDGER_KEY_FILE", &kf);
        let got = secret_hex().expect("密档道应过");
        std::env::remove_var("READER_LEDGER_KEY_FILE");
        std::env::remove_var("READER_LEDGER_KEY");
        assert_eq!(got, "ab".repeat(32), "取首行 hex");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
