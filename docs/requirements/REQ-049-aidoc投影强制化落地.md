---
id: REQ-049
title: aidoc 投影强制化落地
status: implemented
priority: must
trace: cargo aidoc --check --strict 退出 0（17 artifacts clean）；cargo test --locked 全绿；PEVO_CHECK_ALLOW='^docs/aidoc/' check.py 退出 0
---

# REQ-049:aidoc 投影强制化落地

## Scenario

dev-evo 第五十九批（tool-rust 引 ADR-0006）将 Rust 栈 aidoc 投影强制化：bin-only 不再豁免，投影受众是维护者与 agent。本仓 2026-09-16 早间按「无自有 API 面项目」范式立的不适用裁定撤换，库面公开契约补齐。

## Criteria

- [x] 撤换 AGENTS 与地图的不适用裁定句，改引强制口径（tool-rust；ADR-0005）
- [x] 立本 REQ 登记此重构
- [x] `///` 契约注释覆盖公开项（编译器钉账 57 处补齐：字段、变体、方法；bin 与测试面 crate 文档同补）
- [x] missing_docs 策略落地：deny（Cargo.toml `[lints.rust]`，随 cargo check/test/clippy 全目标生效）
- [x] cargo aidoc 生成投影进 Git（`docs/aidoc/` 17 件，含 llms.txt 与 llms-full.txt 与 api JSON）
- [x] `cargo aidoc --check --strict` 入 AGENTS Commands 作漂移门禁
- [x] 渲染格式禁字按 tool-rust 豁免实务：`PEVO_CHECK_ALLOW` 指 `docs/aidoc/` 在册（rumdl 与 md-char-scan 同步整目录豁免，G004 豁免区登记）
- [x] doctest 补 3 例（parse_page_spec、is_image_ext、Matcher::is_match），`cargo test --doc` 转正为 Commands 在册门禁
- [x] 第六十批格式纪律增量：`# Errors` 22 处与 `# Panics` 1 处补齐、clippy 开 missing_errors_doc 与 missing_panics_doc 与 missing_safety_doc、示例 `# Examples` 归位并补 filter_value 一例（断言收尾）、首段成句重排或补写 15 处；投影重生成同提交
