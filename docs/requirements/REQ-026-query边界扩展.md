---
id: REQ-026
title: query边界扩展 原D26
status: draft
priority: should
trace: null
---

# REQ-026:query边界扩展 原D26

## Scenario

`reader query` 当前只接受单文件输入（目录不支持）且输出形态为 markdown 片段；批量目录查询与更多输出形态（json 字段细化、行号锚）是候选扩展。

## Criteria

- [ ] 待追问链澄清：目录输入的单元口径（对齐 search 批量的 files.scanned 加 skipped 形态？）与输出形态清单
- [ ] 图片输入已裁定拒绝并指路 `--ocr`（D43），扩展不动此裁定
- [ ] 若扩：mq-lang 能力边界先核（S007 结论：全引擎嵌入可用）
