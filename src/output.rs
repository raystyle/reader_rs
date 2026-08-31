//! 输出层：JSON 包膜（ok/data/error 加 meta）与 filter 点路径裁剪。设计依据 S002（P0006）。

use serde::Serialize;
use serde_json::Value;
use std::time::Instant;

/// 包膜 meta：`command` 与 `duration_ms` 稳定字段；extract 分页有剩余时附 `next_offset` 与 `cta`。
#[derive(Serialize)]
pub struct Meta {
    pub command: &'static str,
    pub duration_ms: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta: Option<String>,
}

/// 成功包膜 `{ok:true, data, meta}`；`data` 先落 `Value`，便于 filter 裁剪后入膜。
#[derive(Serialize)]
pub struct OkEnvelope {
    pub ok: bool,
    pub data: Value,
    pub meta: Meta,
}

/// 失败包膜 `{ok:false, error, meta}`（stdout 补充通道；stderr 人读行另出，见 lib.rs）。
#[derive(Serialize)]
pub struct ErrEnvelope {
    pub ok: bool,
    pub error: String,
    pub meta: Meta,
}

/// 成功包膜串行化（compact 单行，Agent 省 token）。
pub fn ok_json(command: &'static str, started: Instant, data: Value) -> Result<String, String> {
    let env = OkEnvelope {
        ok: true,
        data,
        meta: meta(command, started, None, None),
    };
    serde_json::to_string(&env).map_err(|e| format!("JSON 串行化失败: {e}"))
}

/// 失败包膜串行化（compact 单行）。
pub fn err_json(command: &'static str, started: Instant, error: String) -> String {
    let env = ErrEnvelope {
        ok: false,
        error,
        meta: meta(command, started, None, None),
    };
    serde_json::to_string(&env).unwrap_or_else(|_| "{\"ok\":false}".to_string())
}

/// extract 分页成功包膜：有剩余页时 meta 带 next_offset 与 cta。
pub fn ok_json_paged(
    command: &'static str,
    started: Instant,
    data: Value,
    next_offset: Option<usize>,
    cta: Option<String>,
) -> Result<String, String> {
    let env = OkEnvelope {
        ok: true,
        data,
        meta: meta(command, started, next_offset, cta),
    };
    serde_json::to_string(&env).map_err(|e| format!("JSON 串行化失败: {e}"))
}

fn meta(
    command: &'static str,
    started: Instant,
    next_offset: Option<usize>,
    cta: Option<String>,
) -> Meta {
    Meta {
        command,
        duration_ms: started.elapsed().as_millis(),
        next_offset,
        cta,
    }
}

/// 点路径裁剪：键访问（`a.b`）、数组映射（`hits[].text`）、下标（`units[0].lines`）。
/// 非法路径（键不存在、对非数组用 `[]`）报错不静默。
pub fn filter_value(root: &Value, path: &str) -> Result<Value, String> {
    let path = path.trim();
    if path.is_empty() {
        return Err("无效 filter 路径: 为空".to_string());
    }
    if let Some((head, tail)) = split_first_map(path) {
        let arr = if head.is_empty() {
            root.clone()
        } else {
            filter_value(root, head)?
        };
        let tail = tail.trim_start_matches('.');
        match arr {
            Value::Array(items) => items
                .iter()
                .map(|item| filter_value(item, tail))
                .collect::<Result<Vec<_>, _>>()
                .map(Value::Array),
            other => Err(format!(
                "无效 filter 路径 {path:?}: {head:?} 不是数组（实为 {}）",
                type_name(&other)
            )),
        }
    } else {
        let mut cur = root.clone();
        for seg in path.split('.') {
            let (key, index) = split_index(seg);
            if key.is_empty() {
                return Err(format!("无效 filter 路径 {path:?}: 段 {seg:?} 缺键名"));
            }
            cur = cur
                .get(key)
                .cloned()
                .ok_or_else(|| format!("无效 filter 路径 {path:?}: 无键 {key}"))?;
            if let Some(i) = index {
                cur = cur
                    .get(i)
                    .cloned()
                    .ok_or_else(|| format!("无效 filter 路径 {path:?}: {key} 无下标 {i}"))?;
            }
        }
        Ok(cur)
    }
}

/// 取首个 `[]` 的切分：`hits[].text` → (`hits`, `.text`)；无则 None。
fn split_first_map(path: &str) -> Option<(&str, &str)> {
    path.find("[]").map(|pos| (&path[..pos], &path[pos + 2..]))
}

/// 段切下标：`units[0]` → (`units`, Some(0))；多重下标不支持，报错由键缺失兜住。
fn split_index(seg: &str) -> (&str, Option<usize>) {
    match seg.find('[') {
        Some(pos) => {
            let key = &seg[..pos];
            let inner = seg[pos + 1..].trim_end_matches(']');
            let index = inner.parse().ok();
            (key, index)
        }
        None => (seg, None),
    }
}

fn type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample() -> Value {
        json!({
            "hits": [
                {"unit": 1, "line": 1, "text": "aa"},
                {"unit": 2, "line": 3, "text": "bb"}
            ],
            "units": [{"no": 1, "lines": ["x", "y"]}]
        })
    }

    #[test]
    fn filter_key_map_and_index() {
        let root = sample();
        assert_eq!(
            filter_value(&root, "hits[].text").unwrap(),
            json!(["aa", "bb"])
        );
        assert_eq!(
            filter_value(&root, "units[0].lines").unwrap(),
            json!(["x", "y"])
        );
        assert_eq!(filter_value(&root, "hits[1].unit").unwrap(), json!(2));
        assert_eq!(
            filter_value(&root, "hits")
                .unwrap()
                .as_array()
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn filter_dies_on_bad_path() {
        let root = sample();
        assert!(filter_value(&root, "").is_err());
        assert!(filter_value(&root, "nope").is_err());
        assert!(filter_value(&root, "hits[].nope").is_err());
        assert!(filter_value(&root, "units[].nope").is_err());
        assert!(filter_value(&root, "hits[9].text").is_err());
        assert!(filter_value(&root, ".a").is_err());
    }
}
