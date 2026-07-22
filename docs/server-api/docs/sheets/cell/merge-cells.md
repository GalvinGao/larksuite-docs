---
document_id: '6967331158356017158'
directory_id: '6956134701804355590'
title: 合并单元格
full_path: /ukTMukTMukTM/ukDNzUjL5QzM14SO0MTN
breadcrumb:
- Server API
- Docs
- Sheets
- Cell
- Merge Cells
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:32Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukDNzUjL5QzM14SO0MTN
---

# 合并单元格


该接口用于根据 spreadsheetToken 和维度信息合并单元格；单次操作不超过5000行，100列。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/merge_cells |
| HTTP Method | POST |
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
|spreadsheetToken|string|是|spreadsheet 的 token，获取方式见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)|
### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|range|string|是|查询范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见 [在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)|
|mergeType|string|是|可选三个类型，"MERGE_ALL"  将所选区域直接合并、"MERGE_ROWS"  将所选区域按行合并、"MERGE_COLUMNS"  将所选区域按列合并响应| 
### 请求体示例
```json
{
        "range": "string", 
        "mergeType": "string"
}
```
### cURL  请求示例
```
curl --location --request POST 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/merge_cells' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
        "range": "Q7PlXT!F11:G12", 
        "mergeType": "MERGE_ROWS"
}'
```
## 响应
### 响应体
 |参数|类型|说明|
|--|-----|--|
|spreadsheetToken|string |spreadsheet 的 token|
### 响应体示例
```json
{
    "code": 0,
    "data": {
        "spreadsheetToken": "***"
    },
    "msg": "Success"
}

```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
