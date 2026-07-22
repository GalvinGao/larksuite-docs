---
document_id: '6967261316286611461'
directory_id: '6907567266536652801'
title: 创建群
full_path: /ukTMukTMukTM/ukDO5QjL5gTO04SO4kDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Group Chat
- Create Group
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:16Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukDO5QjL5gTO04SO4kDN
---

# 创建群
机器人创建群并拉指定用户进群。  

:::html
<md-alert type="tip">
需要启用机器人能力
</md-alert>
:::

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/chat/v4/create/ |
| HTTP Method | POST |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |



### 请求体
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
|name|string|必填| 群的名称|空|group name|
|description|string|选填|群描述|空|group description|
|owner_open_id|string|选填|群主的 open_id （转让群主时, owner_open_id 和 owner_user_id 任意填写一个即可）|无|ou_4065981088f8ef67a504ba8bd6b24d85|
|owner_user_id|string|选填|群主的 user_id （转让群主时, owner_open_id 和 owner_user_id 任意填写一个即可）|无|cb93bdca|
|open_ids|list|选填| 成员 open_id 列表，最多可以传200个 (open_ids 和 user_ids 参数不能同时为空)|nil|["ou_4065981088f8ef67a504ba8bd6b24d85","ou_111111111111111111111111111111111"]|
|user_ids|list|选填|成员 user_id 列表，最多可以传200个 (open_ids 和 user_ids 参数不能同时为空)|nil|["33417745","cb93bdca"]|
|i18n_names|map|选填|不同语种的群名称，目前支持中英文，i18n_names 会覆盖 name|nil|{"zh_cn": "zh_cn name","en_us": "en_us name"}| 
|only_owner_add|bool|选填|是否仅群主可以添加人|false|false| 
|share_allowed|bool|选填|是否允许分享群|true|true|  
|add_member_verify|bool|选填|是否开启入群验证|false|
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
    "name": "group name",
    "description": "group description",
    "owner_open_id": "ou_xxxx",
    "owner_user_id": "cb93bxxx",
    "user_ids": [
        "33417745",
        "cb93bdca"
    ],
    "open_ids": [
        "ou_4065981088f8ef67a504ba8bd6b24d85",
        "ou_111111111111111111111111111111111"
    ],
    "i18n_names": {
        "zh_cn": "zh_cn name",
        "en_us": "en_us name"
    },
    "only_owner_add": false,
    "share_allowed": true,
    "only_owner_at_all": false,
    "only_owner_edit": false,
    "join_message_visibility": "all",  
    "leave_message_visibility": "all",  
    "add_member_verify": false,
    "send_message_permission": "all",  
    "group_email_enabled": true,
    "send_group_email_permission": "tenant_member" 
}
```

## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码描述|
|data|-|-
&emsp;∟chat_id|string| 创建群的 ID|
&emsp;∟invalid_open_ids|list| 无效的 open_id 	列表|
&emsp;∟invalid_user_ids|list| 无效的 user_id 列表|


### 响应体示例 
```json
{
    "code": 0,
    "data": {
        "chat_id": "oc_4f65b883a624c59414157668c91637ab",
        "invalid_open_ids": [
            "ou_111111111111111111111111111111111"
        ],
        "invalid_user_ids": [
            "33417745"
        ]
    },
    "msg": "ok"
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


