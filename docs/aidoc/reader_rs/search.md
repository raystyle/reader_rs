# reader_rs::search

搜索层：匹配器（字面 / 正则 / 忽略大小写）与命中收集。

## Functions

- `search` — 在所有文本单元的重建行中搜索，`context` 为前后各带的上下文行数。

## Types

- `Hit` — 一次命中：单元序号（页/章）、单元内行号（均 1 起）、命中行文本与上下文。
- `Matcher` — 行匹配器。

