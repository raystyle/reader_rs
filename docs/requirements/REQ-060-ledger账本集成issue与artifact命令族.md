---
id: REQ-060
title: ledger账本集成issue与artifact命令族
status: implemented
priority: must
trace: src/ledger.rs（签名道加写入封装加 issue 与 artifact 面，单测六件含 kid 对账与签名基逐字）加 src/lib.rs（issue new/list/show/close 加 artifact publish/attest/promote/list 命令族与行式）加 tests/ledger.rs（回归五件：签名可回核、家族翻页、错误归因、publish 形状、关单两连发）加 README 加 AGENTS 加 introspect curated（快照重出）；门禁全绿与实弹证据见 diary 当日 [实证: 2026-09-20 门禁退出码与 ledger.ohmygh.com 实弹]
---

# REQ-060:ledger账本集成issue与artifact命令族

## Scenario

总台令（2026-09-20）：集成 ledger 标准的 issue 与产物（artifact）命令族，替代原 issues.ohmygh.com issue 功能（REQ-056 旧面退役），完毕封小版本。契约 = ohmycloud REQ-063（仓级公共账本，服务已上线 ledger.ohmygh.com），服务端源码本机可读，客户端面按其契约定形。

## Criteria

- [x] 签名道：Ed25519 五头（Idempotency-Key/X-Key-Id/X-Timestamp/X-Nonce/X-Signature），签名基七行换行连逐字同构服务端 verifyEd25519；公钥 JWK 内置常量，kid = sha256hex(字母序紧凑 JWK) 且单测对账；私钥环境 `READER_LEDGER_KEY` 或本地密档（缺省 `~/.config/reader/ledger-key`），不进仓不进 argv
- [x] 发送体即签名体：ureq send_json 出 pretty 形与 to_string 不同字节，按字符串原样发送（回归测试以测试公钥回核签名实证）
- [x] 幂等键语义：随机键每请求新生成；409 撞键自动换键重试一次；单测断言键与 nonce 取样不重
- [x] issue 面：new（title 1 至 200 加 kind=bug|improvement 加 acceptance）、list（家族形：limit 缺省 100 加 before 游标加 more=1 恒带索取 has_more 加 count 语义加翻页提示）、show（投影加验收面加时间线，404 归退出 1）、close（result 引 sha256 digest 加 status done 两连发，服务端校验关单前提）
- [x] artifact 面：publish（name 加 kind 15 值加 digest=sha256:<64hex> 加可选 version 加 git_range 加 deps[]）、attest（六事件型）、promote 简写、list（name/kind/env/current 过滤）
- [x] 客户端先校验（kind 面、digest 小写 hex 形、title 限长）与服务端同则兜底；401/409/429 归因文案
- [x] 家族标准保持：limit 100 缺省、before 游标、饱和/翻页提示、count 语义（--llms 与 README 同步）
- [x] 旧面退役：src/issue.rs 与 READER_ISSUES_API 面 삭제，README 与 AGENTS 注明真源切 ledger.ohmygh.com
- [x] 封版 v0.10.0（能力新增与命令族契约变化取 minor；CHANGELOG 注明 issue 面切 ledger）

## Notes

- 服务端契约源：ohmycloud workers/ledger/src/index.ts（读面取形，未改服务端）；pubkeys 表登记由总台执行（回执附 JWK 全文与 kid）
- artifact 的 kind 面与服务端 ARTIFACT_KINDS 同集 15 值；attest_prod 事件 payload 自动带 env=prod（服务端同补）
- 配额 per-key 50/UTC 日与读面免签为服务端闸；客户端只归因不本地计数
- 旧 issues.ohmygh.com 保役过渡期（REQ-063 切换策），本仓客户端不再指向它
