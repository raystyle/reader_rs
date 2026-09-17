---
id: REQ-058
title: cli-docs细标精对齐
status: implemented
priority: must
trace: src/lib.rs（apply_help_face 全树帮助面模板加裸调用全貌形 exit 0）加 src/introspect.rs（裸调用契约行加 leaves 走 command_tree）加 tests/cli.rs（bare_invocation_full_help_exit_zero 加 help_face_header_injects_name_version；旧 dies_no_args 退役）加 tests/snapshots（手册快照重出人工审收录）加 README（部署节补 sha256 校验命令块）；门禁 cargo fmt 加 clippy 加 test --locked 加 --doc 加文档四件加 cargo aidoc --check --strict 加 PEVO check 全绿 [实证: 2026-09-18 本机门禁退出码，验收记录见 diary 当日]
---

# REQ-058:cli-docs细标精对齐

## Scenario

总台派单（2026-09-18 全仓 cli-docs 采纳轮）：以 project-evo cli-docs 技能为标准权威（甲面 README 四节骨架、乙面 agent 五件），逐件盘点本仓 CLI 对外面，缺啥补啥，改进后封版。本 REQ 即 REQ-057 Notes 与 G008 二节挂起的「细标总台定稿广播后按标精对齐」承诺的兑现件。

## Criteria

- [x] 对照表成文：甲乙两面逐件给现状、本批改动、不适用与理由（见下节，裁剪显式记录）
- [x] 帮助面头行：全树注入 `路径@版本 描述`（版本编译期自 Cargo.toml，与 `--version`、`--llms` 同源；守卫测试断言根与叶）
- [x] 裸调用面：无参运行出帮助体（stdout 全貌形，含 `--llms` 发现指引），退出 0；旧 stderr 加退出 2 行为退役（组子命令缺叶仍退出 2，属用法错误不在标准辖内）
- [x] `--llms` 手册补裸调用契约行；快照重出人工审收录
- [x] README 部署节补二进制校验命令块（sha256sum / shasum / Get-FileHash 三平台）
- [x] 门禁与封版按 R008 全走（本 REQ 承载代码与文档面，封版验收记 diary 与回执）

## 对照表

盘点时点 2026-09-18，标准原文见 Notes。

甲面 README（四节骨架，G008 为本仓正源）：

| 件 | 现状 | 本批改动 | 不适用与理由 |
| --- | --- | --- | --- |
| H1 加徽章（至多 3 枚） | H1 与 CI、MIT 两枚 | 无 |  |
| 项目介绍（定位、特性、演示、边界） | 一句话定位（与 About 逐字一致）、速览表、特性行、仓间分工边界 | 无 |  |
| 部署（安装、升级、校验） | 预编译资产表、cargo 两路、self update 三路升级 | 补 sha256 校验命令块 |  |
| 配置（配置文件、环境变量表） | 环境变量五行表加模型档位与缓存结构 | 无 | 无配置文件：档位持久化文件（model-size）与缓存目录已述，键值样例即命令示例 |
| 使用（渐进教程、集成、指向 --help 与 --llms） | 快速开始、Agent 发现（--llms 两形）、命令参考、JSON 输出、支持格式 | 无 |  |
| 可选尾节 | 文档导航、贡献与支持、致谢、License 置末 | 无 |  |
| 锚点目录 | 未设 | 无 | 八个二级节未超阈，G008 裁定不设目录保扫读 |

乙面 agent 五件：

| 件 | 现状 | 本批改动 | 不适用与理由 |
| --- | --- | --- | --- |
| 一、`--llms` 手册面 | 47 行（预算 120）、活树渲染加 curated 段、`--llms --json` 机器形、stdout 退出 0 无交互、版本注入 | 契约节补裸调用行 |  |
| 二、输出协议旗标七件 | 见下分件表 | 无（裁剪理由在册） |  |
| 二、信封与字段序 | `{ok,data|error,meta}` 声明序稳定、compact 单行 | 无 |  |
| 二、类型化 CTA | extract 分页 meta.cta 字符串形（下一条可直接执行命令） | 无 | 块形（description 加 commands 数组）升级属 meta.cta 形变即契约破裂，记候选不动；字符串形在标准载体形内 |
| 二、错误分道（stderr 单行 JSON） | 错误包膜走 stdout（ok:false）加 stderr 人读行双通道 | 无 | REQ-057 已定的双通道契约；改道属 major 契约破裂，不在采纳轮 |
| 二、退出码 0/1/2 | grep 语义族全覆盖 | 无 |  |
| 三、默认帮助面节序 | clap 渲染：Usage、Commands（对齐）、Arguments、Options（默认后缀加枚举全值）、Examples（after_long_help 长形出） | 头行补 `路径@版本`（全树模板注入） | Global Options 节不设：format 与 filter 按命令声明即本命令 Options，根级 --llms 与 --json 在根 Options，语义等价不另造节；Environment Variables 节不进 help：env 唯一权威在 README 配置节加 --llms curated 行，help 再列即无守卫第三份 |
| 四、自省三面同源加漂移守卫 | 活树唯一真源（help、手册、JSON 派生）；tests/cli.rs 旗标全覆盖加机器形叶数全等两守卫 | 新增头行注入守卫；introspect 走 command_tree 同入口 |  |
| 五、裸调用面 | stderr 帮助加退出 2（反例形） | 改全貌形：stdout 帮助体含 --llms 指引，退出 0 |  |

输出协议旗标分件（标准必选七件加可选件）：

| 旗标 | 现状 | 裁剪理由 |
| --- | --- | --- |
| `--help` / `-h` | 在（clap） |  |
| `--llms` | 在 |  |
| `--format` | 在，值域 text 与 json | 标准 toon 族映射：text 即人读缺省形；yaml 与 md 不引入，文本层工具两形态已足（REQ-057 定） |
| `--filter` | 在（同义 --filter-output） | 点路径含数组下标（hits[].text、units[0].lines），语义等价；名按 REQ-057 已广播契约 |
| `--json` | 顶旗标仅配 `--llms` 出机器形 | 命令级 JSON 走 `--format json` 防歧义（单独 --json 报错退出 2，REQ-057 定） |
| `--full-output` | 无 | json 恒全信封输出，--filter 只裁 data 不裁信封；text 形态即纯数据。无裸数据模式可切换，件失去语义 |
| `--schema` | 无 | 输出 schema 由 --llms 契约节加 README JSON 节承载，库面类型契约由 docs/aidoc 投影强制门禁承载；再加第四面即无守卫第二真相 |
| 可选件（version、config 等） | `--version` 在；`--config` 无 | 无选项缺省文件可配（配置面全走 env），件不适用 |

## Notes

- 标准原文：project-evo cli-docs SKILL.md 加 references 三件（readme-standard、agent-face、templates）
- 帮助面模板机制：`command_tree()` 套 `apply_help_face` 递归模板，`run()` 经同树解析，解析错误与 --help 面同源；快照与守卫均自该树派生
- 候选（触发再动）：CTA 块形升级（需 meta.cta 形变，随下次契约批）；`--schema` 引入（需活树派生 args 与 options 面，output 面单源方案先立）
