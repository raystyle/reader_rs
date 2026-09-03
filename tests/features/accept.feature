Feature: reader CLI 验收（对外契约与需求口径的可机检部分）

  Scenario: 版本号与 Cargo.toml 一致
    When 执行 "--version"
    Then 退出码为 0
    And 版本与清单一致

  Scenario: agent 发现面可用
    When 执行 "--llms"
    Then 退出码为 0
    And 标准输出非空

  Scenario: 搜索命中退出 0
    When 执行 "search README.md reader"
    Then 退出码为 0

  Scenario: 搜索无命中退出 1
    When 执行 "search README.md zz-绝不存在的词-zz"
    Then 退出码为 1

  Scenario: 文件不存在退出 2
    When 执行 "search 不存在的文件.pdf x"
    Then 退出码为 2

  Scenario: 扫描件给 needs_ocr 提示
    When 执行 "extract tests/ab/assets/scan-cjk.pdf"
    Then 退出码为 0
    And 标准输出包含 "[needs_ocr"

  Scenario: JSON 包膜字段齐备
    When 执行 "search README.md reader --format json"
    Then 退出码为 0
    And 包膜字段齐备

  Scenario: query 对 markdown 可用
    When 执行 "query README.md .h1"
    Then 退出码为 0
    And 标准输出非空
