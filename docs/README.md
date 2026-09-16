# docs 地图

> 全仓文档索引（2026-09-16 起承接旧根 INDEX 职责，dev-evo 文档即代码体系）。旧 INDEX.md 迁移期保留（编号表与代码文件位置表仍有效），新文档按本地图落位。
>
> 红线（dev-evo 用户裁定 2026-09-16）：`diary/` 与 `research/` 是保留核心结构，不可裁撤。
> 双目录并存裁定：`guide/`（G 编号元规范，被 AGENTS 与 G006 等全仓引用的活档体系）与 `guides/`（dev-evo 任务导向指南与留档）语义不同故并存，不合并。

## 活跃体系

> dev-evo 文档即代码体系。

| 位置 | 讲什么 | 何时看 |
|---|---|---|
| `../AGENTS.md` | 五节合同（Commands / Must / Must not / Read first / 环境） | 每轮开工前 |
| `adr/README.md` | 架构决策索引（ADR-NNNN，仍约束现状的决策择要） | 立不可逆选择前 |
| `requirements/README.md` | 需求登记索引（REQ-NNN，draft 到 implemented 带 trace；D 号即 REQ 号口径见其 README） | 立需求或查验收时 |
| `guides/` | 任务指南（getting-started、旧三节协作规则留档） | 做事前查方法 |
| `diary/` | YYYY-MM-DD 一天一篇过程与自省 | 查当天做了什么 |
| `research/` 加 README | SNNN 研究档案（六态标注） | 找为什么时 |
| `../CHANGELOG.md` 加 `../ROADMAP.md` | 版本成果与阶段 | 查历史与进度 |
| `../SKILL.md` | agent 命令契约（`reader skill` 生成物，漂移守卫） | 查命令面 |

## 历史体系

> 迁移留档，指针有效。

| 位置 | 讲什么 | 迁移去向 |
|---|---|---|
| `proven/` | P 编号方案归档（P0001 至 P0018，已完成方案全文） | 择要升 ADR（四件已转），全文留档 |
| `references/` | R 编号做事的流程（R001 定位、R002 选型、R007 工作流、R008 封版仍现役） | 活档保留 |
| `guide/` | G 编号元规范（G001 命名、G002 六态、G004 禁字、G005/G006 测试、G007 基线、G008 README 仍活） | 活档保留 |
| `mistakes/` | M 编号错误档案（M1xx 分类文件、M0xx 行级） | 留档不再接编，新坑走 ADR 或 diary 沉淀 |
| 根 `PRD.md` | D 编号需求清单（D01 至 D47 历史） | 新需求走 REQ；历史留档 |
| 根 `GOAL.md` / `PLAN.md` / `TODO.md` | 旧四原语（目标轨迹与方案与进度） | 历史留档（顶部迁移注记） |
| 根 `INDEX.md` | 旧唯一索引（编号表与代码文件位置表） | 本地图承接，迁移期保留 |
| `../poc/` | S 编号 PoC 原型产物（登记表见其 README） | 研究配套保留 |
| `../.tools/` | 项目脚本工具（md 四件门禁加跑批与镜像件，清单见其 README） | 活档保留 |

## 迁移映射

> dev-evo base-init 存量迁移口径（2026-09-16 全量迁移，楷模仓 hst_rs）。

PRD 条目对应 REQ（编号连续性用 D 号即 REQ 号：D01 至 D47 历史留档，活跃队列 D23 至 D26 与 D36 至 D39 按 D 号转 REQ，新需求自 REQ-049 接编）；PLAN 与 TODO 对应 REQ 的 Criteria 与 trace 或 ROADMAP；GOAL 定位句并入 AGENTS 头部与 README；INDEX 职责由本地图加 AGENTS Read first 承接；proven 语义由 implemented 需求加关联 ADR 承接；mistakes 并入 ADR 或登记留档。迁移不是搬运是重审：历史不回填、活档不搬家、断链必回归。迁移本身立 REQ-048。

### 存量禁字债口径

PE-11 历史档案豁免走 `PEVO_CHECK_ALLOW` 机制（分号分隔正则，匹配 `docs/` 下相对路径加行号，命中报 SKIP 带处数；根三件 AGENTS / README / CHANGELOG 永不受益）。本仓存量禁字为零（G004 存量清零，D22），当前无需豁免正则，标准命令直跑：

```bash
uv run /mnt/d/ProjectEvo/plugins/project-evo/skills/dev-evo/scripts/check.py .
```

日后历史档案留档面若新增禁字（历史不改写），按路径级豁免登记在册（只作用 `docs/` 档案，活跃面零容忍），例如 `PEVO_CHECK_ALLOW='^docs/proven/'`；正则口径与 scan 的 `PEVO_SCAN_ALLOW` 同一惯例。
