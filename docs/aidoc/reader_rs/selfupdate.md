# reader_rs::selfupdate

self update（P0015；D42 加镜像通道）：`reader self update` 先读镜像
`reader.ohmygh.com/reader/latest.json`（Tauri v2 形状加 sha256；signature 字段
解析不验，minisign 首轮不上），任何失败回退 GitHub Releases API（403 限流再回退
gh api）。模式参考 ohmyenv-rs selfupdate.rs 与 ohmyagents-rs update.rs：
版本判新（stable 资产是压缩包，digest 与 exe 哈希不可比）、资产 sha256 钉死校验、
staged 加 rename 原子替换（Windows 运行中 exe 可改名不可删）。
边界：只 stable 通道（无 dev/git）；只显式命令不自动更新。

## Functions

- `asset_target` — 本编译目标对应的 release 资产名（release.yml 矩阵命名约定）。
- `self_update` — `reader self update` 主流程。`force` 为真时版本相同也重装。

## Types

- `Outcome` — 升级结果（lib.rs 拼稳定输出行用）。

