# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

P0015 self update（登记日 2026-09-03，触发：用户点名参考 ohmyenv-rs 与 ohmyagents-rs 加入 self update，要求判断自身路径）。**2026-09-03 已达成**：`reader self update [--force]` 落地；临时目录 `--force` 端到端实测通过（换上件 sha256 与官方 v0.2.1 资产一致）。过程与经验回填 `docs\proven\P0015-self-update.md`。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| 参考实现研读 | 已完成 | ohmyenv-rs selfupdate.rs（digest 对比加改名替换）与 ohmyagents-rs update.rs（staged 原子替换、gh api 兜底） | 2026-09-03 |
| 立项登记 | 已完成 | GOAL/TODO/PLAN 三原语 | 2026-09-03 |
| src\selfupdate.rs | 已完成 | 平台资产名映射、latest 元数据（GH_TOKEN 加 gh api 兜底）、digest 校验下载、zip/tar.gz 解包、current_exe 加兄弟二进制替换 | 2026-09-03 |
| CLI 接线与文案 | 已完成 | `reader self update [--force]`；introspect 双文本加 SKILL 再生成 | 2026-09-03 |
| 测试 | 已完成 | 单元（资产名映射、版本比较）；临时目录复制二进制 force 实跑端到端 | 2026-09-03 |
| 门禁与回填 | 已完成 | 门禁三件加 rumdl 三件；AGENTS/INDEX/CHANGELOG/diary/P0015 归档 | 2026-09-03 |
