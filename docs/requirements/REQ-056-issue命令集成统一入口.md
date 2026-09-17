---
id: REQ-056
title: issue命令集成统一入口
status: implemented
priority: must
trace: src/issue.rs（契约面：校验与截断、ureq json feature、http_status_as_error 分流）加 src/lib.rs（三叶接线）加 tests/issue.rs（独立 target 本机一次性 HTTP 服务六例）加 tests/cli.rs（--llms 覆盖守卫自然含新旗标）；实弹：issue #9 首报与 #10 host 修正复报（host 兜 /etc/hostname），list 与 show 及 GET /api/issues 三方可见 [实证: 2026-09-17 实弹退出码与 API 回执]；cargo 全门禁绿
---

# REQ-056:issue命令集成统一入口

## Scenario

总台对齐单(2026-09-17,ohmycloud REQ-057 契约):统一 issue 入口 issues.ohmygh.com 已上线(Worker 加 D1 真源,每 IP 10 条/时),各仓自集成 `issue` 子命令,agent 使用中遇缺陷一键反馈。reader 侧:`issue new` 一键提交自动带 tool=reader 与版本/平台/主机名,`issue list` / `issue show` 读面;契约 = POST /api/issues JSON {tool,title,body,version,platform,host},校验 tool `^[a-z][a-z0-9_-]{0,31}$` 加 title trim 后 1 至 200 加 body 至多 20000,回执 201 {ok,id,url};读面 GET /api/issues?tool=&status=&limit=(新到旧)与 GET /api/issues/<id>。

## Criteria

- [ ] `issue new <标题> [--body]` 提交并输出回执行(`issue: filed #<id> <url>`;成功 0、出错 2;429 与 400 转人读错误)
- [ ] `issue list [--tool] [--status] [--limit]` 行式列表(新到旧;有行 0、空 1、出错 2)
- [ ] `issue show <id>` 详情(存在 0、不存在 1、出错 2)
- [ ] 自动上下文:tool=reader 恒定、version 取 Cargo.toml、platform 取编译目标三元组、host 取环境(截断 64)
- [ ] 客户端先校验与截断(title trim 1 至 200、body 20000、version 40、platform 与 host 64),不过即退出 2 不发请求
- [ ] `READER_ISSUES_API` env 覆盖基址(测试与灰度);`--format json` 包膜与 `--filter` 裁剪(list/show)
- [ ] 契约面三处同步:README、`--llms` curated 文本、`--help`;漂移守卫与 `--llms` 快照随更新
- [ ] AGENTS 纪律入合同:遇缺陷即 `issue new` 一键反馈
- [ ] 测试:单元(校验与截断)加集成(本机一次性 HTTP 服务,env 隔离独立 target:三叶正负例)
- [x] 实弹:提一条真 issue 且 list 可见(回执附证据:#9 首报加 #10 host 修正复报,list/show/API 三方可见)
- [x] 门禁:fmt 加 clippy 加 test --locked 加 test --doc 加 aidoc check --strict 加文档四件全绿

## Notes

- 参考实现 omc(ohmycloud src/commands/issue.ts);契约文档真源 = ohmycloud docs/requirements/REQ-057
- 轻量件随仓批次走;不新增依赖(ureq 与 serde_json 已在树)
