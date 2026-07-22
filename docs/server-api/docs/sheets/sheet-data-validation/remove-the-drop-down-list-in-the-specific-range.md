---
document_id: '6967331173081677829'
directory_id: '6935677787475738626'
title: 删除下拉列表设置
full_path: /ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/delete-datavalidation
breadcrumb:
- Server API
- Docs
- Sheets
- Sheet - Data Validation
- Remove the Drop-down List in the Specific Range
document_type: GuideDocumentType
updated_at: 2025-08-19T04:56:45Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/delete-datavalidation
---

# 删除下拉列表设置

该接口根据 spreadsheetToken 、range 移除选定数据范围单元格的下拉列表设置，但保留选项文本。单个删除范围不超过5000单元格。单次请求range最大数量100个。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation |
| HTTP Method | DELETE |
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
|dataValidationRanges|array||范围数组，每个range 最大单元格数量5000，每个range独立执行，一个range的失败不影响其他range的执行。返回结果会返回每个range的执行结果
|&emsp;∟range|string|是|删除范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)| 

### 请求体示例

```json
{
    "dataValidationRanges":[
        {
            "range": "4d30c6!A10:C10"
        }
    ]
}
```
### cURL 请求示例
  ```
  curl --location --request DELETE 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/dataValidation' \
--header 'Authorization: Bearer t-ce3540c5f02ac074535f1f14d64fa90fa49621c0' \
--header 'Content-Type: application/json' \
--data-raw '{
    "dataValidationRanges":[
        {
            "range": "BzY8T5!A1:A1"
        }
    ]
}'
  ```
 ## 响应

### 响应体
  
|参数|类型|必须|说明|
|--|-----|--|----|
|code|int|是|状态码，0代表成功|
|msg|string|否|状态信息|
|data||||
|&emsp;∟rangeResults|array|||
|&emsp;&emsp;∟range|string|是|执行的range,与请求入参中的range 对应|
|&emsp;&emsp;∟msg|string|否|结果信息|
|&emsp;&emsp;∟success|bool|是|执行结果|
|&emsp;&emsp;∟updatedCells|int|是|影响的单元格数量|

  
### 响应体示例  

```json
{
    "code": 0,
    "data": {
        "rangeResults": [
            {
                "msg": "",
                "range": "4d30c6!A10:C10",
                "success": true,
                "updatedCells": 3
            }
        ]
    },
    "msg": "Success"
}
```  
  
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
