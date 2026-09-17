# ADR 索引

> 架构决策记录：不可逆技术选择先立 ADR 再动手。新建拷 0000-template.md，编号接当前最大号，退役不复用。状态流转 proposed 到 accepted 到 superseded；supersede 须两篇互指。P 编号方案档案与 M 编号错误档案已清退（2026-09-17，ADR-0006）：存量知识由各 ADR 与 ADR-0006 蒸馏表承接，P / M / D 编号仅作历史沿革标签；被否决的选择也是决策。

| id | 状态 | 标题 | 替代 |
|---|---|---|---|
| ADR-0001 | accepted | 产品定位Agent原生文档阅读搜索和提取工具 | 承接 P0002 / R001 |
| ADR-0002 | accepted | anydoc统一文档引擎与PDF直连双轨 | 承接 P0009 / S004 |
| ADR-0003 | accepted | OCR兜底纯Rust管线ppocr-rs跑PP-OCRv6 | 承接 P0014 / P0017 / P0018、S006 / S008 |
| ADR-0004 | accepted | 模型与升级自维护镜像三级回退 | 承接 D42 / ISSUE #1 |
| ADR-0005 | accepted | aidoc投影强制化与不适用裁定撤换 | 撤换 2026-09-16 早间「无自有 API 面」裁定；REQ-049 |
| ADR-0006 | accepted | 历史档案清退与知识融入 | 撤 REQ-048 留档口径；REQ-054 |
