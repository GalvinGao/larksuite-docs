---
document_id: '6967331158355099654'
directory_id: '6956134701804355590'
title: 向单个范围写入数据
full_path: /ukTMukTMukTM/uAjMzUjLwIzM14CMyMTN
breadcrumb:
- Server API
- Docs
- Sheets
- Cell
- Write Data to a Single Range
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:19Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uAjMzUjLwIzM14CMyMTN
---

# 向单个范围写入数据

该接口用于根据 spreadsheetToken 和 range 向单个范围写入数据，若范围内有数据，将被更新覆盖；单次写入不超过5000行，100列，每个格子不超过5万字符。
## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values |
| HTTP Method | PUT |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag> 或 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |

### 路径参数
|参数|类型|必须|说明|
|--|-----|--|----|
|spreadsheetToken|string|是|spreadsheet的token，获取方式见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)| URL PATH|
### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|valueRange||是|值与范围|
|&emsp;∟range|string|是|更新范围，包含 sheetId 与单元格范围两部分，目前支持三种索引方式，详见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)，range所表示的范围需要大于等于values占用的范围|
|&emsp;∟values|array<array<interface>>|是|需要写入的值，如要写入公式、超链接、email、@人等，可详看附录[sheet 支持写入数据类型](/document/ukTMukTMukTM/ugjN1UjL4YTN14CO2UTN)|
### 请求体示例
```json
{
"valueRange":{
    "range": "string",
    "values": [
      [
        "string", 1 ,"http://www.xx.com"
      ]
    ]
    }
}
```
  
###  cURL 请求示例
```
curl --location --request PUT 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/values' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
"valueRange":{
    "range": "Q7PlXT!A1:B2",
    "values": [
      [
        "Hello", 1
      ],
      [
        "World", 1
      ]
    ]
    }
}'
```
  
## 响应
### 响应体
  |参数|类型|说明|
|--|-----|--|
|spreadsheetToken|string |spreadsheet 的 token |
|updatedRange|string |写入的范围|
|updatedRows|int|写入的行数|
|updatedColumns|int|写入的列数|
|updatedCells|int|写入的单元格总数|
|revision|int|sheet 的版本号|
### 响应体示例  
```json
{
    "code": 0,
    "data": {
        "revision": 0,
        "spreadsheetToken": "***",
        "updatedCells": 0,
        "updatedColumns": 0,
        "updatedRange": "***",
        "updatedRows": 0
    },
    "msg": "Success"
}

```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
