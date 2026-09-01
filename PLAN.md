# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**——基于 `docs\research\`（为什么）与 `docs\references\`（怎么做）撰写的执行计划；每条挂依据来源，随目标变化更新，不存历史目标。
> 分工：`TODO.md` = 做到哪；本文件 = 怎么做；通用工作流见 `docs\guide\G003-工作流标准细则-从登记到归档五步.md`。

## 当前目标：封版 v0.1.0——tag 触发的三端二进制 release 流水线

> 对应 `GOAL.md`，方案 `docs\proven\P0008-封版v0.1与三端二进制release.md`，登记日 2026-08-31。

### 1. 闸门

P0004 的 CI 三系统矩阵已验证三平台构建链全通；release 流水线是同构延伸（构建加打包上传），无新选型。gh CLI 上传免第三方 action（P0008 备选方案表）。

### 2. 实施件与依据

| 件 | 要做什么 | 依据 |
| --- | --- | --- |
| `.github\workflows\release.yml`（新） | tag v* 触发；四目标矩阵 build --release --locked --target；版本一致性闸；打包 zip/tar.gz 加 sha256sums；gh CLI create 幂等加 upload --clobber | P0008 方案 1-4 条 |
| `CHANGELOG.md` | Unreleased 封版为 `[0.1.0] - 2026-08-31` | G003 交付变更规则 |
| `README.md` | 安装节补「预编译二进制」小节（Releases 链接与资产命名规则） | 用户要求：README 专注安装部署 |
| git | 提交推送 main；`git tag v0.1.0` 推 tag 触发发布 | P0008 实施步骤 4 |

### 3. 每件验收

本地门禁三件加 rumdl 三件套全绿；tag 推送后 release run 四 job 全绿、Release 页资产齐；本机下载 windows 资产实测 `--version` 与 search。失败当场记 `docs\mistakes\`。验收通用口径见 G003 第四节。

### 4. 边界

不改代码行为；不做 musl/arm-linux/包管理器分发；不做签名校验和之外的供应链加固。[依据: P0008 非目标节]

## 完成的定义

> 本目标验收口径。

- TODO 表全部已完成或明确跳过（跳过须写 mistakes 原因）
- `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked` 三件过
- `rumdl check .` 尽量零告警；P0008 与 INDEX 已登记；CHANGELOG 封版
- 研究与测试文档的关键结论标六态（AGENTS 写研究与测试文档规则）
