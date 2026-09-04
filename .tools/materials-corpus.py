# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""materials-corpus.py:本地研究资料库(E:\\研究资料,D46 第 2 轮裁定弃 E:\\ebook 改此,主性能与质量测试面)
的语料登记、质量基线与性能报告工具。外部真样本不入仓:manifest 钉身份(sha256),缺盘即跳过,CI 免跑。

用法:
  uv run --script .tools/materials-corpus.py --scan             # 只扫盘出/更新 tests/materials/manifest.json(身份层)
  uv run --script .tools/materials-corpus.py --baseline         # 扫盘加逐件跑 extract 记质量基线(status/units/needs_ocr)
  uv run --script .tools/materials-corpus.py --perf             # 逐件计时跑 extract,报告落 tests/materials/reports/(不断言)
  uv run --script .tools/materials-corpus.py --verify           # 对照 manifest 基线逐件核验(退出码 0 一致 / 2 漂移)
选项:
  --root DIR   语料根(缺省 E:\\研究资料;或 env READER_MATERIALS_ROOT)
  --reader PATH  reader 二进制(缺省 target/release/reader[.exe],perf 与 baseline 用 release 口径)
约定:
  支持面 = reader is_supported 的扩展名(pdf/epub/md/图片/anydoc 族);其余跳过登记。
  基线字段 status(exit 码)、units(单元数)、needs_ocr(needs_ocr 单元数)全部确定性,
  duration 只进 perf 报告不进基线(G005 计时不进断言)。
  tests/materials.rs(gated)与 --verify 同口径:manifest 或盘缺失即整体跳过不算失败。
退出码 0 成功 / 2 失败或漂移。
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import platform
import subprocess
import sys
import time
from pathlib import Path

SUPPORTED = {
    "pdf", "md", "markdown", "png", "jpg", "jpeg", "bmp", "gif", "webp", "tiff", "tif",
    "doc", "docx", "docm", "odt", "pptx", "pptm", "ppsx", "ppsm", "ppt", "pps", "pot",
    "rtf", "epub", "xlsx", "xlsm", "xlsb", "xls", "ods", "odp", "csv",
}
REPO = Path(__file__).resolve().parent.parent
MANIFEST = REPO / "tests" / "materials" / "manifest.json"
REPORTS = REPO / "tests" / "materials" / "reports"


def die(msg: str) -> None:
    print(f"materials-corpus: {msg}", file=sys.stderr)
    raise SystemExit(2)


def default_root() -> Path:
    return Path("E:/研究资料")


def default_reader() -> Path:
    exe = REPO / "target" / "release" / ("reader.exe" if platform.system() == "Windows" else "reader")
    if not exe.is_file():
        die(f"release 二进制缺 {exe};先 cargo build --release")
    return exe


def scan(root: Path) -> list[dict]:
    files = sorted(
        p for p in root.rglob("*")
        if p.is_file() and p.suffix.lower().lstrip(".") in SUPPORTED
    )
    entries = []
    for p in files:
        h = hashlib.sha256()
        with p.open("rb") as f:
            for chunk in iter(lambda: f.read(1 << 20), b""):
                h.update(chunk)
        entries.append(
            {"rel": p.relative_to(root).as_posix(), "size": p.stat().st_size, "sha256": h.hexdigest()}
        )
        print(f"scan {entries[-1]['rel']} ({entries[-1]['size']} B)")
    return entries


def run_reader(reader: Path, root: Path, rel: str) -> dict:
    """一次 extract(json 加 filter 取结构面,避免大书全文进内存/管道)并计时。"""
    out = subprocess.run(
        [str(reader), "extract", str(root / rel), "--format", "json", "--filter", "units[].needs_ocr"],
        capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=600,
    )
    result: dict = {"status": out.returncode}
    if out.returncode == 0:
        try:
            data = json.loads(out.stdout)
            needs = data.get("data")
            result["units"] = len(needs) if isinstance(needs, list) else None
            result["needs_ocr"] = sum(1 for v in needs if v) if isinstance(needs, list) else None
        except json.JSONDecodeError:
            result["units"] = None
            result["needs_ocr"] = None
    return result


def cmd_scan(root: Path) -> list[dict]:
    if not root.is_dir():
        die(f"语料根不存在 {root}(本机无盘即跳过:tests/materials.rs 会整体 skip)")
    entries = scan(root)
    MANIFEST.parent.mkdir(parents=True, exist_ok=True)
    meta = {
        "root": str(root),
        "scanned_at": dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds"),
        "count": len(entries),
        "note": "外部真样本不入仓;身份钉 sha256;质量基线字段确定性,duration 只进 perf 报告(D46)",
    }
    old = json.loads(MANIFEST.read_text(encoding="utf-8")) if MANIFEST.is_file() else {}
    # --scan 保留既有 baseline 字段(身份更新不抹质量基线)
    merged = {e["rel"]: e for e in old.get("entries", [])}
    for e in entries:
        if e["rel"] in merged and merged[e["rel"]].get("sha256") == e["sha256"]:
            e.update({k: merged[e["rel"]][k] for k in ("status", "units", "needs_ocr") if k in merged[e["rel"]]})
    MANIFEST.write_text(
        json.dumps({**meta, "entries": entries}, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    print(f"materials-corpus: manifest {len(entries)} 件 → {MANIFEST}")
    return entries


def cmd_baseline(root: Path, reader: Path) -> None:
    entries = cmd_scan(root)
    for e in entries:
        t0 = time.perf_counter()
        got = run_reader(reader, root, e["rel"])
        wall = round((time.perf_counter() - t0) * 1000)
        e.update(got)
        print(f"baseline {e['rel']}: {got} ({wall} ms)")
    MANIFEST.write_text(
        json.dumps(
            {
                "root": str(root),
                "scanned_at": dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds"),
                "count": len(entries),
                "note": "外部真样本不入仓;身份钉 sha256;质量基线字段确定性,duration 只进 perf 报告(D46)",
            } | {"entries": entries},
            ensure_ascii=False, indent=2,
        ) + "\n",
        encoding="utf-8",
    )
    print(f"materials-corpus: 基线 {len(entries)} 件 → {MANIFEST}")


def cmd_perf(root: Path, reader: Path) -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    rows = []
    total_ms = 0.0
    total_bytes = 0
    for e in manifest["entries"]:
        t0 = time.perf_counter()
        got = run_reader(reader, root, e["rel"])
        wall_ms = round((time.perf_counter() - t0) * 1000)
        total_ms += wall_ms
        total_bytes += e["size"]
        rows.append((e["rel"], e["size"], got["status"], got.get("units"), wall_ms))
        print(f"perf {e['rel']}: {wall_ms} ms status={got['status']} units={got.get('units')}")
    REPORTS.mkdir(parents=True, exist_ok=True)
    today = dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%d")
    rep = REPORTS / f"{today}-extract-corpus.md"
    lines = [
        f"# {today} 全语料 extract 性能(机器:本机,release 口径)",
        "",
        f"- 件数 {len(rows)};总字节 {total_bytes / 1e6:.1f} MB;总耗时 {total_ms / 1000:.1f} s;"
        f"吞吐 {total_bytes / 1e6 / (total_ms / 1000):.1f} MB/s",
        "",
        "| 件 | MB | exit | units | ms |",
        "| --- | --- | --- | --- | --- |",
    ]
    lines += [f"| {r} | {s / 1e6:.1f} | {st} | {u} | {ms} |" for r, s, st, u, ms in rows]
    rep.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"materials-corpus: 报告 → {rep}(不覆盖旧报告)")


def cmd_verify(root: Path, reader: Path) -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    drift = []
    for e in manifest["entries"]:
        got = run_reader(reader, root, e["rel"])
        for k in ("status", "units", "needs_ocr"):
            if e.get(k) is not None and got.get(k) != e.get(k):
                drift.append(f"{e['rel']}: {k} 基线 {e.get(k)} 实测 {got.get(k)}")
    if drift:
        for d in drift:
            print(f"DRIFT {d}", file=sys.stderr)
        die(f"{len(drift)} 处漂移;有意变更须 --baseline 重钉并人工审")
    print(f"materials-corpus: {len(manifest['entries'])} 件全数一致")


def main() -> None:
    ap = argparse.ArgumentParser(description="E:\\研究资料 语料登记/基线/性能/核验")
    ap.add_argument("--scan", action="store_true", help="只扫盘出/更新 manifest 身份层")
    ap.add_argument("--baseline", action="store_true", help="扫盘加逐件记质量基线")
    ap.add_argument("--perf", action="store_true", help="逐件计时,报告落 tests/materials/reports/")
    ap.add_argument("--verify", action="store_true", help="对照基线逐件核验(漂移退出 2)")
    ap.add_argument("--root", type=Path, default=None)
    ap.add_argument("--reader", type=Path, default=None)
    args = ap.parse_args()
    modes = [m for m, on in (("scan", args.scan), ("baseline", args.baseline), ("perf", args.perf), ("verify", args.verify)) if on]
    if len(modes) != 1:
        die("须且只须一个模式:--scan / --baseline / --perf / --verify")
    root = args.root or Path(os.environ.get("READER_MATERIALS_ROOT") or default_root())
    reader = args.reader or default_reader()
    {"scan": lambda: cmd_scan(root),
     "baseline": lambda: cmd_baseline(root, reader),
     "perf": lambda: cmd_perf(root, reader),
     "verify": lambda: cmd_verify(root, reader)}[modes[0]]()


if __name__ == "__main__":
    main()
