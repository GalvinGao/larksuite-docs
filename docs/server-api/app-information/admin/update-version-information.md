---
document_id: '7070695615567249414'
directory_id: '6907567266537291777'
title: 更新应用审核状态
full_path: /uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/patch
breadcrumb:
- Server API
- App Information
- Admin
- Update version information
document_type: ReferenceDocumentType
updated_at: 2022-03-03T02:30:14Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/patch
---

# 更新应用审核状态

通过接口来更新应用版本的审核结果：通过后应用可以直接上架；拒绝后则开发者可以看到拒绝理由，并在修改后再次申请发布。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=application&version=v6&resource=application.app_version&method=patch)

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
| HTTP URL | https://open.larksuite.com/open-apis/application/v6/applications/:app_id/app_versions/:version_id |
| HTTP Method | PATCH |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="application:application.app_version" desc="更新应用版本信息" support_app_types="custom" tags="">⁣更新应用版本信息</md-perm> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 应用 id<br>**示例值**："cli_9f3ca975326b501b" |
| <md-text type="field-name" >version_id</md-text> | <md-text type="field-type" >string</md-text> | 唯一标识应用版本的 ID<br>**示例值**："oav_d317f090b7258ad0372aa53963cda70d" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >user_id_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户 ID 类型<br>**示例值**："open_id"<br>**可选值有**：<br>- `open_id`：用户的 open id<br>- `union_id`：用户的 union id<br>- `user_id`：用户的 user id<br>**默认值**：`open_id`<br>**当值为 `user_id`，字段权限要求**：<br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |
| <md-text type="field-name" >operator_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 操作者的 open_id<br>**示例值**："ou_4065981088f8ef67a504ba8bd6b24d85" |
| <md-text type="field-name" >reject_reason</md-text> | <md-text type="field-type" >string</md-text> | 否 | 当修改版本状态为被驳回时，这一项必填<br>**示例值**："拒绝理由" |




### 请求体
:::html
引用类型：<md-text type="field-type" >application.app_version</md-text>
:::

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >status</md-text> | <md-text type="field-type" >int</md-text> | 否 | 版本状态<br>**示例值**：1<br>**可选值有**：<br>- `0`：未知状态<br>- `1`：审核通过<br>- `2`：审核拒绝<br>- `3`：审核中<br>- `4`：未提交审核 |




### 请求体示例

```json
{
    "status": 1
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success"
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 210503 | invalid app_id | 请检查请求路径中的 app_id 是否合法 |
| 400 | 210504 | no such app in tenant | 请检查被查询应用与当前调用接口应用是否在同一企业内 |
| 400 | 210505 | target app not a custom app | 请检查被查询应用是否是自建应用 |
| 400 | 210506 | no such app | 请检查请求路径中的 app_id 是否存在 |
| 400 | 210507 | no such user_id | 请检查传入的用户 id 是否存在 |
| 400 | 211002 | no such version_id | 请检查路径中的 version_id 是否合法 |
| 400 | 211003 | no such version of desired app | 请检查 version_id 是否属于 app_id 对应应用 |
| 400 | 211004 | no authority for quota limit | 检查是否是企业版/旗舰版租户 |
| 400 | 211005 | invalid app id | 检查 app id |
| 400 | 211006 | invalid department id | 检查 department id |





