---
id: REQ-063
title: artifact发布对齐总台新标准
status: implemented
priority: must
trace: Cargo.toml 加 Cargo.lock（ledger-client git tag v0.1.1 升 v0.1.3：read_only 只读构造加 publish_full 结构化字段）加 src/lib.rs（publish 面增 --summary 必填、--outcome 必填客户端先拒 success|failure、--git-sha 提交锚；运行器走 artifact_publish_full）加 tests/ledger.rs（新标面一件：outcome 先拒加 summary 必拒加帮助面旗标，零网络）加 README 加 src/introspect.rs curated（三处同步）；实弹即两件真题按新标准重发（总台周知 2026-09-22 项 2 回册动作），证据见 diary 当日 [实证: 2026-09-22 ledger.ohmygh.com publish 回执与 artifact list]
---

# REQ-063:artifact发布对齐总台新标准

## Scenario

总台标准升级（用户令 2026-09-22「没有按标准发」，publish 硬校验已上）：summary 经验描述与 outcome 结果倾向必填，空发即 400；lifecycle 与删除集中 omc。本仓两件 artifact 被按垃圾数据删除（outcome 空加描述空），真题回册须按新标准重发。本仓 publish 面（ledger-client v0.1.1 七参形）无 summary/outcome/git_sha 通道，对现役服务端**任何 publish 必 400**，补面为先决件。

## Criteria

- [x] ledger-client 升 v0.1.3（v0.1.2 read_only 与 v0.1.3 artifact_publish_full 结构化 summary/outcome/git_sha；加法式 API，无破坏）
- [x] publish 面增旗标：`--summary`（经验描述，必填；成功经验加失败教训加研究成果的文字本体）、`--outcome`（必填，success 成功经验 / failure 失败教训，客户端先拒不挂网络，同 attest 类型收口姿势）、`--git-sha`（提交锚，可缺省，服务端落 artifacts 表）
- [x] CLI 必填化属破坏性变更（旧调用缺 --summary/--outcome 即拒）：随下版 minor 滚出（与 REQ-060/061 收口批同车）；服务端已先拒同形调用，CLI 侧必填是诚实快败非新增限制
- [x] 家族面位对齐：旗标名与语义同 ark_rs 已上先例（--summary 一行本体、--outcome success|failure）
- [x] 实弹验收：两件真题（S010 图片本体研究 kind=research outcome=success；ureq 发送体即签名体坑 kind=lesson outcome=failure）按新标准重发成功，artifact list 可见

## Notes

- digest 口径：ureq lesson 取经验描述正文哈希（本体即 summary 文字）；S010 研究取 docs/research/S010 记录文件哈希（本体即研究档），各随其性
- 同 digest 内容寻址唯一：被删行硬删后同 digest 可重发；若服务端留行致 409，真题回册改走 omc supersede 道（届时回执注明）
- promote/demote/supersede 与删除仍走 omc 工位（总台令），本面不新增 lifecycle 旗标
