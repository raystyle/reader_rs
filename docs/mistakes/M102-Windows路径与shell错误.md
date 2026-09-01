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

