# /// script
# requires-python = ">=3.10"
# ///
"""regress.py：回归测试层（G006 六层之四）。真样本行为基线对照。

用法：uv run --script tests/regress.py [--reader PATH]

基线来源（独立期望，非被测输出反抄）：
- CLR 书（Command-Line Rust, O'Reilly 2022）：search `assert_cmd` 25 命中行 [S001/P0006 实证]；
  extract 页标记 399 [2026-09-03 本机实测；S001 记 390 系旧管线口径]
- 安全牛水印 PDF：extract 页标记 81 [2026-09-03 本机实测]
- scan-cjk 合成样本（入仓）：extract 报 [needs_ocr: scanned]

external 样本缺失或 sha256 不符时跳过并告警（不算失败）；入仓样本失败即回归。
退出码：0 全过 / 1 有回归 / 2 跑批错误。
"""

import argparse
import hashlib
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

EXTERNAL = [
    {
        "name": "clr-book",
        "path": "D:/Command-Line Rust_ A Project-Based Primer for Writing Rust CLIs 1 (2022, O'Reilly Media).pdf",
        "sha256": "05fc14c57ee757355621988315978280bbf41158646be3fc7f25ced25ac78de9",
    },
    {
        "name": "anniu-watermark",
        "path": "D:/安全牛《新一代自动化渗透测试工具与应用实践指南》--水印.pdf",
        "sha256": "edc1ea37c8e75489ffe91e3c8acd50ab139fd5eb9a65f502137649aedafd3dff",
    },
]


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


def page_markers(stdout: str) -> int:
    return sum(1 for ln in stdout.splitlines() if ln.startswith("== page "))


def main() -> None:
    ap = argparse.ArgumentParser(description="reader 真样本回归测试")
    ap.add_argument("--reader", type=Path, default=None)
    args = ap.parse_args()
    reader = (args.reader or default_reader()).resolve()
    if not reader.is_file():
        print(f"错误: reader 二进制不存在: {reader}", file=sys.stderr)
        sys.exit(2)

    checks = []

    # 入仓合成样本：needs_ocr 检出基线
    sample = ROOT / "tests" / "ab" / "assets" / "scan-cjk.pdf"
    p = run(reader, "extract", str(sample))
    checks.append(("scan-cjk 报 needs_ocr scanned", p.returncode == 0 and "[needs_ocr: scanned]" in p.stdout))

    for s in EXTERNAL:
        path = Path(s["path"])
        if not path.is_file():
            print(f"SKIP  {s['name']}：样本不存在（{path}），external 样本换机需调路径")
            continue
        if hashlib.sha256(path.read_bytes()).hexdigest() != s["sha256"]:
            print(f"SKIP  {s['name']}：sha256 不符，样本被换过")
            continue
        if s["name"] == "clr-book":
            p = run(reader, "search", str(path), "assert_cmd")
            hits = len([ln for ln in p.stdout.splitlines() if ln.strip()])
            checks.append((f"clr search assert_cmd 25 命中（实得 {hits}）", p.returncode == 0 and hits == 25))
            p = run(reader, "extract", str(path))
            n = page_markers(p.stdout)
            checks.append((f"clr extract 399 页标记（实得 {n}）", p.returncode == 0 and n == 399))
        if s["name"] == "anniu-watermark":
            p = run(reader, "extract", str(path))
            n = page_markers(p.stdout)
            checks.append((f"安全牛 extract 81 页标记（实得 {n}）", p.returncode == 0 and n == 81))

    failed = 0
    for name, ok in checks:
        print(f"{'PASS' if ok else 'FAIL'}  {name}")
        failed += 0 if ok else 1
    print(f"回归: {len(checks) - failed}/{len(checks)} 过")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
