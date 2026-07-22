---
document_id: '6967261389551419397'
directory_id: '6907567266536652801'
title: 更新群信息
full_path: /ukTMukTMukTM/uYTO5QjL2kTO04iN5kDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Group Chat
- Update Group Info
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:18Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYTO5QjL2kTO04iN5kDN
---

# 更新群信息
更新群名称、群配置、转让群主等。

:::html
<md-alert type="tip">
需要启用机器人能力；“机器人是群主” 或 “机器人在群里，是群的创建者且具备 更新应用创建的群信息 权限”。
</md-alert>
:::

:::html
<md-alert type="tip">
- 当"机器人是群主" 或  "机器人不是群主，但机器人是群的创建者且具备 更新应用创建的群信息 权限"时，可修改群参数；
- 当机器人不是上述角色，且对应群开启了"仅群主可编辑信息"，则机器人无法更新任何参数；
- 当机器人不是上述角色，但对应群未开启"仅群主可编辑信息"，则机器人可修改"群名称"与"群描述"
</md-alert>
:::

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/chat/v4/update/ |
| HTTP Method | POST |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
|chat_id| string| 必填 |群 ID ||oc_1e16cf86b174f490cb97d47532a5011e
|owner_open_id| string| 选填 | 群主的 open_id （转让群主时, owner_open_id 和 owner_user_id 任意填写一个即可）||ou_4065981088f8ef67a504ba8bd6b24d85|
|owner_user_id| string| 选填 | 群主的 user_id （转让群主时, owner_open_id 和 owner_user_id 任意填写一个即可）||cb93bdca|
|name | string| 选填 | 默认显示的群名称||group name|
|i18n_names | map| 选填 | 国际化的群名称||{"zh_cn":"zh_cn name","en_us":"en_us name","ja_jp":"ja_jp name"}|
|description|string|选填|群描述|||
|only_owner_add|bool|选填|是否仅群主可以添加人|false|false| 
|share_allowed|bool|选填|是否允许分享群|true|true|  
|add_member_verify|bool|选填|是否开启入群验证|false||
|only_owner_at_all|bool|选填|是否仅群主@all|false|false| 
|only_owner_edit|bool|选填|是否仅群主可编辑群信息，群信息包括头像、名称、描述、公告|false|false|  
|send_message_permission|string|选填|允许谁发送消息<br>all:  所有人<br> owner：仅群主|all|all|
|join_message_visibility|string|选填|成员入群通知<br>all：通知所有人<br>owner：只通知 owner<br>not_anyone：不通知任何人|all|all|
|leave_message_visibility|string|选填|成员退群通知<br> all：通知所有人<br>owner：只通知 owner<br>not_anyone：不通知任何人|owner|owner|
|group_email_enabled|bool|选填|是否开启群邮件|false|false|
|send_group_email_permission|string|选填|发送群邮件的权限<br>owner：仅群主 <br>group_member：群组内成员 <br>tenant_member：团队成员 <br>all：所有人|tenant_member|tenant_member|

### 请求体示例
```json
{
    "chat_id": "oc_020ff1d91a0295fc3961032768d41f39",
    "owner_user_id":"cb93bdca",
    "owner_open_id":"ou_4065981088f8ef67a504ba8bd6b24d85",
    "name":"group name",
    "i18n_names":{
		"zh_cn":"zh_cn name",
		"en_us":"en_us name",
		"ja_jp":"ja_jp name"
    },
    "only_owner_add": false,
    "share_allowed": true,
    "only_owner_at_all": false,
    "only_owner_edit": false,
    "join_message_visibility": "owner", 
    "leave_message_visibility": "owner",
    "add_member_verify": true,
    "send_message_permission": "owner",  
    "group_email_enabled": true,
    "send_group_email_permission": "group_member"
}
```
## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
code |int| 返回码，非 0 表示失败
msg |string| 返回码描述
data|-|-
&emsp;∟chat_id |string| 群 ID，与请求中相同


### 响应体示例
```json
{
    "code": 0,
    "data": {
        "chat_id": "oc_020ff1d91a0295fc3961032768d41f39"
    },
    "msg": "ok"
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


