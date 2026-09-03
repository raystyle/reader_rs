# G007-Rust CLI工程基线-供稿要点逐项裁定

> 角色：**规范**：Rust 全平台 CLI 的工程基线在本仓的逐项裁定。来源：2026-09-03 用户供稿《现代 Rust CLI 共识：库先行、clap derive、进程级测试隔离、跨平台矩阵发布》（自称核对 2025-2026 社区实践与 clap / cargo-nextest / trycmd / cargo-dist 文档）。
> 裁定分四态：**已符合**（现状即如此）、**已落地**（本细则登记时落地）、**候选**（入 PRD 待澄清，有触发条件再动）、**不适用**（记原因防复问）。事实性断言标六态，标准见 G002。

## 一、项目结构

| 供稿要点 | 裁定 | 本仓现状与说明 |
| --- | --- | --- |
| 库先行、二进制薄壳（lib.rs 业务、main.rs 薄） | 已符合 | main.rs 双 bin 薄壳共用 lib `run()` [实证: src\main.rs] |
| clap derive 解析 | 已符合 | clap 4 derive（P0001 起） |
| 库内 thiserror、边界 anyhow 加 cause chain | 候选 | 现状 `Result<T, String>` 全仓统一，CLI 单层无错误分类痛点；触发条件：公开库 API 或错误分类需求出现。PRD D36 |
| tracing 加 RUST_LOG 诊断 | 候选 | 现状 eprintln 进 stderr（M010 教训：stdout 只出数据）；CLI 无守护进程，结构化日志收益低。PRD D36 |
| edition 2024 加 rust-toolchain.toml 钉死 | 候选 | 现 edition 2021；lan-linux 踩过工具链 1.97 ICE 坑（2026-09-03 diary），钉 toolchain 有真实价值。PRD D39 |

## 二、跨平台与发布

| 供稿要点 | 裁定 | 本仓现状与说明 |
| --- | --- | --- |
| 路径 PathBuf join、cfg 分平台、不猜 shell | 已符合 | M005 反斜杠 join 坑后全仓收敛 |
| HTTP 纯 Rust（reqwest 加 rustls 或同级） | 已符合 | 用 ureq 加 rustls（P0015 选型，守 musl 边界）；同为纯 Rust 路线 |
| 目标矩阵：win msvc、linux gnu 加 musl、macOS 双架构 | 已符合 | P0008 / P0013 五 job 矩阵 |
| aarch64-linux-musl 资产 | 候选 | 矩阵未覆盖 aarch64 Linux；触发条件：ARM Linux 用户需求出现。PRD D37 |
| release profile：strip 加 LTO 加 codegen-units 1 | 已落地（部分） | `strip = "symbols"` 加 `lto = "thin"` 入 Cargo.toml（D35）；codegen-units = 1 不采纳（构建变慢收益小） |
| cargo-dist 生成安装器 | 候选 | 自建 release.yml 已稳（10 资产）；触发条件：需要 brew/winget/deb 安装器时整体迁移。PRD D37 |

## 三、测试分层与工具

| 供稿要点 | 裁定 | 本仓现状与说明 |
| --- | --- | --- |
| 能在库里证明的不写 CLI 黑盒 | 已符合 | G005 集成优先加单元守私有逻辑；G006 六层 |
| assert_cmd 黑盒加 insta 快照加 proptest 属性 | 已符合 | D31 / D33 / D34 全落地 |
| trycmd 跑帮助与文档示例 | 候选 | --llms 与 SKILL 已有 insta 快照加逐字节守卫；触发条件：README 示例数量上来后。PRD D38 |
| 默认 nextest 运行器 | 候选 | 触发条件（G005 演进路线）：测试上量后；进程隔离对本仓并行夹具有现实意义。PRD D39 |
| llvm-cov 覆盖率 | 候选 | 同上批处理。PRD D39 |
| cargo deny / audit 供应链闸 | 候选 | 触发条件：进 CI 门禁扩展轮。PRD D39 |

## 四、明确不适用

| 供稿要点 | 不适用原因 |
| --- | --- |
| testcontainers-rs / wiremock | 纯 CLI 无服务依赖（D33 已裁定 testcontainers 不适用；wiremock 同理：唯一网络面是模型下载与 self update，用真 GitHub 加离线档覆盖） |
| trybuild 编译期 API 回归 | 公开面是 CLI 不是库 API（D34 已裁定） |
| 交互式 TUI 测试（rexpect / PTY） | 本仓无交互无 TUI（AGENTS 边界） |

## 五、执行纪律

- 供稿类知识的默认动作是**裁定登记**，不是全量照搬：每条先问本仓有没有真实对象。
- 候选项的触发条件写在 PRD 行内；触发后按 R007 五步立项。
