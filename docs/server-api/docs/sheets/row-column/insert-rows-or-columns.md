---
document_id: '6967331173081989125'
directory_id: '7072290825036922886'
title: 插入行列
full_path: /ukTMukTMukTM/uQjMzUjL0IzM14CNyMTN
breadcrumb:
- Server API
- Docs
- Sheets
- Row Column
- Insert Rows or Columns
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:52Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uQjMzUjL0IzM14CNyMTN
---

# 插入行列

该接口用于根据 spreadsheetToken 和维度信息 插入空行/列。  
如 startIndex=3， endIndex=7，则从第 4 行开始开始插入行列，一直到第 7 行，共插入 4 行；单次操作不超过5000行或列。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/insert_dimension_range |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag> 或 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |



### 路径参数

| 参数 | 类型 | 描述 |
| --- | --- | --- |
| spreadsheetToken | string | spreadsheet 的 token，获取方式见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview) |



### 请求体  
|参数|类型|必须|说明|
|--|-----|--|----|
|dimension||是|需要插入行列的维度信息| 
|&emsp;∟sheetId|string|是|sheet 的 Id| 
|&emsp;∟majorDimension|string|否|默认 ROWS ，可选 ROWS、COLUMNS| 
|&emsp;∟startIndex|int|是|开始的位置|
|&emsp;∟endIndex|int|是|结束的位置|
|inheritStyle|string|否|BEFORE 或 AFTER，不填为不继承 style| 

### 请求体示例  
```json
{
    "dimension":{
        "sheetId":"string",
        "majorDimension":"ROWS",
        "startIndex":0,
        "endIndex":0
    },
    "inheritStyle":"BEFORE"
}
```
### cURL 请求示例
```
curl --location --request POST 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/insert_dimension_range' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
    "dimension":{
        "sheetId":"Q7PlXT",
        "majorDimension":"ROWS",
        "startIndex":3,
        "endIndex":5
    },
    "inheritStyle":"AFTER"
}'
```
## 响应  

### 响应体示例    

```json
{
    "code": 0,
    "msg": "Success",
    "data": {}
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


