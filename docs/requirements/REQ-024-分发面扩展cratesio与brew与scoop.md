---
id: REQ-024
title: 分发面扩展cratesio与brew与scoop 原D24
status: draft
priority: should
trace: null
---

# REQ-024:分发面扩展cratesio与brew与scoop 原D24

## Scenario

预编译二进制与 self update 之外，用户希望经包管理器安装（cargo install 走 crates.io，macOS 走 brew，Windows 走 scoop）。

## Criteria

- [ ] 待追问链澄清：三渠道全做还是择先；crates.io 发布是否连带启用库面文档义务（missing_docs 与 doctest 届时重裁，见 AGENTS Commands 裁定行）
- [ ] ppocr-rs 为 git 依赖，上 crates.io 前须核实发布链可行性（git 依赖不阻 crates.io 发布，但需锁定策略）
- [ ] 各渠道版本与 release 资产 sha256 对账口径
