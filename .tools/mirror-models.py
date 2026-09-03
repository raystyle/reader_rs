# /// script
# requires-python = ">=3.10"
# ///
"""mirror-models.py:从 HuggingFace 取 PP-OCRv6 四仓模型,校验后staging成镜像上传树(D42)。

用法:
  uv run --script .tools/mirror-models.py [--work DIR] [--date ISO8601] [--dry-run]
产出(--work 下,默认系统临时目录):
  r2/models/<repo>/<rev>/{四件 + LICENSE + NOTICE}   → rclone copy 到 R2 桶 models/ 路径
  r2/models/models.manifest.json                     → 最后传(60s 缓存头)
  gh/<包名>-<rev12>-<文件>  +  PP-OCRv6-LICENSE.txt / PP-OCRv6-NOTICE.txt
                                                     → gh release upload models-v6(恒 prerelease)
  upload.sh                                          → 上述 rclone 命令(调用方供 RCLONE_CONFIG_R2_* env)

事实源:模型清单取 raw.githubusercontent.com/weidix/ppocr-rs/<Cargo.toml 里的 rev>/models.json
(单一事实源,rev 从 Cargo.toml 解析,与客户端 src/mirror.rs pin 表同源)。
源校验:HF tree API——LFS 件(model.safetensors)比 lfs.oid 与 lfs.size;普通 git 件只有
size 可比(无 sha256);漂移只告警不自动跟(硬校验是下载后逐件 sha256 对 models.json)。
许可:HF 源仓无 LICENSE 文件(实证),Apache-2.0 文本取自 apache.org,取不到按失败
(随分发物带许可文本与出处声明是再分发的硬义务)。
GitHub 资产名约定(与 src/mirror.rs gh_asset_name 一字不差):<包名>-<rev 前 12>-<文件名>,
字符集只用 [A-Za-z0-9._-](GitHub 对特殊字符自动改名)。
--dry-run:只取清单与源校验并打印计划,不下载不落盘。
退出码 0 就绪 / 2 失败。
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import re
import shutil
import sys
import tempfile
import time
import urllib.request
from pathlib import Path

MODELS_JSON_URL = "https://raw.githubusercontent.com/weidix/ppocr-rs/{rev}/models.json"
APACHE_URL = "https://www.apache.org/licenses/LICENSE-2.0.txt"
HF_RESOLVE = "https://huggingface.co/{repo}/resolve/{rev}/{name}"
HF_TREE = "https://huggingface.co/api/models/{repo}/tree/{rev}"
WANT = ("tiny-det", "tiny-rec", "small-det", "small-rec")
UA = "reader-mirror-models/0.1"
RCLONE_IMMUTABLE = "Cache-Control: public, max-age=31536000, immutable"
RCLONE_MANIFEST = "Cache-Control: public, max-age=60"


def die(msg: str) -> None:
    print(f"mirror-models: {msg}", file=sys.stderr)
    raise SystemExit(2)


def http_get(url: str, timeout: int = 60) -> bytes:
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        return resp.read()


def http_get_retry(url: str, attempts: int = 3, timeout: int = 120) -> bytes:
    last: Exception | None = None
    for i in range(1, attempts + 1):
        try:
            return http_get(url, timeout)
        except Exception as exc:  # noqa: BLE001 计划内:网络抖动统一重试
            last = exc
            if i < attempts:
                time.sleep(i * 2)
    raise RuntimeError(f"下载 {url} {attempts} 次全败: {last}")


def ppocr_rev_from_cargo() -> str:
    cargo = Path(__file__).resolve().parent.parent / "Cargo.toml"
    text = cargo.read_text(encoding="utf-8")
    m = re.search(r'ppocr-rs\s*=\s*\{[^}]*?rev\s*=\s*"([0-9a-f]{40})"', text, re.S)
    if not m:
        die("Cargo.toml 解析不到 ppocr-rs 40 位 rev")
    return m.group(1)


def notice_text(repo: str, rev: str, date: str) -> str:
    return (
        f"上游:HuggingFace PaddlePaddle/{repo},revision {rev}\n"
        f"许可:Apache-2.0(全文见同目录 LICENSE,文本取自 {APACHE_URL})\n"
        f"本目录由 reader_rs mirror-models 工作流于 {date} 自上游镜像分发,内容未做修改;\n"
        f"逐件 sha256 与 ppocr-rs models.json 钉死值一致(下载后硬校验)。\n"
    )


def main() -> None:
    ap = argparse.ArgumentParser(description="staging PP-OCRv6 四仓模型镜像上传树")
    ap.add_argument("--work", type=Path, default=None, help="工作目录(默认系统临时目录)")
    ap.add_argument("--date", default=None, help="mirrored_at 时间戳(缺省取当前 UTC,ISO8601)")
    ap.add_argument("--dry-run", action="store_true", help="只取清单与源校验并打印计划")
    args = ap.parse_args()

    rev = ppocr_rev_from_cargo()
    print(f"mirror-models: ppocr-rs rev {rev}")
    catalog = json.loads(http_get(MODELS_JSON_URL.format(rev=rev)))
    models = [m for m in catalog["models"] if m["name"] in WANT]
    if {m["name"] for m in models} != set(WANT):
        die(f"models.json 四包不全: {sorted(m['name'] for m in models)}")

    # 源校验(漂移只告警):LFS 件比 lfs.oid/lfs.size,普通件只比 size
    for model in models:
        tree = json.loads(http_get(HF_TREE.format(repo=model["repository"], rev=model["revision"])))
        entries = {e["path"]: e for e in tree}
        for file in model["files"]:
            entry = entries.get(file["name"])
            if entry is None:
                print(f"WARN {model['name']} {file['name']}: HF tree 无此文件", file=sys.stderr)
                continue
            lfs = entry.get("lfs")
            if lfs:
                if lfs.get("oid") != file["sha256"] or lfs.get("size") != file["bytes"]:
                    print(
                        f"WARN {model['name']} {file['name']}: 上游 LFS 与钉死值漂移 "
                        f"(oid {lfs.get('oid')} size {lfs.get('size')})",
                        file=sys.stderr,
                    )
            elif entry.get("size") != file["bytes"]:
                print(
                    f"WARN {model['name']} {file['name']}: 上游 size {entry.get('size')} 与钉死 {file['bytes']} 不符",
                    file=sys.stderr,
                )
    if args.dry_run:
        for model in models:
            print(f"plan r2/models/{model['repository']}/{model['revision']}/ ({len(model['files'])} 件 + LICENSE + NOTICE)")
        print("plan r2/models/models.manifest.json(最后传,max-age=60)")
        print("plan gh/ 16 件 + PP-OCRv6-LICENSE.txt + PP-OCRv6-NOTICE.txt → release models-v6(prerelease)")
        return

    date = args.date or dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds")
    work = args.work or Path(tempfile.mkdtemp(prefix="reader-mirror-models-"))
    r2_models = work / "r2" / "models"
    gh_dir = work / "gh"
    gh_dir.mkdir(parents=True, exist_ok=True)

    apache_text = http_get_retry(APACHE_URL).decode("utf-8")

    manifest = []
    for model in models:
        pkg_rev12 = model["revision"][:12]
        repo_dir = r2_models / model["repository"] / model["revision"]
        repo_dir.mkdir(parents=True, exist_ok=True)
        for file in model["files"]:
            blob = http_get_retry(HF_RESOLVE.format(repo=model["repository"], rev=model["revision"], name=file["name"]))
            if len(blob) != file["bytes"]:
                die(f"{model['name']}/{file['name']}: 字节 {len(blob)} 与钉死 {file['bytes']} 不符")
            digest = hashlib.sha256(blob).hexdigest()
            if digest != file["sha256"]:
                die(f"{model['name']}/{file['name']}: sha256 {digest} 与钉死值不符")
            (repo_dir / file["name"]).write_bytes(blob)
            (gh_dir / f"{model['name']}-{pkg_rev12}-{file['name']}").write_bytes(blob)
            manifest.append(
                {
                    "repo": model["repository"],
                    "rev": model["revision"],
                    "file": file["name"],
                    "sha256": file["sha256"],
                    "license": "Apache-2.0",
                    "mirrored_at": date,
                }
            )
            print(f"ok {model['name']}/{file['name']} ({file['bytes']} B)")
        (repo_dir / "LICENSE").write_text(apache_text, encoding="utf-8")
        (repo_dir / "NOTICE").write_text(notice_text(model["repository"], model["revision"], date), encoding="utf-8")
    (gh_dir / "PP-OCRv6-LICENSE.txt").write_text(apache_text, encoding="utf-8")
    (gh_dir / "PP-OCRv6-NOTICE.txt").write_text(
        "PP-OCRv6 模型(dist/small 两档四仓)来自 HuggingFace PaddlePaddle,Apache-2.0;\n"
        "各仓 revision 与逐件 sha256 见 models.manifest.json(镜像侧为真)。\n"
        f"镜像分发:reader.ohmygh.com(D42),取回时间 {date}。\n",
        encoding="utf-8",
    )
    (r2_models / "models.manifest.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )

    upload = work / "upload.sh"
    work_abs = work.resolve().as_posix()
    # 单件上传不走 copyto:rclone 对 R2 的 copyto 会触发 HeadBucket 加 CreateBucket,
    # 桶级 token 无建桶权即 403(实测);隔离目录加 copy 同效。
    # 树与清单分目录、两条命令同形(免 --exclude:与 --header-upload 并用时实测头不落对象)。
    manifest_only = work / "r2" / "manifest-only"
    manifest_only.mkdir(parents=True, exist_ok=True)
    shutil.move(str(r2_models / "models.manifest.json"), str(manifest_only / "models.manifest.json"))
    upload.write_text(
        "#!/usr/bin/env bash\n"
        "set -euo pipefail\n"
        "# 调用方供 RCLONE_CONFIG_R2_TYPE/_PROVIDER/_ENDPOINT/_ACCESS_KEY_ID/_SECRET_ACCESS_KEY env\n"
        # --ignore-times 强制重传:免得对象已存在被跳过,缓存头修复不到存量件
        f'rclone copy -v --ignore-times "{work_abs}/r2/models" R2:reader-dl/models \\\n'
        f'  --header-upload "{RCLONE_IMMUTABLE}"\n'
        f'rclone copy -v "{work_abs}/r2/manifest-only" R2:reader-dl/models \\\n'
        f'  --header-upload "{RCLONE_MANIFEST}"\n',
        encoding="utf-8",
    )
    print(f"mirror-models: staging 完成 {work_abs}")
    print(f"mirror-models: rclone 命令在 {upload.as_posix()}(manifest 最后传,清单即发布提交点)")


if __name__ == "__main__":
    main()
