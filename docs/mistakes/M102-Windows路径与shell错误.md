# M102-Windows路径与shell错误

> 错误速查分类文件：Git Bash / MSYS 与原生 Windows 程序之间的路径形态、shell 引号相关踩坑。行级编号 M0xx 全局递增不复用；迭代规则见 `INDEX.md` 八、错误速查。

## M002 MSYS 路径喂给原生 Windows 二进制

- 日期：2026-08-31
- 现象：在 Git Bash 里把 `/d/...` 形态路径作为参数传给 `reader`（原生 Windows 程序），报 `IO error: 系统找不到指定的路径。 (os error 3)`。
- 根因：MSYS 路径只在 MSYS 工具链内有效；原生 Windows 二进制不识别 `/d/` 挂载形态。同类的坑还有命令里单引号包 Windows 路径被外层包装拆断（语法错误 near `)`）。
- 正确处理：给原生二进制的路径参数一律用 Windows 形态（`D:\...`），在 bash 里用双引号包裹；MSYS 形态只留给 MSYS 工具自己。

## M005 测试代码里反斜杠拼仓内路径，CI linux 与 macOS 红

- 日期：2026-09-01
- 现象：v0.2.0 封版 CI 三系统中 ubuntu 与 macos 的 `cargo test --locked` 红（`legacy_doc_asset_extracts` 找不到 `tests\assets\legacy.doc`），Windows 本机与 CI 绿。
- 根因：`Path::join("tests\\assets\\legacy.doc")` 用了 Windows 反斜杠；Linux/macOS 上反斜杠是文件名的一部分，路径不存在。在 Windows 上开发、测试全绿，跨平台问题只有 CI 能暴露。
- 正确处理：仓内相对路径拼接一律 `join("tests/assets/legacy.doc")` 正斜杠分段（Windows 同样接受）；新增测试路径断言前想一步「这条在 CI 三系统上等价吗」。

## M008 Git Bash 里用 cmd 风格 2>nul，仓里落出保留名文件

- 日期：2026-09-01
- 现象：Git Bash 冒烟命令写 `... 2>nul`，以为同 cmd 丢 stderr；实际 MSYS 把 `nul` 当普通文件名，仓根生成名为 `nul` 的真实文件（Windows 保留设备名），`git add -A` 报 `invalid path 'nul'` 阻断提交。
- 根因：cmd 的 `nul` 设备语义不跨 shell；bash 里写 `nul` 就是文件。
- 正确处理：bash 侧一律 `2>/dev/null`；已落出的保留名文件用 MSYS 的 `rm ./nul` 删（绕过 Win32 保留名限制），或 PowerShell `Remove-Item '\\?\D:\...\nul'`。

## M007 Rust 默认忽略 SIGPIPE，Linux 管道早退 panic exit 101

- 日期：2026-09-01（Linux 实机验收 R004 第四节发现）
- 现象：`reader search <目录> <关键词> | head` 在 Linux 上 panic：`failed printing to stdout: Broken pipe (os error 32)`，退出码 101 且 stderr 喷栈信息；Windows 侧同命令无恙（验收基线全绿）。
- 根因：Rust 运行时把 SIGPIPE 置为 `SIG_IGN`，管道读者早退后写 stdout 返回 `EPIPE`，`println!` 对错误直接 panic。Windows 无 SIGPIPE 机制，此路径在 Windows 开发机上天然不可复现：又一类「只有对端平台能暴露」的坑（同 M005 形态）。
- 正确处理：`main()` 最早点恢复 SIGPIPE 默认处置（`libc::signal(SIGPIPE, SIG_DFL)`，仅 unix；libc 升直接依赖），行为对齐 grep/rg：被 SIGPIPE 静默终止，shell 报 141。回归测试 `sigpipe_on_closed_stdout_kills_quietly`（unix-only，大输出夹具超管道缓冲保证死因确定）。验收口径相应修正：管道截断退出码预期 141 而非 0。

## M014 重生成 SKILL.md 命中 target 里无扩展名旧产物，旧文案覆盖新文件

- 首踩：2026-09-03（D42 镜像分发落地，introspect 更新后重生 SKILL.md）
- 现象：Git Bash 里按 R008 封版件命令 `./target/debug/reader skill > SKILL.md` 重生，SKILL.md 变成 v0.2.0 时代旧文案（带破折号、无 mq、无 ocr 组）；`git diff` 全文变红。
- 根因：`target\debug\` 里躺着 9 月 1 日 WSL 构建留下的无扩展名 Linux 旧二进制 `reader`，bash 相对路径 `./target/debug/reader` 恰好命中它（同名优先于 `reader.exe`），跑的是旧版。另有两处连带隐患：PowerShell `>` 重定向按行重组会转 CRLF（与仓内 LF 不一致）；同型坑 M005（bash 的 `nul`）。
- 正确处理：重生命令一律显式写 `.exe`（bash：`./target/debug/reader.exe skill > SKILL.md`）；用 bash 原样重定向保 LF，不用 PowerShell `>`；重生后先 `git diff SKILL.md` 核对变更量与预期一致再收。R008 封版件第三步已加注。
