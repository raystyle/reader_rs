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

## 补记：当日重新定位为 Reader

用户同日重新定位：项目叫 Reader，定位 Agent 原生文档阅读、搜索和提取工具（P0002 当日达成）。关键裁决：定位落成三条可验收的设计约束（输出稳定可解析、单调用无交互、机器可读优先），不写口号；JSON 输出提为阶段 3 首要候选；代码与 CLI 形态不动。教训：改名类操作先 rg 旧名圈出下游引用面再动笔；历史名称写散文不带路径前缀，避免断链扫描误报。

## 补记：真实样本回归

以《Command-Line Rust》原书 PDF（5.8MB、390 页）做非自造样本回归：全量提取 0.57s（release），`assert_cmd` 搜索 25 行页码与书内容一致，regex/-C/--pages 组合正确。发现两处行重建瑕疵（旁注 URL 粘连、图片占位符粘连），记入 S001「真实样本回归」节，归阶段 2 提取质量处理。另一坑：Git Bash 的 `/d/` 路径喂给原生 Windows 二进制会 os error 3，CLI 测试与脚本里要用 Windows 路径形态。

## 补记：incurs 模块经验研究

用户指定研究 incurs（wevm/incur 的 Rust 移植）。结论：库本体信号弱（1 star、0.5.x）不引为依赖，设计全盘可学——输出包膜（ExecuteResult/OutputEnvelope）、cta 与 next_offset 分页原语、TOON 默认格式与 token 计数、SKILL.md/--llms agent 发现、MCP 命令树投影、OutputPolicy 受众分路。落 S002，五条映射全部列为阶段 3 候选待立项。方法论收获：R002 的「选依赖 vs 选学习对象」分流第一次真实用上。

## 补记：EPUB 支持达成

用户追加 EPUB 需求并给真实样本。选型 S003：`epub` crate GPL-3.0 一票出局（license 一查到底），选 rbook 加 quick-xml。结构升级 TextUnit 统一页/章，搜索层零逻辑改动。三个实测坑：quick-xml 0.42 版本敏感（tag 名 &str、实体 GeneralRef）；行内标签边界丢空格（单元测试抓到）；pre 代码块塌一行（真实样本抓到，加 pre 通道）。cargo test 20 绿，样本 37 章回归正确。P0003 当日达成。

## 补记：README 改为使用者向

用户指出 README 应是部署安装和命令参数使用方法。重写为使用者向：安装（cargo install --path / --git）、search/extract 参数表与输出格式、退出码、支持格式表；协作者导航压到末节。

## 补记：跨平台接管准备

用户要在 mac/Linux 接管开发测试。立项 P0004：GitHub Actions 三系统矩阵跑门禁三件（仓库已在 GitHub，零新增设施，R003 第三段既定路线落地）、.gitattributes 钉 LF、README 命令给 bash 形态。审计结论：代码层无平台专属假设（提取库全纯 Rust、测试用 temp_dir 加 pid），唯一已知差异是使用层路径形态（M002）。验收以 CI 首跑三系统绿为准。
