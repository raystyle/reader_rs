# /// script
# requires-python = ">=3.9"
# dependencies = []
# ///
"""md-ref-scan.py - 全仓 markdown 仓内路径引用断链扫描

扫描指定根下所有 .md，提取文中引用的仓内相对路径（正反斜杠皆可、
docs 前缀或根级大写文件名），验证目标文件存在；不存在的报为断链。
用于文档结构大改（改名、编号、移目录）后的回归验证，可进门禁。

用法:
  uv run --script .tools/md-ref-scan.py                 # 扫当前目录，输出全部断链
  uv run --script .tools/md-ref-scan.py --root docs     # 只扫 docs
  uv run --script .tools/md-ref-scan.py --allow known.txt  # 白名单（每行一条正则，匹配引用则豁免）

退出码: 0 无断链; 1 有断链; 2 参数或 IO 错误。
仓库自带豁免清单 .tools/md-ref-allow.txt（历史快照、占位符、外部仓路径）。
"""

import argparse
import glob
import io
import os
import re
import sys

REF_PAT = re.compile(
    r'(?:docs[\\/][\w\\/\-一-鿿\.]+?\.md'
    r'|(?:INDEX|GOAL|PLAN|TODO|AGENTS|README|CHANGELOG|ROADMAP|CLAUDE|template)\.md)'
)
SKIP_DIRS = ('.git', 'target', 'node_modules', '.tools', 'vendor')


def load_allow(path):
    if not path:
        return []
    with io.open(path, encoding='utf-8') as f:
        return [re.compile(line.strip()) for line in f if line.strip() and not line.startswith('#')]


def scan(root, allow):
    mds = [m for m in glob.glob(os.path.join(root, '**', '*.md'), recursive=True)
           if not any(s in m for s in SKIP_DIRS)]
    bad = []
    for m in sorted(mds):
        try:
            text = io.open(m, encoding='utf-8').read()
        except OSError as e:
            bad.append((m, f'<read-error {e}>'))
            continue
        for ref in sorted(set(REF_PAT.findall(text))):
            if any(p.search(ref) for p in allow):
                continue
            if not os.path.exists(ref.replace('\\', '/')):
                bad.append((m, ref))
    return mds, bad


def main():
    ap = argparse.ArgumentParser(description='markdown 仓内引用断链扫描')
    ap.add_argument('--root', default='.', help='扫描根（默认当前目录）')
    ap.add_argument('--allow', default='.tools/md-ref-allow.txt', help='豁免正则清单文件（默认仓库自带清单；传空串关闭')
    args = ap.parse_args()

    allow = load_allow(args.allow)
    mds, bad = scan(args.root, allow)
    print(f'checked {len(mds)} markdown files, {len(bad)} broken refs')
    for m, r in bad:
        print(f'  {m} -> {r}')
    return 1 if bad else 0


if __name__ == '__main__':
    sys.exit(main())
