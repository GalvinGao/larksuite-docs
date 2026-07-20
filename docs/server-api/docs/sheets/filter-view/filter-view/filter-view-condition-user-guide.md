---
document_id: '7073817463670964230'
directory_id: '7072290825037004806'
title: 筛选视图的筛选条件指南
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/filter-view-condition-user-guide
breadcrumb:
- Server API
- Docs
- Sheets
- Filter view
- filter view
- Filter View Condition User Guide
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:40Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/filter-view-condition-user-guide
---

# 筛选视图的筛选条件使用指南

## 应用场景

筛选条件应用到筛选区域的某列用于展示特定数据

## 支持的接口

筛选视图的筛选条件目前提供5个接口，单个筛选视图只有一个筛选范围，筛选范围的第一行不参与筛选，筛选范围内的每一列都可以设置筛选条件。

1. 【[获取筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/get)】获取筛选视图的某列的筛选条件
2. 【[创建筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/create)】在筛选视图的某列创建筛选条件
3. 【[更新筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/update)】更新筛选视图的某列的筛选条件
4. 【[删除筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/delete)】删除筛选视图的某列的筛选条件
5. 【[查询筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/query)】查询筛选视图所有的筛选条件

## 筛选参数
### **filter_view_id**
什么是 filter_view_id?
filter_view_id 是一个筛选视图的唯一标识。可以通过以下方式获取：
1. 通过[获取筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/get)接口获取；
2. 进入一个筛选视图后，从链接中获取：
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d70575214389a7ebec394427787771aa_9eROZjWvTu.png)

### **range**

支持以下5种应用范围，详见[概述](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)：

1. sheetId ：表示整表应用
2. sheetId!1:2 ：表示整行应用
3. sheetId!A:B ：表示整列应用
4. sheetId!A1:B2 ：表示普通的range
5. sheetId!A1:C ：省略结束行，会使用表格的最后行作为结束行

### **condition_id**

在 range 范围内的列字母

### **filter_type**

支持如下4种筛选类型（filter_type），每种 filter_type 需要填写对用的比较类型（compare_type）和值（expected）

1. hiddenValue ：隐藏值筛选
2. number ：数字筛选
3. text ：文本筛选
4. color ：颜色筛选

***hiddenValue***

想要隐藏的值

compare_type ：不填

Expected ：要隐藏的值，数组长度大于0，小于筛选范围的行数，每个值字符长度不超过5w
```json
{
    "filter_type": "hiddenValue",
    "expected": ["100", "200", "300"]
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
    "filter_type": "number",
    "compare_type": "between",
    "expected": ["100", "200"]
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
    "filter_type": "text",
    "compare_type": "contains",
    "expected": ["abc"]
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
    "filter_type": "color",
    "compare_type": "backColor",
    "expected": ["#ffffff"]
}
```
