# ROADMAP

项目全局路线：**大里程碑**。状态：`未开始` / `进行中` / `已完成` / `挂起`。细碎轨迹见 `docs\diary\YYYY-MM-DD-*.md`。方案详情见 `docs\proven\PNNNN-*.md`。

## 阶段总览

| 阶段 | 目标 | 状态 |
| --- | --- | --- |
| 0 | 项目基础设施：对照 ohmyagents 的文档与目录；定位为 Agent 原生文档阅读、搜索和提取工具（P0002） | 已完成 |
| 1 | 最小闭环：search / extract + 集成测试 + 门禁（P0001）；EPUB 支持（P0003） | 已完成 |
| 2 | 提取质量：多栏阅读序、中文与编码问题页提示、扫描件检出提示（P0005） | 已完成 |
| 3 | 输出形态：Agent 原生（JSON 包膜、filter、分页、agent 发现，依据 S002） | 进行中 |
| 4 | 跨平台接管：CI 三系统门禁、文档双形态（P0004） | 已完成 |
| 5 | 格式扩展：anydoc 统一文档引擎，格式面 2 到 14 种（P0009，选型 S004） | 已完成 |

## 阶段 0：项目基础设施

AGENTS 四段职责、GOAL/TODO/PLAN 三原语、guide 元规范 G001-G003、references 定位与细则、rumdl 与 `.tools` 门禁。

## 阶段 1：最小闭环

pdf-inspector 提取层；`reader search` / `reader extract`；`rr` 缩写；tests\cli.rs 集成测试；本地门禁三件（fmt / clippy / test --locked）。同阶段完成 EPUB 支持（P0003：rbook 加 quick-xml，TextUnit 格式分派）。

## 阶段 2：提取质量

多栏排版阅读序（pdf-inspector layout 管线）；中文与编码问题页、扫描件页明确提示（`needs_ocr` 信号，质量承诺只面向英文与中文）；大文档性能观察。方案 P0005（PDF 通道整体切 `extract_pages_markdown`，TextUnit 扩展 needs_ocr，extract/search 提示路径）。**2026-08-31 达成**：22 测全绿，中英文真样本双验，S001 两瑕疵修复，399 页 0.92s。

## 阶段 3：输出形态

Agent 原生优先，设计依据 `docs\research\S002-incurs模块经验研究-Agent原生CLI的命令输出与帮助设计.md`：结构化输出包膜（`{ok,data,error}` 加 meta，search/extract 的 `--format json` 为首要候选）；`--filter` 点路径裁剪；extract 分页原语（offset/limit 加 next_offset 与 cta）；agent 发现（`--llms` 索引与 SKILL.md 生成）；远期 MCP stdio 暴露。Markdown 导出已由 P0005 顺带达成（PDF 默认输出即 markdown 行）；目录批量扫描同阶段候选。第一刀 P0006 已达成（2026-08-31：JSON 包膜、分页、filter，34 测全绿）；agent 发现、MCP、批量、token 计数留候选。

## 阶段 4：跨平台接管

macOS / Linux 接管开发与测试：GitHub Actions 三系统矩阵跑门禁三件（`.github\workflows\ci.yml`）；`.gitattributes` 钉 LF；文档命令双形态。方案 P0004。验收以 CI 首跑三系统绿为准。

## 阶段 5：格式扩展（anydoc 统一引擎）

用户裁定大重构：Word（含 legacy .doc）/ EPUB / ODT / RTF / PowerPoint / Excel / ODF / CSV 统一走 anydoc 0.2.4 出 GFM markdown 按标题分节；PDF 保持 pdf-inspector 直连（页契约）。破坏性变更：EPUB 单元由章改节。**2026-09-01 达成**（P0009）：37 集成加 9 单元测试全绿，真样本四路回归。选型反复与保真实测记 S004。
