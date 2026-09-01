# P0013-musl静态Linux资产

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：进行中
- 日期：2026-09-01
- 关联：TODO.md / 前序方案 `docs\proven\P0008-封版v0.1与三端二进制release.md`（流水线）与 `docs\proven\P0009-anydoc统一文档引擎大重构.md`（引入 zstd-sys C 编译，musl 链路首验）

## 背景与问题

发布面挂账候选「musl 静态」点名：产出 x86_64-unknown-linux-musl 静态二进制资产，覆盖 Alpine / 容器 / 最小镜像场景。anydoc 传递依赖 zstd-sys 的 C 编译让 musl 工具链成为首个真实验证点。

## 目标与非目标

- 目标：
  - release 流水线矩阵增 `x86_64-unknown-linux-musl`（ubuntu runner，apt 装 musl-tools 后交叉构建）。
  - 资产命名与既有规则一致：`reader-v<版本>-x86_64-unknown-linux-musl.tar.gz` 加 sha256。
  - README 资产表补 musl 行。
- 非目标：
  - 不做 aarch64-musl（无真实需求前不加矩阵）；不动 CI 三系统门禁（仍 gnu 测）；不做镜像内验证（以 run 绿与资产 ldd 不可用即静态为口径——见验收）。

## 方案

`.github\workflows\release.yml`：matrix 增一项；Linux musl 目标前加一步 `apt-get install -y musl-tools`（条件 `matrix.target == 'x86_64-unknown-linux-musl'`）；构建命令与打包步骤复用（tar.gz 分支已按 runner.os 分流，musl 走 Linux 路径）。

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| ubuntu + musl-tools 交叉（选定） | 与 Intel mac 交叉编译同思路（M004 先例），runner 现成 |
| Alpine 容器跑构建 | 引 container 增流水线复杂度，无必要，弃 |

## 实施步骤

1. 立项登记（本文件）。
2. release.yml matrix 与 apt 步骤；README 资产表。
3. 验收：v0.2.1 tag 首跑（musl job 绿、资产齐）——本方案与 P0011/P0012 合并一个版本发布时一并验证。

## 风险与回滚

- 风险：zstd-sys 在 musl 工具链下编译失败。缓解：musl-gcc 为标准 cc 目标；若失败记 M 并评估 zip 特性裁剪。回滚：matrix 减一行。

## 实施过程与经验

> 完成时补全，不是留空。

## 验收标准

- release run 五 job 全绿（四存量加 musl），9 资产齐（8 加 musl 双件）。
- CHANGELOG 记 musl 资产。
