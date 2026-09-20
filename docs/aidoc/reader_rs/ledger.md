# reader_rs::ledger

ledger 胶水层：统一客户端 crate ledger-client（REQ-063 全舰队唯一实现）
的仓内接线（总台修正令 2026-09-20 收口：签名道加只增面全在 crate，本仓
不再自研副本；关闭与删除唯一道 = 开发工作台经 herdr 委托 omc 工位）。
本模块只做三件：私钥装载（env `READER_LEDGER_KEY` 的 32 字节 hex，或本地
密档 `READER_LEDGER_KEY_FILE` 路径，缺省 `~/.config/reader/ledger-key`，
不进仓不进 argv）、在册 kid 对账（换对须同步 [`KEY_ID`] 与总台登记）、
[`ledger_client::Ledger`] 装配。命令面与行式在 lib.rs。

## Functions

- `connect` — 装配 Ledger 客户端：装载私钥、对账在册 kid（不符即拒，防换对后写面静默 401）。
- `err_line` — 把 crate 错误转 CLI 人读行（按状态码归因）。

## Constants

- `KEY_ID` — 在册 kid（2026-09-20 总台 pubkeys 登记值；换密钥对须同步本常量与登记）。
- `REPO_ID` — 账本仓标识（规范化 remote；URL 路径原样嵌入，含斜杠）。

