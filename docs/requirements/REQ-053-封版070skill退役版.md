---
id: REQ-053
title: 封版070skill退役版
status: draft
priority: must
trace: dev/v0.7.0 承载（Cargo.toml 0.7.0、CHANGELOG 定版、--llms 快照仅版本行人工审收录、Cargo.lock 与 aidoc 版本头随封重生）；门禁全绿后 FF 合并 main 打 tag v0.7.0；release 六 job 绿加资产与镜像验收回填本文验收记录节
---

# REQ-053:封版070skill退役版

## Scenario

Unreleased 有货：REQ-052（skill 子命令与 SKILL.md 退役，breaking）加 REQ-050（统一分发体系接入收口，流程面）。按 R008 封版。semver 判据：skill 子命令属公开命令契约破裂，按仓规（契约破裂取 major，0.x 形态即 0.x.0）取 0.7.0（0.6.1 至 0.7.0）。

## Criteria

- [x] 封版件：Cargo.toml 0.7.0；CHANGELOG `[Unreleased]` 转 `[0.7.0] - 2026-09-16`；`--llms` 快照重出仅版本行 diff 人工审收录；Cargo.lock 与 docs/aidoc 版本头随封重生
- [ ] dev/v0.7.0 承载与验收：本机门禁（cargo 三件加文档四件加 aidoc check）加实机（lan-mac、lan-linux）加 CI dev 分支三系统绿
- [ ] FF 合并 main 打 tag v0.7.0 触发 release.yml；六 job 绿（五平台构建加 mirror）
- [ ] 发行验收：资产十件（五平台乘二进制加 sha256 边车，包内不再附 SKILL.md）；latest.json 广告 0.7.0；镜像域 immutable；解包冒烟（`reader --version` 0.7.0、`skill` 退位、`--llms` 新契约行在位）
- [ ] 发版后分发跟进（REQ-050 既定循环）：omc catalog pin 节滚 0.7.0 加 dispatch catalog-seed 加 fleet 域对账
- [ ] dev 分支发布后即删（本地加远端加实机克隆归位 main）；ROADMAP 与 diary 收尾钩子

## 验收记录

待回填。
