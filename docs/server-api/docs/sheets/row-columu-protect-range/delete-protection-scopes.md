---
document_id: '6967331158355165190'
directory_id: '6956134701804306438'
title: 删除保护范围
full_path: /ukTMukTMukTM/uYTM5YjL2ETO24iNxkjN
breadcrumb:
- Server API
- Docs
- Sheets
- Row Columu - Protect Range
- Delete Protection Scopes
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:05Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYTM5YjL2ETO24iNxkjN
---

# 删除保护范围

该接口用于根据保护范围ID删除保护范围，最多支持同时删除10个ID。
## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_del |
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
| spreadsheetToken | string        | 是   | sheet 的 token，获取方式见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview) |
### 请求体
| 参数             | 类型          | 必须 | 说明&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;                           |
| ---------------- | ------------- | ---- | ------------------------------------------------------------ |
| protectIds       | array<string> | 是   | 需要删除的保护范围ID，可以通过[获取表格元数据](/document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN)接口获取     |
### 请求体示例

```json
{
    "protectIds": ["******"]
}
```
###  cURL 请求示例
```
curl --location --request DELETE 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/protected_range_batch_del' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
    "protectIds": ["6947942538267541505","6946456074476339204"]
}'
```
## 响应
### 响应体
  | 参数          |类型| 说明                 |
| ------------- |-----| -------------------- |
| delProtectIds |array<string>| 成功删除的保护范围ID |

### 响应体示例
```json
{
    "code": 0,
    "msg": "Success",
    "data": {
        "delProtectIds": [
            "******"
        ]
    }
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
