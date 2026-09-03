# M105-Rust依赖与库行为错误

> crate 选型与接入期的库行为坑。一行一事，行级编号全局递增。

## M009 ureq 默认响应体上限 10MB 撞大文件下载

- 首踩：2026-09-03（P0014 模型下载，rec 模型 16.6MB）
- 现象：`the response body is larger than request limit: 10485760`，下载中途报错。
- 根因：ureq 3 的 `read_to_vec` 默认响应上限 10MB（防内存炸弹设计），文档不显眼。
- 正确处理：按文件量级显式放宽：`resp.body_mut().with_config().limit(64 * 1024 * 1024).read_to_vec()`；下载类调用先看默认限额。

## M010 vendored 库 println 污染 stdout

- 首踩：2026-09-03（P0014 真样本回归；S006 PoC 未暴露：PoC 只看结果不看管道纯净）
- 现象：`--ocr` 输出里混进 `[DetInfer]`、`[RecInfer]` 引擎日志行，破 stdout 纯输出契约。
- 根因：pure-onnx-ocr 0.1.0 库代码（非 bin）用 `println!` 打进度日志。
- 正确处理：vendor 补丁把库代码（检测、识别、加载三处）的 `println!` 改 `eprintln!`；接库进 CLI 前先跑一遍真管道看 stdout 是否被污染。经验教训：PoC 验收口径要含「stdout 纯净度」，不能只看功能跑通。

## M011 OcrEngine 非 Send/Sync 不能进 static 复用

- 首踩：2026-09-03（P0014 引擎复用尝试）
- 现象：`OnceLock` / `Mutex` 静态持有 OcrEngine 均编译失败（内含 `RefCell<HashMap<…Arc<SimplePlan…>>>` 计划缓存）。
- 根因：tract 的 SimplePlan 不满足 Sync 约束，pure-onnx-ocr 引擎随之为 !Send/!Sync。
- 正确处理：不进静态，每次调用现建，引擎构建实测约 29ms，相对 19 到 42 秒/页的推理可忽略；先量再优化，不为省 29ms 上 unsafe。

## M012 flate2 0.2 无纯 Rust 后端 feature，miniz-sys C 库混进依赖树

- 首踩：2026-09-03（P0015 self update 解 tar.gz）
- 现象：`flate2 = "0.2"` 默认拉 miniz-sys（C 编译）；写 `default-features = false, features = ["rust_backend"]` 报 feature 不存在。
- 根因：flate2 0.2.x 的后端 feature 只有 miniz-sys / libz-sys / zlib（全 C）；纯 Rust 的 rust_backend（miniz_oxide）从 1.x 起才是默认。
- 正确处理：解 gzip 用 `flate2 = "1"` 默认 feature；写完依赖先 `grep miniz-sys Cargo.lock` 之类确认没有 C 后端混进来（纯 Rust 边界仓的例行检查）。

## M013 Cargo.toml 里 [[test]] 表插进 dev-dependencies 中间吞后续键

- 首踩：2026-09-03（D33 验收层 BDD 化）
- 现象：在 `[dev-dependencies]` 节中间插入 `[[test]]` 块后，其后的 `rbook` / `zip` 依赖变成 test 表的键，cargo 警 unused manifest key 且依赖丢失。
- 根因：TOML 节序即语义：表头之后的键值对都归最近的表头，直到下一个表头。
- 正确处理：新增 `[[bin]]` / `[[test]]` 等表一律放文件末尾；改 Cargo.toml 后留意 unused manifest key 警告。

## M016 路径断言用反斜杠字面量在 unix 假失败

- 首踩：2026-09-03（D42 settings_path 兄弟位单测：Windows 绿、lan-mac / lan-linux 双红）
- 现象：`Path::new(r"C:\a\b\models").with_file_name("x")` 在 unix 得 `x` 而非 `C:\a\b\x`，断言左边只剩文件名。
- 根因：反斜杠在 unix 不是路径分隔符，整串被当成单个文件名；`with_file_name` 语义是替换最后一段组件。Windows 上开发时绿掩盖了平台差异（同 M005 形态：只有对端平台能暴露）。
- 正确处理：跨平台断言一律用正斜杠字面量（两平台都认 `/` 为分隔符）；确要测反斜杠形态用 `#[cfg(windows)]` 圈住。
