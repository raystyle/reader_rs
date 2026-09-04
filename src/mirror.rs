//! 镜像源链与镜像清单(D42):OCR 模型三级回退下载(镜像 到 HF 直连 到 GitHub
//! Releases 模型 tag)与 self update 的 latest.json 通道共用面。裁决与规范见
//! ISSUE #1(ohmycloud S009):清单只发一份(镜像侧为真),兜底只兜资产可用性,
//! 两渠道资产共用同一 sha256(源=ppocr-rs 内嵌钉死值)。
//! 接入形态为预取入缓存目录:ppocr-rs 公开的 `resolve_pair(Offline)`/`verify()`
//! 会按其内嵌 models.json 全量 sha256 校验并补缓存标记,是最终校验闸;本模块的
//! pin 表校验只是前置层(坏件不落缓存,免得离线解析当场才红)。

use ppocr_rs::{ModelKind, ModelSize};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// 镜像缺省基址(ISSUE #1:reader.ohmygh.com,R2 桶 reader-dl 自定义域)。
const DEFAULT_MIRROR: &str = "https://reader.ohmygh.com";
/// ppocr-rs 依赖钉死的 git rev(Cargo.toml)。pin 表抄自该 rev 的 models.json,
/// 升 rev 必须同步换表(单测 pins_match_pinned_ppocr_rs_rev 钉住)。
pub const PPOCR_RS_REV: &str = "d07857c35457f90bb8df92e245f614cdc3d5e236";
/// GitHub Releases 模型兜底 tag;恒 prerelease,防遮蔽 /releases/latest
/// (正式 release 会霸占 latest,self update 永远判已最新)。
const GH_MODELS_TAG: &str = "models-v6";
/// 模型件最大约 21.2MB(small-rec weights),留三倍余量;ureq 3 默认 10MB 上限(M009)。
const FILE_LIMIT: u64 = 64 * 1024 * 1024;
/// latest.json 清单体积极小,1MB 封顶。
const MANIFEST_LIMIT: u64 = 1024 * 1024;

/// 镜像基址:env `READER_MIRROR` 覆盖(测试与自建源用),去尾斜杠。
pub fn mirror_base() -> String {
    mirror_base_from(std::env::var("READER_MIRROR").ok())
}

/// `mirror_base` 的纯函数核(单测用,不碰进程 env)。
fn mirror_base_from(env: Option<String>) -> String {
    env.map(|v| v.trim().trim_end_matches('/').to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| DEFAULT_MIRROR.to_string())
}

/// 模型包钉死件:名字、字节数、sha256(值抄 ppocr-rs models.json @ PPOCR_RS_REV)。
pub struct FilePin {
    pub name: &'static str,
    pub bytes: u64,
    pub sha256: &'static str,
}

/// 模型包钉死元数据:缓存目录名(`<size>-<kind>`)、HF 源仓与 revision。
pub struct PackagePin {
    pub name: &'static str,
    pub repo: &'static str,
    pub revision: &'static str,
    pub files: &'static [FilePin],
}

/// 两档四包(tiny/small 是 v6 档位;medium 不分发)。顺序即 doctor 报告序。
pub static PACKAGES: &[PackagePin] = &[
    PackagePin {
        name: "tiny-det",
        repo: "PaddlePaddle/PP-OCRv6_tiny_det_safetensors",
        revision: "07595f982703daf0d4e120a12a01da8073542f3a",
        files: &[
            FilePin {
                name: "model.safetensors",
                bytes: 1_786_412,
                sha256: "cae3c88d2a9902fd0293e6b17990428f54bfa7ec98f800a4368e95423a754d16",
            },
            FilePin {
                name: "config.json",
                bytes: 1_085,
                sha256: "911288a948ee0aa617bd70cdd299315b990db08c611aebea8afb087d041d7d36",
            },
            FilePin {
                name: "inference.yml",
                bytes: 883,
                sha256: "3ac018be6f97499a08faa3bbdeb33640968d9307f6736d152902747a9f259593",
            },
            FilePin {
                name: "preprocessor_config.json",
                bytes: 838,
                sha256: "74421569a28a78d417db320f9bc039ce0997a1defe2b35f259dfc74299c9f1ed",
            },
        ],
    },
    PackagePin {
        name: "tiny-rec",
        repo: "PaddlePaddle/PP-OCRv6_tiny_rec_safetensors",
        revision: "6f2d2d51b4b4226d7a2329a02f416f4994106f3a",
        files: &[
            FilePin {
                name: "model.safetensors",
                bytes: 4_474_264,
                sha256: "cc3892aba0fbd89afbf6a76d8b7817bb58802668be7a1384ca761ce65612f3f7",
            },
            FilePin {
                name: "config.json",
                bytes: 788,
                sha256: "d8a130cc18e1833aa5f352e057b5cd0e254c46efc5709823f73d53a8b5a42969",
            },
            FilePin {
                name: "inference.yml",
                bytes: 55_571,
                sha256: "66170210bad538e83fff3c4a3867e547d6bf20b50d64b20347c4b913f3034ea1",
            },
            FilePin {
                name: "preprocessor_config.json",
                bytes: 75_758,
                sha256: "196f42619e2e9e0e93a6dc1a622fa1c664628e1c63ce116b376275b3e192125f",
            },
        ],
    },
    PackagePin {
        name: "small-det",
        repo: "PaddlePaddle/PP-OCRv6_small_det_safetensors",
        revision: "eae2ee920a39fb3087637d3dbb58df1896ec1f24",
        files: &[
            FilePin {
                name: "model.safetensors",
                bytes: 9_938_124,
                sha256: "89a96a8adc4e9cd0c994098edc76022e496d35844392562b4694c8fbc583f2da",
            },
            FilePin {
                name: "config.json",
                bytes: 1_096,
                sha256: "b42d76ab6325d5234e1afc9987b73dc592b0ec8437c8a29d46ba6209b79252eb",
            },
            FilePin {
                name: "inference.yml",
                bytes: 885,
                sha256: "193f435274bf9f0b5f71a929bbfbcf148282df7e633b34e7c373e8f44741b516",
            },
            FilePin {
                name: "preprocessor_config.json",
                bytes: 838,
                sha256: "74421569a28a78d417db320f9bc039ce0997a1defe2b35f259dfc74299c9f1ed",
            },
        ],
    },
    PackagePin {
        name: "small-rec",
        repo: "PaddlePaddle/PP-OCRv6_small_rec_safetensors",
        revision: "fe049fb103f57443fe8840c54ed06b702f3c1de5",
        files: &[
            FilePin {
                name: "model.safetensors",
                bytes: 21_204_736,
                sha256: "f65a332afe5aa663f0b9d5706f4ae8457b5b4058a842d5c1eb22df505c27d642",
            },
            FilePin {
                name: "config.json",
                bytes: 1_083,
                sha256: "8693fd8485e8543e13d0f6dde3891a3f4af9a47fc4e3a391fbed71f8015c899e",
            },
            FilePin {
                name: "inference.yml",
                bytes: 150_579,
                sha256: "ab078671bb49f06228eadccd34f1bb501e157f7a047095ffb943ba81512c77d1",
            },
            FilePin {
                name: "preprocessor_config.json",
                bytes: 206_177,
                sha256: "2b24fa36f548893f26a931cf44f1a1a6b2b14ed7e2f8f8e28e6848801a8278db",
            },
        ],
    },
];

/// 按档位与角色取包(tiny/small × det/rec 四包之外无分发)。
pub fn package_pin(size: ModelSize, kind: ModelKind) -> Result<&'static PackagePin, String> {
    let name = format!("{}-{}", size.as_str(), kind.as_str());
    PACKAGES
        .iter()
        .find(|p| p.name == name)
        .ok_or_else(|| format!("镜像源链不分发 {name}(只 tiny / small 两档)"))
}

/// 包在缓存根下的目录(`<root>/<size>-<kind>`,与 ppocr-rs 布局一致)。
pub fn package_dir(root: &Path, pin: &PackagePin) -> PathBuf {
    root.join(pin.name)
}

/// 镜像件地址:`<mirror>/models/<repo>/<rev>/<file>`(ISSUE #1 路径规范)。
pub fn mirror_file_url(pin: &PackagePin, file: &FilePin) -> String {
    format!(
        "{}/models/{}/{}/{}",
        mirror_base(),
        pin.repo,
        pin.revision,
        file.name
    )
}

/// HF 直连件地址(ppocr-rs 原生同款路径,302 到 CDN 由默认重定向跟)。
pub fn hf_file_url(pin: &PackagePin, file: &FilePin) -> String {
    format!(
        "https://huggingface.co/{}/resolve/{}/{}",
        pin.repo, pin.revision, file.name
    )
}

/// GitHub 模型资产名:扁平 `<包名>-<rev 前 12>-<文件名>`;字符集只用
/// `[A-Za-z0-9._-]`(GitHub 对特殊字符自动改名),带短 rev 免得同 tag 下
/// rev 更替时旧兜底件被 clobber;与 self update 的 `reader-v<版本>-<目标>`
/// 命名不相交。
pub fn gh_asset_name(pin: &PackagePin, file: &FilePin) -> String {
    format!("{}-{}-{}", pin.name, &pin.revision[..12], file.name)
}

/// GitHub 模型 tag 下载地址。
pub fn gh_file_url(pin: &PackagePin, file: &FilePin) -> String {
    format!(
        "https://github.com/raystyle/reader_rs/releases/download/{GH_MODELS_TAG}/{}",
        gh_asset_name(pin, file)
    )
}

/// 下载命中源(输出契约 `download mirror|huggingface|github` 的 token)。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source {
    Mirror,
    HuggingFace,
    Github,
}

impl Source {
    pub fn as_str(self) -> &'static str {
        match self {
            Source::Mirror => "mirror",
            Source::HuggingFace => "huggingface",
            Source::Github => "github",
        }
    }
}

const SOURCE_LABELS: [&str; 3] = ["mirror", "huggingface", "github"];

/// 探活/清单 agent:全局 10s(doctor 探活与 latest.json 拉取,挂死链快速失败)。
fn probe_agent() -> ureq::Agent {
    ureq::Agent::new_with_config(
        ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(10)))
            .build(),
    )
}

/// 下载 agent:连接 15s、收体 300s(大件慢链不断,坏链快速失败)。
fn download_agent() -> ureq::Agent {
    ureq::Agent::new_with_config(
        ureq::Agent::config_builder()
            .timeout_connect(Some(Duration::from_secs(15)))
            .timeout_recv_body(Some(Duration::from_secs(300)))
            .build(),
    )
}

fn ua() -> String {
    concat!("reader/", env!("CARGO_PKG_VERSION")).to_string()
}

/// 三级回退下载单件到 `dest`:镜像 到 HF 到 GitHub;逐源经 `.part` 临时件
/// (bytes 加 sha256 对 pin 表校验通过才 rename 落盘),返回命中的源。
/// 落盘前自建父目录(空缓存首用时包目录不存在,`fs::write` 对缺失目录是
/// os error 3;`ocr init` 首版实测踩中,下载器不依赖调用方建目录)。
/// 并发写不经 ppocr-rs 私有锁:单件 rename 原子,最坏并发方多做一次
/// 全量哈希,无半损态。
pub fn download_file(pin: &PackagePin, file: &FilePin, dest: &Path) -> Result<Source, String> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("建包目录失败: {e}"))?;
    }
    let urls = [
        mirror_file_url(pin, file),
        hf_file_url(pin, file),
        gh_file_url(pin, file),
    ];
    let mut last = String::new();
    for (i, url) in urls.iter().enumerate() {
        match fetch_to_part(url, file, dest) {
            Ok(()) => {
                return Ok(match i {
                    0 => Source::Mirror,
                    1 => Source::HuggingFace,
                    _ => Source::Github,
                })
            }
            Err(e) => {
                eprintln!(
                    "reader: {} 从 {} 下载失败: {e}",
                    file.name, SOURCE_LABELS[i]
                );
                last = e;
            }
        }
    }
    Err(format!("三通道全败: {last}"))
}

/// 单源下载到 `.part` 并校验后 rename 覆盖 `dest`。
fn fetch_to_part(url: &str, file: &FilePin, dest: &Path) -> Result<(), String> {
    let mut resp = download_agent()
        .get(url)
        .header("User-Agent", &ua())
        .call()
        .map_err(|e| format!("{e}"))?;
    let blob = resp
        .body_mut()
        .with_config()
        .limit(FILE_LIMIT)
        .read_to_vec()
        .map_err(|e| format!("读取响应失败: {e}"))?;
    if blob.len() as u64 != file.bytes {
        return Err(format!(
            "字节数 {} 与钉死值 {} 不符",
            blob.len(),
            file.bytes
        ));
    }
    let sha = format!("{:x}", Sha256::digest(&blob));
    if sha != file.sha256 {
        return Err("sha256 与钉死值不符".to_string());
    }
    let part = dest.with_file_name(format!(".{}.part", file.name));
    std::fs::write(&part, &blob).map_err(|e| format!("写临时件失败: {e}"))?;
    if dest.exists() {
        let _ = std::fs::remove_file(dest);
    }
    std::fs::rename(&part, dest).map_err(|e| format!("落盘失败: {e}"))
}

/// 单件只读判定:文件不存在为 Missing;存在但字节或 sha256 不符为 Corrupt。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileState {
    Ok,
    Missing,
    Corrupt,
}

/// 包级只读判定(取首个问题件点名);`root` 为缓存根。
#[derive(Debug, Eq, PartialEq)]
pub enum PackageVerdict {
    Ok,
    /// 缺件(点名第一个缺的)。
    Missing(String),
    /// 损件(字节或 sha256 不符,点名第一个坏的)。
    Corrupt(String),
}

/// 单件只读判定(init 逐件补齐与 doctor 都用;不写任何文件)。
pub fn assess_file(dir: &Path, file: &FilePin) -> FileState {
    let path = dir.join(file.name);
    let Ok(meta) = std::fs::metadata(&path) else {
        return FileState::Missing;
    };
    if !meta.is_file() || meta.len() != file.bytes || sha256_file(&path) != file.sha256 {
        return FileState::Corrupt;
    }
    FileState::Ok
}

/// 只读评估一包四件(doctor 用;不建目录不写任何文件,ppocr-rs 的
/// `verify()` 有建目录/取锁/写标记副作用,诊断禁用)。
pub fn assess_package(root: &Path, pin: &PackagePin) -> PackageVerdict {
    let dir = package_dir(root, pin);
    for file in pin.files {
        match assess_file(&dir, file) {
            FileState::Ok => {}
            FileState::Missing => return PackageVerdict::Missing(file.name.to_string()),
            FileState::Corrupt => return PackageVerdict::Corrupt(file.name.to_string()),
        }
    }
    PackageVerdict::Ok
}

/// 流式 sha256(只读文件)。
fn sha256_file(path: &Path) -> String {
    let Ok(mut reader) = std::fs::File::open(path) else {
        return String::new();
    };
    use std::io::Read;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => hasher.update(&buffer[..n]),
            Err(_) => return String::new(),
        }
    }
    format!("{:x}", hasher.finalize())
}

/// latest.json 单平台条目(signature 解析不验,minisign 首轮不上,字段保留)。
#[derive(Debug, Deserialize)]
pub struct LatestPlatform {
    pub url: String,
    pub sha256: String,
    pub signature: Option<String>,
}

/// latest.json 清单(Tauri v2 形状;platforms key 即 selfupdate 的资产目标三元组)。
#[derive(Debug, Deserialize)]
pub struct LatestManifest {
    pub version: String,
    #[allow(dead_code)]
    pub pub_date: Option<String>,
    pub platforms: std::collections::BTreeMap<String, LatestPlatform>,
}

/// 镜像升级清单地址:`<mirror>/reader/latest.json`。
pub fn latest_json_url() -> String {
    format!("{}/reader/latest.json", mirror_base())
}

/// 拉取并解析镜像升级清单(10s 全局超时;任何失败由调用方回退 GitHub 通道)。
pub fn fetch_latest_manifest() -> Result<LatestManifest, String> {
    let url = latest_json_url();
    let mut resp = probe_agent()
        .get(&url)
        .header("User-Agent", &ua())
        .call()
        .map_err(|e| format!("{e}"))?;
    let body = resp
        .body_mut()
        .with_config()
        .limit(MANIFEST_LIMIT)
        .read_to_vec()
        .map_err(|e| format!("读取清单失败: {e}"))?;
    serde_json::from_slice(&body).map_err(|e| format!("清单解析失败: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_table_covers_two_tiers_four_packages() {
        assert_eq!(PACKAGES.len(), 4, "两档四包");
        for pin in PACKAGES {
            assert!(
                pin.name.starts_with("tiny-") || pin.name.starts_with("small-"),
                "只分发 tiny/small 档: {}",
                pin.name
            );
            assert_eq!(pin.files.len(), 4, "{} 应四件", pin.name);
            for name in [
                "model.safetensors",
                "config.json",
                "inference.yml",
                "preprocessor_config.json",
            ] {
                assert!(
                    pin.files.iter().any(|f| f.name == name),
                    "{name} 不在 {}",
                    pin.name
                );
            }
            assert!(
                pin.repo.starts_with("PaddlePaddle/PP-OCRv6_"),
                "{} 源仓系",
                pin.repo
            );
            assert_eq!(pin.revision.len(), 40, "{} rev 应全 40 位", pin.name);
            for file in pin.files {
                assert!(file.bytes > 0, "{} bytes 应为正", file.name);
                assert_eq!(file.sha256.len(), 64, "{} sha256 应 64 hex", file.name);
                assert!(file.sha256.chars().all(|c| c.is_ascii_hexdigit()));
            }
        }
    }

    /// rev 漂移闸:Cargo.toml 的 ppocr-rs rev 一动,此测即红,提醒重抄 pin 表。
    #[test]
    fn pins_match_pinned_ppocr_rs_rev() {
        let cargo = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
            .expect("读 Cargo.toml");
        let needle = format!(r#"rev = "{PPOCR_RS_REV}""#);
        assert!(
            cargo.contains(&needle),
            "Cargo.toml 不含 ppocr-rs rev {PPOCR_RS_REV};若升了依赖,PACKAGES 表须同步重抄"
        );
    }

    #[test]
    fn package_pin_maps_size_kind_to_package() {
        assert_eq!(
            package_pin(ModelSize::Tiny, ModelKind::Detector)
                .unwrap()
                .name,
            "tiny-det"
        );
        assert_eq!(
            package_pin(ModelSize::Small, ModelKind::Recognizer)
                .unwrap()
                .name,
            "small-rec"
        );
        assert!(package_pin(ModelSize::Medium, ModelKind::Detector).is_err());
    }

    #[test]
    fn url_builders_follow_issue_spec() {
        let pin = &PACKAGES[0];
        let file = &pin.files[0];
        let base = mirror_base_from(None);
        assert_eq!(
            mirror_file_url(pin, file),
            format!("{base}/models/{}/{}/{}", pin.repo, pin.revision, file.name)
        );
        assert_eq!(
            hf_file_url(pin, file),
            format!(
                "https://huggingface.co/{}/resolve/{}/{}",
                pin.repo, pin.revision, file.name
            )
        );
        let asset = gh_asset_name(pin, file);
        assert_eq!(
            asset,
            format!("{}-{}-{}", pin.name, &pin.revision[..12], file.name)
        );
        assert!(
            asset
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "._-".contains(c)),
            "GitHub 资产名只用安全字符集: {asset}"
        );
        assert!(gh_file_url(pin, file).ends_with(&format!("/download/{GH_MODELS_TAG}/{asset}")));
    }

    #[test]
    fn mirror_base_trims_trailing_slash_and_blank_env() {
        assert_eq!(mirror_base_from(None), "https://reader.ohmygh.com");
        assert_eq!(
            mirror_base_from(Some("https://x.example/".into())),
            "https://x.example"
        );
        assert_eq!(
            mirror_base_from(Some("  ".into())),
            "https://reader.ohmygh.com"
        );
        assert_eq!(
            mirror_base_from(Some("http://127.0.0.1:9".into())),
            "http://127.0.0.1:9"
        );
    }

    #[test]
    fn latest_manifest_parses_tauri_shape_with_optional_signature() {
        let raw = r#"{
            "version": "0.4.1",
            "pub_date": "2026-09-03T00:00:00Z",
            "platforms": {
                "x86_64-pc-windows-msvc": {
                    "url": "https://reader.ohmygh.com/reader/0.4.1/reader-v0.4.1-x86_64-pc-windows-msvc.zip",
                    "sha256": "0ec041fa38",
                    "signature": "minisign 可选"
                },
                "x86_64-unknown-linux-musl": {
                    "url": "https://reader.ohmygh.com/reader/0.4.1/reader-v0.4.1-x86_64-unknown-linux-musl.tar.gz",
                    "sha256": "aa"
                }
            }
        }"#;
        let manifest: LatestManifest = serde_json::from_str(raw).expect("解析");
        assert_eq!(manifest.version, "0.4.1");
        assert_eq!(manifest.platforms.len(), 2);
        assert_eq!(
            manifest.platforms["x86_64-pc-windows-msvc"]
                .signature
                .as_deref(),
            Some("minisign 可选")
        );
        assert!(manifest.platforms["x86_64-unknown-linux-musl"]
            .signature
            .is_none());
    }

    #[test]
    fn assess_package_reports_ok_missing_corrupt() {
        let dir = std::env::temp_dir().join(format!("reader-mirror-assess-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("syn-pkg")).expect("建目录");
        // 合成 pin:内容 abc,字节数 3,sha256 现算(表是 'static,测试造的字符串 leak 成 'static)
        let content = b"abc".as_slice();
        let sha: &'static str =
            Box::leak(format!("{:x}", Sha256::digest(content)).into_boxed_str());
        let files: &'static [FilePin] = Box::leak(
            vec![
                FilePin {
                    name: "model.safetensors",
                    bytes: 3,
                    sha256: sha,
                },
                FilePin {
                    name: "config.json",
                    bytes: 3,
                    sha256: sha,
                },
            ]
            .into_boxed_slice(),
        );
        let pin = PackagePin {
            name: "syn-pkg",
            repo: "x/y",
            revision: "0123456789abcdef0123456789abcdef01234567",
            files,
        };

        assert_eq!(
            assess_package(&dir, &pin),
            PackageVerdict::Missing("model.safetensors".into())
        );

        std::fs::write(dir.join("syn-pkg/model.safetensors"), content).unwrap();
        assert_eq!(
            assess_package(&dir, &pin),
            PackageVerdict::Missing("config.json".into())
        );

        std::fs::write(dir.join("syn-pkg/config.json"), b"xyz").unwrap();
        assert_eq!(
            assess_package(&dir, &pin),
            PackageVerdict::Corrupt("config.json".into())
        );

        std::fs::write(dir.join("syn-pkg/config.json"), content).unwrap();
        assert_eq!(assess_package(&dir, &pin), PackageVerdict::Ok);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
