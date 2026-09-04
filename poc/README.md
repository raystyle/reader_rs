# poc：研究原型目录

> 角色：研究（`docs\research\` S 编号）的**原型产物**落点。一个研究主题一个子目录；结论与六态裁决永远以对应 S 文档为准，本目录只存可复现的代码形态。
> 约定依据：D30（用户 2026-09-03 裁定「研究的产物是一个 PoC 原型，对应 poc 目录」）；写作与命名遵守 G001 / G004。

## 约定

1. **落点**：`poc\<主题短名>\`，目录名带研究编号前缀（如 `s006-ocr-mobile`）。
2. **入仓**：PoC 源码、构建清单（`Cargo.toml`）、辅助脚本、自述。**不入仓**：构建产物（`target\`）、模型与样本大件（`models\`、`out\`）、第三方 vendor 源码；一律 `.gitignore` 钉死。
3. **自述**：每个 PoC 子目录带 `README.md`，写清对应 S 编号、复现步骤、裁决结果与退役状态。
4. **上游 clone 类**：若 PoC 是对上游仓库的实测 clone（如 ppocr-rs benchmark），不搬源码入仓，只在本 README 登记表中记指针（仓库、rev、实测结论挂的 S 文档）。
5. **退役**：PoC 转正进 `src\` 或被裁决出局后，源码保留备查，自述标注退役去向。

## 登记

| 目录 | 对应研究 | 内容 | 状态 |
| --- | --- | --- | --- |
| `s006-ocr-mobile\` | S006 | hayro 渲染加 pure-onnx-ocr 跑 PP-OCRv5 mobile 的端到端 PoC（src 加 strip_value_info.py） | 已退役：P0014 转正后被 P0018 换引擎取代；vendor-poon 与 models 未入仓 |
| `s010-chart-geometry\` | S010 | ppocr-rs polygon 盒子几何配对还原图表数据（对安全相似系统学.pdf 图1-1 实测；src 加 Cargo.toml 加 README） | 活跃（D47 T2 验证） |
| （上游 clone 指针） | S008 | ppocr-rs 实测 clone（rev d07857c，BENCHMARK 与四配置对比）；源码不入仓，结论见 S008 | 已转正：P0018 入依赖树 |
