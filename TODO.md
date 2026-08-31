# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

mac 与 Linux 接管开发与跨平台兼容（对应 `GOAL.md`，方案 P0004，登记日 2026-08-31）。本机侧已全部就位（提交 `985edf5`），**剩最后一项：推送后 CI 首跑三系统绿即收官**。

## 任务进度清单

| 任务项 | 进度 | 说明 | 日期 |
| --- | --- | --- | --- |
| 立项 P0004 | 已完成 | `docs\proven\P0004-mac与Linux接管开发与跨平台兼容.md` | 2026-08-31 |
| CI 三系统矩阵 | 已完成 | `.github\workflows\ci.yml`：windows/ubuntu/macos 跑 fmt/clippy/test --locked，带 rust-cache | 2026-08-31 |
| 换行符钉 LF | 已完成 | `.gitattributes` 上 `text=auto eol=lf` | 2026-08-31 |
| 文档双形态 | 已完成 | README 安装/命令 bash 形态；ROADMAP 阶段 4；R001 边界改「CI 接管验证」 | 2026-08-31 |
| 本地门禁与登记 | 已完成 | 门禁三件与 rumdl 三件套全绿；INDEX/GOAL/CHANGELOG/diary 登记 | 2026-08-31 |
| CI 首跑验收 | 未开始 | 推送 `985edf5` 后看 GitHub Actions 三系统结果；若红按日志修并记 mistakes | 待推送 |
