# R005-mac接管开发验收清单-门禁真样本与交叉预建

> 角色：**mac 接管机的开发与验收操作手册**（P0011-P0013 轮接续 Linux 验收后的第三站，含 M007 修复面）。2026-09-01 移交，Linux 侧基线全绿 [实证: 2026-09-01 WSL2 实机 15 单元 + 44 集成、rumdl 41 文件零告警、musl 静态件成，R004 验收记录]；推送后 CI 三系统绿（run 33500773187，sigpipe 测试在 ubuntu 与 macos job 均过）。
> 六态标准见 G002；mac 复跑结果回填本文「验收记录」节。

## 一、前置

2026-09-01 移交时已实测（WSL 侧 ssh ray@192.168.88.3）：

- 机器：arm64，macOS 26.5.2；`~/reader_rs` 已 pull 到 154df2b。
- 工具：cargo 1.97.0、uv、rumdl 0.2.62（rumdl 架构坑 M003 已解，可跑）。
- 样本：`~/Pcap流量分析智能体功能参数和指标项.docx`（无标题参数表 → part 单元预期）已 SCP 就位。

mac 侧开工自查：

```bash
cd ~/reader_rs && git pull --ff-only && git log --oneline -1
cargo --version && uv --version && rumdl --version   # rumdl 先跑通再进门禁（M003）
```

## 二、门禁三件

> 与 CI 同口径。预期 **5 套 result ok，15 单元 + 44 集成**（44 含 M007 回归测 `sigpipe_on_closed_stdout_kills_quietly`——`cfg(unix)` 在 macOS 生效，是 M007 修复面在本机的第一验点）[实证: Linux 与 CI 双侧同数，2026-09-01]。M006：命令不接 `| tail` 之类管道。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

## 三、文档门禁三件

```bash
rumdl check .
uv run --script .tools/md-ref-scan.py
uv run --script .tools/md-heading-scan.py
```

预期：rumdl 41 文件零告警；断链 0；标题括号 0 [实证: Linux 侧 2026-09-01]。

## 四、真样本冒烟

> 五路；PDF 与批量目录样本 mac 侧自备（~/Documents 下任一即可），docx 与 legacy.doc 已就位。

```bash
cargo build --quiet
./target/debug/reader --version                        # reader 0.2.0（封版前）
# 1 PDF 页契约
./target/debug/reader extract <某PDF> | grep -c '^== page '
# 2 docx 分节：无标题参数表 → part；有标题文档 → section
./target/debug/reader extract ~/Pcap流量分析智能体功能参数和指标项.docx | grep '^== '   # 预期 == part 1 ==
# 3 无标题整篇与超长节分片（仓内资产）
./target/debug/reader extract tests/assets/legacy.doc  # 节头 == part 1 ==，中英文与 & 保真
# 4 批量目录 + M007 现场验：管道截断按 Unix 惯例 141 静默（无 panic 无 stderr）
./target/debug/reader search <某文档目录> <关键词> | head -3; echo "pipe=${PIPESTATUS[0]}"  # zsh 用 $pipestatus[1]
./target/debug/reader search <某文档目录> <关键词> --format json --filter 'hits[].file'
# 5 负例
./target/debug/reader search <目录> zzz-no-such; echo $?       # 1
./target/debug/reader search <目录> x --pages 1; echo $?       # 2
```

预期：与 Linux 侧行为一致（R004 验收记录为对账基准）；M007 验点为 141 加零 stderr 噪音。

## 五、x86_64 交叉预建

> 对齐 CI 的 Intel mac 资产口径（M004：ARM runner 交叉编译 x86_64-apple-darwin）；musl 目标不适用于 mac，Linux 侧已验。

```bash
rustup target add x86_64-apple-darwin
cargo build --release --locked --target x86_64-apple-darwin
file target/x86_64-apple-darwin/release/reader    # 预期 Mach-O 64-bit executable x86_64
./target/x86_64-apple-darwin/release/reader --version   # Rosetta 2 在则直接跑；不在则以 file 判形为准
```

## 六、验收记录与上报

- 全绿：结果回填本文「验收记录」节（一行带日期与退出码），mac 接管开发即就位；v0.2.1 tag 发版待用户确认，不阻塞本清单。
- 有红：现象与输出贴 `docs\mistakes\` 对应分类接编 MNNN，或回报 Linux/Windows 侧协同定位。

## 验收记录

- （待 mac 侧回填：日期、机器、各节结果）
