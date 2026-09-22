---
id: REQ-062
title: issue列表增status与kind过滤
status: implemented
priority: must
trace: src/lib.rs（`issue list` 增 `--status` 与 `--kind` 旗标；过滤翻页循环跨页精确到全量，页深上限 20 兜底；has_more 语义按过滤面重定义）加 tests/ledger.rs（CLI 面一件：旗标在帮助面与缺值拒绝）加 README 加 src/introspect.rs curated（三处同步）；服务端 `issuesList` 无 status/kind 查询参数（ohmycloud workers/ledger 实查 2026-09-22），过滤在客户端 post-fetch（沿 REQ-061 artifact list name/kind 先例）；实弹对拍见 diary 当日 [实证: 2026-09-22 ledger.ohmygh.com 四单全量与过滤面对拍]
---

# REQ-062:issue列表增status与kind过滤

## Scenario

账本单 #1（improvement，open）：`issue list` 增状态与 kind 过滤。验收判据：list 面支持 status 与 kind 过滤参数（对齐旧面 --status / 家族标准）；投影行含过滤字段；实弹对拍。总台周知 2026-09-22 认定真功能单，去留与排期本仓自裁：认领，随下版滚出。

## Criteria

- [x] `issue list` 增 `--status <状态>` 与 `--kind <kind>` 旗标（可单用可组配；值缺省不过滤；等值匹配，grep 语义退出码不变：空结果 1）
- [x] 过滤跨页精确：客户端沿 has_more/before 翻页累积匹配行直到满 `--limit` 或账本穷尽；页深上限 20 页兜底，触顶 stderr 明示未穷尽
- [x] has_more 语义（过滤面）：服务端末页 has_more 或已取匹配行超 limit 任一成立即为真（未过滤路径与旧行为逐字节等价：limit 100 单页）
- [x] 投影行含过滤字段：行式 `#<n> <status> <kind> <assignee|-> <标题>`（既有面已在册，本单不改动）
- [x] 实弹对拍：全量 list 与 `--status open` / `--kind bug` / 组配面互核对账（diary 当日出输出）
- [x] 服务端查询参数化（status/kind 入 `issuesList`）属 ohmycloud ledger 工位面；本仓旗标语意与透传形兼容，服务端参数落地后可切透传无 CLI 契约破坏（回执已向总台报备）

## Notes

- 过滤在客户端而非查询参数：crate 读面（ledger-client v0.1.3 `issue_list`）只带 limit/before 形参；服务端亦无该二参数（实查）。沿 REQ-061 Notes 在册姿势「name/kind 过滤在客户端 post-fetch」
- status 取值随服务端投影（open / done；claimed / in_progress / blocked 家族态），本仓不做白名单校验（等值匹配，错值自然空结果 exit 1，同 grep 契约）
- kind 取值 bug / improvement（同 `issue new --kind` 面）
