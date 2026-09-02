# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

P0016 markdown 支持落地（登记日 2026-09-03；依据 `docs\research\S007`）。.md 进 search/extract 格式面（零新依赖复用 split_markdown）；`query` 子命令嵌 mq-lang 全引擎做结构化提取（支持全部已支持格式：md 原文、anydoc 家族 GFM、PDF markdown 管线）。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| S007 研究与裁决 | 已完成 | 双通道核实加 PoC 实测；研究文档落盘 | 2026-09-03 |
| .md 进格式面 | 进行中 | document.rs 分派加 is_supported；anydoc.rs 出 md 直读路径；错误文案与帮助面扩格式清单 | 2026-09-03 |
| query 子命令 | 未开始 | src\query.rs：格式到 markdown 文本、mq-lang eval、空渲染过滤；text/json 两形态、退出码 0/1/2 同 search 语义 | 2026-09-03 |
| 测试 | 未开始 | md 夹具分节/分片/搜索；query 正例（.h2/.code/select）、无命中退出 1、坏表达式退出 2、json 形态 | 2026-09-03 |
| 文案与门禁 | 未开始 | introspect 双文本、SKILL 再生成、AGENTS/INDEX/README/CHANGELOG/diary/P0016 归档；门禁三件加 rumdl 三件 | 2026-09-03 |
