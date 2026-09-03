# /// script
# requires-python = ">=3.10"
# dependencies = ["pillow"]
# ///
"""make-scan-sample.py：生成 tests/ab 的合成扫描件样本（无文本层图片型 PDF）。

用法：uv run --script .tools/make-scan-sample.py

产物（入仓）：
- tests/ab/assets/scan-cjk.pdf   单页图片型 PDF（Pillow 直出，无文本层，触发 needs_ocr）
- tests/ab/expectations/scan-cjk.json   质量检查点（must_contain 取自下方 SOURCE_LINES，
  独立来源为渲染源文本，禁止改成从 OCR 输出反抄）

幂等：每次重跑覆盖同路径产物；改 SOURCE_LINES 后重跑即更新样本与期望。
"""

import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
ASSETS = ROOT / "tests" / "ab" / "assets"
EXPECTATIONS = ROOT / "tests" / "ab" / "expectations"
FONT_PATH = Path("C:/Windows/Fonts/msyh.ttc")

# 渲染源文本：质量检查点的独立来源（中英文混排加数字，覆盖 OCR 常见弱点位）
SOURCE_LINES = [
    "新一代自动化渗透测试工具与应用实践指南",
    "Automated Penetration Testing Guide 2026",
    "第十章 漏洞扫描器与渗透测试框架",
    "攻击者潜伏期高达 287 天，人工完成的排查占比低",
    "采用自动化工具后，报告显示覆盖率显著提升",
]

WIDTH = 1190
FONT_SIZE = 36
MARGIN = 60
LINE_STEP = 96


def main() -> None:
    if not FONT_PATH.is_file():
        raise SystemExit(f"缺 CJK 字体 {FONT_PATH}（Windows 应自带微软雅黑）")
    ASSETS.mkdir(parents=True, exist_ok=True)
    EXPECTATIONS.mkdir(parents=True, exist_ok=True)

    height = MARGIN * 2 + LINE_STEP * len(SOURCE_LINES)
    img = Image.new("RGB", (WIDTH, height), "white")
    draw = ImageDraw.Draw(img)
    font = ImageFont.truetype(str(FONT_PATH), FONT_SIZE)
    y = MARGIN
    for line in SOURCE_LINES:
        draw.text((MARGIN, y), line, fill="black", font=font)
        y += LINE_STEP

    pdf_path = ASSETS / "scan-cjk.pdf"
    img.save(pdf_path, "PDF", resolution=150.0)

    exp = {
        "sample": "scan-cjk",
        "source": "渲染源文本（.tools/make-scan-sample.py 的 SOURCE_LINES），独立期望非 OCR 反抄",
        "must_contain": SOURCE_LINES,
        "discriminators": [],
        "reference": {},
    }
    exp_path = EXPECTATIONS / "scan-cjk.json"
    exp_path.write_text(json.dumps(exp, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"written: {pdf_path}")
    print(f"written: {exp_path}")


if __name__ == "__main__":
    main()
