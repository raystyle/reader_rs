# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**——基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：超长节再分片、批量目录搜索、musl 资产

> 用户点名三项按序推进，登记日 2026-09-01；方案 P0011 / P0012 / P0013。

### 1. 闸门

P0010 行分片机制可直接推广（同预算同函数）；批量搜索复用 document::extract 与 search::Matcher 现成接缝；musl 是 P0008 流水线的矩阵延伸，无新选型。

### 2. 实施件与依据

| 件 | 要做什么 | 依据 |
| --- | --- | --- |
| `src\anydoc.rs`（P0011） | 逐节判定：`len > 200` 切 part，短节留 section；无标题路径不动 | P0011 方案节 |
| `src\batch.rs`（新，P0012） | 递归走目录（排序、支持扩展名）、逐文件 extract 加 search、text/json 聚合、坏文件跳过 | P0012 方案节 |
| `src\document.rs` / `src\lib.rs` | 抽 `is_supported` 谓词；run_search 按 `is_dir` 分流；help 文案 | P0012 方案节 |
| `.github\workflows\release.yml`（P0013） | matrix 增 musl 目标加 musl-tools 安装步 | P0013 方案节 |
| `tests\cli.rs` | 超长节混排夹具；批量三态（命中加跳过、无命中、--pages 拒绝） | R003 |
| introspect / SKILL / README / CHANGELOG | 分片口径推广、批量搜索用法、musl 资产行 | P0007 守卫 |

### 3. 每件验收

门禁三件加 rumdl 三件套全绿；既有用例零改动；新用例绿；真样本冒烟（测试V2 15 节、材料目录批量命中带路径）。musl 以 v0.2.1 tag 首跑五 job 全绿为验收。失败当场记 `docs\mistakes\`。

### 4. 边界

不做批量 extract、inventory 子命令、并发遍历；不做 aarch64-musl；预算不做旗标。[依据: 三方案非目标节]

## 完成的定义

> 本目标验收口径。

- TODO 表全部已完成或明确跳过（跳过须写 mistakes 原因）
- 门禁三件与 rumdl 三件套过；既有用例零改动全绿
- 三方案回填实施过程与经验；CHANGELOG Unreleased 收口三件；INDEX/GOAL/diary 登记
- musl 验收挂 v0.2.1 tag 首跑（待用户确认发版）
