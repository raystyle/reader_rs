//! 搜索层：匹配器（字面 / 正则 / 忽略大小写）与命中收集。

use crate::document::TextUnit;
use regex::RegexBuilder;

/// 行匹配器：字面、正则与忽略大小写三种口径，[`search`] 的命中判定用。
pub enum Matcher {
    /// 字面子串匹配。
    Plain {
        /// 模式串（`ignore_case` 时预折叠小写）。
        needle: String,
        /// 忽略大小写。
        ignore_case: bool,
    },
    /// 正则匹配（regex crate 语法）。
    Regex(regex::Regex),
}

impl Matcher {
    /// 构造匹配器；`regex_mode` 时按正则解释。
    ///
    /// # Errors
    ///
    /// `regex_mode` 且模式非法（regex crate 编译错误，信息带原文）。
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

    /// 行是否命中。
    ///
    /// # Examples
    ///
    /// ```
    /// # use reader_rs::search::Matcher;
    /// let m = Matcher::new("配置", false, false).unwrap();
    /// assert!(m.is_match("环境配置说明"));
    /// assert!(!m.is_match("安装"));
    /// ```
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

/// 一次命中：单元序号（页/章）、单元内行号（均 1 起）、命中行文本与上下文。
pub struct Hit {
    /// 单元序号（页/节，1 起）。
    pub unit: u32,
    /// 单元内行号（1 起）。
    pub line_no: usize,
    /// 命中行文本。
    pub text: String,
    /// 前置上下文（行号，文本）。
    pub before: Vec<(usize, String)>,
    /// 后置上下文（行号，文本）。
    pub after: Vec<(usize, String)>,
}

/// 在所有文本单元的重建行中搜索，`context` 为前后各带的上下文行数。
pub fn search(units: &[TextUnit], matcher: &Matcher, context: usize) -> Vec<Hit> {
    let mut hits = Vec::new();
    for unit in units {
        for (idx, line) in unit.lines.iter().enumerate() {
            if matcher.is_match(line) {
                let lo = idx.saturating_sub(context);
                let hi = (idx + context).min(unit.lines.len() - 1);
                hits.push(Hit {
                    unit: unit.no,
                    line_no: idx + 1,
                    text: line.clone(),
                    before: (lo..idx).map(|j| (j + 1, unit.lines[j].clone())).collect(),
                    after: ((idx + 1)..=hi)
                        .map(|j| (j + 1, unit.lines[j].clone()))
                        .collect(),
                });
            }
        }
    }
    hits
}
