# PLAN：当前目标实施计划

> 角色：**当前目标方案文档**：基于 `docs\research\`(为什么)与 `docs\references\`(怎么做)撰写的执行计划;每条挂依据来源,随目标变化更新,不存历史目标。
> 分工:`PRD.md` = 要什么;`GOAL.md` = 要达成什么;`TODO.md` = 做到哪;本文件 = 怎么做;通用工作流见 `docs\references\R007-工作流标准细则-从登记到归档五步.md`。

## 当前目标:D43 图片文件分析 加 D44 全格式冒烟覆盖(随行 D42 收尾)

> 2026-09-04 立项。依据:PRD D43 / D44;S009 研究(零新依赖实证);用户四裁(格式集八种、--ocr opt-in、首帧、冒烟主干全补)与官方语料裁定(anydoc fixtures 入仓)。D42 主体已收官(v0.5.0),收尾两件随本目标携带:M017 补丁(下载器自建父目录)随下版发布;P0019 归档等 ISSUE #1 大陆侧第三级回退演练回执。

## 完成的定义

> 本目标验收口径。

- D43 图片管线:`ocr.rs` 引擎构建抽 `build_engine` 共用(PDF 行为零变化),`ocr_image`(ImageReader 内容嗅探、首帧、EXIF 方向、alpha 白底、recognize);`document.rs` 八扩展名分派加 `is_supported` 真源,单图即 page 1 恒标 `[needs_ocr: image]`;query 拒图片指路 `--ocr`。[实证: 2026-09-04 本机 GDI+ 文字图端到端识别出全文]
- D43 契约:无 `--ocr` 仅提示行退出 0;`--ocr` 回填 lines 标记保留(同 PDF);`--pages` 只认 1;批量目录含图片进扫描面;目录 `--ocr` 仍拒;README / SKILL / introspect / `--help` 全同步(漂移守卫绿)。[实证: 2026-09-04 手动六契约加 cli 集成 7 例]
- D44 全格式冒烟:`tests\smoke.rs` 全格式活体(现造 pdf / md / csv / epub 加 `tests\assets\anydoc\` 官方语料九族:odt / ods / odp / pptx / ppt / xlsx / xls / xlsb / rtf),legacy .doc 资产;秒级完成;语料来源 commit 与 sha256 登记(目录 README)。[实证: 2026-09-04 smoke 5 项 0.3 秒全绿,ppt 二进制族真解析出 Deck Title Slide]
- 验收门:cargo 三件(fmt / clippy --all-targets / test 全量)加文档四件全绿;README About 一致性同步(仓外 GitHub About 描述随发布手更)。
