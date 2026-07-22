---
document_id: '6967331173082087429'
directory_id: '7031445675032182789'
title: 转移拥有者
full_path: /ukTMukTMukTM/uQzNzUjL0czM14CN3MTN
breadcrumb:
- Server API
- Docs
- Permission
- Member
- Transfer Ownership
document_type: GuideDocumentType
updated_at: 2022-03-11T12:21:16Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uQzNzUjL0czM14CN3MTN
---

# 转移拥有者

该接口用于根据文档信息和用户信息转移文档的所有者。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive_permission&version=v1&resource=member&method=transfer)
## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/permission/member/transfer |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:file" desc="上传、下载文件到云空间" support_app_types="custom,isv" tags="">上传、下载文件到云空间</md-perm><br><md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理表格</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag> 或 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|token|string|是|文件的 token，获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction) |
|type|string|是|文档类型  "doc"  or  "sheet" or "bitable"  or "file"|
|owner||是|要转移到的新的文档所有者|
|&ensp;∟member_type|string|是|用户类型，可选 **email、openid、userid、unionid**|
|&ensp;∟member_id|string|是|用户类型下的值，获取方式见 [如何获得 User ID、Open ID 和 Union ID？](/document/home/user-identity-introduction/how-to-get) |
|remove_old_owner|bool|否|true 为转移后删除旧 owner 的权限，默认为false|
|cancel_notify|bool|否|true为不通知新owner，默认为false|
### 请求体示例
```json
{
    "type": "string", // "doc" or "sheet" or "file",
    "token": "string",
    "owner": {  
         "member_type": "openid",
         "member_id": "string"
    },
   "remove_old_owner": false,
   "cancel_notify": false
}
```
## 响应
### 响应体
|参数|说明|
|--|--|
|is_success|请求是否成功| 
|type|文档类型 "doc" or "sheet" or "file"| 
|token|文档的 token| 
|owner|文档当前所有者| 
|&ensp;∟member_type|用户类型，有 **email、openid、userid、unionid**| 
|&ensp;∟member_id|用户类型下的值| 
### 响应体示例 
```json
{
    "code": 0,
    "msg": "Success",
    "data": {
        "is_success": true,
        "token": "string",
        "type": "doc",
        "owner": {
            "member_type": "openid",
            "member_id": "string"
        }
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
