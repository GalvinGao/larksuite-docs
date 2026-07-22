---
document_id: '6967331173081448453'
directory_id: '7072290825036939270'
title: 操作工作表
full_path: /ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
breadcrumb:
- Server API
- Docs
- Sheets
- sheet
- Operate Sheets
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:18Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
---

# 操作工作表


该接口用于根据 spreadsheetToken 操作表格，如增加工作表，复制工作表、删除工作表。
::: note
该接口和 [更新工作表属性](/document/ukTMukTMukTM/ugjMzUjL4IzM14COyMTN) 的请求地址相同，但参数不同，调用前请仔细阅读文档。
:::

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update |
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
|参数|类型|必须|说明
|--|-----|--|----|
|requests| |是|请求操作，支持增、删、复制工作表 ，三个操作选一个| 
|&emsp;∟addSheet| |否|增加工作表 | 
|&emsp;&emsp;∟properties| |是| 工作表属性| 
|&emsp;&emsp;&emsp;∟title|string |是| 工作表标题| 
|&emsp;&emsp;&emsp;∟index|int |否|新增工作表的位置，不填默认往前增加工作表| 请求 body|
|&emsp;∟copySheet| |否|复制工作表| 
|&emsp;&emsp;∟source| |是|需要复制的工作表资源| 
|&emsp;&emsp;&emsp;∟sheetId|string |是|源 sheetId | 
|&emsp;&emsp;∟destination| |是|工作表 的属性| 
|&emsp;&emsp;&emsp;∟title|string |否|目标工作表名称。不填为 old_title(副本_0)| 
|&emsp;∟deleteSheet| |否|删除 sheet| 
|&emsp;&emsp;∟sheetId| |是| sheetId| 

### 请求体示例  
```json
{
  "requests": [
    {
      "addSheet": {
        "properties": {
          "title": "string",
          "index": 0
        }
      }
    },
    {
      "copySheet": {
        "source": {
          "sheetId": "string"
        },
        "destination": {
          "title": "string"
        }
      }
    },
    {
      "deleteSheet": {
        "sheetId": "string"
      }
    }
  ]
}

```
### cURL 请求示例
```
curl --location --request POST 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/sheets_batch_update' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
  "requests": [
    {
      "addSheet": {
        "properties": {
          "title": "Title 1",
          "index": 1
        }
      }
    },
    {
      "copySheet": {
        "source": {
          "sheetId": "0b6377"
        },
        "destination": {
          "title": "Title 2"
        }
      }
    },
    {
      "deleteSheet": {
        "sheetId": "l8Gdub"
      }
    }
  ]
}
'
```
## 响应  

### 响应体
|参数|类型|说明|
|--|-----|--|
|replies| |返回本次相关操作工作表的结果|
|&emsp;∟addSheet/copySheet| |增加/复制工作表的属性|
|&emsp;&emsp;∟properties|string| 表格属性|
|&emsp;&emsp;&emsp;∟sheetId|string| sheetId|
|&emsp;&emsp;&emsp;∟title|string| 工作表标题|
|&emsp;&emsp;&emsp;∟index|int| 工作表位置|
|&emsp;∟deleteSheet| |删除工作表|
|&emsp;&emsp;∟result|bool|删除工作表是否成功|
|&emsp;&emsp;∟sheetId|string| sheetId|

### 响应体示例  
```json
{
  "code": 0,
  "msg": "Success",
  "data": {
    "replies": [
      {
        "addSheet": {
          "properties": {
            "sheetId": "string",
            "title": "string",
            "index": 0
          }
        },
        "copySheet": {
          "properties": {
            "sheetId": "string",
            "title": "string",
            "index": 0
          }
        },
        "deleteSheet": {
          "result": true,
          "sheetId": "string"
        }
      }
    ]
  }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)  



