# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""md-char-scan.py：Markdown 禁用字符机械判定（G004 门禁）。

先掩掉豁免区（围栏代码块、行内代码、链接目标、裸 URL），再逐字符扫描四类
P0 禁令：破折号、箭头、emoji 与装饰符号、非法全角与智能引号。

用法：uv run --script .tools/md-char-scan.py [路径...]（缺省全仓 markdown，跳过 .git/target/vendor/.tools）
基线：.tools/md-char-baseline.txt 每行一个仓内相对路径（正斜杠），列出的存量文件整体豁免
（渐进清理制：文件清干净即从基线除名；新文件一律须过检）。
退出码：0 无违规；1 有违规。
"""
import re
import sys
from pathlib import Path

RULES = [
    ("DASH", re.compile(r"[\u2013\u2014\u2015\u2212\u30FC]")),
    ("ARROW", re.compile(r"[\u2190-\u21FF\u2794\u279C\u27A1\u2B05-\u2B07]")),
    ("EMOJI", re.compile(
        "[\U0001F000-\U0001FAFF\u2600-\u27BF\u2B00-\u2B0F"
        "\u203C\u2049\u2139\uFE00-\uFE0F\u200D]")),
    ("FULLWIDTH", re.compile(r"[\uFF01-\uFF60\u3000\u2018-\u201D]")),
]
CJK_OK = set("，。：；？！、（）《》「」『』·")
INLINE_CODE = re.compile(r"`[^`]*`")
LINK_TARGET = re.compile(r"\]\([^)]*\)")
BARE_URL = re.compile(r"https?://\S+")
SKIP_DIRS = ('.git', 'target', 'node_modules', '.tools', 'vendor', '.rumdl_cache')


def mask(line):
    line = INLINE_CODE.sub(lambda m: "`" + " " * (len(m.group()) - 2) + "`", line)
    line = LINK_TARGET.sub("]( )", line)
    return BARE_URL.sub(" ", line)


def scan(text):
    in_fence = False
    for no, raw in enumerate(text.splitlines(), 1):
        if raw.lstrip().startswith(("```", "$$")):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        for col, ch in enumerate(mask(raw), 1):
            for name, pattern in RULES:
                if pattern.match(ch) and not (name == "FULLWIDTH" and ch in CJK_OK):
                    yield no, col, name, ch
                    break


def collect(paths):
    if paths:
        for p in paths:
            yield Path(p)
        return
    for f in Path('.').rglob('*.md'):
        if not any(s in f.parts for s in SKIP_DIRS):
            yield f


def load_baseline():
    f = Path(__file__).with_name('md-char-baseline.txt')
    if not f.is_file():
        return set()
    return {
        line.strip().replace('\\', '/')
        for line in f.read_text(encoding='utf-8').splitlines()
        if line.strip() and not line.startswith('#')
    }


def main():
    baseline = load_baseline()
    hits = 0
    skipped = 0
    for path in collect(sys.argv[1:]):
        key = path.as_posix().lstrip('./')
        if not sys.argv[1:] and key in baseline:
            skipped += 1
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        for no, col, name, ch in scan(text):
            print(f"{path}:{no}:{col}: {name} U+{ord(ch):04X}")
            hits += 1
    if skipped:
        print(f"基线豁免 {skipped} 个存量文件", file=sys.stderr)
    print(f"违规 {hits} 处" if hits else "通过：未发现违规字符")
    return 1 if hits else 0


if __name__ == "__main__":
    sys.exit(main())
