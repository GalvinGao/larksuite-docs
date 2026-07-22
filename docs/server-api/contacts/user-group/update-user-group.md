---
document_id: '7055272807039483910'
directory_id: '7050040770682830854'
title: 更新用户组
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/patch
breadcrumb:
- Server API
- Contacts
- User group
- Update User Group
document_type: ReferenceDocumentType
updated_at: 2022-03-16T13:35:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/patch
---

# 更新用户组

使用该接口更新用户组信息，请注意更新用户组时应用的通讯录权限范围需为“全部员工”，否则会更新失败。[点击了解通讯录权限范围](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=contact&version=v3&resource=group&method=patch)

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
| HTTP URL | https://open.larksuite.com/open-apis/contact/v3/group/:group_id |
| HTTP Method | PATCH |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="contact:group" desc="更新用户组信息" support_app_types="custom" tags="">更新用户组信息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >group_id</md-text> | <md-text type="field-type" >string</md-text> | 用户组ID<br>**示例值**："g187131" |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 否 | 用户组的名字，企业内唯一，最大长度：100 字符<br>**示例值**："外包 IT 用户组" |
| <md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 否 | 用户组描述信息<br>最大长度：500 字<br>**示例值**："IT 外包用户组，需要进行细粒度权限管控" |




### 请求体示例

```json
{
    "name": "外包 IT 用户组",
    "description": "IT 外包用户组，需要进行细粒度权限管控"
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
    "msg": "success",
    "data": {}
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 40003 | internal error | 内部错误，请提供 X-Request-Id向客服反馈。[联系客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。 |
| 400 | 42002 | invalid group_id | 用户组 ID 无效 |
| 400 | 42013 | group name exceed limit | 用户组名字长度超过最大限制，最大限制100字符 |
| 400 | 42014 | group description exceed limit | 用户组描述长度超过最大限制，最大限制500字符 |
| 403 | 42009 | no user group authority error | 缺少用户组权限。应用的通讯录权限范围需包含该用户组或为“全部员工”，[点击了解更多](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority) |
| 400 | 42015 | user group disable | 用户组功能未开启，请联系Lark客服处理，[联系客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 400 | 47009 | duplicated name error | 用户组名称不得在企业内重复 |





