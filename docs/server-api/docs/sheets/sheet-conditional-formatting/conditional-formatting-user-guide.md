---
document_id: '6967331158355378182'
directory_id: '6931268519897120795'
title: 条件格式指南
full_path: /ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-guide
breadcrumb:
- Server API
- Docs
- Sheets
- Sheet - Conditional Formatting
- Conditional Formatting User Guide
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:21Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-guide
---

# 条件格式使用指南

## 应用场景

条件格式用于根据指定的条件更改单元格的外观

## 支持的接口

条件格式目前提供4个接口，单个子表内条件格式的数量限制为20

1. 【[获取条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-get)】获取子表的条件格式详细信息，单次请求子表数量限制为10
2. 【[创建条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-set)】 批量设置条件格式，单次请求设置条件格式数量限制为10
3. 【[更新条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-update)】批量更新已有的条件格式，单次请求更新条件格式数量限制为10
4. 【[删除条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-delete)】删除条件格式，单次请求删除条件格式数量限制为10

## 条件格式约束

### **ranges**

支持以下5种应用范围：

1. sheetId ：表示整表应用
2. sheetId!1:2 ：表示整行应用
3. sheetId!A:B ：表示整列应用
4. sheetId!A1:B2 ：表示普通的range
5. sheetId!A1:C ：省略结束行，会使用表格的最后行作为结束行

### **style**

仅支持以下几种样式，皆为可选项，不填默认不设置，但是必须至少要有以下的一种，即style必须有值

1. font: bold 类型为 bool，加粗；italic 类型为 bool，斜体
2. text_decoration: 类型 int，文本装饰 ，0 默认，1 下划线，2 删除线 ，3 下划线和删除线
3. fore_color: 类型 string，字体颜色
4. back_color: 类型 string，背景颜色

### **rule_type & attrs**

- rule_type 限制为7种，有4个 rule_type不需要提供 attrs 参数：***containsBlanks（为空）、notContainsBlanks（不为空）、duplicateValues（重复值）和uniqueValues（唯一值）*** 。还有 3 个 rule_type：***cellIs（限定值范围）、containsText（包含内容）、timePeriod（日期）*** 对应的 attrs 参数限制如下，后面有更详细的介绍
  - formula: string 数组，仅在 rule_type 为 ***cellIs*** 时才需填写，operator 为 between 和 notBetween 时需要填写两个元素，其他只需填一个元素，值为用户自定义。数字填写为 **"1"**，文本填写为 **"\"a\""**。
  - text: string，仅在 rule_type 为 ***containsText*** 时才需填写，值为用户自定义
  - timeperiod: string，仅在 rule_type 为 ***timePeriod*** 时才需填写，operator 目前只支持 is，time_period 字段限制为 yesterday、today、tomorrow和last7Days
- 用户自定义输入的内容长度限制为1000
- attrs参数为数组，目前数组中只需填一个



***3 个 rule_type 对应的 attrs 参数限制如下***

***cellIs（限定值范围）***

attrs参数中的formula为数组，以下formula1和formula2表示为formula数组中的元素

| operator           | formula1 | formula2 | 备注          |
| ------------------ | -------- | -------- | ----------- |
| equal              | 必填       |          | 限定值范围：等于    |
| notEqual           | 必填       |          | 限定值范围：不等于   |
| greaterThan        | 必填       |          | 限定值范围：大于    |
| greaterThanOrEqual | 必填       |          | 限定值范围：大于或等于 |
| lessThan           | 必填       |          | 限定值范围：小于    |
| lessThanOrEqual    | 必填       |          | 限定值范围：小于或等于 |
| between            | 必填       | 必填       | 限定值范围：介于    |
| notBetween         | 必填       | 必填       | 限定值范围：未介于   |
```json
{
  "condition_format": {
    "rule_type": "cellIs",
    "attrs": [
      {
        "operator": "equal",
        "formula": [
              "\"aaaaa\""  //文本需要用""包裹
        ]
      }
    ]
  },
  "condition_format": {
    "rule_type": "cellIs",
    "attrs": [
      {
        "operator": "between",
        "formula": [
              "1",
              "10"
        ]
      }
    ]
  }
}
```
***containsText（包含内容）***

| operator     | text | 备注           |
| ------------ | ---- | ------------ |
| containsText | 必填   | 包含以下内容：文本包含  |
| notContains  | 必填   | 包含以下内容：文本不包含 |
| is           | 必填   | 包含以下内容：文本为   |
| beginsWith   | 必填   | 包含以下内容：开头为   |
| endsWith     | 必填   | 包含以下内容：结尾为   |
```json
{
  "condition_format": {
    "rule_type": "containsText",
    "attrs": [
      {
        "operator": "is",
        "text": "******"
      }
    ]
  }
}
```
***timePeriod（日期）***

| operator | timePeriod | 备注       |
| -------- | ---------- | -------- |
| is       | yesterday  | 日期为：昨天   |
| is       | today      | 日期为：今天   |
| is       | tomorrow   | 日期为：明天   |
| Is       | last7Days  | 日期为：最近7天 |
```json
{
  "condition_format": {
    "rule_type": "timePeriod",
    "attrs": [
      {
        "operator": "is",
        "time_period": "today"
      }
    ]
  }
}
```
