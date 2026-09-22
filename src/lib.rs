//! Reader：Agent 原生文档阅读、搜索和提取工具。为 Agent 管线设计的 Rust 单二进制 CLI
//! （`reader` 与 `rr` 双名，同一 `main` 薄壳）：从本地 PDF、markdown、图片与 anydoc 家族
//! （Word 含 legacy .doc、EPUB、ODT、RTF、Office、CSV 等 14 种格式）读文本层。
//! 能力面：按页/节读（extract）、字面与正则搜加目录批量（search）、mq 结构化提取（query）、
//! OCR 兜底识图（`--ocr`，PP-OCRv6 三级回退源链）、图片本体导出与一键完整提取（figures/export）。
//! 输出契约：行式标记、grep 语义退出码 0/1/2、`--format json` 包膜加 `--filter` 裁剪，
//! 机器可读优先于人类美观。本文件承载 CLI 定义与 `run()` 分发；各模块以
//! `document::TextUnit` 为统一文本单元。

pub mod anydoc;
pub mod batch;
pub mod document;
pub mod figures;
pub mod introspect;
pub mod ledger;
pub mod mirror;
pub mod ocr;
pub mod output;
pub mod pdf;
pub mod query;
pub mod search;
pub mod selfupdate;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand, ValueEnum};
use document::OcrOpts;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// 输出形态：text 行式（缺省）或 json 包膜（P0006）。
#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum Format {
    Text,
    Json,
}

/// 输出选项：形态与 filter 裁剪路径（两子命令共用）。
struct OutputOpts {
    format: Format,
    filter: Option<String>,
}

/// 匹配选项：正则开关、大小写、上下文行数（search 专用）。
struct SearchOpts {
    regex_mode: bool,
    ignore_case: bool,
    context: usize,
}

#[derive(Parser)]
#[command(
    name = "reader",
    version,
    about = "Agent 原生文档阅读、搜索和提取工具（PDF 按页；markdown 与 Word / EPUB / ODT / RTF / Office / CSV 按节）"
)]
struct Cli {
    /// 输出紧凑命令手册（agent 发现与接入的说明书；命令表自活命令树渲染）
    #[arg(long)]
    llms: bool,
    /// 与 --llms 同用出机器形态 JSON（命令级 JSON 输出走各命令 --format json）
    #[arg(long)]
    json: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 按页/节搜索文档文本（命中退出 0，无命中退出 1，出错退出 2）；目录输入递归批量搜
    #[command(after_long_help = "\
示例:
  reader search ./doc.pdf \"error\" -i -C 1
  reader search ./doc.pdf \"err(or|code)\" --regex --pages 2-10
  reader search ./docs \"配置\" --format json --filter 'hits[].file'")]
    Search {
        /// 文档或目录路径（.pdf、.md/.markdown、图片（.png/.jpg/.bmp/.gif/.webp/.tiff 等）及 Word / EPUB / ODT / RTF / Office / CSV 家族；目录递归批量搜）
        #[arg(value_name = "文件或目录")]
        file: PathBuf,
        /// 关键词；`--regex` 时按正则解释
        #[arg(value_name = "关键词或正则")]
        pattern: String,
        /// 按正则匹配
        #[arg(long)]
        regex: bool,
        /// 忽略大小写
        #[arg(short = 'i', long)]
        ignore_case: bool,
        /// 命中行前后各带 N 行上下文
        #[arg(short = 'C', long, default_value_t = 0, value_name = "N")]
        context: usize,
        /// 限定页/节范围（1 起），如 1-3,5
        #[arg(long, value_name = "范围")]
        pages: Option<String>,
        /// 输出形态：text（行式，缺省）或 json（包膜）
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// 裁剪 JSON data 的点路径（如 hits[].text）；仅 --format json 下可用
        #[arg(long)]
        filter: Option<String>,
        /// 对 needs_ocr 页与图片文件走 OCR 兜底（PDF 与图片单文件；首用下载约 6.2MB 模型，多核并行约 1-5 秒/页）
        #[arg(long)]
        ocr: bool,
        /// 禁模型下载（须与 --ocr 同用；模型未就位时报错）
        #[arg(long)]
        offline: bool,
    },
    /// 按页/节提取文档文本（默认输出到 stdout）
    #[command(after_long_help = "\
示例:
  reader extract ./doc.pdf
  reader extract ./doc.pdf --pages 1-3,5
  reader extract ./scan.pdf --ocr
  reader extract ./photo.jpg --ocr
  reader extract ./report.docx --format json --offset 0 --limit 5")]
    Extract {
        /// 文档路径（.pdf、.md/.markdown、图片（.png/.jpg/.bmp/.gif/.webp/.tiff 等）及 Word / EPUB / ODT / RTF / Office / CSV 家族）
        #[arg(value_name = "文件")]
        file: PathBuf,
        /// 限定页/节范围（1 起），如 1-3,5
        #[arg(long, value_name = "范围")]
        pages: Option<String>,
        /// 写入文件（缺省输出到 stdout）
        #[arg(short, long, value_name = "文件")]
        out: Option<PathBuf>,
        /// 输出形态：text（行式，缺省）或 json（包膜）
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// 裁剪 JSON data 的点路径（如 units[].no）；仅 --format json 下可用
        #[arg(long)]
        filter: Option<String>,
        /// 跳过前 N 个单元（0 起，两形态同用）
        #[arg(long, default_value_t = 0, value_name = "N")]
        offset: usize,
        /// 最多输出 M 个单元
        #[arg(long, value_name = "M")]
        limit: Option<usize>,
        /// 对 needs_ocr 页与图片文件走 OCR 兜底（PDF 与图片单文件；首用下载约 6.2MB 模型，多核并行约 1-5 秒/页）
        #[arg(long)]
        ocr: bool,
        /// 禁模型下载（须与 --ocr 同用；模型未就位时报错）
        #[arg(long)]
        offline: bool,
    },
    /// 提取图片本体并与文本元数据对齐（PDF 按页渲染 PNG、markdown 图片引用、Office 家族内嵌件、图片文件；有图退出 0，无图 1，出错 2）
    #[command(after_long_help = "\
示例:
  reader figures ./scan.pdf --pages 12-32
  reader figures ./report.docx --format json --filter 'figures[].anchor'
  reader figures ./photo.jpg --out ./shots")]
    Figures {
        /// 文档路径（.pdf、.md/.markdown、图片与 anydoc 家族；扫描书整页即图本体）
        #[arg(value_name = "文件")]
        file: PathBuf,
        /// 限定页范围（仅 PDF），如 1-3,5
        #[arg(long, value_name = "范围")]
        pages: Option<String>,
        /// 输出目录（缺省 <文件名>-figures/）
        #[arg(short, long, value_name = "目录")]
        out: Option<PathBuf>,
        /// 输出形态：text（行式，缺省）或 json（包膜）
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// 裁剪 JSON data 的点路径（如 figures[].caption）；仅 --format json 下可用
        #[arg(long)]
        filter: Option<String>,
    },
    /// 一键完整提取：文本、图片与对齐元数据落一个目录（D47）；导出目录可直接 search 二次复用
    #[command(after_long_help = "\
示例:
  reader export ./paper.pdf
  reader export ./scan.pdf --pages 12-32 --ocr
  reader export ./book.epub --out ./book-everything
  # 二次复用：导出目录当语料搜（pages/ 逐单元文本，命中行带 p0012.md 前缀）
  reader search ./paper-export/ \"certificate\"")]
    Export {
        /// 文档路径（全格式面；图片文件本体即自身）
        #[arg(value_name = "文件")]
        file: PathBuf,
        /// 限定页/单元范围（1 起），如 1-3,5
        #[arg(long, value_name = "范围")]
        pages: Option<String>,
        /// 输出目录（缺省 <文件名>-export/）
        #[arg(short, long, value_name = "目录")]
        out: Option<PathBuf>,
        /// 对 needs_ocr 页与图片走 OCR 兜底（文本回填，标记保留；首用下载约 6.2MB 模型）
        #[arg(long)]
        ocr: bool,
        /// 禁模型下载（须与 --ocr 同用）
        #[arg(long)]
        offline: bool,
    },
    /// 用 mq 表达式结构化提取文档（jq 风格：.h2 标题、.code 代码块、.link 链接、select 管道；命中退出 0，无命中退出 1，出错退出 2）
    #[command(after_long_help = "\
示例:
  reader query ./README.md \".h2\"
  reader query ./doc.pdf \".code\" --format json
  reader query ./notes.md \".[] | select(contains(\\\"关键词\\\"))\" --filter 'results[]'")]
    Query {
        /// 文档路径（.md/.markdown 原文、.pdf 及 anydoc 家族转 markdown 后查询；图片无文本层不支持）
        #[arg(value_name = "文件")]
        file: PathBuf,
        /// mq 表达式（完整语法见 mqlang.org）
        #[arg(value_name = "mq表达式")]
        expression: String,
        /// 输出形态：text（markdown 片段，缺省）或 json（包膜）
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// 裁剪 JSON data 的点路径（如 results[]）；仅 --format json 下可用
        #[arg(long)]
        filter: Option<String>,
    },
    /// 自升级（镜像通道优先整对回落 GitHub；digest 硬校验；本地领先不动；替换后 --version 自证，证败回滚）
    #[command(name = "self")]
    SelfCmd {
        #[command(subcommand)]
        command: SelfCommands,
    },
    /// OCR 模型管理（init 下载、doctor 诊断、switch 切换档位；D42）
    Ocr {
        #[command(subcommand)]
        command: OcrCommands,
    },
    /// 统一 issue 面（ledger.ohmygh.com 仓级公共账本，REQ-063；只增不关不删，收口总台令 2026-09-20）：开单、列表、详情；关闭走 omc 工位
    #[command(after_long_help = "\
示例:
  reader issue new \"search 中文关键词误报\" --kind bug --acceptance \"复现与修复判据\"
  reader issue list --limit 20 --before 5
  reader issue list --status open --kind bug
  reader issue show 3")]
    Issue {
        #[command(subcommand)]
        command: IssueCommands,
    },
    /// 产物共享库面（ledger.ohmygh.com artifact 流，REQ-063；只增不删，promote/demote 走 omc 工位）：publish 登记至 attest 验证
    #[command(after_long_help = "\
示例:
  reader artifact publish \"S010 图表理解定界\" --kind research --digest sha256:<64hex> --summary \"研究成果本体\" --outcome success --git-sha <sha>
  reader artifact attest <id> --type attest_dev
  reader artifact list --current")]
    Artifact {
        #[command(subcommand)]
        command: ArtifactCommands,
    },
}

#[derive(Subcommand)]
enum SelfCommands {
    /// 升级到最新正式版（已最新或本地领先时明示；--force 同版本重装；semver 只升不降）
    Update {
        /// 版本相同也强制重装（不用于降级：本地领先时加 --force 仍不动）
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum OcrCommands {
    /// 下载/修复模型进缓存（镜像 到 HF 到 GitHub 三级回退；缺省档位取 env > switch 设置 > tiny）
    Init {
        /// 指定档位（tiny / small；缺省取当前档）
        #[arg(long, value_name = "tiny或small")]
        size: Option<String>,
        /// 只校验不下载（缓存件无效时报错，零网络）
        #[arg(long)]
        offline: bool,
    },
    /// 诊断本地 OCR 模型就位情况（只读；退出码 0 为当前档双包完整，1 为有缺损）
    Doctor,
    /// 切换模型档位并持久化（env READER_OCR_MODEL_SIZE 优先于本设置）
    Switch {
        /// 目标档位（tiny / small）
        #[arg(value_name = "tiny或small")]
        size: String,
    },
}

#[derive(Subcommand)]
enum IssueCommands {
    /// 开单（BUG 错误任务或改进优化任务，自动带仓库标识；写入走 Ed25519 五头签名；成功 0 / 出错 2）
    New {
        /// 标题（trim 后 1 至 200 字）
        #[arg(value_name = "标题")]
        title: String,
        /// 任务性质（bug 缺省 = BUG 错误任务；improvement = 改进优化任务）
        #[arg(long, default_value = "bug", value_name = "bug或improvement")]
        kind: String,
        /// 验收条件（完成判据；关单须 result 事件引 digest）
        #[arg(long, value_name = "判据")]
        acceptance: String,
        /// 补充正文（缺省空）
        #[arg(long, value_name = "正文")]
        body: Option<String>,
    },
    /// 集中列表（新到旧；has_more 精确翻页；count 是本次返回条数非在册总数；有行 0 / 空 1 / 出错 2）
    List {
        /// 最多 N 条（1 至 100，缺省 100 即服务端上限；更早仍有条目时 stderr 提示翻页）
        #[arg(long, default_value_t = 100, value_name = "N")]
        limit: u32,
        /// keyset 游标：取该 id 之前更早的一页（响应带 has_more；json 面随 data 透出）
        #[arg(long, value_name = "id")]
        before: Option<u64>,
        /// 按状态过滤（open / done 等服务端投影值；过滤跨页精确到全量）
        #[arg(long, value_name = "状态")]
        status: Option<String>,
        /// 按性质过滤（bug / improvement）
        #[arg(long, value_name = "kind")]
        kind: Option<String>,
        /// 输出形态：text（行式，缺省）或 json（包膜）
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// 裁剪 JSON data 的点路径（如 issues[].title）；仅 --format json 下可用
        #[arg(long)]
        filter: Option<String>,
    },
    /// 单条详情（投影加验收面加事件时间线；存在 0 / 不存在 1 / 出错 2）
    Show {
        /// issue 编号
        #[arg(value_name = "编号")]
        n: u64,
        /// 输出形态：text（行式，缺省）或 json（包膜）
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// 裁剪 JSON data 的点路径（如 issue.status）；仅 --format json 下可用
        #[arg(long)]
        filter: Option<String>,
    },
}

#[derive(Subcommand)]
enum ArtifactCommands {
    /// 登记产物（共享库本体；digest 恒为正文或记录哈希，库不收二进制实体；成功 0 / 出错 2）
    Publish {
        /// 名称（1 至 200 字）
        #[arg(value_name = "名")]
        name: String,
        /// 产物性质（experience|lesson|research|prototype|binary|image|wasm|sbom|schema|openapi|eval-set|benchmark|runbook|decision|attested-report）
        #[arg(long, value_name = "kind")]
        kind: String,
        /// 内容哈希（sha256:<64hex>；一律正文或记录哈希为身份）
        #[arg(long, value_name = "sha256hex")]
        digest: String,
        /// 经验描述（成功经验加失败教训加研究成果的文字本体；总台标准必填）
        #[arg(long, value_name = "经验描述")]
        summary: String,
        /// 结果倾向（success 成功经验 / failure 失败教训；如实标注）
        #[arg(long, value_name = "success或failure")]
        outcome: String,
        /// 提交锚（产物对应的 commit sha，服务端落表；缺省空）
        #[arg(long, value_name = "sha")]
        git_sha: Option<String>,
        /// 版本信息（tag 或版本号）
        #[arg(long, value_name = "版本")]
        version: Option<String>,
        /// 开发记录区间（如 v0.9.0..v0.9.1）
        #[arg(long, value_name = "区间")]
        git_range: Option<String>,
        /// 依赖出处（artifact id，可多次）
        #[arg(long, value_name = "id")]
        deps: Vec<String>,
        /// 补充说明正文（缺省空）
        #[arg(long, value_name = "正文")]
        body: Option<String>,
    },
    /// 产物验证事件（attest_dev|attest_prod|verification_failed；promote/demote/supersede 走 omc 工位；成功 0 / 出错 2）
    Attest {
        /// artifact 标识
        #[arg(value_name = "id")]
        artifact_id: String,
        /// 事件类型（attest_dev / attest_prod / verification_failed）
        #[arg(long, value_name = "type")]
        attest_type: String,
        /// 验证证据（JSON 对象，缺省空对象）
        #[arg(long, value_name = "JSON")]
        checks: Option<String>,
        /// 说明（缺省空）
        #[arg(long, value_name = "说明")]
        note: Option<String>,
    },
    /// 产物列表（有行 0 / 空 1 / 出错 2）
    List {
        /// 按名过滤
        #[arg(long, value_name = "名")]
        name: Option<String>,
        /// 按 kind 过滤
        #[arg(long, value_name = "kind")]
        kind: Option<String>,
        /// 按环境过滤（dev / prod）
        #[arg(long, value_name = "dev或prod")]
        env: Option<String>,
        /// 只看各 name 当前持有者（最新 promote 且未退役）
        #[arg(long)]
        current: bool,
        /// 输出形态：text（行式，缺省）或 json（包膜）
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// 裁剪 JSON data 的点路径（如 artifacts[].name）；仅 --format json 下可用
        #[arg(long)]
        filter: Option<String>,
    },
}

/// CLI 入口：返回进程退出码。
pub fn run() -> i32 {
    // 经 command_tree 解析（而非 Cli::parse）：--help 与解析错误的面走已套
    // `名@版本` 头行模板的同一棵树，三面同源
    let cli = Cli::from_arg_matches(&command_tree().get_matches()).unwrap_or_else(|e| e.exit());
    if cli.json && !cli.llms {
        eprintln!("reader: --json 仅与 --llms 同用（命令级 JSON 输出走 --format json）");
        return 2;
    }
    if cli.llms {
        if cli.json {
            print!("{}", introspect::llms_json());
        } else {
            print!("{}", introspect::llms_text());
        }
        return 0;
    }
    match cli.command {
        Some(Commands::Figures {
            file,
            pages,
            out,
            format,
            filter,
        }) => {
            let opts = OutputOpts { format, filter };
            match run_figures(&file, pages, out, &opts) {
                Ok(true) => 0,
                Ok(false) => 1,
                Err(err) => fail("figures", opts.format, err),
            }
        }
        Some(Commands::Export {
            file,
            pages,
            out,
            ocr,
            offline,
        }) => {
            let ocr = OcrOpts { ocr, offline };
            match run_export(&file, pages, out, ocr) {
                Ok(()) => 0,
                Err(err) => fail("export", Format::Text, err),
            }
        }
        Some(Commands::Query {
            file,
            expression,
            format,
            filter,
        }) => {
            let opts = OutputOpts { format, filter };
            match run_query(&file, &expression, &opts) {
                Ok(true) => 0,
                Ok(false) => 1,
                Err(err) => fail("query", opts.format, err),
            }
        }
        Some(Commands::SelfCmd {
            command: SelfCommands::Update { force },
        }) => match selfupdate::self_update(force) {
            Ok(outcome) => {
                match outcome.action {
                    "current" => println!(
                        "self_update: current {}（latest {}，已是最新）",
                        outcome.current, outcome.latest
                    ),
                    "local_newer" => println!(
                        "self_update: local_newer 本地 {} 领先 latest {}，不动（semver 只升不降；如确要回退走 GitHub Releases 手动装）",
                        outcome.current, outcome.latest
                    ),
                    _ => {
                        println!(
                            "self_update: updated {} -> {}",
                            outcome.current, outcome.latest
                        );
                        for path in &outcome.replaced {
                            println!("path: {}", path.display());
                        }
                    }
                }
                0
            }
            Err(err) => fail("self update", Format::Text, err),
        },
        Some(Commands::Ocr { command }) => match command {
            OcrCommands::Init { size, offline } => {
                let size_arg = match size.as_deref().map(ocr::parse_model_size) {
                    Some(Ok(v)) => Some(v),
                    Some(Err(e)) => return fail("ocr init", Format::Text, e),
                    None => None,
                };
                match ocr::init_models(size_arg, offline) {
                    Ok(out) => emit_ocr(out, 2),
                    Err(err) => fail("ocr init", Format::Text, err),
                }
            }
            OcrCommands::Doctor => match ocr::doctor_models() {
                Ok(out) => emit_ocr(out, 1),
                Err(err) => fail("ocr doctor", Format::Text, err),
            },
            OcrCommands::Switch { size } => match ocr::parse_model_size(&size) {
                Ok(target) => match ocr::switch_model(target) {
                    Ok(out) => emit_ocr(out, 2),
                    Err(err) => fail("ocr switch", Format::Text, err),
                },
                Err(err) => fail("ocr switch", Format::Text, err),
            },
        },
        Some(Commands::Issue { command }) => match command {
            IssueCommands::New {
                title,
                kind,
                acceptance,
                body,
            } => match run_issue_new(&title, &kind, &acceptance, body.as_deref().unwrap_or("")) {
                Ok(()) => 0,
                Err(err) => fail("issue new", Format::Text, err),
            },
            IssueCommands::List {
                limit,
                before,
                status,
                kind,
                format,
                filter,
            } => {
                let opts = OutputOpts { format, filter };
                match run_issue_list(limit, before, status.as_deref(), kind.as_deref(), &opts) {
                    Ok(hit) if hit => 0,
                    Ok(_) => 1,
                    Err(err) => fail("issue list", opts.format, err),
                }
            }
            IssueCommands::Show { n, format, filter } => {
                let opts = OutputOpts { format, filter };
                match run_issue_show(n, &opts) {
                    Ok(found) if found => 0,
                    Ok(_) => 1,
                    Err(err) => fail("issue show", opts.format, err),
                }
            }
        },
        Some(Commands::Artifact { command }) => match command {
            ArtifactCommands::Publish {
                name,
                kind,
                digest,
                summary,
                outcome,
                git_sha,
                version,
                git_range,
                deps,
                body,
            } => match run_artifact_publish(
                &name,
                &kind,
                &digest,
                version.as_deref(),
                git_range.as_deref(),
                &deps,
                body.as_deref().unwrap_or(""),
                &summary,
                &outcome,
                git_sha.as_deref(),
            ) {
                Ok(()) => 0,
                Err(err) => fail("artifact publish", Format::Text, err),
            },
            ArtifactCommands::Attest {
                artifact_id,
                attest_type,
                checks,
                note,
            } => {
                match run_artifact_attest(
                    &artifact_id,
                    &attest_type,
                    checks.as_deref(),
                    note.as_deref().unwrap_or(""),
                ) {
                    Ok(()) => 0,
                    Err(err) => fail("artifact attest", Format::Text, err),
                }
            }
            ArtifactCommands::List {
                name,
                kind,
                env,
                current,
                format,
                filter,
            } => {
                let opts = OutputOpts { format, filter };
                match run_artifact_list(
                    name.as_deref(),
                    kind.as_deref(),
                    env.as_deref(),
                    current,
                    &opts,
                ) {
                    Ok(hit) if hit => 0,
                    Ok(_) => 1,
                    Err(err) => fail("artifact list", opts.format, err),
                }
            }
        },
        Some(Commands::Search {
            file,
            pattern,
            regex,
            ignore_case,
            context,
            pages,
            format,
            filter,
            ocr,
            offline,
        }) => {
            let opts = OutputOpts { format, filter };
            let ocr = OcrOpts { ocr, offline };
            let search_opts = SearchOpts {
                regex_mode: regex,
                ignore_case,
                context,
            };
            match run_search(&file, &pattern, &search_opts, pages, &opts, ocr) {
                Ok(true) => 0,
                Ok(false) => 1,
                Err(err) => fail("search", opts.format, err),
            }
        }
        Some(Commands::Extract {
            file,
            pages,
            out,
            format,
            filter,
            offset,
            limit,
            ocr,
            offline,
        }) => {
            let opts = OutputOpts { format, filter };
            let ocr = OcrOpts { ocr, offline };
            match run_extract(&file, pages, out, &opts, offset, limit, ocr) {
                Ok(()) => 0,
                Err(err) => fail("extract", opts.format, err),
            }
        }
        // 裸 reader（无子命令无旗标）：全貌形帮助走 stdout，退出 0（cli-docs 裸调用面
        // 标准：无参是导航事件非错误，帮助体含 --llms 发现指引；组子命令缺叶仍走
        // clap 错误路径退出 2，不在此列）
        None => {
            println!("{}", command_tree().render_help());
            0
        }
    }
}

/// 失败出口：stderr 人读行恒出；json 形态下 stdout 补错误包膜（R001 错误走 stderr 不破）。
fn fail(command: &'static str, format: Format, err: String) -> i32 {
    if format == Format::Json {
        println!("{}", output::err_json(command, Instant::now(), err.clone()));
    }
    eprintln!("reader: {err}");
    2
}

/// ocr 三子命令输出出口：stdout 逐行打出稳定行；不健康按 `code_when_unhealthy`
/// （init/switch 操作失败语义 2，doctor 诊断发现语义 1，D42 输出契约）。
fn emit_ocr(out: ocr::OcrOutcome, code_when_unhealthy: i32) -> i32 {
    for line in &out.lines {
        println!("{line}");
    }
    if out.healthy {
        0
    } else {
        code_when_unhealthy
    }
}

/// 帮助面头行注入 `路径@版本 描述`（cli-docs 帮助面节序首件；版本编译期自
/// Cargo.toml 注入，禁手写，与 `--version`、`--llms` 手册同源）。递归全树同形：
/// 根为 `reader@<版本>`，子命令带父路径（如 `reader self update@<版本>`）。
fn apply_help_face(cmd: &mut clap::Command, path: &str) {
    // help_template 是消费式 builder：mem::take 原地换回，免 clone
    *cmd = std::mem::take(cmd).help_template(format!(
        "{path}@{} {{about}}\n\n{{usage-heading}} {{usage}}\n\n{{all-args}}{{after-help}}",
        env!("CARGO_PKG_VERSION")
    ));
    for sub in cmd.get_subcommands_mut() {
        let name = sub.get_name().to_string();
        apply_help_face(sub, &format!("{path} {name}"));
    }
}

/// 暴露 clap 命令树（tests 的旗标漂移守卫用；P0007）；已套帮助面模板
/// （头行 `路径@版本`，见 [`apply_help_face`]），裸调用帮助与守卫共用此树。
pub fn command_tree() -> clap::Command {
    let mut cmd = Cli::command();
    apply_help_face(&mut cmd, "reader");
    cmd
}

fn run_search(
    file: &Path,
    pattern: &str,
    search_opts: &SearchOpts,
    pages: Option<String>,
    opts: &OutputOpts,
    ocr: OcrOpts,
) -> Result<bool, String> {
    let started = Instant::now();
    let page_set = parse_optional_pages(pages)?;
    check_ocr_opts(ocr)?;
    if file.is_dir() {
        if page_set.is_some() {
            return Err("--pages 不适用于目录搜索".to_string());
        }
        if ocr.ocr {
            return Err("--ocr 不适用于目录搜索（请对单个 PDF 或图片文件使用）".to_string());
        }
        check_filter(opts)?;
        let matcher =
            search::Matcher::new(pattern, search_opts.regex_mode, search_opts.ignore_case)?;
        return batch::run(
            file,
            &matcher,
            search_opts.context,
            opts.format,
            opts.filter.as_deref(),
            started,
        );
    }
    let matcher = search::Matcher::new(pattern, search_opts.regex_mode, search_opts.ignore_case)?;
    let extracted = document::extract(file, page_set.as_ref(), ocr)?;
    warn_unreliable(&extracted, ocr.ocr);
    let hits = search::search(&extracted, &matcher, search_opts.context);
    check_filter(opts)?;
    match opts.format {
        Format::Text => {
            for hit in &hits {
                for (line_no, text) in &hit.before {
                    println!("{}-{}-{}", hit.unit, line_no, text);
                }
                println!("{}:{}:{}", hit.unit, hit.line_no, hit.text);
                for (line_no, text) in &hit.after {
                    println!("{}-{}-{}", hit.unit, line_no, text);
                }
            }
        }
        Format::Json => {
            let mut data = search_data(&extracted, &hits);
            if let Some(path) = opts.filter.as_deref() {
                data = output::filter_value(&data, path)?;
            }
            println!("{}", output::ok_json("search", started, data)?);
        }
    }
    Ok(!hits.is_empty())
}

fn run_extract(
    file: &Path,
    pages: Option<String>,
    out: Option<PathBuf>,
    opts: &OutputOpts,
    offset: usize,
    limit: Option<usize>,
    ocr: OcrOpts,
) -> Result<(), String> {
    let started = Instant::now();
    let page_set = parse_optional_pages(pages)?;
    check_filter(opts)?;
    check_ocr_opts(ocr)?;
    if limit == Some(0) {
        return Err("无效 --limit: 须为正整数".to_string());
    }
    let extracted = document::extract(file, page_set.as_ref(), ocr)?;
    let total = extracted.len();
    let visible: Vec<&document::TextUnit> = extracted
        .iter()
        .skip(offset)
        .take(limit.unwrap_or(usize::MAX))
        .collect();
    let next_offset = (offset + visible.len() < total).then_some(offset + visible.len());
    let content = match opts.format {
        Format::Text => units_text(visible.iter().copied()),
        Format::Json => {
            let mut data =
                json!({ "units": visible.iter().map(|u| unit_value(u)).collect::<Vec<_>>() });
            if let Some(path) = opts.filter.as_deref() {
                data = output::filter_value(&data, path)?;
            }
            let cta = next_offset.map(|next| {
                let limit_arg = limit.map(|l| format!(" --limit {l}")).unwrap_or_default();
                format!(
                    "reader extract {} --offset {next}{limit_arg} --format json",
                    file.display()
                )
            });
            format!(
                "{}\n",
                output::ok_json_paged("extract", started, data, next_offset, cta)?
            )
        }
    };
    match out {
        Some(path) => {
            std::fs::write(&path, content).map_err(|e| format!("写入 {} 失败: {e}", path.display()))
        }
        None => {
            print!("{content}");
            Ok(())
        }
    }
}

/// filter 仅在 json 形态下可用。
fn check_filter(opts: &OutputOpts) -> Result<(), String> {
    if opts.filter.is_some() && opts.format != Format::Json {
        return Err("--filter 仅在 --format json 下可用".to_string());
    }
    Ok(())
}

/// units 的 text 形态渲染（extract 与 export 的 text.md 共用）。
fn units_text<'a>(units: impl IntoIterator<Item = &'a document::TextUnit>) -> String {
    let mut buf = String::new();
    for unit in units {
        buf.push_str(&format!("== {} {} ==\n", unit.kind.label(), unit.no));
        if let Some(reason) = &unit.needs_ocr {
            buf.push_str(&format!("[needs_ocr: {reason}]\n"));
        }
        for line in &unit.lines {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    buf
}

/// export 子命令（D47 第 3 轮：一键完整提取）：文本（连续加逐单元）、图片与对齐
/// manifest 落一个目录；pages/ 逐单元 markdown 让 `reader search <导出目录>` 直接
/// 二次复用（md 是支持格式，命中行带 p0012.md 前缀即页锚）。
fn run_export(
    file: &Path,
    pages: Option<String>,
    out: Option<PathBuf>,
    ocr: OcrOpts,
) -> Result<(), String> {
    let page_set = parse_optional_pages(pages)?;
    check_ocr_opts(ocr)?;
    let default_out = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(format!(
            "{}-export",
            file.file_stem().and_then(|s| s.to_str()).unwrap_or("doc")
        ));
    let dir = out.unwrap_or(default_out);
    let pages_dir = dir.join("pages");
    let images_dir = dir.join("images");
    std::fs::create_dir_all(&pages_dir)
        .map_err(|e| format!("建目录失败 {}: {e}", pages_dir.display()))?;

    let units = document::extract(file, page_set.as_ref(), ocr)?;
    let figures = figures::extract_figures(file, page_set.as_ref(), &images_dir)?;

    std::fs::write(dir.join("text.md"), units_text(units.iter()))
        .map_err(|e| format!("写 text.md 失败: {e}"))?;
    let units_json: Vec<Value> = units.iter().map(unit_value).collect();
    std::fs::write(
        dir.join("text.json"),
        serde_json::to_string(&json!({ "units": units_json, "count": units.len() }))
            .map_err(|e| format!("序列化 text.json 失败: {e}"))?,
    )
    .map_err(|e| format!("写 text.json 失败: {e}"))?;
    let mut page_files = Vec::new();
    for unit in &units {
        let name = format!("p{:04}.md", unit.no);
        let mut body = String::new();
        if let Some(reason) = &unit.needs_ocr {
            body.push_str(&format!("[needs_ocr: {reason}]\n"));
        }
        for line in &unit.lines {
            body.push_str(line);
            body.push('\n');
        }
        std::fs::write(pages_dir.join(&name), body)
            .map_err(|e| format!("写 pages/{name} 失败: {e}"))?;
        page_files.push(name);
    }
    let manifest = json!({
        "file": file.display().to_string(),
        "units": units.iter().enumerate().map(|(i, u)| json!({
            "no": u.no,
            "kind": u.kind.label(),
            "needs_ocr": &u.needs_ocr,
            "lines": u.lines.len(),
            "page_file": &page_files[i],
        })).collect::<Vec<_>>(),
        "figures": figures.iter().map(|f| json!({
            "kind": f.kind,
            "anchor": f.anchor,
            "caption": &f.caption,
            "context": &f.context,
            "file": f.file.file_name().and_then(|n| n.to_str()).unwrap_or(""),
            "bytes": f.bytes,
            "format": f.format,
        })).collect::<Vec<_>>(),
        "counts": { "units": units.len(), "figures": figures.len() },
    });
    std::fs::write(
        dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest)
            .map_err(|e| format!("序列化 manifest 失败: {e}"))?,
    )
    .map_err(|e| format!("写 manifest.json 失败: {e}"))?;

    println!(
        "export: text {} ({} 单元)",
        dir.join("text.md").display(),
        units.len()
    );
    println!(
        "export: pages {} ({} 件,search 二次复用:reader search {} 关键词)",
        pages_dir.display(),
        page_files.len(),
        pages_dir.display()
    );
    println!(
        "export: figures {} 件 -> {}",
        figures.len(),
        images_dir.display()
    );
    println!("export: manifest {}", dir.join("manifest.json").display());
    Ok(())
}

/// figures 子命令（D47）：图本体落盘加行式清单 / json 包膜；返回是否导出至少一件。
fn run_figures(
    file: &Path,
    pages: Option<String>,
    out: Option<PathBuf>,
    opts: &OutputOpts,
) -> Result<bool, String> {
    let started = Instant::now();
    let page_set = parse_optional_pages(pages)?;
    check_filter(opts)?;
    let default_out = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(format!(
            "{}-figures",
            file.file_stem().and_then(|s| s.to_str()).unwrap_or("doc")
        ));
    let out_dir = out.unwrap_or(default_out);
    let figures = figures::extract_figures(file, page_set.as_ref(), &out_dir)?;
    match opts.format {
        Format::Text => {
            for f in &figures {
                println!(
                    "figure: {} | {} | {} | {} | {}B",
                    f.kind,
                    f.anchor,
                    f.caption.as_deref().unwrap_or("-"),
                    f.file.display(),
                    f.bytes
                );
            }
            eprintln!(
                "reader: {} 件图本体导出至 {}（figures,元数据对齐:kind/锚/图题/路径）",
                figures.len(),
                out_dir.display()
            );
        }
        Format::Json => {
            let mut data = json!({
                "figures": figures.iter().map(|f| json!({
                    "kind": f.kind,
                    "anchor": f.anchor,
                    "caption": &f.caption,
                    "context": &f.context,
                    "file": f.file.display().to_string(),
                    "bytes": f.bytes,
                    "format": f.format,
                })).collect::<Vec<_>>(),
                "count": figures.len(),
            });
            if let Some(path) = opts.filter.as_deref() {
                data = output::filter_value(&data, path)?;
            }
            println!("{}", output::ok_json("figures", started, data)?);
        }
    }
    Ok(!figures.is_empty())
}

/// query 子命令：格式转 markdown 后跑 mq 表达式；命中与否映射退出码 0/1（P0016）。
fn run_query(file: &Path, expression: &str, opts: &OutputOpts) -> Result<bool, String> {
    let started = Instant::now();
    check_filter(opts)?;
    if file.is_dir() {
        return Err("query 不支持目录（请对单个文件使用；批量找内容用 search <目录>）".to_string());
    }
    let markdown = query::to_markdown(file)?;
    let results = query::run_query(&markdown, expression)?;
    match opts.format {
        Format::Text => {
            for r in &results {
                println!("{r}");
            }
        }
        Format::Json => {
            let mut data = json!({ "results": results, "count": results.len() });
            if let Some(path) = opts.filter.as_deref() {
                data = output::filter_value(&data, path)?;
            }
            println!("{}", output::ok_json("query", started, data)?);
        }
    }
    Ok(!results.is_empty())
}

fn run_issue_new(title: &str, kind: &str, acceptance: &str, body: &str) -> Result<(), String> {
    let client = ledger::connect()?;
    let n = client
        .issue_new(
            title,
            kind,
            acceptance,
            if body.is_empty() { None } else { Some(body) },
        )
        .map_err(ledger::err_line)?;
    println!("issue: opened #{n} kind {kind}");
    println!(
        "issue: https://ledger.ohmygh.com/repos/{}/i/{n}",
        ledger::REPO_ID
    );
    Ok(())
}

/// 列表行（服务端投影字段同形；`hasResult` 服务端为驼峰）。
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct IssueRow {
    /// 仓内 issue 号。
    #[serde(rename = "issue_n")]
    issue_n: u64,
    /// 标题。
    title: String,
    /// 投影状态（open/claimed/in_progress/blocked/done）。
    status: String,
    /// 任务性质（bug/improvement）。
    kind: String,
    /// 认领者（无则 null）。
    assignee: Option<String>,
    /// 是否已有 result 事件（关单判据面）。
    #[serde(rename = "hasResult")]
    has_result: bool,
}

/// 行匹配（REQ-062 过滤面）：status 与 kind 等值匹配，未给的维度不过滤。
fn issue_row_matches(row: &IssueRow, status: Option<&str>, kind: Option<&str>) -> bool {
    status.is_none_or(|s| row.status == s) && kind.is_none_or(|k| row.kind == k)
}

/// has_more 过滤面语义：服务端末页 has_more，或已取匹配行超 limit（截断即更早
/// 侧仍有未展示匹配）任一成立为真。未过滤路径与单页直取行为等价。
fn list_has_more(server_has_more: bool, matched: usize, want: usize) -> bool {
    server_has_more || matched > want
}

fn run_issue_list(
    limit: u32,
    before: Option<u64>,
    status: Option<&str>,
    kind: Option<&str>,
    opts: &OutputOpts,
) -> Result<bool, String> {
    let started = Instant::now();
    check_filter(opts)?;
    let client = ledger::connect()?;
    let want = limit.clamp(1, 100) as usize;
    // 过滤跨页精确到全量（REQ-062）：沿 has_more/before 翻页累积匹配行直到满
    // limit 或账本穷尽；页深上限兜底防失控。未过滤时缺省 limit 100 单页即止，
    // 行为与旧单页直取等价。
    const MAX_PAGES: usize = 20;
    let mut rows: Vec<IssueRow> = Vec::new();
    let mut cursor = before;
    let mut server_has_more = false;
    let mut pages = 0usize;
    while rows.len() < want && pages < MAX_PAGES {
        let v = client.issue_list(100, cursor).map_err(ledger::err_line)?;
        pages += 1;
        let page: Vec<IssueRow> =
            serde_json::from_value(v.get("issues").cloned().ok_or("回执缺 issues 数组")?)
                .map_err(|e| format!("回执形状不对: {e}"))?;
        server_has_more = v.get("has_more").and_then(Value::as_bool).unwrap_or(false);
        let oldest = page.last().map(|r| r.issue_n);
        rows.extend(
            page.into_iter()
                .filter(|r| issue_row_matches(r, status, kind)),
        );
        match oldest {
            Some(n) if server_has_more => cursor = Some(n),
            _ => break,
        }
    }
    let more = list_has_more(server_has_more, rows.len(), want);
    if pages == MAX_PAGES && server_has_more && rows.len() < want {
        eprintln!(
            "reader: 过滤翻页达页深上限（{MAX_PAGES} 页），更早条目未穷尽；缩条件或分批 --before"
        );
    }
    rows.truncate(want);
    // 翻页提示（家族标准）：has_more 精确判定。stdout 保纯数据。
    if more {
        eprintln!(
            "reader: 更早仍有{}（has_more）；--before <id> 翻更早一页（网页面 ledger.ohmygh.com 可看全量）",
            if status.is_some() || kind.is_some() {
                "匹配条目"
            } else {
                "条目"
            }
        );
    }
    match opts.format {
        Format::Text => {
            for r in &rows {
                println!(
                    "#{} {} {} {} {}",
                    r.issue_n,
                    r.status,
                    r.kind,
                    r.assignee.as_deref().unwrap_or("-"),
                    r.title
                );
            }
        }
        Format::Json => {
            let mut data = json!({ "issues": rows, "count": rows.len(), "has_more": more });
            if let Some(path) = opts.filter.as_deref() {
                data = output::filter_value(&data, path)?;
            }
            println!("{}", output::ok_json("issue list", started, data)?);
        }
    }
    Ok(!rows.is_empty())
}

fn run_issue_show(n: u64, opts: &OutputOpts) -> Result<bool, String> {
    let started = Instant::now();
    check_filter(opts)?;
    let client = ledger::connect()?;
    let v = match client.issue_show(n) {
        Ok(v) => v,
        Err(ledger_client::LedgerError::Api { status: 404, .. }) => return Ok(false),
        Err(e) => return Err(ledger::err_line(e)),
    };
    let projection = v.get("projection").cloned().ok_or("回执缺 projection")?;
    let timeline = v
        .get("timeline")
        .and_then(Value::as_array)
        .cloned()
        .ok_or("回执缺 timeline")?;
    let open_payload = timeline
        .iter()
        .find(|e| e.get("type").and_then(Value::as_str) == Some("issue_open"))
        .and_then(|e| e.get("payload").and_then(Value::as_str))
        .and_then(|s| serde_json::from_str::<Value>(s).ok())
        .unwrap_or(Value::Null);
    let status = projection
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let kind = projection
        .get("kind")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let assignee = projection
        .get("assignee")
        .and_then(Value::as_str)
        .map(|s| s.to_string());
    let acceptance = open_payload
        .get("acceptance")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    match opts.format {
        Format::Text => {
            println!(
                "issue: #{n} [{status}] {kind} assignee {}",
                assignee.as_deref().unwrap_or("-")
            );
            println!("acceptance: {acceptance}");
            for e in &timeline {
                println!(
                    "  seq {} {}",
                    e.get("seq").and_then(Value::as_u64).unwrap_or(0),
                    e.get("type").and_then(Value::as_str).unwrap_or("?")
                );
            }
        }
        Format::Json => {
            let mut data = json!({
                "issue": n,
                "status": status,
                "kind": kind,
                "assignee": assignee,
                "acceptance": acceptance,
                "timeline": timeline,
            });
            if let Some(path) = opts.filter.as_deref() {
                data = output::filter_value(&data, path)?;
            }
            println!("{}", output::ok_json("issue show", started, data)?);
        }
    }
    Ok(true)
}

// 参数形随 crate artifact_publish_full 十参面（同 crate 侧 allow 姿势）
#[allow(clippy::too_many_arguments)]
fn run_artifact_publish(
    name: &str,
    kind: &str,
    digest: &str,
    version: Option<&str>,
    git_range: Option<&str>,
    deps: &[String],
    body: &str,
    summary: &str,
    outcome: &str,
    git_sha: Option<&str>,
) -> Result<(), String> {
    // 总台标准（2026-09-22 硬校验已上）：outcome 仅 success|failure，客户端先拒
    // 不挂网络（同 attest 类型收口姿势）。
    if outcome != "success" && outcome != "failure" {
        return Err(
            "--outcome 仅 success|failure（success 成功经验 / failure 失败教训，如实标注）"
                .to_string(),
        );
    }
    let client = ledger::connect()?;
    let artifact_id = client
        .artifact_publish_full(
            name,
            kind,
            digest,
            version,
            git_range,
            deps,
            if body.is_empty() { None } else { Some(body) },
            Some(summary),
            Some(outcome),
            git_sha,
        )
        .map_err(ledger::err_line)?;
    println!("artifact: published {artifact_id} {kind} {name}");
    println!("artifact: digest {digest}");
    Ok(())
}

fn run_artifact_attest(
    artifact_id: &str,
    attest_type: &str,
    checks: Option<&str>,
    note: &str,
) -> Result<(), String> {
    // 收口（总台修正令 2026-09-20）：验证类三型之外（promote/demote/supersede）
    // 与关闭删除同归 omc 工位，CLI 客户端先拒并指路
    if !ledger_client::ATTEST_TYPES.contains(&attest_type) {
        return Err(format!(
            "attest --attest-type 仅 {}（验证类）；promote/demote/supersede 与关闭删除走 omc 工位（经 herdr 委托）",
            ledger_client::ATTEST_TYPES.join("|")
        ));
    }
    let checks = match checks {
        None => json!({}),
        Some(text) => {
            serde_json::from_str(text).map_err(|e| format!("--checks 须为 JSON 对象: {e}"))?
        }
    };
    let client = ledger::connect()?;
    let v = client
        .artifact_attest(
            artifact_id,
            attest_type,
            checks,
            if note.is_empty() { None } else { Some(note) },
        )
        .map_err(ledger::err_line)?;
    let seq = v
        .get("event")
        .and_then(|e| e.get("seq"))
        .and_then(Value::as_u64)
        .unwrap_or(0);
    println!("artifact: {attest_type} {artifact_id} seq {seq}");
    Ok(())
}

/// 产物列表行。
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct ArtifactRow {
    /// artifact 标识。
    artifact_id: String,
    /// 名称。
    name: String,
    /// kind。
    kind: String,
    /// digest。
    digest: String,
    /// dev 验证与否。
    dev_verified: bool,
    /// prod 验证与否。
    prod_verified: bool,
    /// 是否该 name 的当前持有者。
    current: bool,
}

fn run_artifact_list(
    name: Option<&str>,
    kind: Option<&str>,
    env: Option<&str>,
    current: bool,
    opts: &OutputOpts,
) -> Result<bool, String> {
    let started = Instant::now();
    check_filter(opts)?;
    let client = ledger::connect()?;
    let v = client
        .artifact_list(current, env)
        .map_err(ledger::err_line)?;
    let mut rows: Vec<ArtifactRow> =
        serde_json::from_value(v.get("artifacts").cloned().ok_or("回执缺 artifacts 数组")?)
            .map_err(|e| format!("回执形状不对: {e}"))?;
    // name/kind 过滤在客户端（crate 读面只带 current/env 参数）
    if let Some(n) = name {
        rows.retain(|r| r.name.contains(n));
    }
    if let Some(k) = kind {
        rows.retain(|r| r.kind == k);
    }
    match opts.format {
        Format::Text => {
            for r in &rows {
                println!(
                    "artifact: {} {} {} dev{} prod{} cur{} {}",
                    r.name,
                    r.kind,
                    &r.digest[..19.min(r.digest.len())],
                    if r.dev_verified { 1 } else { 0 },
                    if r.prod_verified { 1 } else { 0 },
                    if r.current { 1 } else { 0 },
                    &r.artifact_id[..8.min(r.artifact_id.len())]
                );
            }
        }
        Format::Json => {
            let mut data = json!({ "artifacts": rows, "count": rows.len() });
            if let Some(path) = opts.filter.as_deref() {
                data = output::filter_value(&data, path)?;
            }
            println!("{}", output::ok_json("artifact list", started, data)?);
        }
    }
    Ok(!rows.is_empty())
}

/// search 的 data 树：hits 加 needs_ocr_units（不可靠页序号）。
fn search_data(units: &[document::TextUnit], hits: &[search::Hit]) -> Value {
    json!({
        "hits": hits.iter().map(|h| json!({
            "unit": h.unit,
            "line": h.line_no,
            "text": &h.text,
            "before": h.before.iter().map(|(l, t)| json!({"line": l, "text": t})).collect::<Vec<_>>(),
            "after": h.after.iter().map(|(l, t)| json!({"line": l, "text": t})).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "needs_ocr_units": units
            .iter()
            .filter(|u| u.needs_ocr.is_some())
            .map(|u| u.no)
            .collect::<Vec<_>>(),
    })
}

/// extract 的 data 树：units（kind / no / needs_ocr / lines）。
fn unit_value(unit: &document::TextUnit) -> Value {
    json!({
        "kind": unit.kind.label(),
        "no": unit.no,
        "needs_ocr": &unit.needs_ocr,
        "lines": &unit.lines,
    })
}

/// `--offline` 须与 `--ocr` 同用（单挂无意义，属于旗标误用）。
fn check_ocr_opts(ocr: OcrOpts) -> Result<(), String> {
    if ocr.offline && !ocr.ocr {
        return Err("--offline 须与 --ocr 同用".to_string());
    }
    Ok(())
}

/// 文本层不可靠的单元给一条 stderr 警示（stdout 保持纯命中输出；退出码语义不变）。
/// `ocr` 为真时这些页已经 OCR 兜底（`needs_ocr` 标记保留）。
pub(crate) fn warn_unreliable(units: &[document::TextUnit], ocr: bool) {
    let bad: Vec<&document::TextUnit> = units.iter().filter(|u| u.needs_ocr.is_some()).collect();
    if bad.is_empty() {
        return;
    }
    let label = bad[0].kind.label();
    let list = bad
        .iter()
        .map(|u| u.no.to_string())
        .collect::<Vec<_>>()
        .join(",");
    if ocr {
        eprintln!(
            "reader: 提示: {label} {list} 已经 OCR 兜底（needs_ocr 标记保留，OCR 文本仍可能有误，命中可能失真）"
        );
    } else {
        eprintln!(
            "reader: 提示: {label} {list} 文本层不可靠（needs_ocr，疑似扫描件、图片或编码问题），命中可能失真；可加 --ocr 兜底识别（PDF 与图片单文件）"
        );
    }
}

fn parse_optional_pages(pages: Option<String>) -> Result<Option<HashSet<u32>>, String> {
    pages.map(|spec| parse_page_spec(&spec)).transpose()
}

/// 解析页范围串（如 `1-3,5`）为 1 起页码集合。
///
/// 段间逗号分隔、段内 `起-止` 闭区间；空白容忍。
///
/// # Errors
///
/// 段非正整数、页号为 0、或起页大于止页；错误串带原段。
///
/// # Examples
///
/// ```
/// use reader_rs::parse_page_spec;
/// let set = parse_page_spec("1-3,5").unwrap();
/// assert_eq!(set.len(), 4);
/// assert!(set.contains(&2) && set.contains(&5));
/// assert!(parse_page_spec("0").is_err());
/// ```
pub fn parse_page_spec(spec: &str) -> Result<HashSet<u32>, String> {
    let mut set = HashSet::new();
    for part in spec.split(',') {
        let part = part.trim();
        if let Some((lo, hi)) = part.split_once('-') {
            let lo = parse_page_no(lo, part)?;
            let hi = parse_page_no(hi, part)?;
            if lo > hi {
                return Err(format!("无效页范围 {part:?}: 起页大于止页"));
            }
            set.extend(lo..=hi);
        } else {
            set.insert(parse_page_no(part, part)?);
        }
    }
    if set.is_empty() {
        return Err(format!("无效页范围 {spec:?}: 为空"));
    }
    Ok(set)
}

fn parse_page_no(text: &str, part: &str) -> Result<u32, String> {
    let no: u32 = text
        .trim()
        .parse()
        .map_err(|_| format!("无效页范围 {part:?}: {text:?} 不是正整数"))?;
    if no == 0 {
        return Err(format!("无效页范围 {part:?}: 页码从 1 起"));
    }
    Ok(no)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_spec_single_and_range() {
        let set = parse_page_spec("1-3,5").unwrap();
        assert_eq!(set, HashSet::from([1, 2, 3, 5]));
    }

    #[test]
    fn page_spec_dies_zero_and_inverted() {
        assert!(parse_page_spec("0").is_err());
        assert!(parse_page_spec("3-1").is_err());
        assert!(parse_page_spec("a").is_err());
        assert!(parse_page_spec("").is_err());
    }

    // D34：页范围解析属性测试（proptest；最小反例自动 shrink，失败即永久回归用例）
    proptest::proptest! {
        #[test]
        fn page_spec_singles_roundtrip(
            pages in proptest::collection::btree_set(1u32..=500u32, 1..20usize)
        ) {
            let spec = pages.iter().map(u32::to_string).collect::<Vec<_>>().join(",");
            let got = parse_page_spec(&spec).unwrap();
            proptest::prop_assert_eq!(got, pages.iter().copied().collect::<HashSet<_>>());
        }

        #[test]
        fn page_spec_range_covers(lo in 1u32..=400u32, len in 0u32..=100u32) {
            let hi = lo + len;
            let got = parse_page_spec(&format!("{lo}-{hi}")).unwrap();
            proptest::prop_assert_eq!(got, (lo..=hi).collect::<HashSet<_>>());
        }

        #[test]
        fn page_spec_any_zero_rejected(n in 1u32..=20u32) {
            let spec = format!("{n},0");
            proptest::prop_assert!(parse_page_spec(&spec).is_err());
        }
    }

    fn trow(status: &str, kind: &str) -> IssueRow {
        IssueRow {
            issue_n: 1,
            title: "t".into(),
            status: status.into(),
            kind: kind.into(),
            assignee: None,
            has_result: false,
        }
    }

    /// REQ-062 过滤面：status/kind 等值匹配，未给维度直通，组配取交。
    #[test]
    fn issue_row_matches_status_kind_both_dimensions() {
        let r = trow("open", "bug");
        assert!(issue_row_matches(&r, None, None));
        assert!(issue_row_matches(&r, Some("open"), None));
        assert!(!issue_row_matches(&r, Some("done"), None));
        assert!(issue_row_matches(&r, None, Some("bug")));
        assert!(!issue_row_matches(&r, None, Some("improvement")));
        assert!(issue_row_matches(&r, Some("open"), Some("bug")));
        assert!(!issue_row_matches(&r, Some("done"), Some("bug")));
    }

    /// REQ-062 has_more 过滤面语义：服务端末页有余或匹配行超 limit 任一即真。
    #[test]
    fn list_has_more_covers_truncation_and_server_page() {
        assert!(list_has_more(true, 3, 5));
        assert!(list_has_more(false, 8, 5));
        assert!(!list_has_more(false, 5, 5));
        assert!(!list_has_more(false, 3, 5));
    }
}
