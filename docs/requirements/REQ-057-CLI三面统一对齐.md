---
id: REQ-057
title: CLI三面统一对齐
status: implemented
priority: must
trace: src/introspect.rs（活树渲染手册与机器形；clap value_name 单源命名）加 src/lib.rs（--json 顶旗标仅与 --llms 同用）加 tests/cli.rs（机器形叶数与活树全等测试）加 tests/snapshots（手册快照重出人工审收录）加 README 四节重排（项目介绍/部署/配置/使用方法）；cargo fmt 加 clippy 加 test --locked 加 --doc 全绿；cargo aidoc --check --strict 绿 [实证: 2026-09-17 本机门禁退出码]
---

# REQ-057:CLI三面统一对齐

## Scenario

总台对齐单二号与更正单（2026-09-17，ohmycloud REQ-060 五仓 CLI 三面统一标准）：一面 `--llms` 裸出 markdown 紧凑手册（名加版本加定位、子命令表、通用旗标、常用例，至多 120 行）加 `--llms --json` 机器形，命令表自活命令树渲染禁手维护双份（旗标名以 `--llms` 为准，前单 `--llm` 系笔误）；三面 README 四节重排（项目介绍/部署/配置/使用方法，中文紧凑，禁营销话术）；二面 issue 即 REQ-056 在途批。

## Criteria

- [x] `--llms` 裸出 markdown 紧凑手册：命令表三列（用法含位置参数占位、一句描述、特有旗标）自活 clap 树 DFS 渲染；curated 段（退出码、行式契约、env）保留；实测 47 行（预算 120）
- [x] `--llms --json` 机器形：{name,version,description,globalFlags,commands:[{path,description,flags}]} 叶数组；path 裸命令路径不含位置参数占位
- [x] 位置参数与取值旗标命名单源化：clap value_name（文件或目录、关键词或正则、范围、N、M、正文、编号等），手册与 --help 同源
- [x] `--json` 顶旗标：仅与 `--llms` 同用，单独使用报错退出 2
- [x] 漂移守卫改造：旗标全覆盖断言自然成立（表含特有旗标、通用旗标单列含 --llms/--json）；新增机器形叶数与活树全等测试
- [x] README 四节重排：项目介绍（是什么/为谁/与 ark 与 omc 分工）、部署（安装/升级）、配置、使用方法（快速开始/Agent 发现/命令/JSON/格式）；深度文档归仓内 docs
- [x] `--llms` 手册快照重出人工审收录；cargo 全门禁绿

## Notes

- 细标（README 与 JSON 协议的仓无关 SKILL）总台定稿广播后按标精对齐（更正单第二条）
- issue 面回执见 REQ-056（实弹 #9 与 #10）
