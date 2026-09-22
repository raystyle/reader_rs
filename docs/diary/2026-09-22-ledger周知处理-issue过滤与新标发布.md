# 2026-09-22 ledger 周知处理：issue 过滤与新标发布

> 来源：总台周知 2026-09-22 两件（账本单 #1 去留自裁；两件 artifact 按垃圾数据删除、真题按新标准重发），完毕立 REQ-062 与 REQ-063 并实弹闭环。

## 概貌

周知两件先查证后自裁。单 #1（issue list 增状态与 kind 过滤）认领：服务端 `issuesList` 实查只有 limit/before 参数，投影行已含 status/kind；沿 REQ-061 在册姿势（artifact list name/kind 客户端 post-fetch）落 `--status`/`--kind` 旗标，过滤翻页循环跨页精确到全量（页深上限 20 兜底，触顶 stderr 明示），has_more 语义按过滤面重定义（服务端末页有余或匹配行超 limit 任一即真，未过滤路径与旧单页行为逐字节等价）。artifact 侧发现硬事实：总台 publish 硬校验已上（summary 必填、outcome 恒 success|failure），本仓 publish 面（ledger-client v0.1.1 七参形）对新役服务端任何发布必 400，补面是重发的先决件：升 ledger-client v0.1.3（read_only 加 artifact_publish_full），publish 面增 `--summary` 必填、`--outcome` 必填客户端先拒、`--git-sha` 提交锚（旗标位对齐 ark_rs 先例）。两件真题按新标准重发并 attest_dev。README ledger 段顺手清一处漂移（「基址可由 READER_LEDGER 覆盖」随自研客户端退役已不实，REQ-061 Notes 在册但 README 未同步）。

## 实弹证据

- 过滤对拍（reader 0.10.0+ 本批，实弹 ledger.ohmygh.com）：全量 4 单（#4/#3/#2 done bug 加 #1 open improvement）；`--status open` 恰出 #1；`--kind bug` 恰出 #2/#3/#4；`--status open --kind improvement` 恰出 #1；`--status done --kind bug` 恰出 #2/#3/#4；`--status blocked` 空结果退出 1（grep 语义）；json 面 count 1 加 has_more false 包膜完整 [实证: 2026-09-22 六组实弹输出与退出码]
- 重发两件：ureq 签名基失配（lesson，failure，artifact 99cbfd3d-51d4-4d3d-8058-9e29df662718，digest = 经验描述正文哈希 df49668a，git_sha 4b9c358）加 attest_dev seq 202；S010 图片本体研究（research，success，artifact 57a3fc4a-d500-46d0-bbca-5c6c80655fc6，digest = 研究档哈希 d1edc983，git_sha c1c0e03）加 attest_dev seq 203；`artifact list` 两行可见 [实证: 2026-09-22 publish 回执与 list 回读]

## 关键裁决

- 单 #1 过滤落客户端不落查询参数：crate 读面与服务端均无 status/kind 形参（双端实查），仓内先例在册（REQ-061 Notes「name/kind 过滤在客户端 post-fetch」）；旗标语意与透传形兼容，服务端参数化后可切透传无 CLI 契约破坏，回执已向总台报备
- digest 口径随本体性：lesson 取经验描述正文哈希（本体即 summary 文字，发布前用 `printf '%s'` 逐字节复算防 $(cat) 尾换行歧义）；research 取研究档文件哈希（本体即记录档）
- publish CLI 侧 summary/outcome 必填化属破坏性变更：服务端已先拒同形调用，CLI 必填是诚实快败非新增限制；随下版 minor 滚出（与 REQ-060/061 收口批同车）

## 门禁

fmt 加 clippy -D warnings 加 `cargo test --locked`（48 加 75 加 64 加 6 加 1 加 1 件）加 `cargo test --doc`（4 件）全绿；rumdl 加 md 三扫过；`cargo aidoc --check --strict` 无漂移（pub 面未动）；PEVO 合规 11 PASS [实证: 2026-09-22 各门禁退出码]

提交锚：REQ-062 = 079e254（中间态重建快照，逐提交可过门禁）；REQ-063 = 1847f8b。未推远端（评审闸门前不动）。
