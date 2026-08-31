# /// script
# requires-python = ">=3.9"
# dependencies = []
# ///
"""md-replace.py - markdown 字面批量替换（中文与反斜杠路径安全）

对指定 glob 的 .md 做**字面**字符串替换（非正则、无转义层），专为
中文文件名与反斜杠路径设计——规避 sed 在该组合下的静默失配与 grep
多层转义假阴性（M023、M026 同源教训）。写入保持 UTF-8 无 BOM、
不改动行尾。

用法:
  uv run --script .tools/md-replace.py --glob 'AGENTS.md' --glob 'docs/**/*.md' \
      --map old.txt [--dry]
  # old.txt 每行一条 `old<TAB>new`（制表符分隔，字面生效）
  uv run --script .tools/md-replace.py --glob README.md --old 旧词 --new 新词 [--dry]

选项: --dry 只打印将发生的替换不写盘。退出码 0（无论是否命中）。
"""

import argparse
import glob as globmod
import io
import sys

SKIP_DIRS = ('.git', 'target', 'node_modules')


def load_map(path):
    pairs = []
    with io.open(path, encoding='utf-8') as f:
        for line in f:
            line = line.rstrip('\n').rstrip('\r')
            if not line or line.startswith('#'):
                continue
            if '\t' not in line:
                raise SystemExit(f'map 行缺少制表符分隔: {line[:60]}')
            old, new = line.split('\t', 1)
            pairs.append((old, new))
    return pairs


def main():
    ap = argparse.ArgumentParser(description='markdown 字面批量替换')
    ap.add_argument('--glob', action='append', required=True, help='目标文件 glob，可多次')
    ap.add_argument('--map', help='替换映射文件（old<TAB>new 每行一条）')
    ap.add_argument('--old', help='单条替换原串')
    ap.add_argument('--new', help='单条替换新串')
    ap.add_argument('--dry', action='store_true', help='只打印不写盘')
    args = ap.parse_args()

    if args.map:
        pairs = load_map(args.map)
    elif args.old is not None and args.new is not None:
        pairs = [(args.old, args.new)]
    else:
        ap.error('需要 --map 或 --old/--new')

    files = sorted({f for g in args.glob for f in globmod.glob(g, recursive=True)
                    if not any(s in f for s in SKIP_DIRS)})
    total = 0
    for path in files:
        text = io.open(path, encoding='utf-8').read()
        n = 0
        for old, new in pairs:
            n += text.count(old)
            text = text.replace(old, new)
        if n:
            print(f'{path} : {n}')
            if not args.dry:
                io.open(path, 'w', encoding='utf-8', newline='').write(text)
        total += n
    print(f'total replacements: {total}{" (dry)" if args.dry else ""}')
    return 0


if __name__ == '__main__':
    sys.exit(main())
