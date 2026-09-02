# /// script
# requires-python = ">=3.9"
# dependencies = []
# ///
"""md-heading-scan.py - markdown 标题规范扫描（G001 标题干净）

扫描根下所有 .md 的标题行（行首 1-6 个 #），报出带全角或半角括号的
标题。代码围栏（``` 或 ~~~）内的 # 是注释不是标题，不计。G001 规定
标题不带括号、不喊口号、不用破折号（解释放标题下一行引用 >）；
本工具只机检括号一项，口号与破折号仍靠人工。

用法:
  uv run --script .tools/md-heading-scan.py              # 扫当前目录
  uv run --script .tools/md-heading-scan.py --root docs  # 只扫 docs

退出码: 0 无违规; 1 有违规; 2 参数或 IO 错误。
"""

import argparse
import glob
import io
import os
import re
import sys

SKIP_DIRS = ('.git', 'target', 'node_modules', '.tools', 'vendor')
HEADING = re.compile(r'^(#{1,6}) (.*)$')
BRACKET = re.compile(r'[（）()]')
FENCE = re.compile(r'^\s*(```|~~~)')


def scan(root):
    mds = [m for m in glob.glob(os.path.join(root, '**', '*.md'), recursive=True)
           if not any(s in m for s in SKIP_DIRS)]
    bad = []
    for m in sorted(mds):
        try:
            lines = io.open(m, encoding='utf-8').read().splitlines()
        except OSError as e:
            bad.append((m, 0, f'<read-error {e}>'))
            continue
        in_fence = False
        for no, line in enumerate(lines, 1):
            if FENCE.match(line):
                in_fence = not in_fence
                continue
            if in_fence:
                continue
            hit = HEADING.match(line)
            if hit and BRACKET.search(hit.group(2)):
                bad.append((m, no, line.strip()))
    return mds, bad


def main():
    ap = argparse.ArgumentParser(description='markdown 标题括号规范扫描（G001）')
    ap.add_argument('--root', default='.', help='扫描根（默认当前目录）')
    args = ap.parse_args()

    mds, bad = scan(args.root)
    print(f'checked {len(mds)} markdown files, {len(bad)} bracketed headings')
    for m, no, line in bad:
        print(f'  {m}:{no}: {line}')
    return 1 if bad else 0


if __name__ == '__main__':
    sys.exit(main())
