# A/B 对比报告：tiny vs small

> 日期 2026-09-03；CPU 逻辑核 32；reader `D:\reader_rs\target\release\reader.exe`；逐页 wall 含引擎装载与渲染。

## scan-cjk：1 页

| 指标 | tiny | small |
| --- | --- | --- |
| wall 秒 | 0.2 | 0.5 |
| 秒/页 | 0.20 | 0.46 |
| 行召回 | 5 | 5 |
| must 命中 | 1/5 | 4/5 |
| 判别点命中 | 0/0 | 0/0 |

- tiny 缺失检查点：Automated Penetration Testing Guide 2026、第十章 漏洞扫描器与渗透测试框架、攻击者潜伏期高达 287 天，人工完成的排查占比低、采用自动化工具后，报告显示覆盖率显著提升
- small 缺失检查点：采用自动化工具后，报告显示覆盖率显著提升

## anniu-p10：1 页

| 指标 | tiny | small |
| --- | --- | --- |
| wall 秒 | 2.0 | 5.0 |
| 秒/页 | 2.03 | 5.00 |
| 行召回 | 37 | 51 |
| must 命中 | 5/5 | 5/5 |
| 判别点命中 | 1/1 | 1/1 |
