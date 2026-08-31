# R002-选型研究细则-cratesio与github双通道

> 角色：**选 Rust 库与项目的研究操作手册**。AGENTS 写 Rust 规则「先查最流行、最稳定的库」的细则载体。2026-08-31 自 ohmyagents `R005-选型研究细则-cratesio与github双通道.md` 引入，其依据为 ohmyagents S013 双通道实证研究。

## 一、何时走哪条通道

| 问题 | 通道 |
| --- | --- |
| 选依赖 crate（稳定性） | crates.io 通道 |
| 选学习对象 / 评估项目质量 | GitHub 通道 |
| 某概念有没有人写过、官方文档怎么讲 | GitHub 文档搜索 |
| 源码深读 | GitHub 通道 clone |

## 二、crates.io 通道五步

```bash
# 1 拿候选（本机 rsproxy 镜像必须 --registry crates-io）
cargo search <关键词> --limit 20 --registry crates-io

# 2 快筛：一屏看版本、双下载量、updated、cargo add 行
cratesinfo search <关键词> --limit 10

# 3 核证：license、repository、features、90 日下载
cargo info <name> --registry crates-io
cratesinfo info <name>

# 4 谁在用（依赖方在 versions[].crate，总数在 meta.total；1 req/s 带 UA）
curl -sS -A "<工具名> (联系方式)" \
  "https://crates.io/api/v1/crates/<name>/reverse_dependencies" \
| jq -r '.meta.total, (.versions[].crate)'

# 5 收尾
cargo audit && cargo add <name> --features <...>
```

稳度四信号：`max_stable_version` 非空、`recent_downloads` 高、`updated_at` 近 6 到 12 个月、反向依赖不低。阈值是启发式不是门禁：窄领域新库会被误杀，人工核 repository 与文档再定。

批量比稳度才上 API 加 jq（`select(.max_stable_version != null)` 加 recent 阈值）；日常 cratesinfo 够。

## 三、GitHub 通道四步

```bash
# 1 领域扫描：流行（stars）与活跃（pushedAt）一屏分辨；新秀加 created 限定
gh search repos "<领域词>" --language=Rust --sort=stars --limit 10 \
  --json fullName,stargazersCount,pushedAt,description \
  --jq '.[] | "\(.stargazersCount)\t\(.fullName)\t\(.pushedAt[0:10])"'

# 2 定点核证（注意 view 用 issues.totalCount，无 openIssues 字段）
gh repo view <owner>/<repo> --json stargazerCount,pushedAt,licenseInfo,isArchived,issues,latestRelease,repositoryTopics

# 3 发布节奏与真实用法
gh api repos/<owner>/<repo>/releases --jq '.[0:4] | .[] | "\(.tag_name)\t\(.published_at[0:10])"'
gh search code "<签名片段>" --language=Rust --limit 5

# 4 深读：先看历史再拿代码
gh repo clone <owner>/<repo> <dir> -- --filter=blob:none --no-checkout
git clone --depth 1 https://github.com/<owner>/<repo>
```

## 四、坑速查

| 坑 | 正解 |
| --- | --- |
| 镜像源劫持 cargo search / info | 加 `--registry crates-io` |
| 反向依赖取 `.dependencies[].crate_id` | 依赖方在 `.versions[].crate`，总数 `.meta.total` |
| crates 与仓库 license 不一致 | 定型前人工核仓库 LICENSE 文件 |
| qualifier 混独立 flag 报错 | created 等限定词与关键词同在引号内，语言用 `--language=Rust` |
| search 与 view 字段名不同 | `stargazersCount`（search）对 `stargazerCount`（view） |
| 星数当唯一标准 | stars 与 pushedAt 并看；新秀另走 created 筛选 |
| 全量 clone 浪费 | `--filter=blob:none --no-checkout` 先行，要代码再 depth 1 |
| gh 搜索大小写敏感 | 搜词与源码一致；rg 加 `-i`，结构匹配用 ast-grep |

## 五、验收自查

1. 选型结论附证据：双通道至少各一条（crates 稳度字段或 gh 质量信号）
2. 引入的依赖有 pin 或版本理由（窄领域新库写明复核依据）
3. 研究文档断言标六态（`docs\guide\G002-研究标准细则-结构与六态标记.md`）
