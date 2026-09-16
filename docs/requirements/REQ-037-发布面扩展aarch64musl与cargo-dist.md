---
id: REQ-037
title: 发布面扩展aarch64musl与cargo-dist 原D37
status: draft
priority: should
trace: null
---

# REQ-037:发布面扩展aarch64musl与cargo-dist 原D37

## Scenario

ARM Linux 用户或安装器需求出现时，发布矩阵扩 aarch64-unknown-linux-musl 资产并评估 cargo-dist 安装器（brew / winget / deb 一键安装脚本）。裁定表见 `../guide/G007-RustCLI工程基线-供稿要点逐项裁定.md` 二节。

## Criteria

- [ ] 触发条件：ARM Linux 用户实名或安装器需求；未触发不动
- [ ] aarch64 musl 交叉链路先 PoC（ppocr-rs 与 hayro 的 aarch64 musl 构建核实）
- [ ] cargo-dist 采纳前核与既有 release.yml 镜像腿的共存方式（不推倒自有流水线）
