---
document_id: '6967331173081038853'
directory_id: '7031445675029594117'
title: 获取文档公共设置V2
full_path: /ukTMukTMukTM/uITM3YjLyEzN24iMxcjN
breadcrumb:
- Server API
- Docs
- Permission
- Setting v1
- Get Document Sharing Settings V2
document_type: GuideDocumentType
updated_at: 2022-03-11T12:21:19Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uITM3YjLyEzN24iMxcjN
---

# 获取文档公共设置V2

该接口用于根据 filetoken 获取文档的公共设置。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive_permission&version=v2&resource=public&method=get)
## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/permission/v2/public/ |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="drive:drive.metadata:readonly" desc="查看云空间中文件元数据" support_app_types="custom,isv" tags="">查看云空间中文件元数据</md-perm><br><md-perm name="drive:file" desc="上传、下载文件到云空间" support_app_types="custom,isv" tags="">上传、下载文件到云空间</md-perm><br><md-perm name="drive:file:readonly" desc="查看和下载云空间中的文件" support_app_types="custom,isv" tags="">查看和下载云空间中的文件</md-perm><br><md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm><br><md-perm name="docs:doc:readonly" desc="查看、评论和导出文档" support_app_types="custom,isv" tags="">查看、评论和导出文档</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理表格</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出表格" support_app_types="custom,isv" tags="">查看、评论和导出表格</md-perm> |

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
|token|string|是| 文件的 token，获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction) |
|type|string|是|文档类型 "doc", "sheet" or "isv"|
### 请求体示例
```json
{
        "token": "doccnBKgoMyY5OMbUG6FioTXuBe",
        "type": "doc"
}
```
## 响应
### 响应体
|参数|类型|说明|
|--|--|----|
|security_entity|string|可创建副本/打印/导出/复制设置：<br>"anyone_can_view" - 所有可访问此文档的用户<br>"anyone_can_edit" - 有编辑权限的用户|
|comment_entity|string|可评论设置：<br>"anyone_can_view" - 所有可访问此文档的用户<br>"anyone_can_edit" - 有编辑权限的用户| 
|share_entity|string|谁可以添加和管理协作者：<br>"anyone"-所有可阅读或编辑此文档的用户<br>"same_tenant"-组织内所有可阅读或编辑此文档的用户<br>"only_me"-只有我可以| 请求 body |
|link_share_entity|string|链接共享：<br>"tenant_readable" - 组织内获得链接的人可阅读<br>"tenant_editable" - 组织内获得链接的人可编辑<br>"anyone_readable" - 获得链接的任何人可阅读<br>"anyone_editable" - 获得链接的任何人可编辑| 
|external_access|bool|是否允许分享到租户外开关| 
|invite_external|bool|非owner是否允许邀请外部人| 
|permission_version|string|权限版本号|
### 响应体示例
```json
{
    "code": 0,
    "data": {
        "security_entity": "anyone_can_view",
        "comment_entity": "anyone_can_view",
        "share_entity":"only_me",
        "link_share_entity":"tenant_editable",
        "external_access":false,
        "invite_external":false，
        "permission_version":"1024",
    },
    "msg": "Success"
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
