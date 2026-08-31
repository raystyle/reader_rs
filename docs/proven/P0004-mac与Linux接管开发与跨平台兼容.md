# P0004-mac与Linux接管开发与跨平台兼容

> 本方案文档照 `docs\guide\template.md` 写。进行中与否以 `TODO.md` 为准。

- 状态：已完成
- 日期：2026-08-31
- 关联：TODO.md 当前目标 / `docs\references\R003-测试标准细则-分层断言与门禁流程.md`（演进路线第三段 CI 化，本方案即立项）

## 背景与问题

开发主机一直是 Windows。用户要在 macOS 与 Linux 下接管开发与测试，需要仓库对三平台开箱可用：CI 有跨平台门禁、换行符不打架、文档命令给出各平台形态。

## 目标与非目标

- 目标：
  - GitHub Actions CI：windows / ubuntu / macos 三系统矩阵跑门禁三件（fmt、clippy -D warnings、test --locked）。
  - `.gitattributes` 钉 LF，消除跨平台换行漂移。
  - README 安装与命令示例给 bash 形态（pwsh 保留）。
  - 代码与测试的 Windows 假设审计一遍（路径、shell、文件名）。
- 非目标：
  - 不在本机模拟 mac/Linux 实测（无环境；以 CI 为验收面）。
  - 不上 rumdl 等文档门禁进 CI（文档门禁保持本地）。
  - 不引新依赖。

## 方案

1. `.github\workflows\ci.yml`：三系统矩阵，checkout + rust-toolchain stable + rust-cache + 门禁三件。
2. `.gitattributes`：`text=auto eol=lf`。
3. 审计结论：代码层无平台专属路径（提取库全纯 Rust）；测试用 `std::env::temp_dir()` 加 pid 命名，跨平台安全；唯一已知差异是命令行用法（M002 路径形态），属使用层非代码层。
4. README 安装/验证给 pwsh 与 bash 双形态。

## 备选方案

| 方案 | 取舍 |
| --- | --- |
| GitHub Actions 矩阵（选定） | 仓库已在 GitHub，零新增设施；R003 第三段既定路线 |
| 只写跨平台注意事项不上 CI | 无验收面，兼容承诺不可查 |
| 本机 WSL 实测 | 可作补充非验收；WSL 环境与真实 mac/Linux 有差 |

## 实施步骤

1. 本方案立项，三原语登记。
2. ci.yml 加 .gitattributes。
3. README 双形态；ROADMAP 阶段 4 立项化；R001 边界措辞更新。
4. 门禁回归；INDEX/CHANGELOG/diary 登记。

## 风险与回滚

- 风险：CI 首战可能红（Linux/mac 特有失败，如依赖系统库）。缓解：依赖全纯 Rust，预期无系统库需求；若红，按 CI 日志修并记 mistakes。回滚：删 workflow 文件即回。

## 实施过程与经验

- 实际怎么做：按步骤走完，无偏差。CI 首跑（run 33378905306）windows/ubuntu/macos 三 job 全 success，约 3.5 分钟。[实证: 2026-08-31 gh run view]
- 踩了什么坑 + 怎么解决：ROADMAP 追加「阶段 4」节时空行缺失被 rumdl MD022 拦下；补空行即过。append 写文件时注意节间空行。
- 沉淀的经验：
  - 纯 Rust 依赖树让三系统 CI 一次全绿，无系统库、无平台分支代码——选型阶段守住「纯 Rust 单二进制」边界的回报。
  - 验收面设在 CI 而非本机模拟，是「无 mac/Linux 环境」下的正确分工；CI 注解（Node.js 20 deprecation 警告）不影响结论，后续 actions 大版本升级时顺带处理。

## 验收标准

- CI workflow 推送后三系统绿（以 GitHub Actions 运行结果为准）。[实证: 2026-08-31 run 33378905306 三 job success]
- mac 与 Linux 实机接管验收通过（lan-mac、lan-linux；原「以 CI 为验收面」的边界由实机验收补齐）。[实证: 2026-08-31 用户验收确认]
- 本地门禁三件与 rumdl 三件套全过；`git add` 后无换行警告残留（.gitattributes 生效）。[实证: 2026-08-31]
- INDEX 与三原语登记完整。[实证: 2026-08-31]
