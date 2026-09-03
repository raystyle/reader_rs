# tests\ab：A/B 对比测试

> 角色：**A/B 测试层**的落点（六层之一，标准见 `docs\guide\G006`）：同一批对象资源上跑两个变体（A/B），对比**质量**（检查点命中、行召回、噪声）与**性能**（秒/页），出可存档的对比报告。
> 需求来源：D29。当前唯一 A/B 面是 OCR 模型档位（`READER_OCR_MODEL_SIZE`：tiny / small，P0018 引擎）。

## 构成

| 件 | 作用 |
| --- | --- |
| `manifest.json` | 对象资源登记：样本名、类型（synthetic 入仓 / external 版权样本只记路径与 sha256）、页码、期望文件 |
| `assets\` | 入仓的合成样本（如 `scan-cjk.pdf`，无文本层扫描件形态，由 `.tools\make-scan-sample.py` 生成） |
| `expectations\` | 质量检查点：`must_contain`（应命中文本，独立来源：合成样本为渲染源文本，真样本为 S008 实证的掉字点） |
| `reports\` | 跑批报告存档（markdown，带日期与变体对） |

## 跑法

```powershell
cargo build --release
uv run --script .tools\ab_run.py --a tiny --b small            # 全样本
uv run --script .tools\ab_run.py --a tiny --b small --sample scan-cjk
```

- 首次跑 small 档会在线下载模型（ModelStore 钉 sha256）；`READER_OCR_CACHE_DIR` 可指定缓存目录。
- 报告同时打印 stdout 并落 `reports\<日期>-<A>-vs-<B>.md`。
- A/B 是对比不是门禁：报告命中差与倍速差，裁决回 S 文档；退出码只表跑批本身成败（0 成 / 2 错）。

## 规则

1. 检查点必须有独立来源，禁止从 OCR 输出反向抄成期望（G005 重言式禁令适用）。
2. external 样本不入仓；路径换机失效时跑批器跳过并告警，不算失败。
3. 新样本先进 manifest 再跑；新变体对报告单独存档，不覆盖旧报告。
