---
document_id: '7063034427644297222'
directory_id: '6956134701804322822'
title: 电子表格常见问题
full_path: /ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/guide/sheets-faq
breadcrumb:
- Server API
- Docs
- Sheets
- Guide
- sheets-faq
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:06Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/guide/sheets-faq
---

#  电子表格常见问题
##  名词解释
####  DateTimeRenderOption
在获取表格数据时，作为请求参数指定单元格数据类型为日期，时间，时间日期的返回值格式。
<br><br>参数若缺省时间日期会返回一个浮点数值，整数部分计算自1899年12月30日以来的天数，小数部分将时间计算为一天的一小部分。例如：1900年1月1日中午12点是2.5，2是因为它是1899年12月30日之后的2 天，0.5是因为12/24=0.5。

| 参数值         | 解释               |
| --------- | --------------- | -------   | ----------- | --------- | 
|FormattedString | 计算并将时间日期按照其格式进行格式化，但不会对数字进行格式化，返回格式化后的字符串 | 

## 接口限流
| 接口               | 限流值                                         |
| ------------------ | ---------------------------------------------- |
| [创建表格](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create) |每个应用每个租户20次/分钟                      |
| [获取表格元数据](/document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN) | 每个应用每个租户100次/秒                       |
| [更新表格属性](/document/ukTMukTMukTM/ucTMzUjL3EzM14yNxMTN)  | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [操作工作表](/document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN)   | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [更新工作表属性](/document/ukTMukTMukTM/ugjMzUjL4IzM14COyMTN) | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [移动行列](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/move_dimension)    | 每个应用每个租户100次/分钟                     |
| [插入数据](/document/ukTMukTMukTM/uIjMzUjLyIzM14iMyMTN)    | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [追加数据](/document/ukTMukTMukTM/uMjMzUjLzIzM14yMyMTN)    | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [插入行列](/document/ukTMukTMukTM/uQjMzUjL0IzM14CNyMTN)    | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [增加行列](/document/ukTMukTMukTM/uUjMzUjL1IzM14SNyMTN)    | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [更新行列](/document/ukTMukTMukTM/uYjMzUjL2IzM14iNyMTN)    | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [删除行列](/document/ukTMukTMukTM/ucjMzUjL3IzM14yNyMTN)    | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [读取单个范围](/document/ukTMukTMukTM/ugTMzUjL4EzM14COxMTN)  | 每个应用每个租户100次/秒                       |
| [读取多个范围](/document/ukTMukTMukTM/ukTMzUjL5EzM14SOxMTN)  | 每个应用每个租户100次/秒                       |
| [向单个范围写入数据](/document/ukTMukTMukTM/uAjMzUjLwIzM14CMyMTN)| 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [向多个范围写入数据](/document/ukTMukTMukTM/uEjMzUjLxIzM14SMyMTN)| 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [设置单元格样式](/document/ukTMukTMukTM/ukjMzUjL5IzM14SOyMTN) | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [批量设置单元格样式](/document/ukTMukTMukTM/uAzMzUjLwMzM14CMzMTN)| 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [合并单元格](/document/ukTMukTMukTM/ukDNzUjL5QzM14SO0MTN)   | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [拆分单元格](/document/ukTMukTMukTM/uATNzUjLwUzM14CM1MTN)   | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [写入图片](/document/ukTMukTMukTM/uUDNxYjL1QTM24SN0EjN)    | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [查找单元格](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/find)   | 每个应用每个租户100次/分钟                     |
| [替换单元格](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/replace)   | 每个应用每个租户20次/分钟                      |
| [创建条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-set)  | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [获取条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-get)  | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [更新条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-update)  | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [删除条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-delete)  | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [增加保护范围](/document/ukTMukTMukTM/ugDNzUjL4QzM14CO0MTN)  | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [获取保护范围](/document/ukTMukTMukTM/uQTM5YjL0ETO24CNxkjN)  | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [修改保护范围](/document/ukTMukTMukTM/uUTM5YjL1ETO24SNxkjN)  | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [删除保护范围](/document/ukTMukTMukTM/uYTM5YjL2ETO24iNxkjN)  | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [设置下拉列表](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/set-dropdown)  | 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [删除下拉列表设置](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/delete-datavalidation)| 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [更新下拉列表设置](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/update-datavalidation)| 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [查询下拉列表设置](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/query-datavalidation)| 每个应用每个租户100次/秒；单个文档只能串行调用 |
| [获取筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/get)    | 每个应用每个租户100次/分钟                     |
| [创建筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/create)    | 每个应用每个租户20次/分钟                      |
| [更新筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/update)    | 每个应用每个租户20次/分钟                      |
| [删除筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/delete)    | 每个应用每个租户100次/分钟                     |
| [创建筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/create)  | 每个应用每个租户100次/分钟                     |
| [获取筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/get)  | 每个应用每个租户100次/分钟                     |
| [查询筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/query)  | 每个应用每个租户100次/分钟                     |
| [更新筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/patch)  | 每个应用每个租户100次/分钟                     |
| [删除筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/delete)  | 每个应用每个租户100次/分钟                     |
| [创建筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/create)  | 每个应用每个租户100次/分钟                     |
| [获取筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/get)  | 每个应用每个租户100次/分钟                     |
| [查询筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/query)  | 每个应用每个租户100次/分钟                     |
| [更新筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/update)  | 每个应用每个租户100次/分钟                     |
| [删除筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/delete)  | 每个应用每个租户100次/分钟                     |
| [创建浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/create)  | 每个应用每个租户100次/分钟                     |
| [获取浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/get)  | 每个应用每个租户100次/分钟                     |
| [查询浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/query)  | 每个应用每个租户100次/分钟                     |
| [更新浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/patch)  | 每个应用每个租户100次/分钟                     |
| [删除浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/delete)  | 每个应用每个租户100次/分钟                     |

## 电子表格限制

| 限制项                 | 限制值                            |
| ---------------------- | --------------------------------- |
| 单个工作表总单元格数量 | 小于等于2000000个（包含空行空列） |
| 电子表格中工作表数量   | 小于等于300个                     |
| 文档中电子表格数量     | 小于等于1500个                    |
| 单个单元格字符数量限制 | 小于等于45000字符                 |
| 单个工作表列数         | 小于等于13000列                   |
