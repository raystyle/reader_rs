# M105-Rust依赖与库行为错误

> crate 选型与接入期的库行为坑。一行一事，行级编号全局递增。

## M009 ureq 默认响应体上限 10MB 撞大文件下载

- 首踩：2026-09-03（P0014 模型下载，rec 模型 16.6MB）
- 现象：`the response body is larger than request limit: 10485760`，下载中途报错。
- 根因：ureq 3 的 `read_to_vec` 默认响应上限 10MB（防内存炸弹设计），文档不显眼。
- 正确处理：按文件量级显式放宽——`resp.body_mut().with_config().limit(64 * 1024 * 1024).read_to_vec()`；下载类调用先看默认限额。

## M010 vendored 库 println 污染 stdout

- 首踩：2026-09-03（P0014 真样本回归；S006 PoC 未暴露——PoC 只看结果不看管道纯净）
- 现象：`--ocr` 输出里混进 `[DetInfer]`、`[RecInfer]` 引擎日志行，破 stdout 纯输出契约。
- 根因：pure-onnx-ocr 0.1.0 库代码（非 bin）用 `println!` 打进度日志。
- 正确处理：vendor 补丁把库代码（检测、识别、加载三处）的 `println!` 改 `eprintln!`；接库进 CLI 前先跑一遍真管道看 stdout 是否被污染。经验教训：PoC 验收口径要含「stdout 纯净度」，不能只看功能跑通。

## M011 OcrEngine 非 Send/Sync 不能进 static 复用

- 首踩：2026-09-03（P0014 引擎复用尝试）
- 现象：`OnceLock` / `Mutex` 静态持有 OcrEngine 均编译失败（内含 `RefCell<HashMap<…Arc<SimplePlan…>>>` 计划缓存）。
- 根因：tract 的 SimplePlan 不满足 Sync 约束，pure-onnx-ocr 引擎随之为 !Send/!Sync。
- 正确处理：不进静态，每次调用现建——引擎构建实测约 29ms，相对 19 到 42 秒/页的推理可忽略；先量再优化，不为省 29ms 上 unsafe。
