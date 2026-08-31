# .tools：项目自定义脚本工具

> 角色：项目管理与操作过程中**按需自定义、临时编写**的 PowerShell、Python、Rust 工具及代码的归档目录，进 git。用完即归档进此目录，不散落仓库根或会话里。
> 2026-08-31 自 ohmyagents 同名目录引入（md 三件套原样拷贝，脚本本身通用）。

## 使用规则

1. **归档时机**：会话中为完成某操作临时写出的脚本，若具备复用价值（第二次会用到的），当轮收尾前移入 `.tools\` 并加 PEP 723 头（Python）或用法注释（ps1）；纯一次性的留在对话里不进仓。
2. **Python 统一 uv 载体**：脚本头部带 `# /// script` 内联元数据，运行用 `uv run --script .tools\xxx.py`（不建 venv、不装依赖进环境）。
3. **命名**：小写连字符加用途动词或名词（`md-ref-scan.py`、`md-replace.py`）；ps1 同风格；Rust 专用工具（若有）先 `cargo new` 独立子目录再入 `.tools\`。
4. **工具自述**：每个脚本 docstring 写清用法、参数、退出码；改动同步本 README 清单。
5. **门禁联动**：`md-ref-scan.py` 在文档结构大改（改名、编号、移目录）后必跑；退出码非 0 即有断链，先修后提交。

## 工具清单

| 工具 | 用途 | 用法 |
| --- | --- | --- |
| `md-ref-scan.py` | 全仓 markdown 仓内路径引用断链扫描（结构大改后的回归门禁） | `uv run --script .tools/md-ref-scan.py [--root docs] [--allow 豁免.txt]`；退出码 0/1 |
| `md-heading-scan.py` | 标题括号规范扫描（G001 标题干净的机检项；代码围栏内的注释不计） | `uv run --script .tools/md-heading-scan.py [--root docs]`；退出码 0/1 |
| `md-replace.py` | 中文与反斜杠路径安全的字面批量替换（规避 sed 转义坑） | `uv run --script .tools/md-replace.py --glob 'docs/**/*.md' --map 映射.txt [--dry]` |

## 历史注记

- 三件套在 ohmyagents 2026-08-31 文档整编中验证过，本仓原样继承；豁免清单 `md-ref-allow.txt` 同来，按需改。
