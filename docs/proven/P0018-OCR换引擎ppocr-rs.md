# P0018-OCR换引擎ppocr-rs

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：已完成
- 日期：2026-09-03
- 关联：TODO.md / 研究 `docs\research\S008-OCR质量升级-ppocr-rs的PP-OCRv6原生内核双优胜出现管线换引擎.md` / 前序 `docs\proven\P0014-OCR兜底落地.md`、`docs\proven\P0017-OCR性能优化-宽度分组分批加组间并行.md`

## 背景与问题

P0014/P0017 管线（pure-onnx-ocr 加 tract 跑 PP-OCRv5 mobile）有系统性掉字。S008 实测裁决：weidix/ppocr-rs（PP-OCRv6 safetensors 原生 CPU 内核）质量与速度双优，换引擎。

## 目标与非目标

- 目标：`--ocr` 走 ppocr-rs v6 tiny；S006 掉字点修复；模型管理换 ppocr ModelStore（钉 rev 加 sha256、缓存目录与 offline 语义不变）；旧引擎与 vendor 出树。
- 非目标：不引 gpu feature；不做 v6 small 质量档旗标（留候选）；不发版本。

## 方案

- Cargo.toml：`ppocr-rs` git rev 钉 `d07857c`；删 pure-onnx-ocr/tract-onnx/prost 与 patch；`vendor\pure-onnx-ocr\` 删除退役。
- `src\ocr.rs` 重写：ModelStore（缓存目录解析与 READER_OCR_CACHE_DIR 保留）加 OcrOptions（threads 取 available_parallelism，核数自适应由引擎原生 rayon 承担）；hayro 渲染页转 RgbImage；recognize 出阅读序行、空行滤除；`--offline` 映射 ModelAccess::Offline 加自写错误文案。
- 文案口径更新：mobile 掉字表述全改 PP-OCRv6 tiny（lib.rs 警示、help、introspect、SKILL、README、AGENTS）。

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| ppocr-rs v6 tiny（选定） | 0.8s/页加掉字全修（S008 实测）；0.1.0 社区项目风险以 git rev 钉死加 vendor 兜底对冲 |
| v5 server 换件 | 质量好但 125.6s/页（S008），出局 |
| 留 mobile 加并行（P0017 态） | 掉字不解决，弃 |

## 实施过程与经验

- 实际怎么做：按 PLAN 走完；真样本双路（离线缓存与在线首用下载）exit 0。
- 质量实情：正文掉字全修且比 P0017 态更全（「组织和个人」补齐）；水印区噪声行增多（「AQNIUNET」类）且有一条水印覆盖行幻觉长文；封面页大字标题读散（v6 tiny det 限边 736 所致），均在 needs_ocr 不可靠域，如实入档。
- 依赖面：tract/pure-onnx-ocr/prost 出树，release 二进制 32.9MB 回落到 28.3MB；ppocr-rs 自带 ureq 2 与我们的 ureq 3 共存无冲突。
- P0017 的 vendor 分组并行优化随引擎退役退出代码面（其剖析方法论与数据留在 P0017 归档）。

## 验收标准

- 门禁三件加 rumdl 四件全绿，既有用例零改动（冒烟门控改 tiny 目录形态除外）。[实证: 2026-09-03 clippy/test（22 单元加 52 集成）/rumdl 54 文件零告警/断链 0/标题 0/字符扫描过]
- 真样本回归：安全牛 PDF 页 1 加页 2 `--ocr` 双路（离线缓存、在线首用）exit 0；正文掉字点全修复；在线路两页含下载 9.9s。[实证: 2026-09-03 本机 release]
- 文档回填（AGENTS/INDEX/SKILL 漂移守卫/README/CHANGELOG/diary/S008/本归档）。[实证: 2026-09-03]
