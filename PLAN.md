# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**：基于 `docs\research\`(为什么)与 `docs\references\`(怎么做)撰写的执行计划;每条挂依据来源,随目标变化更新,不存历史目标。
> 分工:`PRD.md` = 要什么;`GOAL.md` = 要达成什么;`TODO.md` = 做到哪;本文件 = 怎么做;通用工作流见 `docs\references\R007-工作流标准细则-从登记到归档五步.md`。

## 当前目标:D42 镜像分发落地

> 2026-09-03 立项。依据:PRD D42;ISSUE #1 裁决(ohmycloud S009 规范摘录)与基建回执;架构验证修正要点九项(见 GOAL 时间线立项行)。

## 完成的定义

> 本目标验收口径。

- D42 客户端:模型三级回退源链(镜像 `reader.ohmygh.com` 到 HF 直连到 GitHub Releases `models-v6`,预取入 ppocr-rs 缓存目录,`resolve_pair(Offline)`/`verify()` 终检)进 `src\mirror.rs` 与 `src\ocr.rs`;self update 先读镜像 latest.json 失败回退 GitHub API(`src\selfupdate.rs`);`ocr init / doctor / switch` 三子命令(下载、只读诊断、档位切换,设置文件锚缓存兄弟位,env 优先)。[实证: 进行中]
- D42 CI:release.yml 镜像腿(资产与边车上桶 immutable、latest.json 最后传 max-age=60、dispatch 可对历史 tag 演练、末尾 `gh release edit --latest`);mirror-models.yml(models 目录从 ppocr-rs models.json 派生、HF tree 源校验告警、Apache-2.0 与 NOTICE 随分发、manifest 最后传、`models-v6` 恒 prerelease)。[实证: 进行中]
- D42 验收:本机门禁三件加文档四件全绿;dispatch 演练全链绿(curl 验缓存头与清单、`releases/latest` 恒指 v*);ISSUE #1 知会后 ohmycloud 大陆侧验收(首用 `--ocr` 走镜像全通、self update 镜像通道、三断回退)。[实证: 进行中]
