# R008-封版发布流程-全平台门禁验收与tag触发

> 角色：**做事的流程**：从「Unreleased 有货」到「Release 资产验收」的封版发布操作手册，下次照着做。2026-09-03 用户裁定流程骨架：先本地全平台编译、全平台测试验收，后封版触发 GitHub Action 发布 release（PRD D41）；2026-09-03 D42 加镜像腿。
> 自动化事实：`\.github\workflows\release.yml` 由 `v*` tag 推送触发，五 job（windows msvc、linux gnu、linux musl、macOS 双架构）各带「tag 与 Cargo.toml 版本一致」闸，`--locked` 构建后打包 `reader` / `rr` 双名加 README、LICENSE、SKILL.md 与 `.sha256` 上传 Release；`mirror` job 随后把资产上 R2 桶 `reader/<version>/`（immutable 头）并最后传 `reader/latest.json`（max-age=60，清单即发布提交点），支持 `workflow_dispatch` 对历史 tag 演练 [实证: release.yml]。模型镜像走 `\.github\workflows\mirror-models.yml`（每周一 03:17 UTC 加手动 dispatch：HF 四仓校验后上桶 `models/` 并传 GitHub `models-v6` 兜底 release，恒 prerelease 防 `/releases/latest` 遮蔽）[实证: mirror-models.yml]。

## 一、前置裁定

- **封版的定义：发布一个新版本**。版本号必前进（不存在不改版本号的封版）；封版件、tag 与 Release 资产同属一次发布，全平台验收绿是唯一放行条件。
- Unreleased 有条目才封版；空则不发布。
- 版本号：能力新增或行为变化取 `0.x.0`，修复取 `0.x.y`；与 ROADMAP 阶段对照。
- 发布通道只走 stable（self update 同口径，不做自动更新）。

## 二、全平台门禁与验收

> 三路全绿才进封版。本机交叉编译不可行（cc-rs 缺交叉 gcc，zstd-sys 需 C 工具链），全平台编译靠实机加 CI [实证: diary 2026-09-03 全平台回归节]。

1. **Windows 主开发机**：
   - cargo 三件：`cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --locked`（含 smoke / regress / accept / snapshot 全 target；外部真样本缺失自动跳过）。
   - 文档四件：rumdl、md-heading-scan、md-ref-scan、md-char-scan（见 G001 三节与 G004）。
   - release 构建：`cargo build --release --locked`。
2. **lan-mac 实机**（ssh `lan-mac`，仓 `~/reader_rs`）：`git pull --ff-only` 后门禁三件（`rustup run stable cargo ...` 与 CI 同通道）加 release 构建；OCR 真样本口径见 R005。
3. **lan-linux 实机**（ssh `lan-linux`，仓 `~/reader_rs`）：同上，口径见 R004；默认工具链 1.97 曾对新依赖连爆 rustc ICE，一律 `rustup run stable` [实证: diary 2026-09-03 全平台实机回归节]。
4. **CI**：main 推送后三系统 run 绿（fmt / clippy / test --locked）。

纪律：验收命令不接 `| tail` 之类管道（吞非零退出码，M006）；实机命令用阶段标记串（`&& echo STAGE-OK`）逐段确认。

## 三、封版件

1. `Cargo.toml` `version` 改目标版本号（release.yml 有 tag 一致性闸，不一致 job 直接红）。
2. `CHANGELOG.md` `[Unreleased]` 节改 `[<版本>] - <日期>`，正文只留版本级里程碑（本文件头规则）。
3. SKILL 重生：`cargo build --quiet` 后 bash 里 `./target/debug/reader.exe skill > SKILL.md`（Windows 显式 `.exe`：target 可能残留无扩展名旧 Linux 产物，且 PowerShell `>` 转 CRLF，M014；SKILL 含版本号；`cargo test` 不重建 target/debug 二进制，须先 build [实证: diary 2026-09-03 SKILL 重构节]）。
4. insta 快照复审：`--llms` 快照含版本号，`cargo test --test snapshot` 出 `.snap.new`，逐个人工审后改名入库（insta 纪律，D34）。
5. 门禁复跑（本机 cargo 三件加文档四件）全绿后一次提交：`chore: 封版 v<版本>`。

## 四、tag 触发发布

```bash
git tag v<版本>
git push origin main v<版本>     # tag 推送即触发 release.yml
```

## 五、发布验收

- run 六 job 全绿（build 五目标加 mirror；musl 目标对重依赖是风险点，P0013 / v0.3.0 两轮实证）。
- 资产 10 件齐：五平台 ×（zip 或 tar.gz 加 `.sha256`）。
- 镜像腿（D42）：`mirror` job 绿；`curl -sI https://reader.ohmygh.com/reader/latest.json` 出 `cache-control: public, max-age=60`，`.../reader/<version>/<资产>` 出 `immutable`；`gh api repos/raystyle/reader_rs/releases/latest --jq .tag_name` 恒指 `v*`（models-v6 之类 prerelease 不得占位）。
- 抽查 `.sha256` 校验、解包 `reader --version` 冒烟、发行件 `reader self update` 报已最新（镜像通道）。
- 本机与 CI 资产 sha256 不一致属预期（构建机差异 [实证: v0.3.0 轮]），一致性以官方 `.sha256` 为准。
- **同 version 禁止重发**：R2 侧 `reader/<version>/` 带 immutable 头（缓存一年），资产有误须换版本号重发或先 Cloudflare purge 再重传，不得原地覆盖了事。
- 收尾义务：CHANGELOG 已封版、ROADMAP 阶段状态、diary 当天钩子（对齐义务表「发布」行）。

## 验收记录

- **2026-09-03，v0.4.0 首轮**：三平台全绿 [实证: 各路退出码与阶段标记]：Windows（fmt / clippy / test 8 target 全绿：27 单元、52 集成、smoke 3、regress 4、snapshot 3、accept；release 构建过）；lan-mac（门禁三件 53 集成、release 构建过；首次失败为 rsproxy 镜像拉新依赖网络坑，重试过，同 diary 上轮前科）；lan-linux（同 mac 全绿）；CI main 三系统绿（run 33715506021）。OCR 端到端（release 件加仓内 scan-cjk.pdf）：linux 0.24 秒每页、mac 0.12 秒，exit 0、`[needs_ocr]` 保留、中英正文正确；发现 D42（模型 HF 直连国内不可达，scp 手动放置后 `--offline` 全通，模型分发自维护入下版计划）。封版件：Cargo.toml 0.4.0、CHANGELOG 定版、SKILL 重生（仅版本行）、`--llms` 快照人工审后入库（仅版本行与断言行号）。
- **2026-09-03，v0.4.0 发行验收**：run 33717012416 五 job 绿；10 资产齐（约 20 至 23 MB 每包）；windows zip `.sha256` 校验过（CRLF 复现，`tr -d` 后验，M008 族）；解包冒烟 `reader 0.4.0`、双名加三文档齐；**self update 真实升级路径首验**：v0.3.0 发行件升 0.4.0 全链路（下载、digest 校验、双名原子替换）通，v0.3.0 留白项闭环 [实证: 临时目录实测 0.3.0 报 0.4.0]。
