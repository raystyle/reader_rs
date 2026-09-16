# Requirements 索引

> 需求登记：新需求先立 REQ 再实现，实现后回填 trace（测试或验收命令）。新建拷 0000-template.md，编号接当前最大号。状态 draft 到 implemented 到 rejected。编号承接旧 PRD D 号口径（D 号即 REQ 号）：D01 至 D47 历史留档见根 `PRD.md`（迁移注记在其顶部）；活跃队列 D23 至 D26 与 D36 至 D39 已按原 D 号转登记为 REQ-023 至 REQ-026 与 REQ-036 至 REQ-039；迁移本身立 REQ-048，aidoc 强制化落地立 REQ-049，此后新需求自 REQ-050 接编，三位连续不复用。

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
