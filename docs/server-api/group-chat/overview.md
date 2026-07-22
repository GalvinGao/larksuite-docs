---
document_id: '7198482854850248710'
directory_id: '7186934204960407558'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/group/overview
breadcrumb:
- Server API
- Group Chat
- Overview
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:59Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/group/overview
---

# 概述

## 资源：群组
查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-info/intro)

### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[创建群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create)<br>`POST` /open-apis/im/v1/chats<br>> 创建群并设置群头像、群名、群描述等。<br></md-text> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">创建群</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/get)</md-text><br>`GET` /open-apis/im/v1/chats/:chat_id<br>> 获取群名称、群描述、群头像、群主 ID 等群基本信息。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/update)</md-text><br>`PUT` /open-apis/im/v1/chats/:chat_id<br>> 更新群头像、群名称、群描述、群配置、转让群主等。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新群信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[解散群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/delete)</md-text><br>`DELETE` /open-apis/im/v1/chats/:chat_id<br>>解散群聊。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">解散群</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取用户或机器人所在的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/list)</md-text><br>`GET` /open-apis/im/v1/chats<br>>用户获取用户或者机器人所在的群列表。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[搜索对用户或机器人可见的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/search)</md-text><br>`GET` /open-apis/im/v1/chats/search<br>>用于搜索对用户、机器人可见的群列表 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user<br>_access_token</md-tag> | **✓** | **✓** |



### 事件列表

| **[事件 (Event)](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 触发时机 | 权限要求（满足任一） | 事件类型 | 商店 | 自建 |
| --- | --- | --- | --- | --- | --- |
| <md-text type="field-name" >[群解散](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/events/disbanded)</md-text> | 群组被解散后触发此事件。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群信息</md-perm> | im.chat.disbanded_v1 | **✓** | **✓** |
| <md-text type="field-name" >[群配置修改](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/events/updated)</md-text> | 群组配置被修改后触发此事件，包含：群主转移、群基本信息修改、群权限修改 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群信息</md-perm> | im.chat.updated_v1 | **✓** | **✓** |


## 资源：群组 - 群成员
查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-member/intro)

### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[将用户或机器人拉入群聊](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/create)<br>`POST` open-apis/im/v1/chats/:chat_id/members<br>> 将用户或机器人拉入群聊。<br></md-text> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">添加、移除群成员</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[将用户或机器人移出群聊](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/delete)</md-text><br>`DELETE` /open-apis/im/v1/chats/:chat_id/members<br>> 将用户或机器人移出群聊。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">添加、移除群成员</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[用户或机器人主动加入群聊](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/me_join)</md-text><br>`PATCH` /open-apis/im/v1/chats/:chat_id/members/me_join<br>> 用户或者机器人可以通过接口入群。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">添加、移除群成员</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取群成员列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/get)</md-text><br>`GET` /open-apis/im/v1/chats/:chat_id/members<br>>获取群里成员列表。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群成员</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[判断用户或机器人是否在群里](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/is_in_chat)</md-text><br>`GET` /open-apis/im/v1/chats/:chat_id/members/is_in_chat<br>>判断用户或机器人是否在群里。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群成员</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">usert_access_token</md-tag> | **✓** | **✓** |



### 事件列表

| **[事件 (Event)](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 触发时机 | 权限要求（满足任一） | 事件类型 | 商店 | 自建 |
| --- | --- | --- | --- | --- | --- |
| <md-text type="field-name" >[机器人进群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-member-bot/events/added)</md-text> | 机器人被添加至群聊时触发此事件。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">订阅机器人进、出群事件</md-perm> | im.chat.member.bot.added_v1 | **✓** | **✓** |
| <md-text type="field-name" >[机器人被移出群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-member-bot/events/deleted)</md-text> | 机器人被移出群聊后触发此事件。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">订阅机器人进、出群事件</md-perm> | im.chat.member.bot.deleted_v1 | **✓** | **✓** |
| <md-text type="field-name" >[用户进群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-member-user/events/added)</md-text> | 新用户进群触发此事件。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群成员</md-perm> | im.chat.member.user.added_v1 | **✓** | **✓** |
| <md-text type="field-name" >[用户出群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-member-user/events/deleted)</md-text> | 用户主动退群或被移出群聊时推送事件。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群成员</md-perm> | im.chat.member.user.deleted_v1 | **✓** | **✓** |
| <md-text type="field-name" >[撤销拉用户进群](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/events/deleted)</md-text> | 撤销拉用户进群后触发此事件。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群成员</md-perm> | im.chat.member.user.withdrawn_v1 | **✓** | **✓** |

## 资源：群组 - 群公告
查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-announcement/intro)

### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[获取群公告信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/get)<br>`GET` /open-apis/im/v1/chats/:chat_id/announcement<br>> 获取会话中的群公告信息，公告信息格式与云文档格式相同。<br></md-text> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看群公告信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新群公告信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/patch)</md-text><br>`PATCH` /open-apis/im/v1/chats/:chat_id/announcement<br>> 更新会话中的群公告信息，更新公告信息的格式和更新云文档格式相同。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新群公告内容</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |





