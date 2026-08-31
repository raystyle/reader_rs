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

## 补记：CI 首跑三系统全绿

推送后 GitHub Actions 首跑（run 33378905306）windows/ubuntu/macos 三 job 全 success，约 3.5 分钟。纯 Rust 依赖树零系统库需求，一次全绿。P0004 达成收官。仅有 actions/checkout Node.js 20 弃用注解，不影响结论。

## 补记：WSL 本地接管验证

用户开发环境切到 WSL2 Linux（仓库仍在 `/mnt/d/reader_rs`）。本地门禁六件一次全绿 [实证]：fmt / clippy / test --locked（20 测全过，单元 5 加集成 15）加 rumdl 三件套（26 文件零告警、断链 0、括号标题 0）。clippy 全量检查 19 秒，Windows 旧 target 产物与 Linux 共存无冲突（cargo 按主机指纹自动重建）。CLI 中文输出在 WSL 终端无乱码。Linux 侧接管由 CI 验证升级为本地实证。

## 补记：P0005 立项

用户点名阶段 2 立项。核实 pdf-inspector 1.17.0 本地源码：`extract_pages_markdown` 逐页返回 markdown 加 `needs_ocr` 信号（覆盖 GID 编码、编码问题、乱码、空提取），多栏阅读序与扫描件/编码页检出是同一条管线。关键裁决（用户定）：PDF 通道整体切 markdown 管线，不做双管线——search/extract 共享同一文本层，阶段 3 Markdown 导出顺带半达成；v0.1 输出行为变，CHANGELOG 将记破坏性变更。追加裁决（用户定）：文本质量只承诺英文与中文，落 R001 与 AGENTS 边界。

## 补记：P0005 当日达成

管线切换一次过，22 测全绿（单元 5 加集成 17）。两栏测试 fixture 两轮定形：等距短网格被管线判成表格，读 `split_side_by_side` 阈值（items 至少 40、沟至少 30pt）后改 22 行变宽散文才触发栏检测——阈值门控行为先读实现再设计 fixture。真样本双验：O'Reilly 书 399 页 0.92s（基线 0.57s），24 个图像页全被 needs_ocr 命中，S001 两处粘连瑕疵修复，assert_cmd 25 行命中与旧记录一致；意外收获是 x-quake 论文为中文文档，标题摘要正文提取通顺、中文搜索命中正确，中文质量承诺当场实证。EPUB 37 章 0.083s 无回归。CHANGELOG 记破坏性变更（PDF 行带 markdown 语法）。

## 补记：P0006 立项

用户示意继续，阶段 3 第一刀落输出层三件套（S002 映射 1-3）：`--format json` 包膜、extract `--offset/--limit` 分页（next_offset 加 cta）、`--filter` 点路径裁剪。serde/serde_json 进依赖（事实标准免研究文档）。关键取舍：默认文本形态与退出码 0/1/2 不动，JSON 错误走 stdout 包膜加 stderr 人读行双通道；agent 发现与 MCP 留候选。

## 补记：P0006 当日达成

实现一次过，34 测全绿（单元 7 加集成 27）。clippy `too_many_arguments` 拦下 8 参函数，顺势收成 `OutputOpts` 结构体而非打洞。两个认知点入库：serde_json 默认 BTreeMap 键字母序，包膜顶层用 typed struct 保 `ok/data/meta` 声明序；中文经 serde_json 原样 UTF-8 直出（不转义），对 Agent 省 token。真样本抽查：书 25 命中页号序列与 S001 记录一致，分页 next_offset/cta 正确，中文论文 JSON 通顺。无命中语义定型为 ok:true 加空 hits、退出码仍 1（执行成败与命中有无分轨），README 明示。
