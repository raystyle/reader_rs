# reader_rs::batch

批量目录搜索（P0012）：递归走目录、逐文件 search、聚合 text/json 输出。
单文件 search 的输出与退出码契约不变；目录模式命中行带路径前缀，坏文件 stderr 跳过后继续。
天花板：顺序遍历不并发；符号链接跟随（依赖文件系统无环）；needs_ocr 走 stderr 不进 json。

