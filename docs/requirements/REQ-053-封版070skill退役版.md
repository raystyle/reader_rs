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
- [x] dev/v0.7.0 承载与验收：本机门禁（cargo 三件加文档四件加 aidoc check）加实机（lan-mac、lan-ubuntu；lan-linux 现无 rust 工具链，Linux 面按 R004 同适用走 lan-ubuntu）加 CI dev 分支三系统绿 [实证: 各路退出码与阶段标记]
- [x] FF 合并 main 打 tag v0.7.0 触发 release.yml；六 job 绿（run 35115467735：五平台构建加 mirror，windows 岗 16m11s 最长）
- [x] 发行验收：资产十件；包内 LICENSE 与 README 与双名二进制（SKILL.md 不再打包实证）；latest.json 广告 0.7.0 五平台（max-age=60）；镜像域 0.7.0 资产 immutable；linux-gnu 镜像件 sha256sum 官方校验 OK；解包冒烟 `reader 0.7.0`、`skill` 退位 unrecognized、`--llms` 三条新契约行在位；self update 镜像通道真升级 0.6.1 至 0.7.0 全链通 [实证: 2026-09-16 本机各步退出码]
- [ ] 发版后分发跟进（REQ-050 既定循环）：omc catalog pin 节滚 0.7.0 加 dispatch catalog-seed 加 fleet 域对账（GitHub 面瞬断待补）
- [ ] dev 分支发布后即删（本地已删；远端删除与实机克隆归位待 GitHub 面恢复）；ROADMAP 核毕无阶段态变化与 diary 收尾钩子

## 验收记录

- 2026-09-16 v0.7.0 发行：三路验收（本机、lan-mac、lan-ubuntu 十一套 result ok 同数、CI dev 三系统）绿；release run 35115467735 六 job 绿；`releases/latest` 指 v0.7.0。dev/v0.7.0 本地已删，远端删除待网络。
- 网络插曲：发版后 GitHub 面连接重置（本机与 lan-mac 同伤），镜像域 reader.ohmygh.com 全程可达（self update 真升级即证）；分发跟进件挂起待恢复。
