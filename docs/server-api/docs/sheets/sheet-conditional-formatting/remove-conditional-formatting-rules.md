---
document_id: '6967331173082021893'
directory_id: '6931268519897120795'
title: 删除条件格式
full_path: /ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-delete
breadcrumb:
- Server API
- Docs
- Sheets
- Sheet - Conditional Formatting
- Remove Conditional Formatting Rules
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:33Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-delete
---

# 删除条件格式

该接口用于删除已有的条件格式，单次最多支持删除10个条件格式，每个条件格式的删除会返回成功或者失败，失败的情况包括各种参数的校验。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_delete |
| HTTP Method | DELETE |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag> 或 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |

### 路径参数
| 参数             | 类型          | 必须 | 说明&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;                           |
| ---------------- | ------------- | ---- | ------------------------------------------------------------ | 
| spreadsheetToken | string        | 是   | sheet 的 token，获取方式见 [在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)|
### 请求体
| 参数       |类型| 说明       |
| -------- |-----| -------- |
|sheet_cf_ids| |表格条件格式id|
| &emsp;∟sheet_id |string| sheet的id |
| &emsp;∟cf_id    |string| 条件格式id   |
### 请求体示例
```json
{
    "sheet_cf_ids": [
        {
            "sheet_id": "40a7b0",
            "cf_id": "6hP6Dj6gsd"
        }
    ]
}
```
###  cURL 请求示例
```
curl --location --request DELETE 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/condition_formats/batch_delete' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
    "sheet_cf_ids": [
        {
            "sheet_id": "Q7PlXT",
            "cf_id": "KjRm0JyS1P"
        }
    ]
}'
```
## 响应
### 响应体
| 参数       |类型| 说明                           |
| -------- |-----| ---------------------------- |
|responses|array<interface>|响应|
| &emsp;∟sheet_id |string | sheet的Id                     |
| &emsp;∟cf_id    |string| 条件格式id                       |
| &emsp;∟res_code |int| 条件格式删除状态码，0表示成功，非0表示失败       |
| &emsp;∟res_msg  |string| 条件格式删除返回的状态信息，空表示成功，非空表示失败原因 |
### 响应体示例
```json
{
    "code": 0,
    "data": {
        "responses": [
            {
                "cf_id": "6hP6Dj6gsd",
                "res_code": 555554047,
                "res_msg": "cfId not exist",
                "sheet_id": "40a7b0"
            }
        ]
    },
    "msg": "Success"
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
