//! EPUB 章提取：rbook 按 spine 阅读序出章，quick-xml 事件流把 XHTML 文本化为行。

use crate::document::{TextUnit, UnitKind};
use quick_xml::events::Event;
use quick_xml::Reader;
use rbook::epub::Epub;
use std::collections::HashSet;
use std::path::Path;

/// 提取指定章（`filter` 为 1 起 spine 序，`None` 为全部）并文本化。
pub fn extract_chapters(
    path: &Path,
    filter: Option<&HashSet<u32>>,
) -> Result<Vec<TextUnit>, String> {
    let epub = Epub::open(path).map_err(|e| format!("无法读取 EPUB {}: {e}", path.display()))?;
    let mut reader = epub.reader();
    let mut units = Vec::new();
    let mut no = 0u32;
    while let Some(item) = reader.read_next() {
        no += 1;
        if let Some(f) = filter {
            if !f.contains(&no) {
                continue;
            }
        }
        let xhtml = item
            .map_err(|e| format!("读取 EPUB 第 {no} 章失败（{}）: {e}", path.display()))?
            .content()
            .to_string();
        let lines = xhtml_to_lines(&xhtml);
        if !lines.is_empty() {
            units.push(TextUnit {
                no,
                kind: UnitKind::Chapter,
                lines,
                needs_ocr: None,
            });
        }
    }
    Ok(units)
}

// 简化注记：块级标签断行、行内标签直拼；pre 内保留换行与行首缩进；script/style 内容丢弃；
// 实体显式解析（quick-xml 0.42 把 `&..;` 报为 GeneralRef 事件）。非良构处截断本章剩余内容（尽力而为）。
// 天花板：不保留链接目标、表格不成结构、不做 CSS 隐藏判定；升级路径：换 html2text 类整转换器。
fn xhtml_to_lines(xhtml: &str) -> Vec<String> {
    let mut reader = Reader::from_str(xhtml);
    let mut lines = Vec::new();
    let mut cur = String::new();
    let mut skip_depth = 0u32;
    let mut in_pre = false;
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => match e.local_name().as_ref() {
                "script" | "style" => skip_depth += 1,
                "pre" => {
                    flush_line(&mut lines, &mut cur);
                    in_pre = true;
                }
                name if is_block(name) => flush_line(&mut lines, &mut cur),
                _ => {}
            },
            Ok(Event::End(e)) => match e.local_name().as_ref() {
                "script" | "style" => skip_depth = skip_depth.saturating_sub(1),
                "pre" => {
                    flush_pre_line(&mut lines, &mut cur);
                    in_pre = false;
                }
                name if is_block(name) => flush_line(&mut lines, &mut cur),
                _ => {}
            },
            Ok(Event::Empty(e)) => {
                if e.local_name().as_ref() == "br" {
                    flush_line(&mut lines, &mut cur);
                }
            }
            Ok(Event::Text(t)) => {
                if skip_depth == 0 {
                    if in_pre {
                        push_pre_text(&mut lines, &mut cur, &t.html_content());
                    } else {
                        push_text(&mut cur, &t.html_content());
                    }
                }
            }
            Ok(Event::CData(c)) => {
                if skip_depth == 0 {
                    if in_pre {
                        push_pre_text(&mut lines, &mut cur, &c.html_content());
                    } else {
                        push_text(&mut cur, &c.html_content());
                    }
                }
            }
            Ok(Event::GeneralRef(r)) => {
                if skip_depth == 0 {
                    if let Some(text) = resolve_entity(r.html_content().as_ref()) {
                        cur.push_str(&text);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    flush_line(&mut lines, &mut cur);
    lines
}

fn is_block(name: &str) -> bool {
    matches!(
        name,
        "p" | "div"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "li"
            | "tr"
            | "section"
            | "article"
            | "header"
            | "footer"
            | "blockquote"
            | "table"
            | "body"
            | "title"
    )
}

/// 文本块入当前行：HTML 语义，连续空白折叠为一个空格；块边界空白保留为间隔。
fn push_text(cur: &mut String, text: &str) {
    let leading_ws = text.starts_with(char::is_whitespace);
    let trailing_ws = text.ends_with(char::is_whitespace);
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        if !cur.is_empty() && !cur.ends_with(' ') {
            cur.push(' ');
        }
        return;
    }
    if leading_ws && !cur.is_empty() && !cur.ends_with(' ') {
        cur.push(' ');
    }
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            cur.push(' ');
        }
        cur.push_str(word);
    }
    if trailing_ws && !cur.ends_with(' ') {
        cur.push(' ');
    }
}

fn flush_line(lines: &mut Vec<String>, cur: &mut String) {
    let line = cur.trim();
    if !line.is_empty() {
        lines.push(line.to_string());
    }
    cur.clear();
}

/// pre 内文本：保留换行（每个源行一个输出行），不折叠空白。
fn push_pre_text(lines: &mut Vec<String>, cur: &mut String, text: &str) {
    for (i, part) in text.split('\n').enumerate() {
        if i > 0 {
            flush_pre_line(lines, cur);
        }
        cur.push_str(part);
    }
}

/// pre 行收尾：保留行首缩进（代码结构），只去行尾空白。
fn flush_pre_line(lines: &mut Vec<String>, cur: &mut String) {
    let line = cur.trim_end();
    if !line.trim().is_empty() {
        lines.push(line.to_string());
    }
    cur.clear();
}

/// 解析实体引用（GeneralRef 事件内容为 `amp`、`#x2019`、`#38` 等形态）。
fn resolve_entity(name: &str) -> Option<String> {
    if let Some(num) = name.strip_prefix('#') {
        let code = match num.strip_prefix(['x', 'X']) {
            Some(hex) => u32::from_str_radix(hex, 16).ok()?,
            None => num.parse().ok()?,
        };
        return char::from_u32(code).map(|c| c.to_string());
    }
    if name == "nbsp" {
        return Some(" ".to_string());
    }
    quick_xml::escape::resolve_predefined_entity(name).map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xhtml_blocks_become_lines() {
        let xhtml = "<html><body><h1>Title</h1><p>Hello <b>EPUB</b> world</p><p>Second&amp;third</p></body></html>";
        assert_eq!(
            xhtml_to_lines(xhtml),
            vec!["Title", "Hello EPUB world", "Second&third"]
        );
    }

    #[test]
    fn xhtml_skips_script_and_resolves_char_refs() {
        let xhtml = "<p>a&#x21;</p><script>var x = 1;</script><p>b&nbsp;c</p>";
        assert_eq!(xhtml_to_lines(xhtml), vec!["a!", "b c"]);
    }

    #[test]
    fn xhtml_pre_keeps_line_breaks() {
        let xhtml = "<p>before</p><pre>line one\n  indented two\nline three</pre><p>after</p>";
        assert_eq!(
            xhtml_to_lines(xhtml),
            vec![
                "before",
                "line one",
                "  indented two",
                "line three",
                "after"
            ]
        );
    }
}
