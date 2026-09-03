# /// script
# requires-python = ">=3.10"
# ///
"""smoke.py：冒烟测试层（G006 六层之三）。只断「活着」，秒级完成。

用法：uv run --script tests/smoke.py [--reader PATH]

检查项（退出码与最小输出契约）：
1. --version / --help 退出 0
2. search 命中退出 0、无命中退出 1（grep 语义）
3. extract 合成扫描件（tests/ab/assets/scan-cjk.pdf）报 needs_ocr 提示行

退出码：0 全过 / 1 有失败 / 2 跑批错误（如二进制不存在）。
质量判断不归本层（归验收与 A/B）。
"""

import argparse
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


def main() -> None:
    ap = argparse.ArgumentParser(description="reader 冒烟测试")
    ap.add_argument("--reader", type=Path, default=None)
    args = ap.parse_args()
    reader = (args.reader or default_reader()).resolve()
    if not reader.is_file():
        print(f"错误: reader 二进制不存在: {reader}", file=sys.stderr)
        sys.exit(2)

    checks = []

    p = run(reader, "--version")
    checks.append(("version 退出 0", p.returncode == 0 and p.stdout.strip()))

    p = run(reader, "--help")
    checks.append(("--help 退出 0", p.returncode == 0 and "search" in p.stdout))

    p = run(reader, "search", "README.md", "reader")
    checks.append(("search 命中退出 0", p.returncode == 0))

    p = run(reader, "search", "README.md", "zz-绝不存在的词-zz")
    checks.append(("search 无命中退出 1", p.returncode == 1))

    sample = ROOT / "tests" / "ab" / "assets" / "scan-cjk.pdf"
    p = run(reader, "extract", str(sample))
    checks.append(("扫描件报 needs_ocr", p.returncode == 0 and "[needs_ocr" in p.stdout))

    failed = 0
    for name, ok in checks:
        print(f"{'PASS' if ok else 'FAIL'}  {name}")
        failed += 0 if ok else 1
    print(f"冒烟: {len(checks) - failed}/{len(checks)} 过")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
