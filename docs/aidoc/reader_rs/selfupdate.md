# reader_rs::selfupdate

self update（P0015；D42 镜像通道；家族自更新统一标准批 REQ-059 对齐
build-release 公共契约第六节与 browse REQ-005 口径）：双通道成对回落
（镜像 latest.json 通道任一步传输失败，整对回落 GitHub；digest 锚不符
属安全问题硬拒不回落）、semver 只升不降（本地领先报 local_newer 不动）、
自替换三步舞（旧件挪 pid 备份、新件入位、`--version` 自证五次重试防杀软
瞬时锁；证败回滚并复核终态，回滚受阻报自救路径）、exe 旁更新锁（create_new
语义加 pid 陈旧收割）加暂存落 exe 同目录（防跨文件系统 rename）、ark 管理
布局拦截走 ark 单通道。镜像通道锚源是 latest.json 内嵌 sha256（Tauri 形状，
与 browse 的 `.sha256` 边车是键形差异，判据同为 digest 锚硬校验）。
边界：只 stable 通道（无 dev/git）；只显式命令不自动更新。

## Functions

- `asset_target` — 本编译目标对应的 release 资产名（release.yml 矩阵命名约定）。
- `self_update` — `reader self update` 主流程（家族自更新统一标准件）。

## Types

- `Outcome` — 升级结果（lib.rs 拼稳定输出行用）。

