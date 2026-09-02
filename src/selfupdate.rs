//! self update（P0015）：`reader self update` 从 GitHub Releases 最新正式版升级自身。
//! 模式参考 ohmyenv-rs selfupdate.rs 与 ohmyagents-rs update.rs：
//! 版本判新（stable 资产是压缩包，digest 与 exe 哈希不可比）、资产 sha256 digest 钉死校验、
//! staged 加 rename 原子替换（Windows 运行中 exe 可改名不可删）、403 限流回退 gh api。
//! 边界：只 stable 通道（无 dev/git）；只显式命令不自动更新。

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const REPO: &str = "raystyle/reader_rs";
const UA: &str = "reader-selfupdate";
/// reader 压缩包最大约 35MB 量级，留四倍余量。
const DOWNLOAD_LIMIT: u64 = 128 * 1024 * 1024;

/// 升级结果（lib.rs 拼稳定输出行用）。
pub struct Outcome {
    /// current：已是最新；updated：已替换
    pub action: &'static str,
    pub current: String,
    pub latest: String,
    pub replaced: Vec<PathBuf>,
}

/// 本编译目标对应的 release 资产名（release.yml 矩阵命名约定）。
pub fn asset_target() -> Result<&'static str, String> {
    #[cfg(all(windows, target_arch = "x86_64"))]
    {
        return Ok("x86_64-pc-windows-msvc");
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "musl"))]
    {
        return Ok("x86_64-unknown-linux-musl");
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64", not(target_env = "musl")))]
    {
        return Ok("x86_64-unknown-linux-gnu");
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        return Ok("aarch64-apple-darwin");
    }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        return Ok("x86_64-apple-darwin");
    }
    #[allow(unreachable_code)]
    Err("当前平台无 release 构建资产（release.yml 矩阵未覆盖此目标）".to_string())
}

fn asset_name(version: &str) -> Result<String, String> {
    let ext = if cfg!(windows) { "zip" } else { "tar.gz" };
    Ok(format!("reader-v{version}-{}.{ext}", asset_target()?))
}

/// `reader self update` 主流程。`force` 为真时版本相同也重装。
pub fn self_update(force: bool) -> Result<Outcome, String> {
    let current = env!("CARGO_PKG_VERSION").to_string();
    let release = fetch_latest_release()?;
    let tag = release
        .get("tag_name")
        .and_then(Value::as_str)
        .ok_or_else(|| "release 无 tag_name".to_string())?;
    let latest = tag.strip_prefix('v').unwrap_or(tag).to_string();
    if !force && !version_newer(&latest, &current) {
        return Ok(Outcome {
            action: "current",
            current,
            latest,
            replaced: Vec::new(),
        });
    }
    let name = asset_name(&latest)?;
    let assets = release
        .get("assets")
        .and_then(Value::as_array)
        .ok_or_else(|| "release 无资产列表".to_string())?;
    let asset = assets
        .iter()
        .find(|a| a.get("name").and_then(Value::as_str) == Some(name.as_str()))
        .ok_or_else(|| format!("release 缺资产 {name}（CI 是否已跑完？）"))?;
    let digest = asset
        .get("digest")
        .and_then(Value::as_str)
        .and_then(|d| d.strip_prefix("sha256:"))
        .ok_or_else(|| format!("资产 {name} 无 sha256 digest，拒绝无校验升级"))?
        .to_lowercase();
    let url = asset
        .get("browser_download_url")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("资产 {name} 无下载地址"))?
        .to_string();

    eprintln!("reader: 下载 {name} …");
    let blob = fetch(&url)?;
    let sha = format!("{:x}", Sha256::digest(&blob));
    if sha != digest {
        return Err(format!("资产 {name} 校验失败: 期望 {digest} 实得 {sha}"));
    }

    let stage = std::env::temp_dir().join(format!("reader-selfupdate-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&stage);
    std::fs::create_dir_all(&stage).map_err(|e| format!("建临时目录失败: {e}"))?;
    let result = extract_bins(&blob, &stage)
        .and_then(|bins| replace_all(&bins))
        .map(|replaced| Outcome {
            action: "updated",
            current,
            latest,
            replaced,
        });
    let _ = std::fs::remove_dir_all(&stage);
    result
}

/// 版本三元组比较：`latest` 严格大于 `current` 才判新（非数字段按 0 计）。
fn version_newer(latest: &str, current: &str) -> bool {
    fn parts(v: &str) -> [u64; 3] {
        let mut out = [0u64; 3];
        for (i, seg) in v.split('.').take(3).enumerate() {
            out[i] = seg
                .split(|c: char| !c.is_ascii_digit())
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        }
        out
    }
    parts(latest) > parts(current)
}

/// 取最新正式版元数据：直连 api.github.com（GH_TOKEN 注入），403/限流回退 gh api。
fn fetch_latest_release() -> Result<Value, String> {
    let url = format!("https://api.github.com/repos/{REPO}/releases/latest");
    let mut req = ureq::get(&url)
        .header("User-Agent", UA)
        .header("Accept", "application/vnd.github+json");
    if let Ok(tok) = std::env::var("GH_TOKEN") {
        req = req.header("Authorization", &format!("Bearer {tok}"));
    }
    match req.call() {
        Ok(mut resp) => {
            let body = resp
                .body_mut()
                .with_config()
                .limit(DOWNLOAD_LIMIT)
                .read_to_vec()
                .map_err(|e| format!("读取 release 元数据失败: {e}"))?;
            serde_json::from_slice(&body).map_err(|e| format!("release 元数据解析失败: {e}"))
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("403") || msg.to_lowercase().contains("rate limit") {
                eprintln!("reader: api.github.com 直连受限，改用 gh api（认证通道）");
                gh_api("/repos/raystyle/reader_rs/releases/latest")
            } else if msg.contains("404") {
                Err("尚无正式 release（未封版）".to_string())
            } else {
                Err(format!("查询 release 失败: {msg}"))
            }
        }
    }
}

/// gh api 兜底（认证通道）。
fn gh_api(path: &str) -> Result<Value, String> {
    let out = std::process::Command::new("gh")
        .arg("api")
        .arg(path)
        .output()
        .map_err(|e| format!("gh 不可用，无法回退 gh api: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "gh api 失败: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    serde_json::from_slice(&out.stdout).map_err(|e| format!("gh api 输出解析失败: {e}"))
}

/// 下载为内存字节。
fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let mut resp = ureq::get(url)
        .header("User-Agent", UA)
        .call()
        .map_err(|e| format!("下载失败 {url}: {e}"))?;
    resp.body_mut()
        .with_config()
        .limit(DOWNLOAD_LIMIT)
        .read_to_vec()
        .map_err(|e| format!("读取下载响应失败 {url}: {e}"))
}

/// 解包回调：条目名加内容字节。
type UnpackWant<'a> = dyn FnMut(&str, &[u8]) -> Result<(), String> + 'a;

/// 解包资产，取 reader 与 rr 两个二进制写入 `dir`，返回路径清单。
fn extract_bins(blob: &[u8], dir: &Path) -> Result<Vec<PathBuf>, String> {
    let names = if cfg!(windows) {
        ["reader.exe", "rr.exe"]
    } else {
        ["reader", "rr"]
    };
    let mut found: Vec<PathBuf> = Vec::new();
    let mut want = |name: &str, bytes: &[u8]| -> Result<(), String> {
        if names.contains(&name) {
            let dest = dir.join(name);
            std::fs::write(&dest, bytes).map_err(|e| format!("解包写 {name} 失败: {e}"))?;
            found.push(dest);
        }
        Ok(())
    };
    unpack(blob, &mut want)?;
    if !names.iter().any(|n| dir.join(n).is_file()) {
        return Err("资产中未找到 reader 二进制（打包形态与预期不符）".to_string());
    }
    Ok(found)
}

#[cfg(windows)]
fn unpack(blob: &[u8], want: &mut UnpackWant<'_>) -> Result<(), String> {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(blob))
        .map_err(|e| format!("zip 解包失败: {e}"))?;
    for i in 0..zip.len() {
        let mut entry = zip
            .by_index(i)
            .map_err(|e| format!("zip 读条目失败: {e}"))?;
        let name = entry.name().rsplit('/').next().unwrap_or("").to_string();
        if entry.is_dir() || name.is_empty() {
            continue;
        }
        let mut buf = Vec::new();
        std::io::Read::read_to_end(&mut entry, &mut buf).map_err(|e| format!("zip 读失败: {e}"))?;
        want(&name, &buf)?;
    }
    Ok(())
}

#[cfg(not(windows))]
fn unpack(blob: &[u8], want: &mut UnpackWant<'_>) -> Result<(), String> {
    let tar = flate2::read::GzDecoder::new(blob);
    let mut archive = tar::Archive::new(tar);
    let entries = archive
        .entries()
        .map_err(|e| format!("tar 解包失败: {e}"))?;
    for entry in entries {
        let mut entry = entry.map_err(|e| format!("tar 读条目失败: {e}"))?;
        if !entry.header().entry_type().is_file() {
            continue;
        }
        let path = entry
            .path()
            .map_err(|e| format!("tar 读路径失败: {e}"))?
            .into_owned();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let mut buf = Vec::new();
        std::io::Read::read_to_end(&mut entry, &mut buf).map_err(|e| format!("tar 读失败: {e}"))?;
        want(&name, &buf)?;
    }
    Ok(())
}

/// 替换自身与兄弟二进制：staged 加 rename（Windows 运行中 exe 改名让位，失败回滚）。
fn replace_all(bins: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
    let exe = std::env::current_exe().map_err(|e| format!("定位自身 exe 失败: {e}"))?;
    let self_name = exe
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "自身 exe 文件名非 UTF-8".to_string())?
        .to_string();
    // 自身必须换；兄弟（reader/rr 双名）存在即一并换，没有就算了。
    let sibling = if self_name.starts_with("rr") {
        if cfg!(windows) {
            "reader.exe"
        } else {
            "reader"
        }
    } else if cfg!(windows) {
        "rr.exe"
    } else {
        "rr"
    };
    let sibling_path = exe.with_file_name(sibling);

    let mut replaced = Vec::new();
    let self_bin = bins
        .iter()
        .find(|b| b.file_name().and_then(|s| s.to_str()) == Some(self_name.as_str()))
        .ok_or_else(|| format!("资产中无与自身同名的二进制 {self_name}"))?;
    replace_one(&exe, self_bin)?;
    replaced.push(exe.clone());

    if sibling_path.is_file() {
        if let Some(bin) = bins
            .iter()
            .find(|b| b.file_name().and_then(|s| s.to_str()) == Some(sibling))
        {
            match replace_one(&sibling_path, bin) {
                Ok(()) => replaced.push(sibling_path),
                Err(e) => eprintln!("reader: 兄弟二进制 {sibling} 替换失败（不影响自身升级）: {e}"),
            }
        }
    }
    Ok(replaced)
}

/// 单件替换：新件先写同目录 staged，再 rename 覆盖（Windows 先把运行中旧件改名让位）。
fn replace_one(target: &Path, new_bin: &Path) -> Result<(), String> {
    let pid = std::process::id();
    let staged = target.with_file_name(format!(
        ".{}.new-{pid}",
        target.file_name().and_then(|s| s.to_str()).unwrap_or("bin")
    ));
    std::fs::copy(new_bin, &staged).map_err(|e| format!("写临时文件失败: {e}"))?;
    #[cfg(windows)]
    {
        let old = target.with_file_name(format!(
            ".{}.old-{pid}",
            target.file_name().and_then(|s| s.to_str()).unwrap_or("bin")
        ));
        let _ = std::fs::remove_file(&old); // 上次升级残留（进程已退出才删得掉）
        std::fs::rename(target, &old).map_err(|e| format!("改名旧 exe 失败: {e}"))?;
        if let Err(e) = std::fs::rename(&staged, target) {
            // 回滚：旧件改回来，不留半损状态
            let _ = std::fs::rename(&old, target);
            return Err(format!("替换 exe 失败（已回滚）: {e}"));
        }
    }
    #[cfg(not(windows))]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&staged, std::fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("chmod 755 失败: {e}"))?;
        std::fs::rename(&staged, target).map_err(|e| format!("替换二进制失败: {e}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_target_known_or_explicit_error() {
        match asset_target() {
            Ok(t) => assert!(t.contains('-'), "资产目标应为 target 三元组: {t}"),
            Err(e) => assert!(e.contains("无 release 构建资产")),
        }
    }

    #[test]
    fn asset_name_follows_release_naming() {
        let name = asset_name("0.2.1").unwrap();
        assert!(name.starts_with("reader-v0.2.1-"), "资产名带版本: {name}");
        if cfg!(windows) {
            assert!(name.ends_with(".zip"), "Windows 资产 zip: {name}");
        } else {
            assert!(name.ends_with(".tar.gz"), "非 Windows 资产 tar.gz: {name}");
        }
    }

    #[test]
    fn version_compare() {
        assert!(version_newer("0.2.2", "0.2.1"));
        assert!(version_newer("0.10.0", "0.9.9"));
        assert!(!version_newer("0.2.1", "0.2.1"));
        assert!(!version_newer("0.2.1", "0.2.2"));
        assert!(version_newer("1.0.0", "0.9.9"));
        assert!(!version_newer("0.2.1-rc1", "0.2.1"));
    }
}
