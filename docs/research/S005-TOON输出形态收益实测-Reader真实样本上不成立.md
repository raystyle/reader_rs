# S005-TOON输出形态收益实测-Reader真实样本上不成立

> 2026-09-01。触发点：用户点名 TOON 立项评估；S002 待办 3 遗留假设「TOON 对中文内容的 token 收益未验证，引 toon-format 前用 Reader 真实输出测一轮」。流程按 R002；六态标准见 G002。PoC 工程 `target\poc-toon\`（gitignored，证据以本文记录为准）。

## 背景

TOON（Token-Oriented Object Notation）是面向 LLM 的省 token 结构化格式，S002 从 incurs（默认输出即 TOON）引入候选。要回答：Reader 的 `--format json` 包膜换成或加上 TOON，token 收益是否成立。

## 关键结论

1. **TOON 不引入，销候选**。Reader 真实输出的 token 收益为负或可忽略：中文 search 包膜**更费 21.2%（cl100k）到 25.8%（o200k）**；英文 search 带上下文更费 3.1%；中文大 extract（179 行单元）仅省 0.9% 到 1.5%。[实证: 2026-09-01 target\poc-toon 三样本，tiktoken-rs cl100k_base 加 o200k_base 双编码器]
2. **toon-format 0.5.0 往返破损**：对 search 包膜的 `before[]` / `after[]` 嵌套小数组，编码器产出的串其自家 strict 与 lenient 解码器均报 `Array length mismatch` 解不回；另两样本往返无损。[实证: 同上，encode 后 decode_strict 与 decode 双双失败于同一样本]
3. 根因分析：TOON 收益形态是**同构对象表格**（键去重省 token）；Reader 包膜是 `ok/data/meta` 异构嵌套（hits 内含 before/after 小数组），落在收益形态之外；中文正文 token 由 CJK 字符主导，语法结构差异贡献小，而 TOON 的计数前缀（`hits[10]:`）与键签名（`{line,text}:`）反而新增 token。incurs 默认 TOON 成立是因为其输出为 CLI 清单类表格，形状不同。[推断: 由 PoC 输出形态与 TOON spec 表格优化机制推出]
4. crate 本身健康：MIT、0.5.0、2026-05 更新、292k/90d、176 星、spec v3.0：不引入是**形状不匹配**，非质量问题。[实证: 2026-09-01 cargo info 加 gh repo view toon-format/toon-rust]
5. S002 待办 3 销案：假设已验为不成立。[实证: 本文]

## 现状或实测

### 双通道核实

| 通道 | 证据 |
| --- | --- |
| crates.io | toon-format 0.5.0，MIT，updated 2026-05-22，recent 292,799，total 521,627 [实证] |
| GitHub | toon-format/toon-rust，176 星，2025-11 建仓，2026-05-22 推 [实证] |

### PoC 对照表

样本：`reader search/extract --format json` 真实输出（测试V2.docx 中文命中 10 条；渗透方案 v1.0.docx 无标题整篇 179 行；model_comparison.pdf 英文命中带 `-C 1` 上下文）。

| 样本 | 字节 json 转 toon | 省% | cl100k | 省% | o200k | 省% | 往返 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| search_zh（1637B） | 至1841 | **-12.5** | 520至630 | **-21.2** | 423至532 | **-25.8** | 无损 |
| extract_zh（17690B） | 至17383 | +1.7 | 6993至6931 | +0.9 | 5027至4951 | +1.5 | 无损 |
| search_en（13123B） | 至13466 | -2.6 | 5518至5688 | -3.1 | 4467至4607 | -3.1 | **破损** |

[实证: 2026-09-01 cargo run，负数表示 TOON 更费]

## 踩坑沉淀

| 现象 | 根因 | 正确处理 |
| --- | --- | --- |
| GitHub 仓 lib.rs 文档自称「Full implementation coming soon」，crates.io 却有 0.5.0 实体 | 仓库文档滞后于发布 | 以 registry 缓存的发布源码为准（`encode(&T, &EncodeOptions)` / `decode` / `decode_strict`，无 `to_string`/`from_str`）；API 认知以 cargo 编译错与本地源码校正，不照 README 想当然 |
| 「省 token」宣称与实测相反 | 收益依赖输出形状（同构表格），宣称来自其目标场景 | 引入任何「面向 LLM 的格式」前，先拿本工具真实输出测 token（双编码器），宣称不当依据 |

## 待办

1. 若日后输出面出现大表格形态（如批量目录扫描清单），复测 TOON；届时要求上游修复 `before/after` 形状往返破损后再评。[假设: 表格形态下收益转正]
