# reader_rs::output

输出层：JSON 包膜（ok/data/error 加 meta）与 filter 点路径裁剪。设计依据 S002（P0006）。

## Functions

- `err_json` — 失败包膜串行化（compact 单行）。
- `filter_value` — 点路径裁剪：键访问（`a.b`）、数组映射（`hits[].text`）、下标（`units[0].lines`）。
- `ok_json` — 成功包膜串行化（compact 单行，Agent 省 token）。
- `ok_json_paged` — extract 分页成功包膜：有剩余页时 meta 带 next_offset 与 cta。

## Types

- `ErrEnvelope` — 失败包膜 `{ok:false, error, meta}`（stdout 补充通道；stderr 人读行另出，见 lib.rs）。
- `Meta` — 包膜 meta：`command` 与 `duration_ms` 稳定字段；extract 分页有剩余时附 `next_offset` 与 `cta`。
- `OkEnvelope` — 成功包膜 `{ok:true, data, meta}`；`data` 先落 `Value`，便于 filter 裁剪后入膜。

