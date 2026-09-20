//! Agent 自省与发现（P0007；三面统一批 REQ-057 对齐总台 REQ-060）：`--llms` 旗标
//! 裸出 markdown 紧凑手册（命令表自活 clap 命令树渲染，零手维护双份；curated 段
//! 承载退出码、行式契约与 env 等 clap 不知道的语义），`--llms --json` 出机器形态。
//! 漂移由 tests/cli.rs 守卫兜底：clap 命令树旗标全覆盖 `--llms` 输出断言。

use serde_json::json;

use crate::command_tree;

/// 通用旗标（多命令共享，单列一节；叶子行只列特有旗标，省行数）。
const GLOBAL_FLAGS: [&str; 2] = ["format", "filter"];

/// 手册叶：裸命令路径（如 `issue new`，机器形用）、用法（带位置参数占位，手册表用）、
/// 一句描述、特有旗标串。
struct Leaf {
    path: String,
    usage: String,
    description: String,
    flags: Vec<(String, String)>,
}

/// 取一个参数的展示名：长旗标（带短旗标并写，布尔不带值占位）或位置参数名。
fn arg_name(arg: &clap::Arg) -> Option<String> {
    if arg.get_long().is_some() || arg.get_short().is_some() {
        let mut s = match arg.get_long() {
            Some(l) => format!("--{l}"),
            None => String::new(),
        };
        if let Some(v) = arg.get_short() {
            s = format!("-{v} / {s}");
        }
        let takes_value = matches!(
            arg.get_action(),
            clap::ArgAction::Set | clap::ArgAction::Append
        );
        if takes_value {
            if let Some(vals) = arg.get_value_names() {
                let joined: Vec<String> = vals.iter().map(|v| v.to_string()).collect();
                if !joined.is_empty() {
                    s = format!("{s} <{}>", joined.join("|"));
                }
            }
        }
        Some(s)
    } else if arg.is_positional() {
        arg.get_value_names().map(|vals| {
            vals.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
    } else {
        None
    }
}

/// 表格单元格转义（值名含 | 时不断列）。
fn cell(s: &str) -> String {
    s.replace('|', "\\|")
}

/// 收集叶子的特有旗标（跳过 help/version、通用旗标与位置参数）。
fn leaf_flags(cmd: &clap::Command) -> Vec<(String, String)> {
    cmd.get_arguments()
        .filter(|a| a.get_id().as_str() != "help" && a.get_id().as_str() != "version")
        .filter(|a| !GLOBAL_FLAGS.contains(&a.get_id().as_str()))
        .filter(|a| !a.is_positional())
        .filter_map(|a| {
            let name = arg_name(a)?;
            let help = a
                .get_help()
                .map(|h| h.to_string())
                .unwrap_or_default()
                .split('（')
                .next()
                .unwrap_or("")
                .to_string();
            Some((name, help))
        })
        .collect()
}

/// 深度优先走活命令树：路由节点只贡献路径，叶子收条目（help 子命令跳过）。
fn collect(cmd: &clap::Command, prefix: &str, out: &mut Vec<Leaf>) {
    let subs: Vec<&clap::Command> = cmd
        .get_subcommands()
        .filter(|s| s.get_name() != "help")
        .collect();
    if subs.is_empty() {
        let positionals: Vec<String> = cmd
            .get_positionals()
            .filter_map(arg_name)
            .map(|p| format!("<{p}>"))
            .collect();
        let usage = if positionals.is_empty() {
            prefix.trim().to_string()
        } else {
            format!("{} {}", prefix.trim(), positionals.join(" "))
        };
        out.push(Leaf {
            path: prefix.trim().to_string(),
            usage,
            description: cmd.get_about().map(|a| a.to_string()).unwrap_or_default(),
            flags: leaf_flags(cmd),
        });
    } else {
        for s in subs {
            let next = format!("{prefix} {}", s.get_name());
            collect(s, &next, out);
        }
    }
}

fn leaves() -> Vec<Leaf> {
    let mut out = Vec::new();
    collect(&command_tree(), "", &mut out);
    out
}

/// `reader --llms`：markdown 紧凑手册（命令表自活命令树渲染；总长目标至多 120 行）。
pub fn llms_text() -> String {
    let v = env!("CARGO_PKG_VERSION");
    let mut s = String::new();
    s.push_str(&format!(
        "# reader v{v} — Agent 原生文档阅读、搜索和提取工具\n"
    ));
    s.push_str("PDF 按页；markdown 与 anydoc 家族（Word/EPUB/ODT/RTF/Office/CSV）按标题节；图片单图即单页；只读文本层；缩写 rr 同入口。命令表自活 clap 树渲染（零手维护）；细节契约见下，渐进深入走 --help。\n");
    s.push('\n');
    s.push_str("## 子命令\n");
    s.push('\n');
    s.push_str("| 命令 | 说明 | 特有旗标 |\n");
    s.push_str("|---|---|---|\n");
    for l in &leaves() {
        let flags = l
            .flags
            .iter()
            .map(|(n, _)| n.clone())
            .collect::<Vec<_>>()
            .join(" ");
        s.push_str(&format!(
            "| {} | {} | {} |\n",
            cell(&l.usage),
            cell(&l.description),
            cell(&flags)
        ));
    }
    s.push('\n');
    s.push_str("## 通用旗标\n");
    s.push('\n');
    s.push_str(
        "- --llms：本手册（agent 说明书；markdown 紧凑手册）；--llms --json 出机器形态 JSON\n",
    );
    s.push_str("- --format <text|json>：输出形态（text 行式缺省；json 包膜）\n");
    s.push_str("- --filter <路径>：裁剪 JSON data 点路径（如 hits[].text；仅 --format json）\n");
    s.push('\n');
    s.push_str("## 常用例\n");
    s.push('\n');
    s.push_str("```bash\n");
    s.push_str("reader search ./doc.pdf \"error\" -i -C 1\n");
    s.push_str("reader extract ./scan.pdf --ocr --format json --limit 5\n");
    s.push_str("reader query ./README.md \".h2\"\n");
    s.push_str("reader figures ./report.docx --format json --filter 'figures[].caption'\n");
    s.push_str("reader export ./paper.pdf\n");
    s.push_str("reader issue new \"缺陷标题\" --kind bug --acceptance \"复现与修复判据\"\n");
    s.push_str("```\n");
    s.push('\n');
    s.push_str("## 退出码与输出契约\n");
    s.push('\n');
    s.push_str("- 退出码：0 成功或命中 / 1 无命中（search；issue list 空、show 不存在同）/ 2 出错（stderr 人读行；--format json 时 stdout 另出错误包膜）\n");
    s.push_str("- 裸调用：无参运行出帮助面（stdout 全貌形，含 --llms 指引），退出 0，不弹交互\n");
    s.push_str("- json 包膜：{\"ok\":bool,\"data\":...,\"meta\":{command,duration_ms[,next_offset,cta]}}\n");
    s.push_str("- text 行式：命中行 单元:行号:文本；extract 节头 == page N == / == section N == / == part N ==（超 200 行单元按行分片）；目录批量命中行前缀 路径:\n");
    s.push_str("- figures 行式：figure: kind | 锚 | 图题或- | 落盘路径 | 字节数B；export 摘要行：export: text/pages/figures/manifest <路径>\n");
    s.push_str("- ledger 面（ledger.ohmygh.com 仓级公共账本 REQ-063，真源替代 issues.ohmygh.com 旧面）：issue new 回执行 issue: opened #<n> seq <seq> kind <kind> 加详情页链；list 行 #<n> <status> <kind> <assignee|-> <标题>（--limit 缺省与上限 100 加 --before 游标加 has_more；count 是返回条数非在册总数）；close 两连发（result 引 sha256 digest 加 status done）；show 出投影加验收面加时间线；artifact 行式 artifact: published <id> <kind> <名> 加 digest 行、attest/promote 回 artifact: <type> <id> seq <seq>\n");
    s.push_str("- ledger 写入五头 Ed25519 签名（Idempotency-Key/X-Key-Id/X-Timestamp/X-Nonce/X-Signature；私钥 READER_LEDGER_KEY 或本地密档 ~/.config/reader/ledger-key，不进仓）；读面免签\n");
    s.push_str("- ocr 子命令行式：ocr_init: / ocr_doctor: / ocr_switch: 前缀、ASCII token 前置（ok / missing / corrupt / download mirror|huggingface|github / verdict）；doctor 退出码 0 双包完整 / 1 有缺损\n");
    s.push_str("- env：READER_MIRROR（镜像基址）、READER_OCR_CACHE_DIR、READER_OCR_MODEL_SIZE、READER_LEDGER（账本基址）、READER_LEDGER_KEY / READER_LEDGER_KEY_FILE（账本私钥）、GH_TOKEN（self update 配额）\n");
    s.push_str("- 不可靠页：扫描件、编码问题页与图片以 needs_ocr 提示；--ocr 对 PDF 与图片兜底识别（首用下载约 6.2MB 模型，多核约 1-5 秒/页）；--offline 禁下载\n");
    s
}

/// `reader --llms --json`：机器形态（结构对齐族标准：name、version、description、
/// globalFlags、commands 叶数组）。
///
/// # Panics
///
/// 仅当 `serde_json::to_string_pretty` 序列化失败；载荷全为基础类型组合，实际不可达。
pub fn llms_json() -> String {
    let commands: Vec<serde_json::Value> = leaves()
        .iter()
        .map(|l| {
            json!({
                "path": l.path,
                "description": l.description,
                "flags": l.flags.iter().map(|(n, d)| json!({"name": n, "description": d})).collect::<Vec<_>>(),
            })
        })
        .collect();
    let out = json!({
        "name": "reader",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Agent 原生文档阅读、搜索和提取工具（PDF 按页；markdown 与 anydoc 家族按标题节；图片单图即单页；只读文本层）",
        "globalFlags": [
            {"name": "--format", "description": "输出形态：text 行式（缺省）或 json 包膜"},
            {"name": "--filter", "description": "裁剪 JSON data 的点路径（仅 --format json）"},
        ],
        "commands": commands,
    });
    serde_json::to_string_pretty(&out).expect("手册 JSON 应可序列化")
}
