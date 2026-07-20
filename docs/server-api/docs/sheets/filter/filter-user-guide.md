---
document_id: '7073817463670702086'
directory_id: '7072290825036906502'
title: 筛选指南
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/filter-user-guide
breadcrumb:
- Server API
- Docs
- Sheets
- Filter
- Filter User Guide
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:36Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/filter-user-guide
---

# 数据筛选筛选使用指南

## 应用场景

为工作表添加筛选条件。

## 支持的接口

单个工作表最多只有一个筛选范围。你可以为筛选范围内的每一列都设置一个筛选条件，注意：筛选范围内的第一行不参与筛选。支持通过以下接口管理数据筛选条件：

1. 【[获取筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/get)】获取子表的筛选详细信息
2. 【[创建筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/create)】创建筛选
3. 【[更新筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/update)】更新筛选，必须已经创建了筛选才可以更新
4. 【[删除筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/delete)】删除筛选

## 筛选参数
### **filtered_out_rows**
筛选掉的行，会被隐藏（获取筛选的时候会返回）

### **range**

支持以下5种应用范围，详见[概述](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)：

1. sheetId ：表示整表应用
2. sheetId!1:2 ：表示整行应用
3. sheetId!A:B ：表示整列应用
4. sheetId!A1:B2 ：表示普通的range
5. sheetId!A1:C ：省略结束行，会使用表格的最后行作为结束行

### **col**

在 range 范围内的列字母。

### **filter_type**

支持如下5种筛选类型（filter_type），每种 filter_type 需要填写对应的比较类型（compare_type）和值（expected）

1. multiValue ：多值筛选
2. number ：数字筛选
3. text ：文本筛选
4. color ：颜色筛选
5. clear ：清除某列的筛选条件

***multiValue***

多值筛选

compare_type ：不填

Expected ：要展示的值，数组长度大于0，小于筛选范围的行数，每个值字符长度不超过5w
```json
{
  "condition": {
    "filter_type": "multiValue",
    "expected": ["100", "200", "300"]
  }
}
```
***number***

数字筛选

| compare_type   | expected 长度 | 备注        |
| -------------- | ----------- | --------- |
| equal          | 1           | 数字筛选：等于   |
| notEqual       | 1           | 数字筛选：不等于  |
| greater        | 1           | 数字筛选：大于   |
| greaterOrEqual | 1           | 数字筛选：大于等于 |
| less           | 1           | 数字筛选：小于   |
| lessOrEqual    | 1           | 数字筛选：小于等于 |
| between        | 2           | 数字筛选：介于   |
| notBetween     | 2           | 数字筛选：不介于  |
```json
{
  "condition": {
    "filter_type": "number",
    "compare_type": "equal",
    "expected": ["100"]
  },
  "condition": {
    "filter_type": "number",
    "compare_type": "between",
    "expected": ["100", "200"]
  }
}
```
***text***

文本筛选
字符长度不超过1000

| compare_type     | expected 长度 | 备注        |
| ---------------- | ----------- | --------- |
| beginsWith       | 1           | 文本筛选：开头是  |
| notBeginsWith | 1           | 文本筛选：开头不是 |
| endsWith         | 1           | 文本筛选：结尾是  |
| notEndsWith   | 1           | 文本筛选：结尾不是 |
| contains         | 1           | 文本筛选：包含   |
| notContains   | 1           | 文本筛选：不包含  |
```json
{
  "condition": {
    "filter_type": "text",
    "compare_type": "contains",
    "expected": ["abc"]
  }
}
```
***color***

颜色筛选

| compare_type | expected 长度 | 备注         |
| ------------ | ----------- | ---------- |
| backColor    | 1           | 颜色筛选：单元格颜色 |
| foreColor    | 1           | 颜色筛选：字体颜色  |
```json
{
  "condition": {
    "filter_type": "color",
    "compare_type": "backColor",
    "expected": ["#ffffff"]
  }
}
```
***clear***

清除筛选，清除某一列的筛选条件，compare_type 和 expected 不用填
```json
{
  "condition": {
    "filter_type": "clear",
  }
}
```
