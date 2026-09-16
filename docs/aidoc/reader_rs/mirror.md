# reader_rs::mirror

镜像源链与镜像清单(D42):OCR 模型三级回退下载(镜像 到 HF 直连 到 GitHub
Releases 模型 tag)与 self update 的 latest.json 通道共用面。裁决与规范见
ISSUE #1(ohmycloud S009):清单只发一份(镜像侧为真),兜底只兜资产可用性,
两渠道资产共用同一 sha256(源=ppocr-rs 内嵌钉死值)。
接入形态为预取入缓存目录:ppocr-rs 公开的 `resolve_pair(Offline)`/`verify()`
会按其内嵌 models.json 全量 sha256 校验并补缓存标记,是最终校验闸;本模块的
pin 表校验只是前置层(坏件不落缓存,免得离线解析当场才红)。

## Functions

- `assess_file` — 单件只读判定(init 逐件补齐与 doctor 都用;不写任何文件)。
- `assess_package` — 只读评估一包四件(doctor 用;不建目录不写任何文件,ppocr-rs 的
- `download_file` — 三级回退下载单件到 `dest`:镜像 到 HF 到 GitHub;逐源经 `.part` 临时件
- `fetch_latest_manifest` — 拉取并解析镜像升级清单(10s 全局超时;任何失败由调用方回退 GitHub 通道)。
- `gh_asset_name` — GitHub 模型资产名:扁平 `<包名>-<rev 前 12>-<文件名>`;字符集只用
- `gh_file_url` — GitHub 模型 tag 下载地址。
- `hf_file_url` — HF 直连件地址(ppocr-rs 原生同款路径,302 到 CDN 由默认重定向跟)。
- `latest_json_url` — 镜像升级清单地址:`<mirror>/reader/latest.json`。
- `mirror_base` — 镜像基址:env `READER_MIRROR` 覆盖(测试与自建源用),去尾斜杠。
- `mirror_file_url` — 镜像件地址:`<mirror>/models/<repo>/<rev>/<file>`(ISSUE #1 路径规范)。
- `package_dir` — 包在缓存根下的目录(`<root>/<size>-<kind>`,与 ppocr-rs 布局一致)。
- `package_pin` — 按档位与角色取包(tiny/small × det/rec 四包之外无分发)。

## Types

- `FilePin` — 模型包钉死件:名字、字节数、sha256(值抄 ppocr-rs models.json @ PPOCR_RS_REV)。
- `FileState` — 单件只读判定:文件不存在为 Missing;存在但字节或 sha256 不符为 Corrupt。
- `LatestManifest` — latest.json 清单(Tauri v2 形状;platforms key 即 selfupdate 的资产目标三元组)。
- `LatestPlatform` — latest.json 单平台条目(signature 解析不验,minisign 首轮不上,字段保留)。
- `PackagePin` — 模型包钉死元数据:缓存目录名(`<size>-<kind>`)、HF 源仓与 revision。
- `PackageVerdict` — 包级只读判定(取首个问题件点名);`root` 为缓存根。
- `Source` — 下载命中源(输出契约 `download mirror|huggingface|github` 的 token)。

## Constants

- `PACKAGES` — 两档四包(tiny/small 是 v6 档位;medium 不分发)。顺序即 doctor 报告序。
- `PPOCR_RS_REV` — ppocr-rs 依赖钉死的 git rev(Cargo.toml)。pin 表抄自该 rev 的 models.json,

