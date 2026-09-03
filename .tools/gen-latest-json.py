# /// script
# requires-python = ">=3.10"
# ///
"""gen-latest-json.py:从 release API JSON 与 .sha256 边车生成镜像升级清单 reader/latest.json(D42)。

用法:
  uv run --script .tools/gen-latest-json.py --release release.json --assets-dir dist \
      [--mirror https://reader.ohmygh.com] [--out latest.json]

输入:gh api repos/raystyle/reader_rs/releases/tags/<tag> 的 JSON(--release),与
gh release download 落盘的资产目录(--assets-dir,含 <资产>.sha256 边车)。
sha256 以边车为准、API digest 字段兜底(边车可能 CRLF,读时剥 \\r)。
输出:Tauri v2 形状 {"version": ..., "pub_date": ..., "platforms": {<target>: {"url", "sha256"}}};
signature 不输出(minisign 首轮不上);url 指镜像 <mirror>/reader/<version>/<资产名>。
校验:平台五元组白名单(与 src/selfupdate.rs asset_target() 一致),多平台 / 缺平台 /
未知 target / sha256 缺失即报错(清单即发布提交点,宁红勿缺)。
退出码 0 生成 / 2 输入不合法。
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

# 与 src/selfupdate.rs asset_target() 的五三元组一字不差;扩展名按平台(zip / tar.gz)
TARGET_EXT = {
    "x86_64-pc-windows-msvc": "zip",
    "x86_64-unknown-linux-gnu": "tar.gz",
    "x86_64-unknown-linux-musl": "tar.gz",
    "aarch64-apple-darwin": "tar.gz",
    "x86_64-apple-darwin": "tar.gz",
}
# target 用五元组显式 alternation(通用形态会让版本段贪婪回吞 target)
ASSET_RE = re.compile(
    r"^reader-v(?P<version>[0-9][0-9A-Za-z.]*)-(?P<target>"
    + "|".join(sorted(TARGET_EXT, key=len, reverse=True))
    + r")\.(?P<ext>zip|tar\.gz)$"
)


def die(msg: str) -> None:
    print(f"gen-latest-json: {msg}", file=sys.stderr)
    raise SystemExit(2)


def sidecar_sha256(assets_dir: Path, asset: str) -> str | None:
    path = assets_dir / f"{asset}.sha256"
    if not path.is_file():
        return None
    # 边车行式 "<hex>  <文件名>";Windows 产物可能 CRLF(M008 族),剥 \r
    first = path.read_text(encoding="utf-8").splitlines()[0].strip()
    return first.split()[0] if first else None


def main() -> None:
    ap = argparse.ArgumentParser(description="生成镜像升级清单 reader/latest.json")
    ap.add_argument("--release", type=Path, required=True, help="gh api release JSON 路径")
    ap.add_argument("--assets-dir", type=Path, required=True, help="gh release download 落盘目录")
    ap.add_argument("--mirror", default="https://reader.ohmygh.com", help="镜像基址")
    ap.add_argument("--out", type=Path, default=Path("latest.json"), help="输出路径")
    args = ap.parse_args()

    release = json.loads(args.release.read_text(encoding="utf-8"))
    tag = release.get("tag_name") or die("release JSON 无 tag_name")
    version = tag[1:] if tag.startswith("v") else tag
    if not version[0].isdigit():
        die(f"tag {tag} 剥 v 后非版本号形态: {version}")
    pub_date = release.get("published_at") or release.get("created_at")

    # API digest 兜底映射:资产名 -> sha256:
    api_digest = {
        a.get("name", ""): a.get("digest", "").removeprefix("sha256:").lower()
        for a in release.get("assets", [])
    }

    platforms: dict[str, dict[str, str]] = {}
    for asset in sorted(p.name for p in args.assets_dir.iterdir() if p.is_file()):
        m = ASSET_RE.match(asset)
        if not m:
            continue
        target, ext = m.group("target"), m.group("ext")
        if m.group("version") != version:
            die(f"资产 {asset} 版本段与 tag 不符(期望 v{version})")
        if TARGET_EXT.get(target) is None:
            die(f"资产 {asset} 的 target 不在五元组白名单")
        if TARGET_EXT[target] != ext:
            die(f"资产 {asset} 扩展名应为 .{TARGET_EXT[target]}")
        sha = sidecar_sha256(args.assets_dir, asset) or api_digest.get(asset)
        if not sha or len(sha) != 64 or any(c not in "0123456789abcdef" for c in sha):
            die(f"资产 {asset} 无合法 sha256(边车与 API digest 都缺)")
        platforms[target] = {
            "url": f"{args.mirror.rstrip('/')}/reader/{version}/{asset}",
            "sha256": sha,
        }

    missing = sorted(set(TARGET_EXT) - set(platforms))
    if missing:
        die(f"缺平台资产: {', '.join(missing)}(release 是否已传齐?)")
    extra = sorted(set(platforms) - set(TARGET_EXT))
    if extra:
        die(f"多出未知平台: {', '.join(extra)}")

    manifest = {"version": version, "pub_date": pub_date, "platforms": dict(sorted(platforms.items()))}
    args.out.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"gen-latest-json: {args.out} version {version} platforms {len(platforms)}")


if __name__ == "__main__":
    main()
