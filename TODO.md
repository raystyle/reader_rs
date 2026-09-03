# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

D42 镜像分发落地（登记日 2026-09-03；依据 `PRD.md` D42、ISSUE #1 裁决、`PLAN.md` 完成的定义）。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| 四原语立项 | 已完成 | PRD D42 流转已采纳；GOAL 起点/锚点/时间线；PLAN 当前目标改 D42；TODO 本清单 | 2026-09-03 |
| src\mirror.rs（pin 表与三级回退下载链） | 已完成 | 四包 pin 表（rev 漂移闸单测）、超时 agent、`.part` 校验落盘、只读 assess、latest.json 解析；单元 7 例 | 2026-09-03 |
| src\ocr.rs（三命令与在线分支改造） | 已完成 | 设置锚缓存兄弟位、档位三级、init/doctor/switch、在线分支先 Offline 探测再镜像链再原生兜底 | 2026-09-03 |
| src\lib.rs 接线与输出契约 | 已完成 | ocr 子命令组、`ocr_*:` 行式输出、doctor 0/1 与 init/switch 2 退出码 | 2026-09-03 |
| src\selfupdate.rs 镜像 latest.json 通道 | 已完成 | ReleaseInfo 双通道归一、坏清单形状拒用回退 GH；真机验空桶回退报 current 0.4.0 | 2026-09-03 |
| introspect 与 SKILL 同步 | 已完成 | curated 两文本与快照、SKILL 重生（M014：显式 `.exe`） | 2026-09-03 |
| tests\cli.rs 守卫与新用例 | 已完成 | 守卫二层递归；新集成 7 例；快照人工审入库 | 2026-09-03 |
| .tools 两脚本 | 已完成 | gen-latest-json.py（v0.4.0 真数据五平台实测）、mirror-models.py（dry-run 源校验零漂移）加 README 清单 | 2026-09-03 |
| 两 workflow | 已完成 | release.yml mirror job（dispatch 演练、401 重试、latest.json 最后传、`--latest` 防遮蔽）；mirror-models.yml（cron 加手动、models-v6 恒 prerelease） | 2026-09-03 |
| 全量文档对齐 | 已完成 | README、G006、R008、CHANGELOG、INDEX、AGENTS、M014 登记 | 2026-09-03 |
| 门禁与收尾 | 进行中 | 七件全绿（本机）；待：提交切分（用户逐批点头）、CI dispatch 演练与 ISSUE 知会（用户放行）、大陆侧验收（ohmycloud） | 2026-09-03 |
