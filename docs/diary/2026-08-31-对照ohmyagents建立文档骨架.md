# 2026-08-31-对照ohmyagents建立文档骨架

> 项目日记：一天一篇总结与自省。概貌、关键裁决、得失；不写工作细节（细节进方案与研究）。

## 概貌

建仓日。用户要 Rust 版 PDF 文本搜索提取工具，指定对照 ohmyagents 的项目结构与文档体系，点名研究 firecrawl/pdf-inspector，命令名 `reader`（缩写 `rr`）。当天完成文档骨架（四段 AGENTS、三原语、docs 六目录、guide 元规范三件、`.tools` 三件套）、选型研究 S001 与方案立项 P0001，随后推进最小闭环实现。

## 关键裁决

1. **提取引擎定 pdf-inspector**：用户点名方向经双通道核实成立（crates.io 1.17.0 + GitHub 17k stars 活跃），纯 Rust 默认构建满足「不外挂二进制」边界。证据链在 S001。
2. **双 bin 同入口**：`reader` 与 `rr` 共用一个 `src\main.rs`，两个 `[[bin]]` 段，零额外代码。
3. **测试 PDF 现造不存档**：lopdf 现造两页已知文本的 PDF，期望值天然独立（我们写的文本就是 oracle），免维护二进制样本。
4. **grep 语义**：search 退出码 0/1/2 与 `-C` 上下文直接借 grep 惯例，用户零学习成本，测试好断。

## 得失

- 得：ohmyagents 的文档体系整体平移顺畅，AGENTS 操作规则几乎原样适用（本轮工作本身即按其三原语与五步闭环运转）。
- 失：R002 从 R005 裁剪时删了 Python/PowerShell 两节，若本仓将来上 py 工具需回补或回引。
- 自省：选型研究应先跑通道再下结论，本轮顺序合规；pdfium-render 镜像查无结果未深究，按 [记忆] 降级标注而未写成实证，分寸正确。

## 补记：最小闭环同日达成

P0001 当日收官：`reader search` / `reader extract` 落地，reader/rr 双 bin 同入口；cargo test 13 过（单元 2 + 集成 11），门禁三件与 rumdl 三件套全绿，W3C dummy.pdf 真实冒烟正确。门禁真实拦到两处违规（标题括号、URL 内嵌路径误判），后者记 M001 并首开豁免清单「外部仓内路径」节。git 已初始化并推远端 `https://github.com/raystyle/reader_rs`（main 分支，两笔提交：docs 骨架 + feat 最小闭环）。
