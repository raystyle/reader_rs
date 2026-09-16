---
id: REQ-025
title: OCRv6small质量档旗标 原D25
status: draft
priority: should
trace: null
---

# REQ-025:OCRv6small质量档旗标 原D25

## Scenario

tiny 档快但混排偶有掉字，用户要更干净输出时可切 small 档（3.2 秒/页量级）。现状已支持 `ocr switch small` 与 `READER_OCR_MODEL_SIZE`，本需求评估是否给 extract/search 一个显式质量档旗标（如 `--ocr-quality small`）省去预切换。

## Criteria

- [ ] 待追问链澄清：旗标形态还是维持 switch 加 env 两级；旗标是否一次性（不落设置）
- [ ] A/B 证据在册：`tests/ab/reports/2026-09-03-tiny-vs-small.md`（合成样本 small 4/5 对 tiny 1/5，真样本 51 对 37 行）[实证]
- [ ] 若立旗标：README 与 SKILL 与 `--help` 同步，行为测试进 tests/cli.rs
