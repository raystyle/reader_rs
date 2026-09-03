# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

D42 镜像分发落地（登记日 2026-09-03；依据 `PRD.md` D42、ISSUE #1 裁决、`PLAN.md` 完成的定义）。**v0.5.0 已发布，本仓侧全链收官**；余一项等 ohmycloud 大陆侧验收回执后归档 P0019。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| 四原语立项 | 已完成 | PRD D42 流转已采纳；GOAL 起点/锚点/时间线；PLAN 当前目标改 D42 | 2026-09-03 |
| src\mirror.rs（pin 表与三级回退下载链） | 已完成 | 四包 pin 表（rev 漂移闸单测）、超时 agent、`.part` 校验落盘、只读 assess、latest.json 解析 | 2026-09-03 |
| src\ocr.rs（三命令与在线分支改造） | 已完成 | 设置锚缓存兄弟位、档位三级、init/doctor/switch、在线分支先 Offline 探测 | 2026-09-03 |
| src\lib.rs 接线与输出契约 | 已完成 | ocr 子命令组、`ocr_*:` 行式输出、doctor 0/1 与 init/switch 2 | 2026-09-03 |
| src\selfupdate.rs 镜像 latest.json 通道 | 已完成 | ReleaseInfo 双通道归一、坏清单形状拒用回退 GH | 2026-09-03 |
| introspect 与 SKILL 同步 | 已完成 | curated 两文本、漂移守卫二层递归、快照人工审 | 2026-09-03 |
| tests\cli.rs 守卫与新用例 | 已完成 | 守卫二层递归；新集成 7 例 | 2026-09-03 |
| .tools 两脚本 | 已完成 | gen-latest-json.py、mirror-models.py（含 M015 修复与清单路径修订） | 2026-09-03 |
| 两 workflow | 已完成 | release.yml mirror job 与 mirror-models.yml（models-v6 恒 prerelease） | 2026-09-03 |
| 全量文档对齐 | 已完成 | README、G006、R008、CHANGELOG、INDEX、AGENTS、M014-M016 | 2026-09-03 |
| 全平台验收 | 已完成 | Windows 七件、lan-mac / lan-linux 三件双绿（M016 修后）、CI 三系统绿 | 2026-09-03 |
| 封版与发布 v0.5.0 | 已完成 | 封版 7ae300b；tag 触发六 job 绿、10 资产；镜像面全验；self update 镜像真升级 0.4.0 至 0.5.0 首验 | 2026-09-03 |
| ISSUE 知会与大陆侧验收 | 进行中 | ISSUE 已知会；等 ohmycloud 大陆侧回执（首用 --ocr 走镜像、ocr 组、self update、三断回退）后归档 P0019 | 2026-09-03 |
