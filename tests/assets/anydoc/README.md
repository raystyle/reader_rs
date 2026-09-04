# tests\assets\anydoc\ 官方测试语料

> anydoc 官方仓测试 fixtures 的入仓拷贝（镜像上游目录布局；用户裁定取官方用例，D44；2026-09-04 第 3 轮扩为全量非 pdf corpus，负例与滥用件文件名即期望行为标签）。
> 来源：[firecrawl/anydoc](https://github.com/firecrawl/anydoc) `tests\fixtures\`，commit `261fc257d17c3eab0f673be31c408fd9fdc2171a`（2026-09-04 取样），MIT 许可（随 anydoc 0.2.4 依赖同源，再分发合规）。
> 用途：`tests\smoke.rs` 全格式活体、`tests\corpus.rs` 逐件快照回归与负例断言。
> 期望行为编码（malformed / abuse 文件名 `--<outcome>` 后缀，同上游）：`errors` 提取应失败（exit 2）、`recovers` / `skips` 容错出部分内容（exit 0）。
> 未取：pdf 族（reader 走 pdf-inspector 非 anydoc，真样本由 E:\研究资料 语料承载（D46））与 fixture-src（fodt 等扩展名不在 reader 支持面）。

共 71 件，sha256：

| 件 | sha256 |
| --- | --- |
| abuse/deepnest--errors.ppt | `693b1560dbc0fc91371b5f4ad546d147b998a7560ce28f4c8a9addb1f0d87bcf` |
| abuse/deepxml--errors.docx | `86650b49a31fc5b7c684b15d75af996fabe38c915524b8670f1cc42355af1b73` |
| abuse/emptyrowrepeat--errors.ods | `f1a839dbbd0e7b3d9fc7c1645aae7a44ae1595377d80ce7ca735d49790174e8c` |
| abuse/hugerepeat--errors.ods | `af3026547c709cac7fb379e94057c7f16e5aea5e7a3be68aa48536e937bb86e0` |
| abuse/hugespan--errors.ods | `5e348ac22afd8ec5b41ebfa527987927a1de80f608c9be8a04531f1bbe353332` |
| abuse/hugespan--errors.pptx | `8e5e31a2f08763cedf95ca8676780718cad49c26eb5fdf1b37b05ca8cdf97430` |
| abuse/imagebomb--errors.docx | `2331f48e333162ead5f2897ae948295b743793c85a70cd4227d9db19302fc524` |
| abuse/zipbomb--errors.docx | `132715bbfe9f82b5a5e4e9ea21375dbd32d00300fc127facb73c2214dd2512c2` |
| csv/handmade-quoted.csv | `c50a6a4b653f234e2a7b4d40a0ed0aa1c44c3f21425760c583b838de6c799301` |
| csv/handmade-semicolon.csv | `93528d417f224c4fb46e8409f1b8af8a32bcb3a76cee2f9e6765d8de6e2eeb42` |
| csv/handmade-utf16.csv | `069d375757165ef6285bf82bf22c72dc525c2e1881678a4e5aac1c6de6c5f907` |
| csv/sheet.csv | `4a19b2cd06b8571a6f39cbb11b51c90857effe25878094b1c817ba63300005fd` |
| doc/handmade-blockstyle.doc | `04c252746d7cb2dbeaef607d6a8bcec31713165b88b2ba8cd640f81d53bbd3c5` |
| doc/handmade-cyrillic.doc | `707497f7f28257b7aba290657e2d799cfc4cecddea0626642820f3b33338f301` |
| doc/handmade-shiftjis.doc | `87cf99313cfb3171f966b4bbd38ba797bb4f8e23aa74b0f9a2547a6b9db417e5` |
| doc/text.doc | `0d7c077cf4b49939a05ccd5f8012164649752be0c5841fa50da6258e0517e6e5` |
| docx/handmade-altpath.docx | `fb77751c25853f876f9cda0b819819f690c4baadf080c1170ad06ffc98e52545` |
| docx/handmade-blockstyle.docx | `a8c1013b60b03c6e65dfb404b1901c81661e344a2fb7e2dd12146fa41c48444e` |
| docx/handmade-manyrefs.docx | `219cfa32f83401f6415191d33d391ddbc35edbbbb3e04678b0d1b74ad6d14cb7` |
| docx/handmade-math.docx | `6499c01602a376977b1b1d40496cc0a4808047c2b78806bf5a7aa96e0ebc80dc` |
| docx/handmade-numbering.docx | `6975897719c716d0c323758b05c287bc8b93e8d022a4119aca042e026ad0646c` |
| docx/handmade-ole.docx | `346a10b066ab72098556dd55b3ef7cd5e0be261c019087e9794b095dc0cdf4a9` |
| docx/handmade-outline.docx | `2c4c232cde1ead23360922df6db0d472500c7c5b5d6149d5ccf9ecd2dac90909` |
| docx/handmade-rich.docx | `22afadb7927cc11d7520cd0f471aa1eea658369a1ba85da48123aded0700aafa` |
| docx/handmade-strict.docx | `f65da59839848f8bbe0b72ddc03f8765bf06662994ae980f22095eb85cb0096a` |
| docx/handmade-tables.docx | `cf847fbf73810f6af47181230cd4ad2704a53e8b2668297dba4904f7366da6ce` |
| docx/text.docx | `6b674297884f9ed57809763c9f60ea3a849d5cc6fb28c9837c714e322eceddcf` |
| epub/book.epub | `292cba3ea8019684cb59cb890de6caf31bc9399d9f983b25db1506e275f83563` |
| epub/handmade-css-links.epub | `0d7a72873c570620e5fd425558bdfb014cecc85771b0775c1e992843509755ce` |
| epub/handmade-features.epub | `86417eb749dfe7eb55043fa177a637e5781d7adc4e834022e211fcda587b1c17` |
| epub/handmade-math.epub | `ed1f1a3ff67f91642835e5d92fc47d606dac286e8de1b19c88156de52959b08a` |
| malformed/brokenpersist--recovers.ppt | `9439be914d8b3285c2a8b9a2043401dad713a10289a792e86faee15fb95d79f6` |
| malformed/corrupt-styles--skips.docx | `3abf6167d3cc89fcc478c6074b53246882a887e4b5e5b2c0dbaec7db9890b931` |
| malformed/empty--errors.docx | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| malformed/encrypted--errors.odt | `3143f94edf73d6add594bf1bf835f437c1a8f0597f62c6323fc7887e27ea5a27` |
| malformed/mismatched--recovers.docx | `d56750725a8207b2a29eb19d8da2b200e9dfc743f18bfa4a9cea5bc9e2624244` |
| malformed/missing-styles--skips.docx | `ae46e869662780d0d90e9add25a1f4ee67d731107f18029e575035dadbb82912` |
| malformed/truncated--errors.doc | `0f400750cc84e132fbe085d9c50d039c71d4d48c01dff3c169bdeaa72627741a` |
| malformed/truncated--errors.docx | `65b78f02a11298d0b860247911e34431aedb05637749f49819a6b7a70fe1967b` |
| malformed/unbalanced--recovers.rtf | `51b015fab14996554900089b988d80b2c9df9527621a23f0fbb89a23f2542a63` |
| malformed/unclosed--recovers.docx | `eb22511ee03c793511fc3e4a780dc1c2b1d44ea8bc5b2c117bf0b295e81f2d94` |
| ods/handmade-durations.ods | `e586f077649c168266e10f79f29f229390ebfab03f1c3e341f3dcfb922eeab31` |
| ods/handmade-gaps.ods | `1064c442fd0e1f58ef72ad0b37dc05f420992072511c9503eed878a273ce9cfb` |
| ods/sheet.ods | `e2c092eb2173b9c7ea8dd2c42e8dd7ef284cf37de40e206f3c8e43571de88454` |
| odp/pres.odp | `6b5e859ad2591be8f1cbc0246d7e757fbc0ffa06e8ccd022a3e1612f67169df1` |
| odt/handmade-blockstyle.odt | `fd95495c3c0fc6afeb7d70bcfe65a1cfd63f96f5ff33a0624581f0f7031c1f4a` |
| odt/handmade-defaults.odt | `c61e3e383bccfae70b293b98b258a8e67ba895fb0b5556363b61c0c4781e14f8` |
| odt/handmade-lists.odt | `ad4aeb6e7d68597fadf18dabc9af71929a972d7707d9be4f59fd60c2e1632a59` |
| odt/handmade-manifestcomment.odt | `8f702808b8d054b0eb2eb607d40555446a8a6f8b42fa417e6cb7bb1dd6c152c6` |
| odt/handmade-math.odt | `a3878b670d612f67a3cf600fa2130ce1b49f833ad32b557064b0e2d3f4f42019` |
| odt/text.odt | `614b107dcd9f48364b33fa98ad1db32b43667701adeb510b081961ced54438ee` |
| ppt/handmade-multimaster.ppt | `7f4ccf9350fb4b366f2638f1d1b54bd9f88bd1e7492141df02d709b001ae2026` |
| ppt/handmade-sparsenotes.ppt | `a73a3ec85cba14e7b9adac569bb324dca8b6efc061a5ead677cb17c37999f2fe` |
| ppt/pres.ppt | `8b92d2304598dafc977ff309095aa405151ac741d3374b3d5a214377b57765b3` |
| pptx/handmade-altpath.pptx | `1f364b417eb31799d7fdfabe00af8e15a689a535fb2ec40dd1aadd38654bbfdf` |
| pptx/handmade-inherit.pptx | `4bbab27a28b2532eaa3ad93b86d2667e454f6d40bbf4b178524de3f0a0d0bfe3` |
| pptx/handmade-links.pptx | `32cc7652099218a1f15190615607697583afb5a76c9fffc2d2dea23b29c66533` |
| pptx/handmade-math.pptx | `b95ba383c185208fd171a86f38f0a9ee01e00e5fc72dcdace82c46f844753d4c` |
| pptx/handmade-order.pptx | `b05bc15dd2b901373e566e938d988fa387aa27d9f03c0fc5918097fef1652a07` |
| pptx/handmade-strict.pptx | `8890634f199ede31676cc86ad08db97240fe31eefa3fcf49012466aaa8c53d4b` |
| pptx/pres.pptx | `c96aa52da19f273f602040490203d9319872f512707e1a6c5a3fc53251b6d050` |
| rtf/handmade-bin.rtf | `7b80921eb53bb1a6a51cf158462e3c5cf1ce7cde17781403a0dce4caecddd61a` |
| rtf/handmade-blockstyle.rtf | `e89c59df03996369f284858eddb91865475a909296ac5d26b2a41480980f092e` |
| rtf/handmade-cocoa.rtf | `0693882b8e82c2530d2673b805524188b4896f7525f43f8ff7613d4c676fc28e` |
| rtf/handmade-math.rtf | `2c344424fdfdfb5d9a9f49554feb60b9adc7dc4ba21a08a478ad3dac91e7368d` |
| rtf/handmade-merge.rtf | `c7f4945fe4211ed8eac5bde266647c470a0a7e1e24bc56fabf02659908b2ebe8` |
| rtf/text.rtf | `8af25d8d79f898c5fd8c65782a6b73b04b4869fe621c6d39bf201ce4478ec731` |
| xls/sheet.xls | `1b3fc8f35f4c7ad6bb4dcf9b9f1fdf4ddf1f4c7f2f6748f33b7948204f940136` |
| xlsb/handmade-sheet.xlsb | `6f965c3d4c9c9d4028c8e2bb8409c28473cd6310c4af05266cf54f4852f94fbf` |
| xlsx/handmade-merged.xlsx | `70b735ffc96268b3856348d8fa226f6c8b91e0c52ae1abdbc9f1bf7e34576d53` |
| xlsx/sheet.xlsx | `ddfec7c1e98c7b50611b1c3ac55c0aa0d9d413135aa7afc36732338e44f4d26c` |

升级 anydoc 依赖时如需刷新语料：重取对应 commit 的 fixtures、重算 sha256 更新本表、`cargo test --test smoke --test corpus` 全绿即收（快照漂移即行为漂移，须人工审）。
