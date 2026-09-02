//! mq 结构化提取（P0016；学习 harehare/mq，选型 S007）：全部支持格式转 markdown 文本后
//! 跑 mq 表达式（jq 风格节点选择器与管道）。引擎嵌 mq-lang 全量；非匹配节点产空渲染，
//! 过滤空串即得干净结果（S007 PoC 实证）。

use std::path::Path;

/// 任意支持格式转 markdown 文本：md 读原文；PDF 走 pdf-inspector 布局管线；
/// anydoc 家族走统一引擎 GFM。
pub fn to_markdown(path: &Path) -> Result<String, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .unwrap_or_default();
    if ext == "md" || ext == "markdown" {
        return std::fs::read_to_string(path).map_err(|e| {
            format!(
                "无法读取 markdown 文档 {}（须为 UTF-8）: {e}",
                path.display()
            )
        });
    }
    if ext == "pdf" {
        let result = pdf_inspector::extract_pages_markdown(path, None)
            .map_err(|e| format!("无法读取 PDF {}: {e}", path.display()))?;
        return Ok(result
            .pages
            .into_iter()
            .map(|p| p.markdown)
            .collect::<Vec<_>>()
            .join("\n"));
    }
    if ::anydoc::Format::from_extension(&ext).is_some() {
        let bytes =
            std::fs::read(path).map_err(|e| format!("无法读取文档 {}: {e}", path.display()))?;
        return ::anydoc::to_markdown_bytes(
            &bytes,
            ::anydoc::Format::from_extension(&ext).expect("已判定支持"),
        )
        .map_err(|e| format!("无法解析文档 {}: {e}", path.display()));
    }
    Err(format!(
        "不支持的格式 {}（{}）；query 支持 .pdf、markdown（.md/.markdown）与 anydoc 家族",
        if ext.is_empty() {
            "<无扩展名>"
        } else {
            &ext
        },
        path.display()
    ))
}

/// 跑 mq 表达式，返回非空渲染结果集（markdown 片段原文）。
pub fn run_query(markdown: &str, expression: &str) -> Result<Vec<String>, String> {
    let input =
        mq_lang::parse_markdown_input(markdown).map_err(|e| format!("markdown 解析失败: {e}"))?;
    let mut engine = mq_lang::DefaultEngine::default();
    engine.load_builtin_module();
    let values = engine
        .eval(expression, input.into_iter())
        .map_err(|e| format!("mq 表达式错误: {e}"))?;
    Ok(values
        .into_iter()
        .map(|v| v.to_string())
        .filter(|s| !s.trim().is_empty())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MD: &str = "# 标题一\n\n正文含 [链接](https://example.com)。\n\n## 小节\n\n```rust\nfn main() {}\n```\n";

    #[test]
    fn query_selects_headings() {
        let out = run_query(MD, ".h").unwrap();
        assert_eq!(out, vec!["# 标题一", "## 小节"]);
    }

    #[test]
    fn query_selects_code_and_link() {
        assert_eq!(run_query(MD, ".code").unwrap().len(), 1);
        assert_eq!(
            run_query(MD, ".link").unwrap(),
            vec!["[链接](https://example.com)"]
        );
    }

    #[test]
    fn query_no_match_gives_empty() {
        assert!(run_query(MD, ".h5").unwrap().is_empty());
    }

    #[test]
    fn dies_query_bad_expression() {
        assert!(run_query(MD, "bad syntax here").is_err());
    }
}
