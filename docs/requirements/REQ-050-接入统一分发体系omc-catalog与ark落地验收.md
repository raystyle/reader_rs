---
id: REQ-050
title: 接入统一分发体系omc-catalog与ark落地验收
status: draft
priority: should
trace: null
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

- [ ] 接入面收口：omc catalog pin 节滚 v0.6.0 至 v0.6.1（digest 程序直写，catalog-lint 门禁，fleet 域播种后边车对账）
- [ ] 落地面：五端回执，每端 ark catalog sync 加 query reader（解析 0.6.1）加 install 或 update 加 `reader --version` 加 doctor 零红
- [ ] 自有链保留核对：reader.ohmygh.com 镜像腿与 self update 不退役（双通道镜像优先 GitHub 回退，与统一体系同构；v0.6.1 latest.json 已在线 [实证: 2026-09-16 镜像域边车四件可取]）
