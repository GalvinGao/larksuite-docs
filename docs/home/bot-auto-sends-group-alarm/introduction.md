---
document_id: '7074952334765965318'
directory_id: '7073442394955644933'
title: 简介
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/introduction
breadcrumb:
- Home
- Bot Auto Sends Group Alarm
- Introduction
document_type: GuideDocumentType
updated_at: 2024-07-15T06:20:56Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/introduction
---

# 简介

当业务出现线上故障时，需尽快通知相关的成员解决问题。基于Lark开放能力，可以通过Lark机器人实现自动创建Lark项目群，并自动拉入相关的负责人，进而实现以下场景功能：

- 通过项目群推送报警通知，提示故障内容。

- 在群内跟进处理并及时同步进展，事后也可以通过群组沟通记录及时进行复盘。

- 当问题解决时，机器人可将群名自动修改为 `已解决`。

本教程将帮助你了解如何从零开始创建Lark机器人，并实现上述功能。

## 操作流程

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c071de6fb53bc7fbe8c018830171499e_YIdDDMtCSO.png?height=208&lazyload=true&width=1093)


## 实现效果

1. 机器人自动拉群。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b0c7b803db4257c6b44a987497877651_hz5QwerznT.png?height=649&lazyload=true&maxWidth=600&width=1640)

2. 机器人自动推送报警通知。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0d42c96566869e38611b9fc9197d7052_oSKKOViZpo.png?height=1100&lazyload=true&maxWidth=600&width=1640)

3. 问题解决后自动修改群名。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d84f0768661028591a515747ee663b0c_hGOsqwwu6o.png?height=1100&lazyload=true&maxWidth=600&width=1640)


## 使用到的 API 与事件列表

在机器人自动拉群报警通知的场景中，你需要调用消息与群组业务域的 API，包括：


### 群组 API

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)** |
| --- | --- | --- |
| <md-text type="field-name" >[创建群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create)<br>`POST` /open-apis/im/v1/chats<br>> 创建群并设置群头像、群名、群描述等。<br></md-text> | <md-perm name="im:chat:create" desc="创建群" support_app_types="custom,isv" tags="">创建群</md-perm><br><md-perm name="im:chat" desc="获取与更新群组信息" support_app_types="custom,isv" tags="">获取与更新群组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[获取群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/get)<br>`GET` /open-apis/im/v1/chats/:chat_id<br>> 获取群名称、群描述、群头像、群主 ID 等群基本信息。<br></md-text> | <md-perm name="im:chat:read" desc="查看群信息" support_app_types="custom,isv" tags="">查看群信息</md-perm><br><md-perm name="im:chat:readonly" desc="获取群组信息" support_app_types="custom,isv" tags="">获取群组信息</md-perm><br><md-perm name="im:chat" desc="获取与更新群组信息" support_app_types="custom,isv" tags="">获取与更新群组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[更新群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/update)</md-text><br>`PUT` /open-apis/im/v1/chats/:chat_id<br>> 更新群头像、群名称、群描述、群配置、转让群主等。 | <md-perm name="im:chat:update" desc="更新群信息" support_app_types="custom,isv" tags="">更新群信息</md-perm><br><md-perm name="im:chat" desc="获取与更新群组信息" support_app_types="custom,isv" tags="">获取与更新群组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |



### 消息 API

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)** |
| --- | --- | --- |
| <md-text type="field-name" >[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)<br>`POST` /open-apis/im/v1/messages<br>> 给指定用户或者会话发送消息，支持文本、富文本、卡片、群名片、个人名片、图片、视频、音频、文件、表情包。<br></md-text> | <md-perm name="im:message:send_as_bot" desc="以应用的身份发消息" tags="">以应用的身份发消息</md-perm><br><md-perm name="im:message" desc="获取与发送单聊、群组消息" tags="">获取与发送单聊、群组消息</md-perm><br><md-perm name="im:message:send" desc="发送消息V2" support_app_types="custom,isv" tags="history,offline">发送消息V2</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)<br>`POST` /open-apis/im/v1/images<br>> 上传图片接口，可以上传 JPEG、PNG、WEBP 格式图片。<br></md-text> | <md-perm name="im:resource" desc="获取与上传图片或文件资源 " support_app_types="custom,isv" tags="">获取与上传图片或文件资源 </md-perm><br><md-perm name="im:resource:upload" desc="上传文件V2" support_app_types="custom,isv" tags="history,offline">上传文件V2</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list)<br>`GET` /open-apis/im/v1/messages<br>>获取会话（包括单聊、群组）的历史消息。<br></md-text> | <md-perm name="im:message:readonly" desc="获取单聊、群组消息" support_app_types="custom,isv" tags="">获取单聊、群组消息</md-perm><br><md-perm name="im:message" desc="获取与发送单聊、群组消息" support_app_types="custom,isv" tags="">获取与发送单聊、群组消息</md-perm><br><md-perm name="im:message.history:readonly" desc="获取单聊、群组的历史消息" support_app_types="custom" tags="history,offline">获取单聊、群组的历史消息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |


### 消息事件

| **[事件 (Event)](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 触发时机 | 权限要求（满足任一） | 事件类型 |
| --- | --- | --- | --- |
| <md-text type="field-name" >[接收消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/receive)</md-text> | 机器人接收到用户发送的消息后触发此事件。 | <md-perm name="im:message.group_at_msg:readonly" desc="接收群聊中@机器人消息事件" support_app_types="custom,isv" tags="">接收群聊中@机器人消息事件</md-perm><br><md-perm name="im:message.group_msg" desc="获取群组中所有消息（敏感权限）" support_app_types="custom,isv" tags="">获取群组中所有消息（敏感权限）</md-perm><br><md-perm name="im:message.p2p_msg:readonly" desc="读取用户发给机器人的单聊消息" support_app_types="custom,isv" tags="">读取用户发给机器人的单聊消息</md-perm><br><md-perm name="im:message.p2p_msg" desc="获取用户发给机器人的单聊消息" support_app_types="custom,isv" tags="">获取用户发给机器人的单聊消息</md-perm><br><md-perm name="im:message.group_at_msg" desc="获取用户在群组中@机器人的消息" support_app_types="custom,isv" tags="">获取用户在群组中@机器人的消息</md-perm><br><md-perm name="im:message.group_msg:readonly" desc="获取群聊中所有的用户聊天消息" support_app_types="custom" tags="history,offline">获取群聊中所有的用户聊天消息</md-perm> | im.message.receive_v1 |

