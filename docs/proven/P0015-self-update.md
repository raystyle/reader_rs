# P0015-self-update

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：已完成
- 日期：2026-09-03
- 关联：TODO.md / 参考实现 `D:\ohmyenv-rs\src\selfupdate.rs`、`D:\ohmyagents-rs\src\update.rs` / 发布面 `.github\workflows\release.yml`

## 背景与问题

reader 已有五目标 release 资产（v0.1.0 起），用户点名参考 ohmyenv-rs 与 ohmyagents-rs 加入 self update，要求判断自身路径（current_exe）。

## 目标与非目标

- 目标：
  - `reader self update`：GitHub Releases 最新正式版，版本判新、`--force` 重装。
  - 资产 sha256 digest 钉死校验（API 缺 digest 拒绝升级）；staged 加 rename 原子替换自身（current_exe 判定）与同目录兄弟二进制（reader/rr 双名）。
  - GH_TOKEN 注入认证；403 限流回退 gh api。
- 非目标：
  - 不做 dev/git 通道、不做 downgrade 旗标、不自动更新、不碰 release 流水线、不发版本。

## 方案

新模块 `src\selfupdate.rs`：`asset_target`（cfg 五目标映射 release.yml 命名）再 fetch latest（ureq 加 GH_TOKEN，403 回退 gh api）再版本三元组判新，再下载加 digest 校验，再解包（Windows zip crate、其余 tar 加 flate2 1.x rust_backend）取 reader/rr，最后 staged 加 rename 替换自身与兄弟。`lib.rs` 挂 `self` 子命令组，stdout 稳定行 `self_update: current <v>` / `self_update: updated <旧> -> <新>` 加 `path:` 行，出错退出 2。

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| stable 单通道（选定） | reader 只有 v* 封版流水线；dev 滚动与 git 源码通道无对应设施，YAGNI |
| 三通道对齐 ome | dev 滚动需 pre-release CI，reader 无此流水线，弃 |
| digest 对 exe 判新（ome 模式） | reader 资产是压缩包，digest 与 exe 哈希不可比（oma 经验），改版本判新 |

## 实施步骤

1. 立项登记三原语；研读两仓参考实现。
2. `src\selfupdate.rs` 加 CLI 接线；Cargo.toml 按平台分解包依赖。
3. introspect 双文本加 SKILL 再生成；tests\cli.rs 帮助面用例。
4. 实测：临时目录复制二进制，先 `self update`（已最新）再 `--force`（下载校验解包替换全链路，sha256 对账官方资产）。
5. 门禁全绿后文档回填提交。

## 风险与回滚

- 风险：替换中途失败留半损。缓解：staged 写件加 rename，Windows 改名让位失败即回滚；旧件留 `.old-<pid>` 可手工回退。整体回滚 `git revert`。
- 风险：GitHub 匿名限流。缓解：GH_TOKEN 注入加 gh api 兜底。
- 风险：flate2 版本后端陷阱（0.2 无纯 Rust 后端，M012）。已钉 flate2 1.x 并验 miniz-sys 出树。

## 实施过程与经验

- 实际怎么做：按步骤走完。`self update` 已最新路径与 `--force` 全链路均实测：替换后 reader.exe 与 rr.exe 的 sha256 与官方 v0.2.1 资产内件逐一一致，换新件可运行。
- 与计划偏差：flate2 0.2 默认拉 miniz-sys（C 后端）且无 rust_backend feature，改钉 1.x（M012 沉淀）；Cargo.toml cfg 表少写一个右括号被 toml 解析器当场拦（手滑，无沉淀价值）。
- 沉淀的经验：replace 用「staged 写件加 rename」而非直接 copy 覆盖（oma 模式），Windows 运行中 exe 改名让位、失败回滚不留半损；自升级实测用「复制到临时目录再跑」，不动开发位二进制。

## 验收标准

- 门禁三件加 rumdl 三件套全绿，既有用例零改动。[实证: 2026-09-03 fmt/clippy -D warnings/test --locked（19 单元加 47 集成）全绿；rumdl 零告警]
- 单元：资产名映射五目标、版本三元组比较（含 rc 段）；集成：`self update --help` 出 `--force`、裸 `self` 退出 2。[实证: 2026-09-03 cargo test]
- 端到端实测：临时目录 `self update` 报已最新；`--force` 下载校验解包替换全链路，换上件 sha256 与官方资产一致、可运行。[实证: 2026-09-03 本机真网]
- 文案与登记收口（AGENTS 意图路由、INDEX、SKILL 漂移守卫过、CHANGELOG Unreleased、diary、M012）。[实证: 2026-09-03]
