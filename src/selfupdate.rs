//! self update（P0015；D42 镜像通道；家族自更新统一标准批 REQ-059 对齐
//! build-release 公共契约第六节与 browse REQ-005 口径）：双通道成对回落
//! （镜像 latest.json 通道任一步传输失败，整对回落 GitHub；digest 锚不符
//! 属安全问题硬拒不回落）、semver 只升不降（本地领先报 local_newer 不动）、
//! 自替换三步舞（旧件挪 pid 备份、新件入位、`--version` 自证五次重试防杀软
//! 瞬时锁；证败回滚并复核终态，回滚受阻报自救路径）、exe 旁更新锁（create_new
//! 语义加 pid 陈旧收割）加暂存落 exe 同目录（防跨文件系统 rename）、ark 管理
//! 布局拦截走 ark 单通道。镜像通道锚源是 latest.json 内嵌 sha256（Tauri 形状，
//! 与 browse 的 `.sha256` 边车是键形差异，判据同为 digest 锚硬校验）。
//! 边界：只 stable 通道（无 dev/git）；只显式命令不自动更新。

use crate::mirror;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::{Path, PathBuf};

const REPO: &str = "raystyle/reader_rs";
const UA: &str = "reader-selfupdate";
/// reader 压缩包最大约 35MB 量级，留四倍余量。
const DOWNLOAD_LIMIT: u64 = 128 * 1024 * 1024;
/// 新件自证重试次数（杀软瞬时锁面，家族同形取五）。
const PROOF_RETRIES: usize = 5;

/// 升级结果（lib.rs 拼稳定输出行用）。
#[derive(Debug)]
pub struct Outcome {
    /// current：已是最新；local_newer：本地领先不动；updated：已替换
    pub action: &'static str,
    /// 当前版本。
    pub current: String,
    /// 查得的最新版本。
    pub latest: String,
    /// 已替换的二进制路径（reader 与 rr 双名）。
    pub replaced: Vec<PathBuf>,
}

/// 本编译目标对应的 release 资产名（release.yml 矩阵命名约定）。
///
/// # Errors
///
/// 编译目标不在五平台 release 矩阵内（无对应资产可下载；错误带源码安装出口）。
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
    Err(
        "当前平台无 release 构建资产（release.yml 矩阵未覆盖此目标）；\
         可源码装：cargo install --git https://github.com/raystyle/reader_rs"
            .to_string(),
    )
}

fn asset_name(version: &str) -> Result<String, String> {
    let ext = if cfg!(windows) { "zip" } else { "tar.gz" };
    Ok(format!("reader-v{version}-{}.{ext}", asset_target()?))
}

/// 升级目标描述：镜像 latest.json 与 GitHub API 两条通道都归一到此，主流程只消费它。
#[derive(Debug)]
struct ReleaseInfo {
    version: String,
    asset_name: String,
    sha256: String,
    url: String,
}

/// 判新三态：升、已最新、本地领先（semver 只升不降）。
enum Freshness {
    /// latest 严格大于 current，可升。
    Update,
    /// 字面相等（`--force` 仍可重装）。
    Current,
    /// 本地领先（latest 不大于 current 且不等），不动。
    LocalNewer,
}

fn freshness(latest: &str, current: &str) -> Freshness {
    if latest == current {
        return Freshness::Current;
    }
    if version_newer(latest, current) {
        Freshness::Update
    } else {
        Freshness::LocalNewer
    }
}

/// 镜像通道成对件：latest.json 清单（判新加 sha256 锚同源自带）加清单所指资产。
/// 任一步失败即该通道整对作废（防判新与下载混源）。
fn mirror_pair() -> Result<(ReleaseInfo, Vec<u8>), String> {
    let info = fetch_latest_from_mirror()?;
    let expected = asset_name(&info.version)?;
    if info.asset_name != expected {
        return Err(format!(
            "清单资产名 {actual} 与预期 {expected} 不符",
            actual = info.asset_name
        ));
    }
    eprintln!("reader: 镜像通道取 {}", info.asset_name);
    let blob = fetch(&info.url)?;
    Ok((info, blob))
}

/// GitHub 通道成对件：release API（tag 加资产 digest 锚）加 download 资产。
fn github_pair() -> Result<(ReleaseInfo, Vec<u8>), String> {
    let info = fetch_latest_from_github()?;
    let blob = fetch(&info.url)?;
    Ok((info, blob))
}

/// 双通道成对取「信息加资产」：镜像整对优先，任一步传输失败整对回落 GitHub；
/// digest 锚校验在通道裁决之后统一做，不符属安全问题**硬拒不回落**。
fn fetch_channels() -> Result<(ReleaseInfo, Vec<u8>), String> {
    let (info, blob) = match mirror_pair() {
        Ok(pair) => pair,
        Err(e) => {
            eprintln!("reader: 镜像通道不可用，整对回落 GitHub（{e}）");
            github_pair().map_err(|e2| {
                format!("双通道皆不可用：镜像（{e}）；GitHub（{e2}）；下一步：重试一次，仍失败反馈 reader issue new")
            })?
        }
    };
    let sha = format!("{:x}", Sha256::digest(&blob));
    if sha != info.sha256 {
        return Err(format!(
            "资产 {} 校验失败（硬拒不回落）: 期望 {} 实得 {sha}；下一步：重试一次，仍失败反馈 reader issue new",
            info.asset_name, info.sha256
        ));
    }
    Ok((info, blob))
}

/// 镜像通道：latest.json 取本平台条目。
fn fetch_latest_from_mirror() -> Result<ReleaseInfo, String> {
    release_info_from_manifest(mirror::fetch_latest_manifest()?)
}

/// 清单到 ReleaseInfo 的纯映射（单测用）。版本或 sha256 形状不对按失败处理
/// （通道作废回退 GH），免得坏清单把判新短路成「已是最新」。
fn release_info_from_manifest(manifest: mirror::LatestManifest) -> Result<ReleaseInfo, String> {
    let version = manifest.version;
    if !version.starts_with(|c: char| c.is_ascii_digit()) {
        return Err(format!("清单版本号不合法: {version}"));
    }
    let target = asset_target()?;
    let platform = manifest
        .platforms
        .get(target)
        .ok_or_else(|| format!("latest.json 无本平台 {target} 条目"))?;
    let sha256 = platform.sha256.trim().to_lowercase();
    if sha256.len() != 64 || !sha256.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("latest.json sha256 形状不合法".to_string());
    }
    let asset_name = platform
        .url
        .rsplit('/')
        .next()
        .ok_or("latest.json url 无文件名")?
        .to_string();
    Ok(ReleaseInfo {
        version,
        asset_name,
        sha256,
        url: platform.url.clone(),
    })
}

/// GitHub API 通道：api.github.com 直连（token 注入），403/限流回退 gh api。
fn fetch_latest_from_github() -> Result<ReleaseInfo, String> {
    let release = fetch_latest_release()?;
    let tag = release
        .get("tag_name")
        .and_then(Value::as_str)
        .ok_or_else(|| "release 无 tag_name".to_string())?;
    let version = tag.strip_prefix('v').unwrap_or(tag).to_string();
    let name = asset_name(&version)?;
    let assets = release
        .get("assets")
        .and_then(Value::as_array)
        .ok_or_else(|| "release 无资产列表".to_string())?;
    let asset = assets
        .iter()
        .find(|a| a.get("name").and_then(Value::as_str) == Some(name.as_str()))
        .ok_or_else(|| format!("release 缺资产 {name}（CI 是否已跑完？）"))?;
    let sha256 = asset
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
    Ok(ReleaseInfo {
        version,
        asset_name: name,
        sha256,
        url,
    })
}

/// `reader self update` 主流程（家族自更新统一标准件）。
///
/// ark 管理布局先拦（走 ark 单通道）；exe 旁更新锁加陈旧收割；双通道成对
/// 取件（镜像 latest.json 优先整对回落 GitHub，digest 锚硬校验）；semver
/// 只升不降（本地领先报 local_newer 不动，`force` 跳过判新重装）；暂存落
/// exe 同目录，替换走三步舞（备份、入位、`--version` 自证五次重试，证败
/// 回滚复核）。只走 stable 通道，不做自动更新。
///
/// # Errors
///
/// ark 管理或另一升级在跑（带 CTA）；双通道查新或下载皆失败；digest 校验
/// 不符（硬拒）；平台无资产；解包或替换失败（含自证失败已回滚与回滚受阻
/// 带自救路径）；错误串带阶段与原因。
pub fn self_update(force: bool) -> Result<Outcome, String> {
    let current = env!("CARGO_PKG_VERSION").to_string();
    let exe = std::env::current_exe().map_err(|e| format!("定位自身 exe 失败: {e}"))?;
    if let Some(signal) = ark_managed_signal(&exe) {
        return Err(format!(
            "检测到管理方布局或用户面链接入口（{signal}）；升级建议走 ark update reader（管理方滚 catalog pin）；如确为自管安装，删该链接或落痕后再 self update"
        ));
    }
    let dir = exe
        .parent()
        .ok_or("exe 无父目录，无法自更新")?
        .to_path_buf();
    let _guard = acquire_lock(&exe)?;
    sweep_stale(&dir);

    let (info, blob) = fetch_channels()?;
    let latest = info.version;
    if !force {
        match freshness(&latest, &current) {
            Freshness::Current => {
                return Ok(Outcome {
                    action: "current",
                    current,
                    latest,
                    replaced: Vec::new(),
                })
            }
            Freshness::LocalNewer => {
                return Ok(Outcome {
                    action: "local_newer",
                    current,
                    latest,
                    replaced: Vec::new(),
                })
            }
            Freshness::Update => {}
        }
    }

    let stage = dir.join(format!(".reader-selfupd-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&stage);
    std::fs::create_dir_all(&stage).map_err(|e| format!("建暂存目录失败: {e}"))?;
    let result = extract_bins(&blob, &stage)
        .and_then(|bins| replace_all(&bins, &exe, &latest))
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

/// 取最新正式版元数据：直连 api.github.com（GH_TOKEN 或 GITHUB_TOKEN 注入），
/// 403/限流回退 gh api。
fn fetch_latest_release() -> Result<Value, String> {
    let url = format!("https://api.github.com/repos/{REPO}/releases/latest");
    let mut req = ureq::get(&url)
        .header("User-Agent", UA)
        .header("Accept", "application/vnd.github+json");
    let token = std::env::var("GH_TOKEN")
        .or_else(|_| std::env::var("GITHUB_TOKEN"))
        .ok();
    if let Some(tok) = token {
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
        return Err("资产中未找到 reader 二进制（打包形态与预期不符）；下一步：重试一次，仍失败反馈 reader issue new".to_string());
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
        let mut entry = entry.map_err(|e| format!("tar 读失败: {e}"))?;
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

/// 管理方布局判据（家族同形，落痕生产者契约在 ark 侧）：exe 同目录
/// `ark-managed` 落痕，或用户面 bin 目录存在指向本 exe 的符号链接入口。
/// 返回命中物描述供错误自诊。
fn ark_managed_signal(exe: &Path) -> Option<String> {
    if let Some(d) = exe.parent() {
        let marker = d.join("ark-managed");
        if marker.exists() {
            return Some(format!("落痕 {}", marker.display()));
        }
    }
    let faces = vec![exe.parent().map(Path::to_path_buf), home_local_bin()];
    faces
        .into_iter()
        .flatten()
        .filter_map(|d| std::fs::read_dir(&d).ok())
        .flat_map(|rd| rd.flatten().map(|e| e.path()))
        .filter_map(|p| {
            let t = std::fs::read_link(&p).ok()?;
            match t.canonicalize() {
                Ok(cwd) if exe == cwd => Some(format!("链接 {} -> {}", p.display(), t.display())),
                _ => None,
            }
        })
        .next()
}

fn home_local_bin() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|h| h.join(".local").join("bin"))
}

/// exe 旁的更新锁路径（create_new 语义，占用即另一升级在跑）。
fn lock_path(exe: &Path) -> PathBuf {
    exe.with_file_name(".reader-selfupdate.lock")
}

/// 更新锁守卫：drop 时清锁件（盖 panic 面；SIGKILL 面靠 [`acquire_lock`]
/// 的 pid 陈旧收割）。
#[derive(Debug)]
struct LockGuard {
    path: PathBuf,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// 锁内 pid 是否仍活（linux 走 /proc；他端保守判活不收割）。
fn pid_alive(pid: u32) -> bool {
    #[cfg(target_os = "linux")]
    {
        std::path::Path::new(&format!("/proc/{pid}")).exists()
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = pid;
        true
    }
}

/// 取更新锁：create_new 语义。AlreadyExists 先做 pid 陈旧判据（持有者已死
/// 即收割重取一次，防 SIGKILL 永久锁死）；其余 io 错误报真因（安装位不可写
/// 等），不误报「在跑」。
fn acquire_lock(exe: &Path) -> Result<LockGuard, String> {
    let lock = lock_path(exe);
    let take = || -> std::io::Result<std::fs::File> {
        let mut f = std::fs::File::create_new(&lock)?;
        let _ = writeln!(f, "{}", std::process::id());
        Ok(f)
    };
    match take() {
        Ok(_) => {
            // 陈旧收割竞态：两进程同抢时后到者回读锁 pid 让位
            let owner = std::fs::read_to_string(&lock).unwrap_or_default();
            if owner.trim() == std::process::id().to_string() {
                Ok(LockGuard { path: lock })
            } else {
                Err(format!(
                    "另一 reader self update 正在跑（{}）；若确无升级在跑，删 {} 后重试",
                    lock.display(),
                    lock.display()
                ))
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            // 陈旧判据：锁内 pid 已死即收割重取一次
            let stale = std::fs::read_to_string(&lock)
                .ok()
                .and_then(|t| t.trim().parse::<u32>().ok())
                .is_some_and(|pid| !pid_alive(pid));
            if stale && std::fs::remove_file(&lock).is_ok() && take().is_ok() {
                return Ok(LockGuard { path: lock });
            }
            Err(format!(
                "另一 reader self update 正在跑（{}）；若确无升级在跑，删 {} 后重试",
                lock.display(),
                lock.display()
            ))
        }
        Err(e) => Err(format!(
            "建更新锁失败（{}）: {e}；下一步：确认安装位可写（系统目录需提权）",
            lock.display()
        )),
    }
}

/// 陈旧收割（持锁后调用，锁保证无并发升级，残留皆为死进程所留）：清 exe
/// 目录内 `.reader-selfupd-*` 暂存目录与 `.<bin>.new-` / `.old-` / `.bak-`
/// 前缀散件。返回收割件数。
fn sweep_stale(dir: &Path) -> usize {
    let bins = if cfg!(windows) {
        ["reader.exe", "rr.exe"]
    } else {
        ["reader", "rr"]
    };
    let Ok(rd) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut n = 0usize;
    for entry in rd.flatten() {
        let fname = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        if fname.starts_with(".reader-selfupd-") {
            if std::fs::remove_dir_all(&path).is_ok() {
                n += 1;
            }
        } else if bins.iter().any(|b| {
            fname.starts_with(&format!(".{b}.new-"))
                || fname.starts_with(&format!(".{b}.old-"))
                || fname.starts_with(&format!(".{b}.bak-"))
        }) && std::fs::remove_file(&path).is_ok()
        {
            n += 1;
        }
    }
    if n > 0 {
        eprintln!("reader: 收割陈旧升级残留 {n} 件");
    }
    n
}

/// pid 后缀的备份名（并行互踩面：各升级各的备份）。
fn bak_path(exe: &Path) -> PathBuf {
    let pid = std::process::id();
    exe.with_file_name(format!(
        ".{}.bak-{pid}",
        exe.file_name().and_then(|s| s.to_str()).unwrap_or("bin")
    ))
}

/// 回滚并复核终态：坏新件挪走、备份回位、确认 exe 在位；任何一步失败报
/// 精确自救路径。入位失败臂与自证失败臂共用。
fn rollback_and_verify(exe: &Path, bak: &Path, new_bin: &Path) -> Result<(), String> {
    let _ = std::fs::rename(exe, new_bin);
    match std::fs::rename(bak, exe) {
        Ok(()) if exe.is_file() => Ok(()),
        Ok(()) => Err(format!(
            "回滚后复核 exe 缺位（旧件 {} 与新件 {} 已挪离原位）；下一步：按在位件手动复原到 {}",
            bak.display(),
            new_bin.display(),
            exe.display()
        )),
        Err(e) => Err(format!(
            "回滚受阻（{e}）: 旧件在 {}，新件在 {}；下一步：手动复原 mv {} {} 后反馈 reader issue new",
            bak.display(),
            new_bin.display(),
            bak.display(),
            exe.display()
        )),
    }
}

/// 自替换单件并自证（家族三步舞）：旧件挪 pid 备份、新件入位、`--version`
/// 自证（重试五次防杀软瞬时锁）、证毕清备份；证败回滚并复核终态，回滚
/// 失败报自救路径。
///
/// # Errors
///
/// 备份或入位失败（入位失败臂已回滚复核）；自证失败且回滚成功；回滚自身
/// 失败（错误带备份与新件路径及手动复原命令）。
fn swap_and_proof(exe: &Path, new_bin: &Path, expect_version: &str) -> Result<(), String> {
    let bak = bak_path(exe);
    let _ = std::fs::remove_file(&bak); // 上轮残留（sweep 已清，双保险）
    std::fs::rename(exe, &bak).map_err(|e| {
        format!(
            "备份旧件失败（{} -> {}）: {e}",
            exe.display(),
            bak.display()
        )
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perm = std::fs::metadata(new_bin)
            .map_err(|e| format!("读新件权限失败: {e}"))?
            .permissions();
        perm.set_mode(0o755);
        std::fs::set_permissions(new_bin, perm).map_err(|e| format!("chmod 755 失败: {e}"))?;
    }
    if let Err(e) = std::fs::rename(new_bin, exe) {
        rollback_and_verify(exe, &bak, new_bin)?;
        return Err(format!("新件入位失败（已回滚并复核在位）: {e}"));
    }
    // 自证：杀软瞬时锁面重试五次（家族同形）
    let mut probe_ok = false;
    for i in 0..PROOF_RETRIES {
        if let Ok(out) = std::process::Command::new(exe).arg("--version").output() {
            if out.status.success() && String::from_utf8_lossy(&out.stdout).contains(expect_version)
            {
                probe_ok = true;
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(200 * (i as u64 + 1)));
    }
    if !probe_ok {
        rollback_and_verify(exe, &bak, new_bin)?;
        return Err(format!(
            "新件自证失败（--version 非 {expect_version}，已回滚并复核在位）；下一步：重试一次，仍失败反馈 reader issue new"
        ));
    }
    let _ = std::fs::remove_file(&bak);
    Ok(())
}

/// 替换自身与兄弟二进制：自身必换且自证不过即败；兄弟（reader/rr 双名）
/// 存在即一并换（同自证），失败降级为警示（不影响自身升级）。
fn replace_all(bins: &[PathBuf], exe: &Path, latest: &str) -> Result<Vec<PathBuf>, String> {
    let self_name = exe
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "自身 exe 文件名非 UTF-8".to_string())?
        .to_string();
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

    let self_bin = bins
        .iter()
        .find(|b| b.file_name().and_then(|s| s.to_str()) == Some(self_name.as_str()))
        .ok_or_else(|| format!("资产中无与自身同名的二进制 {self_name}"))?;
    swap_and_proof(exe, self_bin, latest)?;
    let mut replaced = vec![exe.to_path_buf()];

    if sibling_path.is_file() {
        if let Some(bin) = bins
            .iter()
            .find(|b| b.file_name().and_then(|s| s.to_str()) == Some(sibling))
        {
            match swap_and_proof(&sibling_path, bin, latest) {
                Ok(()) => replaced.push(sibling_path),
                Err(e) => eprintln!("reader: 兄弟二进制 {sibling} 替换失败（不影响自身升级）: {e}"),
            }
        }
    }
    Ok(replaced)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_target_known_or_explicit_error() {
        match asset_target() {
            Ok(t) => assert!(t.contains('-'), "资产目标应为 target 三元组: {t}"),
            Err(e) => {
                assert!(e.contains("无 release 构建资产"));
                assert!(e.contains("cargo install"), "无资产错误带源码 CTA: {e}");
            }
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

    /// 判新三态：字面等值已最新；严格更大可升；其余本地领先不动。
    #[test]
    fn freshness_three_way() {
        assert!(matches!(freshness("0.2.1", "0.2.1"), Freshness::Current));
        assert!(matches!(freshness("0.2.2", "0.2.1"), Freshness::Update));
        assert!(matches!(freshness("1.0.0", "0.9.9"), Freshness::Update));
        assert!(matches!(freshness("0.2.1", "0.2.2"), Freshness::LocalNewer));
        assert!(matches!(freshness("0.8.0", "0.8.1"), Freshness::LocalNewer));
        // 预发布 latest 对正式本地：非字面等值且不严格更大，本地领先不动
        assert!(matches!(
            freshness("0.2.1-rc1", "0.2.1"),
            Freshness::LocalNewer
        ));
    }

    fn fixture_manifest(sha256: &str, version: &str) -> mirror::LatestManifest {
        let target = asset_target().expect("测试机在 release 矩阵内");
        let ext = if cfg!(windows) { "zip" } else { "tar.gz" };
        let raw = format!(
            r#"{{
                "version": "{version}",
                "platforms": {{
                    "{target}": {{
                        "url": "https://reader.ohmygh.com/reader/{version}/reader-v{version}-{target}.{ext}",
                        "sha256": "{sha256}"
                    }}
                }}
            }}"#
        );
        serde_json::from_str(&raw).expect("fixture 应合法")
    }

    /// 镜像清单映射:本平台条目归一为 ReleaseInfo,sha256 归一小写。
    #[test]
    fn release_info_from_manifest_maps_current_platform() {
        let target = asset_target().expect("测试机在 release 矩阵内");
        let ext = if cfg!(windows) { "zip" } else { "tar.gz" };
        let sha_up = format!("0E041FA38{}", "0".repeat(55));
        let info = release_info_from_manifest(fixture_manifest(&sha_up, "0.4.1")).unwrap();
        assert_eq!(info.version, "0.4.1");
        assert_eq!(info.asset_name, format!("reader-v0.4.1-{target}.{ext}"));
        assert_eq!(info.sha256, sha_up.to_lowercase());
        assert!(info
            .url
            .starts_with("https://reader.ohmygh.com/reader/0.4.1/"));
    }

    /// 坏清单按失败处理(通道作废回退 GH 的前提):版本非数字、sha256 形状不对、缺本平台。
    #[test]
    fn dies_release_info_from_manifest_rejects_bad_shapes() {
        let good_sha = "e".repeat(64);
        let err = release_info_from_manifest(fixture_manifest(&good_sha, "beta")).unwrap_err();
        assert!(err.contains("版本号不合法"), "{err}");
        let err = release_info_from_manifest(fixture_manifest("abc123", "0.4.1")).unwrap_err();
        assert!(err.contains("sha256 形状"), "{err}");
        let mut no_platform = fixture_manifest(&good_sha, "0.4.1");
        no_platform.platforms.clear();
        let err = release_info_from_manifest(no_platform).unwrap_err();
        assert!(err.contains("无本平台"), "{err}");
    }

    /// 更新锁：空位可取、drop 即清、占用即报「正在跑」；linux 死 pid 陈旧收割。
    #[test]
    fn lock_acquire_conflict_and_stale_harvest() {
        let dir = std::env::temp_dir().join(format!("reader-lock-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let exe = dir.join("reader");
        std::fs::write(&exe, b"bin").unwrap();

        {
            let g = acquire_lock(&exe).expect("空位应取到锁");
            assert!(lock_path(&exe).is_file(), "锁件应存在");
            drop(g);
            assert!(!lock_path(&exe).exists(), "drop 应清锁");
        }

        std::fs::write(lock_path(&exe), std::process::id().to_string()).unwrap();
        let err = acquire_lock(&exe).unwrap_err();
        assert!(err.contains("正在跑"), "占用应报在跑: {err}");

        #[cfg(target_os = "linux")]
        {
            // 死 pid（/proc 不存在）即陈旧，收割重取
            std::fs::write(lock_path(&exe), "4000000").unwrap();
            let g = acquire_lock(&exe).expect("死 pid 锁应被收割");
            drop(g);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// 陈旧收割：只清本工具升级面残留（暂存目录与 .new/.old/.bak 散件），
    /// 不动正经文件与别家前缀。
    #[test]
    fn sweep_stale_harvests_only_upgrade_leftovers() {
        let dir = std::env::temp_dir().join(format!("reader-sweep-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        // 残留：暂存目录 + 双名 new/old/bak 散件（按平台取实名）
        let (r, rr) = if cfg!(windows) {
            ("reader.exe", "rr.exe")
        } else {
            ("reader", "rr")
        };
        std::fs::create_dir_all(dir.join(".reader-selfupd-111")).unwrap();
        std::fs::write(dir.join(".reader-selfupd-111/reader"), b"x").unwrap();
        for f in [
            format!(".{r}.new-111"),
            format!(".{rr}.new-222"),
            format!(".{r}.old-111"),
            format!(".{r}.bak-333"),
        ] {
            std::fs::write(dir.join(&f), b"x").unwrap();
        }
        // 保留件：正经二进制与无关隐藏件
        std::fs::write(dir.join(r), b"real").unwrap();
        std::fs::write(dir.join(".other.new-1"), b"x").unwrap();

        let n = sweep_stale(&dir);
        assert_eq!(n, 5, "暂存目录加四散件");
        assert!(dir.join(r).is_file(), "正经二进制不动");
        assert!(dir.join(".other.new-1").is_file(), "别家前缀不动");
        assert!(!dir.join(".reader-selfupd-111").exists(), "暂存目录已清");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// ark 让位：exe 同目录 ark-managed 落痕即拦（走网络前），CTA 指 ark。
    #[test]
    fn ark_managed_marker_intercepts_before_network() {
        let exe = std::env::current_exe().unwrap();
        let marker = exe.with_file_name("ark-managed");
        std::fs::write(&marker, b"ark").unwrap();
        let out = self_update(false);
        let _ = std::fs::remove_file(&marker);
        let err = out.expect_err("落痕在位应拦自更新");
        assert!(err.contains("ark update reader"), "CTA 走 ark: {err}");
        assert!(!lock_path(&exe).exists(), "拦截不落锁");
    }
}
