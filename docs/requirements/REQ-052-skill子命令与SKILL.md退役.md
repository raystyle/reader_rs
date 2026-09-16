---
id: REQ-052
title: skill子命令与SKILL.md退役
status: implemented
priority: must
trace: src/introspect.rs（skill_md 删除，llms_text 吸收 figures 行式、export 摘要行、ocr 子命令前缀 token 表）；src/lib.rs（Commands::Skill 与 dispatch 删除）；SKILL.md 仓根删除；tests/cli.rs（守卫一改单面 llms_covers_all_clap_flags、新增 skill_subcommand_retired、守卫二退役）；release.yml 打包去 SKILL.md；README 与 AGENTS 与 docs/README 同步；cargo test --locked 与 --doc 全绿（含快照）；cargo aidoc --check --strict 绿 [实证: 2026-09-16 本机门禁退出码]
---

# REQ-052:skill子命令与SKILL.md退役

## Scenario

用户裁定（2026-09-16）：`--llms` 就是紧凑版 agent 说明书，`skill` 子命令与仓根 SKILL.md 长形态不再需要。两裁确认：SKILL.md 整体退役（仓根文件删除，契约面收敛为 `--llms` 加 `--help` 加 README，漂移守卫二随文件退役）；skill 长形态独有的机器契约细节撤前吸收进 `--llms`（示例块与参数速查不搬，保持紧凑）。

## Criteria

- [x] `skill` 子命令删除，调用成 unrecognized subcommand 非零退出（tests/cli.rs skill_subcommand_retired 断言）
- [x] 仓根 SKILL.md 删除，release.yml 打包不再附 SKILL.md
- [x] `--llms` 吸收三件机器契约：figures 行式（`figure: kind | 锚 | 图题或- | 落盘路径 | 字节数B`）、export 摘要行（`export: text/pages/figures/manifest <路径>`）、ocr 子命令前缀 token 表（ok / missing / corrupt / download / verdict 与退出码）
- [x] 漂移守卫改单面：clap 命令树旗标全覆盖 `--llms`（守卫二 SKILL.md 字节一致随文件退役）；`--llms` 快照（insta）随输出更新
- [x] 命令面同步：README、AGENTS（定位句、Commands、Must 双面句、Must not、四处同步句改三处）、docs/README（地图行、三栈投影注记、agent CLI 面对照句）、CHANGELOG Unreleased breaking 条目
- [x] cargo fmt 加 clippy 加 test --locked 加 test --doc 加 aidoc --check --strict 全绿

## Notes

- 契约破裂，semver major，版本号归封版 REQ 定（Cargo.toml 本批不动）
- skill_md 独有且未吸收面：常用例子块、参数速查（仍在 README 与 `--help` examples 节）
- 舰队波及：dev-evo tool-cli-agents.md「skill 生成根 SKILL.md 先例」句需随退役更新，落地后派单 ProjectEvo 工位
