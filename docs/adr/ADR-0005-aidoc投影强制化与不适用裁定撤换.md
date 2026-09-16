---
id: ADR-0005
title: aidoc投影强制化与不适用裁定撤换
status: accepted
date: 2026-09-16
deciders:
  - ray
supersedes: []
superseded_by: null
tags: [文档体系, 工具链]
---

# ADR-0005:aidoc投影强制化与不适用裁定撤换

## Context

2026-09-16 早间按 dev-evo base-projection「无自有 API 面项目」范式裁定：本仓库未发布、契约在 CLI 面，文档投影整栈不适用。同日 dev-evo 升第五十九批（tool-rust 引 ADR-0006）：Rust 栈 aidoc 投影强制化，bin-only 不再豁免，投影受众是维护者与 agent 而非外部用户。旧裁定被标准升格撤换（REQ-049）。

## Decision

库面公开契约以 `///` 与类型签名为准，missing_docs 设 deny（Cargo.toml `[lints.rust]`）；`docs/aidoc/`（llms.txt 入口加分模块 md 加 api JSON）由 cargo aidoc 生成并进 Git，`cargo aidoc --check --strict` 作漂移门禁；aidoc 渲染格式禁字（条目分隔符 em dash，无开关）走路径级豁免在册（PEVO_CHECK_ALLOW 与 rumdl 与 md-char-scan 同一口径）。CLI 面契约维持 SKILL.md 字节确定性产物与漂移守卫不变。

## Consequences

- 好：维护者与 agent 得到渐进披露的库面索引（llms.txt 一行一模块）；`///` 覆盖由 missing_docs deny 经 CI 强制，不再靠自觉
- 坏：改 pub 项多一道 `cargo aidoc` 再生成与 `docs/aidoc/` 同提交的流程义务；投影 manifest 记录生成侧平台三元组，异构机器再生成会带 target 字段噪声（以本仓开发侧 WSL为准，实跑无碍 [实证: 2026-09-16 WSL 生成与 --check --strict 双跑 clean]）
