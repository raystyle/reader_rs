# ROADMAP

项目全局路线：**大里程碑**。状态：`未开始` / `进行中` / `已完成` / `挂起`。细碎轨迹见 `docs\diary\YYYY-MM-DD-*.md`。方案详情见 `docs\proven\PNNNN-*.md`。

## 阶段总览

| 阶段 | 目标 | 状态 |
| --- | --- | --- |
| 0 | 项目基础设施：对照 ohmyagents 的文档与目录；定位为 Agent 原生文档阅读、搜索和提取工具（P0002） | 已完成 |
| 1 | 最小闭环：search / extract + 集成测试 + 门禁（方案 P0001） | 进行中 |
| 2 | 提取质量：多栏阅读序、CJK 与编码问题页提示、扫描件检出提示 | 未开始 |
| 3 | 输出形态：Markdown 导出、JSON 输出、批量目录扫描 | 未开始 |

## 阶段 0：项目基础设施

AGENTS 四段职责、GOAL/TODO/PLAN 三原语、guide 元规范 G001-G003、references 定位与细则、rumdl 与 `.tools` 门禁。

## 阶段 1：最小闭环

pdf-inspector 提取层；`reader search` / `reader extract`；`rr` 缩写；tests\cli.rs 集成测试；本地门禁三件（fmt / clippy / test --locked）。同阶段完成 EPUB 支持（P0003：rbook 加 quick-xml，TextUnit 格式分派）。

## 阶段 2：提取质量

多栏排版阅读序（pdf-inspector layout 管线）；编码问题页与扫描件页明确提示（`needs_ocr` 信号）；大文档性能观察。

## 阶段 3：输出形态

Agent 原生优先，设计依据 `docs\research\S002-incurs模块经验研究-Agent原生CLI的命令输出与帮助设计.md`：结构化输出包膜（`{ok,data,error}` 加 meta，search/extract 的 `--format json` 为首要候选）；`--filter` 点路径裁剪；extract 分页原语（offset/limit 加 next_offset 与 cta）；agent 发现（`--llms` 索引与 SKILL.md 生成）；远期 MCP stdio 暴露。Markdown 导出（pdf-inspector 管线现成）与目录批量扫描同阶段候选。均按需立项。
