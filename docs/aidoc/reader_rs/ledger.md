# reader_rs::ledger

ledger.ohmygh.com 客户端（REQ-063 Phase 3 仓内集成，替代 issues.ohmygh.com
旧面，REQ-056 的 issue 功能迁入本账本）：仓级公共账本双层 append-only，
issue 流管义务（开单加事件加投影）、artifact 流即共享库本体（publish 加
attest 加 promote）。写入五头必签：Idempotency-Key 加 X-Key-Id 加
X-Timestamp（正负 60 秒窗）加 X-Nonce（10 分钟不重）加 X-Signature；签名基
= v1/POST/路径/时间戳/nonce/幂等键/body sha256 各行换行连，Ed25519 私钥签，
签名 base64url。读面沿 REQ-057 家族形：limit 缺省 100 加 before 游标加
has_more 加 count 语义（本次返回条数）。私钥运行时从环境
`READER_LEDGER_KEY`（base64url seed）或本地密档读（`READER_LEDGER_KEY_FILE`
覆写路径，缺省 `~/.config/reader/ledger-key`），不进仓不进 argv。

## Functions

- `artifact_attest` — 产物事件（attest/promote 等）：POST /repos/:repo/artifacts/:id/attestations。
- `artifact_list` — 产物列表：GET /repos/:repo/artifacts?current=&env=（可按 name/kind 过滤）。
- `artifact_publish` — 登记产物：POST /repos/:repo/artifacts {name, kind, digest, version?, git_range?, deps[]}。
- `issue_close` — 关单链：先 result 事件（引用 digest，关单判据）再 status done；返回两事件 seq。
- `issue_event` — 追加 issue 事件：POST /repos/:repo/issues/:n/events {type, payload, body}。
- `issue_list` — 列表（新到旧）：GET /repos/:repo/issues?limit=&before=&more=1。
- `issue_new` — 开单：POST /repos/:repo/issues {title, kind, acceptance, body}。
- `issue_show` — 单条详情：GET /repos/:repo/issues/:n（投影加时间线）；404 归 `Ok(None)`。
- `kid_of_jwk` — 从 JWK 文本推 kid：取 kty/crv/x 三键按字母序紧凑序列化后 sha256hex。
- `load_signing_key` — 读 Ed25519 私钥（32 字节 seed）：环境 `READER_LEDGER_KEY`（base64url）优先，
- `signature_base` — 签名基（服务端 verifyEd25519 逐字同构）：七行换行连。
- `valid_digest` — digest 形校验：`sha256:` 加 64 位小写 hex（服务端契约同则 `[0-9a-f]{64}`）。

## Types

- `ArtifactPublished` — publish 回执。
- `ArtifactRow` — 产物列表行。
- `IssueDetail` — issue 详情（投影加开单验收面加原始时间线）。
- `IssueOpened` — 开单回执（issue 号与首事件 seq）。
- `IssuePage` — 列表页（家族翻页形）。
- `IssueRow` — 列表行（服务端投影字段同形；`hasResult` 服务端为驼峰）。

## Constants

- `ARTIFACT_KINDS` — artifact kind 面（服务端契约 15 值同集）。
- `ATTEST_TYPES` — artifact 事件面（promote 单列命令，其余走 attest --type）。
- `DEFAULT_BASE` — 账本缺省基址（`READER_LEDGER` 覆盖，测试与灰度用）。
- `ISSUE_KINDS` — issue kind 面（服务端契约同值）。
- `KEY_ID` — 本仓 key_id（kid）= sha256hex(规范化 JWK：键序字母的紧凑 JSON)。
- `PUBLIC_JWK` — 本仓公钥 JWK（身份分发面：CLI 自带验签材料，总台账本 pubkeys 表同形登记；
- `REPO_ID` — 账本仓标识（规范化 remote；URL 路径原样嵌入，含斜杠）。

