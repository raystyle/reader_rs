# tests\assets\anydoc\ 官方测试语料

> anydoc 官方仓测试 fixtures 的入仓拷贝（用户裁定取官方用例做全格式冒烟，D44）。
> 来源：[firecrawl/anydoc](https://github.com/firecrawl/anydoc) `tests\fixtures\`，
> commit `261fc257d17c3eab0f673be31c408fd9fdc2171a`（2026-09-04 取样），MIT 许可
> （随 anydoc 0.2.4 依赖同源，再分发合规）。
> 用途：`tests\smoke.rs` 全格式活体（ppt 二进制族与 xls / xlsb 变体现造不出，官方语料直接补齐）。

| 文件 | 原路径 | sha256 | 实测稳定针 |
| --- | --- | --- | --- |
| text.odt | tests\fixtures\odt\text.odt | `614b107dcd9f48364b33fa98ad1db32b43667701adeb510b081961ced54438ee` | Fixture Document |
| text.rtf | tests\fixtures\rtf\text.rtf | `8af25d8d79f898c5fd8c65782a6b73b04b4869fe621c6d39bf201ce4478ec731` | Fixture Document |
| sheet.ods | tests\fixtures\ods\sheet.ods | `e2c092eb2173b9c7ea8dd2c42e8dd7ef284cf37de40e206f3c8e43571de88454` | ## Values |
| sheet.xlsx | tests\fixtures\xlsx\sheet.xlsx | `ddfec7c1e98c7b50611b1c3ac55c0aa0d9d413135aa7afc36732338e44f4d26c` | ## Values |
| sheet.xls | tests\fixtures\xls\sheet.xls | `1b3fc8f35f4c7ad6bb4dcf9b9f1fdf4ddf1f4c7f2f6748f33b7948204f940136` | ## Values |
| handmade-sheet.xlsb | tests\fixtures\xlsb\handmade-sheet.xlsb | `6f965c3d4c9c9d4028c8e2bb8409c28473cd6310c4af05266cf54f4852f94fbf` | \| Region \| |
| pres.odp | tests\fixtures\odp\pres.odp | `6b5e859ad2591be8f1cbc0246d7e757fbc0ffa06e8ccd022a3e1612f67169df1` | Deck Title Slide |
| pres.pptx | tests\fixtures\pptx\pres.pptx | `c96aa52da19f273f602040490203d9319872f512707e1a6c5a3fc53251b6d050` | Deck Title Slide |
| pres.ppt | tests\fixtures\ppt\pres.ppt | `8b92d2304598dafc977ff309095aa405151ac741d3374b3d5a214377b57765b3` | Deck Title Slide |

升级 anydoc 依赖时如需刷新语料：重取对应 commit 的 fixtures、重算 sha256 更新本表、
`cargo test --test smoke` 全绿即收（针漂移即行为漂移，须人工审）。
