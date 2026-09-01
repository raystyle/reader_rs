# R004-Linux实机验收清单-门禁真样本与musl预建

> 角色：**Linux 接管机的开发验收操作手册**（P0004 跨平台接管的例行验收轮，本轮覆盖 P0011 / P0012 / P0013）。2026-09-01 移交，Windows 侧基线全绿 [实证: 2026-09-01 本机 15 单元加 43 集成、rumdl 41 文件零告警、CI 三系统绿 run 33497983997]。
> 六态标准见 G002；基线断言均带 Windows 侧出处，Linux 复跑结果回填本文「验收记录」节。

## 一、前置

```bash
git clone https://github.com/raystyle/reader_rs && cd reader_rs   # 或既有仓 git pull
cargo --version    # stable 工具链即可（CI 用 dtolnay/rust-toolchain@stable）
uv --version       # 文档门禁三件需要；无则 pip 装 uv 或跳过第三节
rumdl --version    # 无则跳过第三节（CI 不跑 rumdl，非阻断）
```

## 二、门禁三件

> 与 CI 同口径。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

预期：fmt 零 diff；clippy 零告警；test **5 套 result ok，其中 15 单元加 43 集成** [实证: Windows 与 CI ubuntu 双侧同数，2026-09-01]。注意本仓 M006：验收命令不要接 `| tail` 之类管道，非零码会被吞。

## 三、文档门禁三件

> 本地有 uv 与 rumdl 时执行；缺件可跳过（CI 不跑 rumdl，非阻断）。

```bash
rumdl check .
uv run --script .tools/md-ref-scan.py
uv run --script .tools/md-heading-scan.py
```

预期：rumdl 41 文件零告警；断链 0；标题括号 0 [实证: Windows 侧 2026-09-01]。

## 四、真样本冒烟

> 五路：PDF 页契约、docx 分节、无标题与超长节分片、批量目录、负例。

自带样本（仓内与现造）：

```bash
cargo build --quiet
./target/debug/reader --version                        # reader 0.2.0（封版前）
# 1 PDF 页契约
./target/debug/reader extract <某PDF> | grep -c '^== page '
# 2 docx 分节
./target/debug/reader extract <某docx> | grep -c '^== section '
# 3 无标题整篇与超长节分片
./target/debug/reader extract tests/assets/legacy.doc   # 节头 == part 1 ==，中英文与 & 保真
# 4 批量目录
./target/debug/reader search <某文档目录> <关键词> | head     # 命中行 路径:单元:行号:文本，exit 0
./target/debug/reader search <某文档目录> <关键词> --format json --filter 'hits[].file'
# 5 负例
./target/debug/reader search <目录> zzz-no-such; echo $?       # 1
./target/debug/reader search <目录> x --pages 1; echo $?       # 2（--pages 不适用于目录）
```

预期：与 Windows 侧行为一致；legacy.doc 仓内资产无需现造 [实证: Windows 侧 2026-09-01 同输出]。

## 五、musl 本地预建

> P0013 首跑去险，可选但建议。

```bash
sudo apt-get install -y musl-tools
rustup target add x86_64-unknown-linux-musl
cargo build --release --locked --target x86_64-unknown-linux-musl
ldd target/x86_64-unknown-linux-musl/release/reader   # 预期 not a dynamic executable（静态）
./target/x86_64-unknown-linux-musl/release/reader --version
```

关注点：zstd-sys（anydoc 传递依赖）在 musl-gcc 下的 C 编译；失败则记 M 并回报，tag 缓发。[假设: musl-gcc 可编 zstd-sys，CI 首跑验证]

## 六、验收记录与上报

- 全绿：结果回填本文「验收记录」节（一行带日期与退出码），P0011 / P0012 的跨平台对账即闭环；v0.2.1 tag 可发（P0013 验收转为 release 首跑五 job 绿）。
- 有红：现象与输出贴 `docs\mistakes\` 对应分类接编 MNNN，或直接回报 Windows 侧协同定位。

## 验收记录

- （待 Linux 侧回填：日期、机器、各节结果）
