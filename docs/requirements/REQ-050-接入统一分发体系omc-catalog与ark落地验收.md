---
id: REQ-050
title: 接入统一分发体系omc-catalog与ark落地验收
status: draft
priority: should
trace: null
---

# REQ-050:接入统一分发体系omc-catalog与ark落地验收

## Scenario

dev-evo 第六十四批确立统一分发体系（flow-release 第八节）：omc 管资源分发运维（catalog 真源、种子签发、镜像运维、版本对齐表），ark 管落地执行验收（各端 install / update / status / doctor）。reader 是舰队实际使用的文档工具（super-research 检索管线在用），当前只走自有分发链（GitHub Releases 加 reader.ohmygh.com 镜像加 self update，ADR-0004），未入 omc catalog。

## Criteria

- [ ] 待追问链澄清：catalog 条目取 stable 通道哪些版本（最新 or 全历史）；三平台资产是否复用 release.yml 现有产物加 sha256 边车；self update 与 ark update 双升级路径并存口径
- [ ] 接入面：omc catalog tools.toml 条目加 sha256 pin；CI 镜像自推走 seed 通道（桶级 token 最小权面）
- [ ] 落地面：ark 侧 `ark install reader` / `ark update reader` / doctor 验收回执（五端按四平台协议抽端）
- [ ] 自有链保留：reader.ohmygh.com 镜像腿与 self update 不退役（双通道镜像优先 GitHub 回退，与统一体系同构）
