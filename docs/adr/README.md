# ADR 索引

> 架构决策记录：不可逆技术选择先立 ADR 再动手。新建拷 0000-template.md，编号接当前最大号，退役不复用。状态流转 proposed 到 accepted 到 superseded；supersede 须两篇互指。历史方案全文在 `docs/proven/`（P 编号归档件），ADR 择要承接仍约束现状的决策并指针回指；被否决的选择也是决策（旧 mistakes 体系升格口径同此）。

| id | 状态 | 标题 | 替代 |
|---|---|---|---|
| ADR-0001 | accepted | 产品定位Agent原生文档阅读搜索和提取工具 | 承接 P0002 / R001 |
| ADR-0002 | accepted | anydoc统一文档引擎与PDF直连双轨 | 承接 P0009 / S004 |
| ADR-0003 | accepted | OCR兜底纯Rust管线ppocr-rs跑PP-OCRv6 | 承接 P0014 / P0017 / P0018、S006 / S008 |
| ADR-0004 | accepted | 模型与升级自维护镜像三级回退 | 承接 D42 / ISSUE #1 |
