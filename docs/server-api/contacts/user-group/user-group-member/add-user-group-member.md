---
document_id: '7055272807039647750'
directory_id: '7050040770682814470'
title: 添加用户组成员
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/add
breadcrumb:
- Server API
- Contacts
- User group
- User group member
- Add User Group Member
document_type: ReferenceDocumentType
updated_at: 2022-03-16T13:35:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/add
---

# 添加用户组成员

向用户组中添加成员(目前成员仅支持用户，未来会支持部门)，如果应用的通讯录权限范围是“全部员工”，则可将任何成员添加到任何用户组。如果应用的通讯录权限范围不是“全部员工”，则仅可将通讯录权限范围中的成员添加到通讯录权限范围的用户组中，[点击了解通讯录权限范围](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=contact&version=v3&resource=group.member&method=add)

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
| HTTP URL | https://open.larksuite.com/open-apis/contact/v3/group/:group_id/member/add |
| HTTP Method | POST |
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
| <md-text type="field-name" >group_id</md-text> | <md-text type="field-type" >string</md-text> | 用户组ID<br>**示例值**："g281721" |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >member_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户组成员的类型，取值为 user<br>**示例值**："user"<br>**可选值有**：<br>- `user`：user<br>**默认值**：`user` |
| <md-text type="field-name" >member_id_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 当member_type =user时候，member_id_type表示user_id_type，枚举值为open_id, union_id, user_id<br>**示例值**："open_id"<br>**可选值有**：<br>- `open_id`：member_type =user时候，表示用户的open_id<br>- `union_id`：member_type =user时候，表示用户的union_id<br>- `user_id`：member_type =user时候，表示用户的user_id |
| <md-text type="field-name" >member_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 添加的成员ID<br>**示例值**："ou_7dab8a3d3cdcc9da365777c7ad535d62" |




### 请求体示例

```json
{
    "member_type": "user",
    "member_id_type": "open_id",
    "member_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62"
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
    "data": {},
    "msg": "success"
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 40003 | internal error | 内部错误，请提供 X-Request-Id向客服反馈。[联系客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。 |
| 400 | 42002 | invalid group_id | 用户组 ID 无效 |
| 400 | 41073 | invalid member_id | 成员ID 无效 |
| 400 | 41074 | invalid member_type, must user | 无效的成员类型，成员类型需为user |
| 400 | 41071 | en_name length exceed 64 character | 英文名长度超过64个字符 |
| 400 | 41072 | nickname length exceed 64 character | 别名长度超过64个字符 |
| 403 | 42009 | no user group authority error | 缺少用户组权限。应用的通讯录权限范围需包含该用户组或为“全部员工”，[点击了解更多](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority) |
| 403 | 40004 | no dept authority error | 操作的部门需在通讯录权限范围中，[了解更多](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority) |
| 403 | 41050 | no user authority error | 操作的用户需在通讯录权限范围中，[了解更多](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority) |
| 400 | 42005 | member exist in group error | 成员已经存在用户组中 |
| 400 | 42006 | user has resigned error | 用户已经离职。 |
| 400 | 42012 | group member user reached the upper limit | 用户组的用户数量达到上限 |
| 400 | 42011 | group member department reached the upper limit | 用户组的部门数量达到上限 |





