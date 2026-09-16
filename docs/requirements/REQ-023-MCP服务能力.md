---
id: REQ-023
title: MCP服务能力 原D23
status: draft
priority: should
trace: null
---

# REQ-023:MCP服务能力 原D23

## Scenario

Agent 管线希望经 MCP stdio 暴露 reader 的读、搜、提取能力，替代拼 shell 命令的调用方式（设计依据见 S002 远期候选节）。

## Criteria

- [ ] 待追问链澄清：工具面取哪些子命令、输出形态如何映射 MCP 工具返回、是否复用 CLI 同一 run() 分发
- [ ] 立项前先核 MCP 协议现状与 Rust SDK 选型（R002 双通道）
- [ ] 不破 CLI 唯一交互面裁定：MCP 是新增暴露面，不是替换
