---
id: REQ-065
title: ledger面全线退役
status: implemented
priority: must
trace: src/lib.rs（pub mod ledger 与 Issue/Artifact 命令族、枚举、分发、run 函数、单测全清）加 src/ledger.rs 与 tests/ledger.rs 删件加 Cargo.toml/Cargo.lock（ledger-client 依赖删）加 .github/workflows/ci.yml 与 release.yml（私有凭据步删）加 README.md 与 AGENTS.md 与 src/selfupdate.rs（反馈道改 GitHub Issues）加 src/introspect.rs 与 tests/snapshots/snapshot__llms_index.snap（curated 与快照重出）加 docs/aidoc/（17 件重出，ledger.md 与孤儿 issue.md 删）；门禁：cargo fmt 加 clippy -D warnings 加 test --locked 11 target 加 --doc 加文档四件加 aidoc --check --strict 加 PEVO 全绿；验收记录见 diary 2026-10-21
---

# REQ-065:ledger面全线退役

## Scenario

总台令 2026-10-21（用户裁）：ledger 全线淘汰。本仓 reader_rs 实检出 ledger 面（ledger 模块、issue 与 artifact 命令族、ledger-client git 依赖、CI 与 release 的私有凭据步、文档三处与测试面），全部剔除。

## Criteria

- [x] `src/ledger.rs` 删除、`lib.rs` 的 `pub mod ledger` 与 Issue/Artifact 命令变体、分发臂、run 函数、单元测试全清
- [x] `tests/ledger.rs` 删除；`introspect.rs` curated 文本（ledger 两契约行、env 行、常用例、退出码行）清
- [x] `Cargo.toml` ledger-client 依赖删；Cargo.lock 随重生
- [x] CI 与 release workflow 的 PRIVATE_GIT_TOKEN 凭据步删（无私有 git 依赖）
- [x] README（env 表两行、agent 发现例、ledger 节）、AGENTS.md（遇缺陷反馈道）、selfupdate.rs CTA 指路 GitHub Issues
- [x] 版本 bump 0.12.0 加 CHANGELOG 记移除；`--llms` 快照重出；aidoc 重出
- [x] 门禁全绿：cargo fmt / clippy / test --locked / --doc、文档四件、aidoc check、PEVO check

## Notes

- 契约破裂取 major 判据在 0.x 约定下折半为 minor（R008 与 0.9.1 判例：minor 承载命令面变化），本批取 0.12.0
- 历史 REQ（056、060 至 063）按历史不改写纪律不动，队列表补退役注记
