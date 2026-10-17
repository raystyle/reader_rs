# 2026-10-17 review 轮 F1 修 README 账本基址漂移

> 来源：总台令 review v0.11.0（刚 clone HEAD a6c2fe6 独立复核，lan-ubuntu 承载），回执判词 CONFIRM 加 F1 一件；lan-win:w19:p1 令落小修（删 README env 表残行、写明不再支持 env 覆写、CHANGELOG 记移除）后推 origin。

## 概貌

review 复核面：全门禁绿（fmt、clippy -D warnings、`cargo test --locked` 12 target 207 测加 doctest 4、aidoc check clean 18、rumdl 89 件、md 三扫 103 件）加读面实弹（curl 无钥直取服务端全量、keyset `before=3` 得 [2,1] 严格排他、hasResult 驼峰与 artifacts snake_case 投影逐字段同形、客户端先拒面 `--outcome` bogus exit 2）[实证: 2026-10-17 本机输出]。F1 详情：`READER_LEDGER` 基址覆写在 v0.11.0 已死（src 全链零引用，crate `BASE_URL` 为常量而 `with_base` 仅 cfg(test)），079e254 清了 README 行文句与 `--llms` env 行独漏 env 表行，且 v0.10.0 在册 env 契约的移除未进 CHANGELOG。修法：删 README env 表残行；ledger 段（issue 与 artifact：仓级公共账本）末尾写明基址固定与覆写口移除出处；CHANGELOG Unreleased 补记移除（0.11.0 段已定版不回改，补记锚明移除发生在 v0.11.0 REQ-061 收口）。

## 验收

文档批门禁：rumdl 89 件、md 三扫 103 件全过 [实证: 2026-10-17 本机输出]；推 origin main（docs 批随发行后回填先例走 main 直提，非版本工作不占 dev/v 分支）。

## 记档 G1 至 G5 候选

> 总台裁定记档，不改面。

- G1 读面私钥硬门槛：issue list/show 与 artifact list 走 `connect()` 强要私钥加 kid 对账，服务端 GET 实际免签（curl 无钥全量取到 [实证: 2026-10-17]），crate 有 `read_only` 构造未用；无钥机器读面 exit 2（本机实测）。修向：读面改 `read_only()`，写面保持钥门。
- G2 `--checks` 文案过称：报「须为 JSON 对象」实际仅验可解析，`--checks '[1,2]'` 放行至钥门（本机实测）。修向：强判 `Value::Object` 或改文案为合法 JSON。
- G3 过滤跨页循环零测试（src/lib.rs:1090-1117）：单测只盖纯函数 `issue_row_matches`/`list_has_more`，集成零网络只盖 CLI 面，活账本仅 4 条永不触发跨页路径。修向：页源可注入（闭包或 trait）后补单测，含游标推进、MAX_PAGES 触顶告警、before 加过滤组配。
- G4 回执缺字段静默兜底：crate `issue_new` `unwrap_or(0)` 可打 `issue: opened #0`，`artifact_publish_full` 空 artifact_id、attest seq 0 同族；契约漂移时 CLI 假成功。修向：CLI 侧守（id 0 或空串即报错），根修在 crate。
- G5 行式截断字节切片（src/lib.rs:1356、1360）：`digest[..19]` 与 `artifact_id[..8]` 非 ASCII 边界可 panic，仅恶意或损坏服务端可达。修向：`.get(..n)` 加回退。

账本开单通道被 G1 所阻（本机无钥），候选记档在本档；后续有钥侧可按需转立账本单。

## 得失

- 得：v0.10.0 在册 env 契约的移除补记在案，README 与实现零漂移；review 轮全量证据随档
- 失：无（纯文档批，二进制零变化）
