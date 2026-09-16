# reader_rs::figures

图片本体导出与文本元数据对齐（D47；S010 定界：只负责提取存储，理解归调用方 Agent）。
四路分派：PDF 按页渲染 PNG（扫描书页即图本体，图题与上下文从页文本层对齐）；
markdown 解析 `![alt](path)` 引用复制；anydoc 家族 zip 直读内嵌图片部件（原字节）；
图片文件本体即自身。输出行式清单与 json 包膜，退出码 0 有图 / 1 无图 / 2 出错。

## Functions

- `extract_figures` — 提取图本体到 `out_dir`,按文档格式分派;`filter` 为 1 起页集合(仅 PDF 生效)。

## Types

- `FigureOut` — 一件导出的图本体：kind 加锚定位回文档，caption 加 context 是与文本元数据的对齐面。

