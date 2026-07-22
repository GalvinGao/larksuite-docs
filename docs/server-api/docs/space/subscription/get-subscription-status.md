---
document_id: '7073817463670931462'
directory_id: '7072190414392475654'
title: 获取订阅状态
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-subscription/get
breadcrumb:
- Server API
- Docs
- Space
- Subscription
- Get subscription status
document_type: ReferenceDocumentType
updated_at: 2022-03-11T12:21:27Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-subscription/get
---

# 获取订阅状态

根据订阅ID获取该订阅的状态{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=file.subscription&method=get)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="tip">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/v1/files/:file_token/subscriptions/:subscription_id |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >file_token</md-text> | <md-text type="field-type" >string</md-text> | 文档token<br>**示例值**："doxcnxxxxxxxxxxxxxxxxxxxxxx" |
| <md-text type="field-name" >subscription_id</md-text> | <md-text type="field-type" >string</md-text> | 订阅关系ID<br>**示例值**："1234567890987654321" |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >subscription_id</md-text> | <md-text type="field-type" >string</md-text> | 否 | 订阅关系ID<br>**示例值**："1234567890987654321" |
| <md-text type="field-name" >subscription_type</md-text> | <md-text type="field-type" >string</md-text> | 否 | 订阅类型<br>**示例值**："comment_update"<br>**可选值有**：<br>- `comment_update`：评论更新 |
| <md-text type="field-name" >is_subcribe</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 是否订阅<br>**示例值**：true |
| <md-text type="field-name" >file_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 文档类型<br>**示例值**："doc"<br>**可选值有**：<br>- `doc`：文档<br>- `docx`：文档2.0<br>- `wiki`：知识库wiki |




### 请求体示例

```json
{"file_type":"docx"}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >subscription</md-text> | <md-text type="field-type" >file.subscription</md-text> | 文档订阅信息 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >subscription_id</md-text> | <md-text type="field-type" >string</md-text> | 订阅关系ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >subscription_type</md-text> | <md-text type="field-type" >string</md-text> | 订阅类型<br>**可选值有**：<br>- `comment_update`：评论更新 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_subcribe</md-text> | <md-text type="field-type" >boolean</md-text> | 是否订阅 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >file_type</md-text> | <md-text type="field-type" >string</md-text> | 文档类型<br>**可选值有**：<br>- `doc`：文档<br>- `docx`：文档2.0<br>- `wiki`：知识库wiki |




### 响应体示例

```json
{
    "code": 0,
    "data": {
        "file_type": "docx",
        "is_subcribe": false,
        "subscription_id": "xxxxxxxx",
        "subscription_type": "comment_update"
    },
    "msg": "success"
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1064000 | Illegal parameter | 检查参数有效性 |
| 403 | 1064030 | Permission denied | 检查文档权限，订阅评论至少需要阅读的权限 |
| 404 | 1064040 | Token not exist | 检查文档是否能正常访问 |
| 500 | 1065000 | Internal Server Error | 重试，若稳定失败请联系相关业务方oncall人员 |





