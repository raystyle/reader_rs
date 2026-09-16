---
id: REQ-050
title: 接入统一分发体系omc-catalog与ark落地验收
status: implemented
priority: should
trace: ohmycloud 5c8b53c（catalog-bump 节滚三平台 pin 0.6.1，catalog-lint 53 全过）；catalog-seed run 35071752768（seed 三件套绿，seed-assets 的 hst 三红属 hst 侧发布面非 reader，已飞轮派单 wT）；fleet 域 env.ohmygh.com/reader/0.6.1/ 三平台资产加边车在线且与 catalog pin 与自有镜像域三方 digest 全等 [实证: 2026-09-16 curl 对账]；五端 ark 回执（本端 WSL 加 Windows 宿主加 lan-mac 加 lan-ubuntu 加 lan-linux，reader 0.6.1 三态齐、self update 五端同报已最新、回归四探针 extract 加 search 命中加未中加 query 全绿、doctor 的 reader 面零红）
---

# REQ-050:接入统一分发体系omc-catalog与ark落地验收

## Scenario

dev-evo 第六十四批确立统一分发体系（flow-release 第八节）：omc 管资源分发运维（catalog 真源、种子签发、镜像运维、版本对齐表），ark 管落地执行验收（各端 install / update / status / doctor）。reader 是舰队实际使用的文档工具（super-research 检索管线在用），自有分发链走 GitHub Releases 加 reader.ohmygh.com 镜像加 self update（ADR-0004）。立项时以为未入册，实查 2026-09-08 已入 omc catalog（`[tools.reader]` 三平台 pin 齐，停在 v0.6.0），fleet 域 `env.ohmygh.com/reader/0.6.0/` 资产加边车在线且 digest 与 pin 全等 [实证: 2026-09-16 curl 边车对账]。本 REQ 实际收口面：追问链裁定回填、catalog pin 节滚跟进发版、ark 五端落地验收回执。

## Criteria

追问链三问裁定（用户 2026-09-16，另加执行范围两裁）：

- [x] catalog 条目取最新 stable 单 pin；历史版本段留桶 immutable 不进 catalog（catalog 结构即单 pin，ark 与 hst 节滚同口径）
- [x] 三平台资产复用 release.yml 现有产物加 sha256 边车（条目 `asset_sha_suffix = ".sha256"` 与资产 pattern 对位，fleet 域 0.6.0 边车 digest 全等 [实证: 2026-09-16 env.ohmygh.com 边车对账]）
- [x] self update 与 ark update 并存同 digest 判据：ark update 走 fleet 域（catalog pin 锚），reader self update 走自有域（latest.json 锚），判新互不干扰（发布器与升级器同 digest 判据，flow-release 第八节既有裁定）
- [x] fleet 域播种归 omc catalog-seed（提交后 dispatch 即时触发，不等 6h 定时），reader CI 零改动；自有镜像腿即准则里的 CI 自推面（reader-dl 桶级 token 最小权面已在位，release.yml mirror job 规避 rclone HeadBucket 的 copy 手法在册）
- [x] 验收范围五端全量：本端 WSL、Windows 宿主、lan-ubuntu、lan-linux、lan-mac

待办件：

- [x] 接入面收口：omc catalog pin 节滚 v0.6.0 至 v0.6.1（catalog-bump 程序直写，catalog-lint 53 全过；dispatch catalog-seed 后 fleet 域三平台边车对账全等 [实证: 2026-09-16 env.ohmygh.com 与 catalog pin 与镜像域三方全等]）
- [x] 落地面：五端回执（本端 WSL 加 Windows 宿主加 lan-mac 加 lan-ubuntu 加 lan-linux；宿主走 127.0.0.1 回环 ssh、lan 三端 mesh ssh、裸端两台 fleet 域 bootstrap ark 1.2.3 过锚后 init）；每端 query 解析 0.6.1、install 或 update 落 0.6.1、`reader --version` 0.6.1、三态齐、doctor 的 reader 面零红（各端全局红为既有环境债，已飞轮派单 hst 与 ark 工位处置）[实证: 2026-09-16 五端回执]
- [x] 自有链保留核对：reader.ohmygh.com 镜像腿与 self update 不退役；AB 双通道互证 = ark 装版（fleet 域）与 reader self update 判新（自有域 latest.json）五端同报 current 0.6.1 已是最新，判新互不干扰；回归四探针（extract 加 search 命中加未中加 query）五端全绿 [实证: 2026-09-16 五端探针退出码]

## 发现与移交

> 非本 REQ 阻塞项，均已有处置去向。

- catalog-seed 的 seed-assets job 红 = hst 1.3.0 资产锚校验不过（边车缺失加 digest 与 pin 漂移，04:58Z 与 16:03Z 两轮同伤），属 hst 侧发布面，已 herdr 派单 hst 工位（wT）处置
- lan-ubuntu 裸端 ark init 首同步撞旧 pin（CF 边缘缓存对覆写对象滞后族），resync 即愈；已派单 ark 工位（wS）研判 bootstrap 二次 sync 兜底
- Windows 宿主 doctor 的 version-drift（claude/grok/git）与缺装清单已随单移交 ark 工位
