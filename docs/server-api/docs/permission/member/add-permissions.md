---
document_id: '7031483915610554373'
directory_id: '7031445675032182789'
title: 增加权限
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create
breadcrumb:
- Server API
- Docs
- Permission
- Member
- Add permissions
document_type: ReferenceDocumentType
updated_at: 2022-03-13T13:40:21Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create
---

# 增加权限

该接口用于根据 filetoken 给用户增加文档的权限。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=permission.member&method=create)

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
| HTTP URL | https://open.larksuite.com/open-apis/drive/v1/permissions/:token/members |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:file" desc="上传、下载文件到云空间" support_app_types="custom,isv" tags="">上传、下载文件到云空间</md-perm><br><md-perm name="wiki:wiki" desc="查看、编辑和管理知识库" support_app_types="custom,isv" tags="">查看、编辑和管理知识库</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >token</md-text> | <md-text type="field-type" >string</md-text> | 文件的 token，获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)<br>**示例值**："doccnBKgoMyY5OMbUG6FioTXuBe" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 文件类型，放于query参数中，如：`?type=doc`<br>**示例值**："doc"<br>**可选值有**：<br>- `doc`：文档<br>- `sheet`：电子表格<br>- `file`：云空间文件<br>- `wiki`：知识库节点（部分支持）<br>- `bitable`：多维表格<br>- `docx`：文档（暂不支持） |
| <md-text type="field-name" >need_notification</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 添加权限后是否通知对方<br>**注意：** 使用`tenant_access_token`访问不支持该参数<br>**示例值**：false<br>**默认值**：`false` |




### 请求体
:::html
引用类型：<md-text type="field-type" >member</md-text>
:::

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >member_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户类型，与请求体中的`member_id`要对应<br>**可选值有：**<br>- `email`: Lark邮箱<br>- `openid`: [开放平台ID](/document/home/user-identity-introduction/how-to-get)<br>- `openchat`: [开放平台群组](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)<br>- `opendepartmentid`:[开放平台部门ID](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview)<br>- `userid`:  [用户自定义ID](/document/home/user-identity-introduction/how-to-get)<br>**注意：** 使用`tenant_access_token`访问不支持添加`opendepartmentid`<br>**示例值**："openid" |
| <md-text type="field-name" >member_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户类型下的值<br>**示例值**："ou_7dab8a3d3cdcc9da365777c7ad535d62" |
| <md-text type="field-name" >perm</md-text> | <md-text type="field-type" >string</md-text> | 是 | 需要更新的权限，可选值有：<br>- `view`: 可阅读<br>- `edit`: 可编辑<br>- `full_access`: 所有权限<br>**示例值**："view" |




### 请求体示例

```json
{
    "member_type": "openid",
    "member_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
    "perm": "view"
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >member</md-text> | <md-text type="field-type" >member</md-text> | 本次添加权限的用户信息 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >member_type</md-text> | <md-text type="field-type" >string</md-text> | 用户类型，与请求体中的`member_id`要对应<br>**可选值有：**<br>- `email`: Lark邮箱<br>- `openid`: [开放平台ID](/document/home/user-identity-introduction/how-to-get)<br>- `openchat`: [开放平台群组](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)<br>- `opendepartmentid`:[开放平台部门ID](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview)<br>- `userid`:  [用户自定义ID](/document/home/user-identity-introduction/how-to-get)<br>**注意：** 使用`tenant_access_token`访问不支持添加`opendepartmentid` |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >member_id</md-text> | <md-text type="field-type" >string</md-text> | 用户类型下的值 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >perm</md-text> | <md-text type="field-type" >string</md-text> | 需要更新的权限，可选值有：<br>- `view`: 可阅读<br>- `edit`: 可编辑<br>- `full_access`: 所有权限 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "member": {
            "member_type": "openid",
            "member_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
            "perm": "view"
        }
    }
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1061001 | internal error | 服务内部错误，包括超时，错误码没处理。 |
| 403 | 1061002 | params error. | 请检查请求参数是否正确，如：`member_id`是否正确、协作者是否真实存在等。 |
| 400 | 1061003 | not found. | 请确认对应上传节点是否存在。 |
| 403 | 1061004 | forbidden. | 请确认当前身份是否有对应上传节点的的权限，如用户是否有上传到指定doc的编辑权限。 |
| 404 | 1061005 | auth failed. | 请使用正确身份访问该接口。 |
| 500 | 1066001 | Internal Error | 服务内部错误，包括超时，错误码没处理。 |
| 500 | 1066002 | Concurrency error, please retry | 服务内部错误，请重试。 |





