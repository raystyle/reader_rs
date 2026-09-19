# reader_rs::issue

issue 域（总台统一入口 issues.ohmygh.com，ohmycloud REQ-057 契约对齐）：
`new` 一键提交（自动带 tool=reader 与版本、平台、主机名）、`list` 集中列表（新到旧）、
`show` 单条详情。提交面每 IP 10 条/时（HTTP 429），读面匿名 GET；基址可由
`READER_ISSUES_API` 覆盖（测试与灰度）。HTTP 走 ureq（全局超时 20 秒，4xx/5xx
不转传输错误、按状态码分流），JSON 走 serde_json；不新增依赖。

## Functions

- `build_report` — 构建提交体并做客户端先校验：tool 形 `^[a-z][a-z0-9_-]{0,31}$`、title trim 后
- `file_new` — 提交一条 issue（tool=reader、上下文自动）：POST /api/issues，期望 201 {ok,id,url}。
- `list` — 列表（新到旧）：GET /api/issues?tool=&status=&limit=&before=（limit 夹取
- `show` — 单条详情：GET /api/issues/<id>；404 归 `Ok(None)`（调用方退出 1）。

## Types

- `FiledReceipt` — 提交回执（201 {ok,id,url} 的有效载荷）。
- `IssueReport` — 提交体（字段序即 POST JSON 序，契约 {tool,title,body,version,platform,host}）。
- `IssueRow` — 一条 issue（list 无 body、show 有；ip 属服务端审计面，不透出到输出）。
- `ListPage` — 列表页（新到旧）：返回行与服务端 `has_more`（仅带 `before` 的请求回执携带，

