//! 搜索层：匹配器（字面 / 正则 / 忽略大小写）与命中收集。

use crate::pdf::PageText;
use regex::RegexBuilder;

/// 行匹配器。
pub enum Matcher {
    Plain { needle: String, ignore_case: bool },
    Regex(regex::Regex),
}

impl Matcher {
    pub fn new(pattern: &str, regex_mode: bool, ignore_case: bool) -> Result<Self, String> {
        if regex_mode {
            RegexBuilder::new(pattern)
                .case_insensitive(ignore_case)
                .build()
                .map(Matcher::Regex)
                .map_err(|e| format!("无效正则 {pattern:?}: {e}"))
        } else {
            Ok(Matcher::Plain {
                needle: if ignore_case {
                    pattern.to_lowercase()
                } else {
                    pattern.to_string()
                },
                ignore_case,
            })
        }
    }

    pub fn is_match(&self, line: &str) -> bool {
        match self {
            Matcher::Plain {
                needle,
                ignore_case,
            } => {
                if *ignore_case {
                    line.to_lowercase().contains(needle)
                } else {
                    line.contains(needle)
                }
            }
            Matcher::Regex(re) => re.is_match(line),
        }
    }
}

/// 一次命中：页码、页内行号（均 1 起）、命中行文本与上下文。
pub struct Hit {
    pub page: u32,
    pub line_no: usize,
    pub text: String,
    pub before: Vec<(usize, String)>,
    pub after: Vec<(usize, String)>,
}

/// 在所有页的重建行中搜索，`context` 为前后各带的上下文行数。
pub fn search(pages: &[PageText], matcher: &Matcher, context: usize) -> Vec<Hit> {
    let mut hits = Vec::new();
    for page in pages {
        for (idx, line) in page.lines.iter().enumerate() {
            if matcher.is_match(line) {
                let lo = idx.saturating_sub(context);
                let hi = (idx + context).min(page.lines.len() - 1);
                hits.push(Hit {
                    page: page.page,
                    line_no: idx + 1,
                    text: line.clone(),
                    before: (lo..idx).map(|j| (j + 1, page.lines[j].clone())).collect(),
                    after: ((idx + 1)..=hi)
                        .map(|j| (j + 1, page.lines[j].clone()))
                        .collect(),
                });
            }
        }
    }
    hits
}
