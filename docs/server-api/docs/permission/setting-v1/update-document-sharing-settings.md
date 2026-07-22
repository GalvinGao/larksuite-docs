---
document_id: '7031483915610570757'
directory_id: '7031445675029594117'
title: 更新文档公共设置
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public/patch
breadcrumb:
- Server API
- Docs
- Permission
- Setting v1
- Update document sharing settings
document_type: ReferenceDocumentType
updated_at: 2022-03-13T13:40:21Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public/patch
---

# 更新文档公共设置

该接口用于根据 filetoken 更新文档的公共设置。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=permission.public&method=patch)

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
| HTTP URL | https://open.larksuite.com/open-apis/drive/v1/permissions/:token/public |
| HTTP Method | PATCH |
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
| <md-text type="field-name" >type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 权限客体类型，放于query参数中，如：`?type=doc`<br>**示例值**："doc"<br>**可选值有**：<br>- `doc`：文档<br>- `sheet`：电子表格<br>- `file`：云空间文件<br>- `wiki`：知识库节点（部分支持）<br>- `bitable`：多维表格<br>- `docx`：文档（暂不支持） |


::: note
**提示** `wiki`: 知识库节点 暂不支持以下设置：
- `external_access`: 是否允许分享到租户外开关
- `share_entity`: 谁可以添加和管理协作者
- `invite_external`: 非所有权限者/所有者是否允许邀请外部人
- `link_share_entity`: 链接共享
  - `tenant_readable`: 获得链接的任何人可阅读
  - `tenant_editable`: 获得链接的任何人可编辑
:::

### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >external_access</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 是否允许分享到租户外开关<br>**示例值**：true |
| <md-text type="field-name" >security_entity</md-text> | <md-text type="field-type" >string</md-text> | 否 | 可创建副本/打印/导出/复制设置<br>**示例值**："anyone_can_view"<br>**可选值有**：<br>- `anyone_can_view`：所有可访问此文档的用户<br>- `anyone_can_edit`：有编辑权限的用户 |
| <md-text type="field-name" >comment_entity</md-text> | <md-text type="field-type" >string</md-text> | 否 | 可评论设置<br>**示例值**："anyone_can_view"<br>**可选值有**：<br>- `anyone_can_view`：所有可访问此文档的用户<br>- `anyone_can_edit`：有编辑权限的用户 |
| <md-text type="field-name" >share_entity</md-text> | <md-text type="field-type" >string</md-text> | 否 | 谁可以添加和管理协作者<br>**示例值**："anyone"<br>**可选值有**：<br>- `anyone`：所有可阅读或编辑此文档的用户<br>- `same_tenant`：组织内所有可阅读或编辑此文档的用户<br>- `only_full_access`：只有所有权限者可以 |
| <md-text type="field-name" >link_share_entity</md-text> | <md-text type="field-type" >string</md-text> | 否 | 链接共享<br>**示例值**："tenant_readable"<br>**可选值有**：<br>- `tenant_readable`：组织内获得链接的人可阅读<br>- `tenant_editable`：组织内获得链接的人可编辑<br>- `anyone_readable`：获得链接的任何人可阅读（仅`external_access=true`时有效）<br>- `anyone_editable`：获得链接的任何人可编辑（仅`external_access=true`时有效）<br>- `closed`：关闭链接分享 |
| <md-text type="field-name" >invite_external</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 非所有权限者/所有者是否允许邀请外部人<br>**示例值**：true |




### 请求体示例

```json
{
    "external_access": true,
    "security_entity": "anyone_can_view",
    "comment_entity": "anyone_can_view",
    "share_entity": "anyone",
    "link_share_entity": "tenant_readable",
    "invite_external": true
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >permission_public</md-text> | <md-text type="field-type" >permission_public</md-text> | 本次更新后的文档公共设置 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >external_access</md-text> | <md-text type="field-type" >boolean</md-text> | 是否允许分享到租户外开关 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >security_entity</md-text> | <md-text type="field-type" >string</md-text> | 可创建副本/打印/导出/复制设置<br>**可选值有**：<br>- `anyone_can_view`：所有可访问此文档的用户<br>- `anyone_can_edit`：有编辑权限的用户 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >comment_entity</md-text> | <md-text type="field-type" >string</md-text> | 可评论设置<br>**可选值有**：<br>- `anyone_can_view`：所有可访问此文档的用户<br>- `anyone_can_edit`：有编辑权限的用户 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >share_entity</md-text> | <md-text type="field-type" >string</md-text> | 谁可以添加和管理协作者<br>**可选值有**：<br>- `anyone`：所有可阅读或编辑此文档的用户<br>- `same_tenant`：组织内所有可阅读或编辑此文档的用户<br>- `only_full_access`：只有所有权限者可以 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >link_share_entity</md-text> | <md-text type="field-type" >string</md-text> | 链接共享<br>**可选值有**：<br>- `tenant_readable`：组织内获得链接的人可阅读<br>- `tenant_editable`：组织内获得链接的人可编辑<br>- `anyone_readable`：获得链接的任何人可阅读（仅`external_access=true`时有效）<br>- `anyone_editable`：获得链接的任何人可编辑（仅`external_access=true`时有效）<br>- `closed`：关闭链接分享 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >invite_external</md-text> | <md-text type="field-type" >boolean</md-text> | 非所有权限者/所有者是否允许邀请外部人 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "permission_public": {
            "external_access": true,
            "security_entity": "anyone_can_view",
            "comment_entity": "anyone_can_view",
            "share_entity": "anyone",
            "link_share_entity": "tenant_readable",
            "invite_external": true
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





