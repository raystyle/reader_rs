# Requirements 索引

> 需求登记：新需求先立 REQ 再实现，实现后回填 trace（测试或验收命令）。新建拷 0000-template.md，编号接当前最大号。状态 draft 到 implemented 到 rejected。编号承接旧 PRD D 号口径（D 号即 REQ 号）：D01 至 D47 历史清单已随旧四原语清退（2026-09-17，ADR-0006，git 历史可查）；活跃队列 D23 至 D26 与 D36 至 D39 已按原 D 号转登记为 REQ-023 至 REQ-026 与 REQ-036 至 REQ-039；迁移本身立 REQ-048，aidoc 强制化落地立 REQ-049，分发接入立 REQ-050，封版 0.6.1 立 REQ-051，skill 退役立 REQ-052，封版 0.7.0 立 REQ-053，档案清退立 REQ-054，发布护栏批 1 立 REQ-055，issue 集成立 REQ-056，CLI 三面统一立 REQ-057，cli-docs 细标精对齐立 REQ-058，此后新需求自 REQ-059 接编，三位连续不复用。

| id | 状态 | 优先级 | 标题 | trace |
|---|---|---|---|---|
| REQ-023 | draft | should | MCP服务能力（原 D23） | |
| REQ-024 | draft | should | 分发面扩展cratesio与brew与scoop（原 D24） | |
| REQ-025 | draft | should | OCRv6small质量档旗标（原 D25） | |
| REQ-026 | draft | should | query边界扩展（原 D26） | |
| REQ-036 | draft | should | 错误与诊断体系thiserror-anyhow-tracing（原 D36） | |
| REQ-037 | draft | should | 发布面扩展aarch64musl与cargo-dist（原 D37） | |
| REQ-038 | draft | should | trycmd帮助与文档示例测试（原 D38） | |
| REQ-039 | draft | should | 工程效率件edition2024-nextest-llvm-cov-deny（原 D39） | |
| REQ-048 | implemented | must | 文档体系迁移dev-evo | uv run /mnt/d/ProjectEvo/plugins/project-evo/skills/dev-evo/scripts/check.py . 退出码 0 |
| REQ-049 | implemented | must | aidoc 投影强制化落地 | cargo aidoc --check --strict 退出 0；cargo test --locked 全绿 |
| REQ-050 | implemented | should | 接入统一分发体系omc-catalog与ark落地验收 | catalog pin 0.6.1 加 catalog-seed 绿；五端 ark 回执齐；回归四探针五端绿 |
| REQ-051 | implemented | must | 封版 0.6.1 治理面攒批 | tag v0.6.1；CI 与 release 六 job 绿；latest.json 0.6.1 |
| REQ-052 | implemented | must | skill子命令与SKILL.md退役 | skill 调用 unrecognized 断言；--llms 吸收三契约行；cargo test --locked 加 --doc 全绿 |
| REQ-053 | implemented | must | 封版070skill退役版 | tag v0.7.0；release 六 job 绿；latest.json 与镜像域 0.7.0；分发循环闭合 |
| REQ-054 | implemented | must | 文档体系档案清退与知识融入 | 根五件与 proven 与 mistakes 删件；知识入 ADR-0006；check.py 与文档四门禁绿 |
| REQ-055 | draft | must | 发布流水线护栏批1 | workflow 六加二处落地；dispatch 演练与首个 tag 实跑后收口 |
| REQ-056 | implemented | must | issue命令集成统一入口 | 实弹 #9 加 #10（list/show/API 三方可见）；契约 = ohmycloud REQ-057 |
| REQ-057 | implemented | must | CLI三面统一对齐 | --llms 活树渲染手册加 --json 机器形；README 四节重排；标准 = ohmycloud REQ-060 |
| REQ-058 | implemented | must | cli-docs细标精对齐 | 帮助面头行名@版本全树注入；裸调用全貌形退出 0；对照表与裁剪理由在册 |
