# /// script
# requires-python = ">=3.10"
# ///
"""accept.py：验收测试层（G006 六层之五）的可机检部分。

用法：uv run --script tests/accept.py [--reader PATH]

检查项（对照对外契约与需求口径）：
1. --version 与 Cargo.toml version 一致（发版闸口径）
2. --llms 紧凑索引退出 0 且非空（agent 发现面）
3. search --format json 包膜字段齐备（ok/data/meta）
4. 文件不存在退出 2（grep 语义出错档）
5. query 子命令对 markdown 可用（.h1 退出 0）

不可机检部分（实机清单、发版资产验收）仍走 R004/R005 与发版流程。
退出码：0 全过 / 1 有失败 / 2 跑批错误。
"""

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def default_reader() -> Path:
    exe = "reader.exe" if sys.platform == "win32" else "reader"
    for profile in ("release", "debug"):
        p = ROOT / "target" / profile / exe
        if p.is_file():
            return p
    sys.exit(2)


def run(reader: Path, *args: str) -> subprocess.CompletedProcess:
    return subprocess.run(
        [str(reader), *args], capture_output=True, text=True, encoding="utf-8", errors="replace"
    )


def cargo_version() -> str:
    text = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    m = re.search(r'(?m)^version\s*=\s*"([^"]+)"', text)
    if not m:
        print("错误: Cargo.toml 找不到 version", file=sys.stderr)
        sys.exit(2)
    return m.group(1)


def main() -> None:
    ap = argparse.ArgumentParser(description="reader 验收测试（可机检部分）")
    ap.add_argument("--reader", type=Path, default=None)
    args = ap.parse_args()
    reader = (args.reader or default_reader()).resolve()
    if not reader.is_file():
        print(f"错误: reader 二进制不存在: {reader}", file=sys.stderr)
        sys.exit(2)

    checks = []

    p = run(reader, "--version")
    want = cargo_version()
    checks.append((f"版本一致（{want}）", p.returncode == 0 and want in p.stdout))

    p = run(reader, "--llms")
    checks.append(("--llms 非空", p.returncode == 0 and len(p.stdout.strip()) > 0))

    p = run(reader, "search", "README.md", "reader", "--format", "json")
    try:
        env = json.loads(p.stdout)
        ok = p.returncode == 0 and env.get("ok") is True and "data" in env and "meta" in env
    except json.JSONDecodeError:
        ok = False
    checks.append(("json 包膜 ok/data/meta", ok))

    p = run(reader, "search", "不存在的文件.pdf", "x")
    checks.append(("缺文件退出 2", p.returncode == 2))

    p = run(reader, "query", "README.md", ".h1")
    checks.append(("query .h1 退出 0", p.returncode == 0 and p.stdout.strip()))

    failed = 0
    for name, ok in checks:
        print(f"{'PASS' if ok else 'FAIL'}  {name}")
        failed += 0 if ok else 1
    print(f"验收: {len(checks) - failed}/{len(checks)} 过")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
