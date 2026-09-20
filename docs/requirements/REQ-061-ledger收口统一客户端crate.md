---
id: REQ-061
title: ledger收口统一客户端crate
status: implemented
priority: must
trace: Cargo.toml（ledger-client git tag v0.1.0 替自研依赖组）加 src/ledger.rs（重写为薄胶水：私钥 hex 装载加在册 kid 对账加 Ledger 装配）加 src/lib.rs（撤 issue close 与 artifact promote 面，attest 收验证类三型并加 --checks；运行器全走 crate API）加 tests/ledger.rs（收口面四件，零网络）；门禁全绿见 diary 当日 [实证: 2026-09-20 门禁退出码与帮助面输出]
---

# REQ-061:ledger收口统一客户端crate

## Scenario

总台修正令（2026-09-20 统一裁，各仓 CLI 随下版对齐）：权限收口（各仓 CLI 只增不关不删，关闭与删除唯一道 = 开发工作台经 herdr 委托 omc 工位）；统一标准代码（ledger-rs crate v0.1.0 为全舰队唯一实现，各仓移除自研客户端以 Cargo 依赖引入）；随下版自然滚出，不强求独立封版。

## Criteria

- [x] 权限收口：移除 `issue close` 面与 `artifact promote` 面（clap 树退役，unrecognized 即拒）；`artifact attest --attest-type` 收验证类三型（attest_dev / attest_prod / verification_failed），promote/demote/supersede 客户端先拒并指路 omc 工位；status 推进面从未暴露，维持
- [x] 统一 crate：移除自研 ledger 客户端（原 src/ledger.rs 679 行签名道与 typed 面），Cargo 依赖 `ledger-client = { git, tag = "v0.1.0" }`；自研依赖组（ed25519-dalek / base64 / getrandom 主依赖）随撤
- [x] 薄胶水面（非客户端副本）：私钥装载（env `READER_LEDGER_KEY` 32 字节 hex，或 `READER_LEDGER_KEY_FILE` 缺省 `~/.config/reader/ledger-key`）、在册 kid 对账（不符即拒，防换对后写面静默 401）、错误归因转行；密档由 base64url 转 hex（同 seed 同 kid，openssl 派生复验一致 [实证: 2026-09-20 转档对账]）
- [x] 行式随 crate 返回面微调：issue new 回执行去 seq 字段（crate 只回 issue 号）；attest 增 --checks JSON 证据参数；CHANGELOG 注明收口行
- [x] 测试收口：签名道与请求形状面归 crate 自测（不再各自维护副本），本仓 tests/ledger.rs 改 CLI 收口面四件（零网络）：close 退役、promote 退役、attest 类型先拒与 checks 校验、帮助面只增口径
- [x] 随下版滚出（本批不独立封版，落 Unreleased）

## Notes

- crate 面：issue_new/issue_list/issue_show/artifact_publish/artifact_attest/artifact_list；无 base 覆写（READER_LEDGER env 随自研客户端退役），灰度换基址属 crate 功能面诉求，走 ledger-rs 仓
- name/kind 过滤在客户端 post-fetch（crate 读面只带 current/env）
- 关闭与删除操作指路：开发工作台经 herdr 委托 omc 工位（omc ledger issue status <repo> <n> <to> 与 omc ledger issue delete，已上线）
