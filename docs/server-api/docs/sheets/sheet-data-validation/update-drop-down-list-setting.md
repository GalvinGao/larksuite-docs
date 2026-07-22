---
document_id: '6967331158355197958'
directory_id: '6935677787475738626'
title: 更新下拉列表设置
full_path: /ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/update-datavalidation
breadcrumb:
- Server API
- Docs
- Sheets
- Sheet - Data Validation
- Update Drop-down List Setting
document_type: GuideDocumentType
updated_at: 2025-08-19T04:56:38Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/update-datavalidation
---

# 更新下拉列表设置

该接口根据 spreadsheetToken 、sheetId、dataValidationId 更新下拉列表的属性。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation/:sheetId |
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
|spreadsheetToken|string|是|spreadsheet 的 token，获取方式见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)| 
|sheetId|string|是|子sheet唯一识别参数



### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|ranges|array<string>|是|更新范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)。注意：如果该范围没有设置下拉列表，更新实际上会转换为设置下拉列表。
|dataValidationType|string|是|下拉列表填"list"|
|dataValidation|||下拉列表规则属性
|&emsp;∟conditionValues|array<string>|是|下拉列表选项值, 需为字符串,不能包含","，选项值最长100字符,选项个数最多500个|
|&emsp;∟options||否|可选属性| 
|&emsp;&emsp;∟multipleValues|bool|否|单选填false, 多选填true，不填默认为false| 
|&emsp;&emsp;∟highlightValidData|bool|否|是否设置颜色和胶囊样式, 不填默认为false|
|&emsp;&emsp;∟colors|array<string>|否|当highlightValidData为true时，color需填颜色,与conditionValues中的值一一对应。需是RGB16进制格式,如"#fffd00"| 

### 请求体示例

```json
{
    "ranges":["BzY8T5!A1:A2", "BzY8T5!B1:B1"],
    "dataValidationType":"list",
    "dataValidation":{
        "conditionValues":["1", "2", "4","2"],
        "options":{
            "multipleValues":false,
            "highlightValidData":true,
            "colors":["#1FB6C1", "#1006C2", "#FB16C3","#FFB6C1"]
        }
    }
}
```
###  cURL 请求示例
  ```
  curl --location --request PUT 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/dataValidation/BzY8T5' \
--header 'Authorization: Bearer t-5be16bd570d0437444c40d5e6b5584109e61b0b1' \
--header 'Content-Type: application/json' \
--data-raw '{
    "ranges":["BzY8T5!A1:A2", "BzY8T5!B1:B1"],
    "dataValidationType":"list",
    "dataValidation":{
        "conditionValues":["1", "2", "4"],
        "options":{
            "multipleValues":false,
            "highlightValidData":true,
            "colors":["#1FB6C1", "#1006C2", "#FB16C3"]
        }
    }
}'
  ```
 ## 响应

### 响应体
  
|参数|类型|必须|说明|
|--|-----|--|----|
|code|int|是|状态码，0代表成功|
|msg|string|否|状态信息|
|data||||
|&emsp; ∟spreadsheetToken|string|是|spreadsheet的token| 
|&emsp; ∟sheetId|string|是|工作表 sheet 的 id| 
|&emsp;∟dataValidation|||| 
|&emsp;&emsp;∟dataValidationType|string|是|下拉列表为"list"|
|&emsp;&emsp;∟conditionValues|array<string>|是|下拉列表选项值|
|&emsp;&emsp;∟options||否|可选属性| 
|&emsp;&emsp;&emsp;∟multipleValues|bool|否|单选填false, 多选填true| 
|&emsp;&emsp;&emsp;∟highlightValidData|bool|否|是否设置颜色和胶囊样式|
|&emsp;&emsp;&emsp;∟colorValueMap|map<string,string>|否|当highlightValidData为true时，colorValueMap的key与conditionValues中的值一一对应，value为对应的颜色参数。| 
  
### 响应体示例  

```json
{
    "code": 0,
    "data": {
        "dataValidation": {
            "conditionValues": [
                "1",
                "2",
                "4"
            ],
            "dataValidationType": "list",
            "options": {
                "colorValueMap": {
                    "1": "#1FB6C1",
                    "2": "#1006C2",
                    "4": "#FB16C3"
                },
                "highlightValidData": true,
                "multipleValues": false
            }
        },
        "sheetId": "yuNGtr",
        "spreadsheetToken": "shtbckBcolBlRfkcMVZbolMdADe"
    },
    "msg": "Success"
}
```  
  
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
