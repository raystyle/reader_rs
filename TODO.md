# TODO：当前目标任务进度清单

> 角色：**当前目标的任务进度清单**。当前目标完成后，过程与经验回填到 `docs\proven\` 对应方案，并起新清单；当天做的事只记 `docs\diary\`。

## 当前目标

D43 图片文件分析 加 D44 全格式冒烟覆盖（登记日 2026-09-04；依据 `PRD.md` D43 / D44、`S009`、`PLAN.md` 完成的定义）。随行 D42 收尾：M017 补丁随下版发布、P0019 归档等大陆侧第三级回退演练回执。D42 主体已发布（v0.5.0，2026-09-03）。

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
| 验收回执 bug 修复（M017） | 已完成 | `ocr init` 对不存在缓存目录三通道 os error 3；建目录归 `download_file` 单一权责，`prefetch_pair` 冗余建目录删；回归入 tests\mirror.rs（红态实证复现）；随下个补丁版发布 | 2026-09-04 |
| D43 图片管线 | 已完成 | `ocr_image`（内容嗅探、首帧、EXIF 方向、alpha 白底）加 `build_engine` 抽出共用；document 八扩展名分派与 is_supported 真源；query 拒图片指路 --ocr；本机 GDI+ 文字图端到端全识 | 2026-09-04 |
| D43 文档面 | 已完成 | README（速览、格式表加图片行、OCR 段）、SKILL 重生、introspect、lib.rs help 全同步；漂移守卫绿 | 2026-09-04 |
| D43 集成测试 | 已完成 | cli 图片 7 例：单页契约、pages 过滤、search 退出码与 needs_ocr_units、query 指路、avif 拒、批量目录、门控 OCR 端到端 | 2026-09-04 |
| D44 全格式冒烟 | 已完成 | anydoc 官方 fixtures 九族入仓 tests\assets\anydoc\（firecrawl/anydoc@261fc25，MIT，README 钉 sha256）；smoke 全格式活体 5 项 0.3 秒全绿；ppt 二进制族与 xls / xlsb 零覆盖缺口消灭 | 2026-09-04 |
| S009 与四原语对齐 | 已完成 | S009 落盘；PRD D43 / D44 已采纳（四裁加官方语料裁定）；AGENTS 边界、G006 冒烟落点、INDEX（S009、代码行、语料行）、GOAL / PLAN / TODO | 2026-09-04 |
| 门禁收口与提交 | 进行中 | cargo 三件加文档四件全绿后按「一次提交只做一件事」分组提交（M017 修复、D43、D44、文档对齐）；diary 钩子随提交 | 2026-09-04 |
| D45 版本分支模型 | 已完成 | 用户裁定 dev/v 命名、FF 合并、dev/main 隔离、合并 main 打 tag 发版；ci.yml 加 dev/** 触发；AGENTS 规则 3、R008、PRD D45 同步；首条分支 dev/v0.6.0 承载分组提交 | 2026-09-04 |
| 镜像分发幂等闸 | 已完成 | 用户裁定(2026-09-04):mirror-models 清单核心比对(排除 mirrored_at)与远端一致即零 HF 下载零 R2 上传零元数据变更;workflow 三步 skip 守卫;本机实跑 NO-CHANGE;客户端幂等回归 ocr init 完整缓存零下载(死镜像证) | 2026-09-04 |
| 测试面扩充:官方语料全量 | 已完成 | 用户点名「测试文档与用例太少」;anydoc corpus 71 件入仓(镜像上游布局,含 malformed 负例与 abuse 滥用件);tests\corpus.rs 63 快照加负例加滥用断言(64 测 0.37s);smoke 路径随布局更新 | 2026-09-04 |
| 测试面扩充:E:\ebook 主性能质量面 | 已完成(后被第 2 轮裁定替换) | 用户点名;`.tools\ebook-corpus.py`(scan/baseline/perf/verify 四模式)加 tests\ebook.rs gated 基线核验;manifest 钉 sha256 外部样本不入仓;PRD D46 | 2026-09-04 |
| D46 第 2 轮:弃 ebook 改研究资料 | 已完成 | 用户裁定;机制平移改名 materials(`.tools\materials-corpus.py` 加 tests\materials.rs),默认根 E:\研究资料(575 件 5.4GB);ebook manifest 与基线删除 | 2026-09-04 |
| D47 图片本体提取与元数据对齐 | 已完成 | S010 定界(只提取存储,模型理解已拒);figures 子命令四路(PDF 页渲染 PNG+图题对齐、md 引用、anydoc zip 内嵌件、图片文件);cli 4 例+单测 2 例;SKILL/README/AGENTS/introspect 同步;真机四路活体+研究资料页实测 | 2026-09-04 |
| D47 第 3/4 轮:export 一键提取加内嵌图直抽 | 已完成 | 用户点名一键(文本+图片+元数据一目录或指定页,PDF 与 EPUB 都要)加导出目录二次搜索;figures v1.1 PDF 内嵌位图 XObject 直抽(DCT jpg 原字节/Flate 解码 png,XObject 内联与引用两形态);export 出 text.md/text.json/pages/ 逐单元/images//manifest.json;真机:论文 28 件内嵌图、EPUB 347 节 26 图、二次搜索页锚命中;cli 新 2 例 | 2026-09-04 |
| 封版 v0.6.0 与发布 | 进行中 | 封版件齐(版本/CHANGELOG/SKILL/快照审);待 CI 绿后 FF 合并 main、tag v0.6.0 触发 release.yml(六 job 含 mirror);发布验收(资产/latest.json/镜像头)后收尾 ISSUE #1 回执 | 2026-09-04 |
| dev/v0.6.0 全平台验收 | 已完成 | Windows(fmt/clippy -D/test --locked 含 materials 576 件 600s 核验/release)加 lan-mac(arm64 全 target 加 release 1m12s)加 lan-linux(x86_64 全 target 加 release 2m23s,rustup stable)加 CI 三系统(run 33840136600)四路全绿;两实机均 checkout dev/v0.6.0@ad09c3e,corpus 63 快照跨三平台逐字节一致 | 2026-09-04 |
| D47-T2 图表几何还原 PoC | 已完成 | poc\s010-chart-geometry:年份轴融合串切分与文本盒分离成立,数值配列两难点(变长小数粘串、双系列列偏移)未达精度;裁定暂不进主仓 | 2026-09-04 |
