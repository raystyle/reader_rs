# /// script
# requires-python = ">=3.10"
# ///
"""ab_run.py：A/B 对比跑批器（tests/ab 层，标准见 docs/guide/G006）。

用法：
  uv run --script .tools/ab_run.py                      # 全样本，tiny vs small
  uv run --script .tools/ab_run.py --a tiny --b small --sample scan-cjk
  uv run --script .tools/ab_run.py --reader target/debug/reader.exe --report out.md

行为：对 manifest 登记的每个样本，分别以 READER_OCR_MODEL_SIZE=<变体> 跑
`reader extract --ocr --pages ...`（每变体先热跑一次再计时），采集 wall 时长、
行召回、must_contain 命中（去空白后比较，空格属 OCR 排版噪声），出 markdown
对比报告（stdout 加 reports/ 存档）。A/B 是对比不是门禁：
退出码 0 跑批完成 / 2 跑批错误；质量与性能差异由人裁决回填 S 文档。
"""

import argparse
import datetime
import hashlib
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MANIFEST = ROOT / "tests" / "ab" / "manifest.json"
REPORTS = ROOT / "tests" / "ab" / "reports"


def default_reader() -> Path:
    exe = "reader.exe" if sys.platform == "win32" else "reader"
    for profile in ("release", "debug"):
        p = ROOT / "target" / profile / exe
        if p.is_file():
            return p
    sys.exit(f"找不到 reader 二进制（target/release 与 target/debug 均无 {exe}），先 cargo build")


def run_variant(reader: Path, sample: dict, variant: str) -> dict:
    env = dict(os.environ, READER_OCR_MODEL_SIZE=variant, PYTHONIOENCODING="utf-8")
    cmd = [
        str(reader), "extract", sample["_abs_path"], "--ocr",
        "--pages", ",".join(str(p) for p in sample["pages"]),
    ]
    # 先热跑一次（模型装载与下载不计时），第二次为计时热跑
    warm = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8", errors="replace", env=env)
    if warm.returncode != 0:
        raise RuntimeError(f"变体 {variant} 跑 {sample['name']} 退出码 {warm.returncode}: {warm.stderr.strip()[:400]}")
    t0 = time.perf_counter()
    proc = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8", errors="replace", env=env)
    wall = time.perf_counter() - t0
    if proc.returncode != 0:
        raise RuntimeError(f"变体 {variant} 跑 {sample['name']} 退出码 {proc.returncode}: {proc.stderr.strip()[:400]}")
    body = re.sub(r"^== page \d+ ==$", "", proc.stdout, flags=re.M)
    lines = [ln for ln in body.splitlines() if ln.strip() and not ln.startswith("[needs_ocr")]
    text = "\n".join(lines)
    return {
        "wall": wall,
        "per_page": wall / max(len(sample["pages"]), 1),
        "lines": len(lines),
        "text": text,
    }


def squash(s: str) -> str:
    """去全部空白后比较：OCR 在中英文与数字两侧的空格属排版噪声，不计质量差。"""
    return re.sub(r"\s+", "", s)


def score(result: dict, expectations: dict) -> dict:
    text = squash(result["text"])
    must = expectations.get("must_contain", [])
    disc = expectations.get("discriminators", [])
    missing = [c for c in must if squash(c) not in text]
    return {
        "must_hit": len(must) - len(missing),
        "must_total": len(must),
        "missing": missing,
        "disc_hit": sum(1 for c in disc if squash(c) in text),
        "disc_total": len(disc),
    }


def main() -> None:
    ap = argparse.ArgumentParser(description="reader A/B 对比跑批（tests/ab 层）")
    ap.add_argument("--a", default="tiny", help="变体 A（OCR 模型档位，默认 tiny）")
    ap.add_argument("--b", default="small", help="变体 B（默认 small）")
    ap.add_argument("--sample", action="append", help="只跑指定样本（可多次给）")
    ap.add_argument("--reader", type=Path, default=None, help="reader 二进制路径（默认 release 优先）")
    ap.add_argument("--report", type=Path, default=None, help="报告落盘路径（默认 tests/ab/reports/<日期>-A-vs-B.md）")
    args = ap.parse_args()

    reader = args.reader or default_reader()
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    samples = manifest["samples"]
    if args.sample:
        wanted = set(args.sample)
        samples = [s for s in samples if s["name"] in wanted]
        if not samples:
            sys.exit(f"manifest 无样本 {sorted(wanted)}")

    date = datetime.date.today().isoformat()
    report = args.report or REPORTS / f"{date}-{args.a}-vs-{args.b}.md"
    out = [
        f"# A/B 对比报告：{args.a} vs {args.b}",
        "",
        f"> 日期 {date}；CPU 逻辑核 {os.cpu_count()}；reader `{reader}`；逐页 wall 含引擎装载与渲染。",
        "",
    ]
    rc = 0
    for s in samples:
        abs_path = (ROOT / s["path"]).resolve() if not os.path.isabs(s["path"]) else Path(s["path"])
        if not abs_path.is_file():
            out.append(f"## {s['name']}\n\n跳过：样本不存在（{abs_path}），external 样本换机需调路径。\n")
            continue
        if s.get("sha256"):
            digest = hashlib.sha256(abs_path.read_bytes()).hexdigest()
            if digest != s["sha256"]:
                out.append(f"## {s['name']}\n\n跳过：sha256 不符（期望 {s['sha256'][:12]}… 实得 {digest[:12]}…）。\n")
                continue
        s["_abs_path"] = str(abs_path)
        exp = json.loads((ROOT / s["expectations"]).read_text(encoding="utf-8"))
        try:
            ra = run_variant(reader, s, args.a)
            rb = run_variant(reader, s, args.b)
        except RuntimeError as e:
            print(f"错误: {e}", file=sys.stderr)
            rc = 2
            continue
        sa, sb = score(ra, exp), score(rb, exp)
        pages = len(s["pages"])
        out += [
            f"## {s['name']}：{pages} 页",
            "",
            f"| 指标 | {args.a} | {args.b} |",
            "| --- | --- | --- |",
            f"| wall 秒 | {ra['wall']:.1f} | {rb['wall']:.1f} |",
            f"| 秒/页 | {ra['per_page']:.2f} | {rb['per_page']:.2f} |",
            f"| 行召回 | {ra['lines']} | {rb['lines']} |",
            f"| must 命中 | {sa['must_hit']}/{sa['must_total']} | {sb['must_hit']}/{sb['must_total']} |",
            f"| 判别点命中 | {sa['disc_hit']}/{sa['disc_total']} | {sb['disc_hit']}/{sb['disc_total']} |",
            "",
        ]
        for tag, sc in ((args.a, sa), (args.b, sb)):
            if sc["missing"]:
                out.append(f"- {tag} 缺失检查点：{'、'.join(sc['missing'])}")
        out.append("")

    text = "\n".join(out).rstrip()
    print(text)
    if rc == 0:
        REPORTS.mkdir(parents=True, exist_ok=True)
        report.parent.mkdir(parents=True, exist_ok=True)
        report.write_text(text + "\n", encoding="utf-8")
        print(f"报告已落盘: {report}")
    sys.exit(rc)


if __name__ == "__main__":
    main()
