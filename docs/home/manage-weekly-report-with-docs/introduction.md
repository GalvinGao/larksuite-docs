---
document_id: '7233612551991705606'
directory_id: '7199928167142227973'
title: 简介
full_path: /home/management-weekly-report-based-docs/introduction
breadcrumb:
- Home
- Manage Weekly Report with Docs
- Introduction
document_type: GuideDocumentType
updated_at: 2024-07-15T06:21:05Z
source_url: https://open.larksuite.com/document/home/management-weekly-report-based-docs/introduction
---

# 简介

本教程介绍如何使用Lark开放平台云文档能力实现团队周报管理。

通过周报模版文档每周定时创建出新一周的周报文档，并给团队成员授权文档编辑权限，自动将新周报文档归档到知识库指定节点，最后通过群机器人发送群消息通知所有人更新本周周报内容。

## 流程简介

本教程将按照以下流程，实现团队周报管理。

:::html
<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0e77a26da7979343fbaf74948996ab89_Gx6u8w1vU6.png?lazyload=true&width=918&height=857" style="width:70%"/>
:::

## 实现效果

按照本教程操作最终可以实现如下图的示意效果。

- 本教程最终创建的周报文档如下：

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9421df083f6cfdd878bbe85e0a1ca5ac_yYND9x65Cq.png?height=1053&lazyload=true&maxWidth=700&width=1640)

- 最终发送在团队群聊中的消息如下：
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6303a46946f9a2454b6f3367c6605bc8_I84gs6CTEK.png?height=957&lazyload=true&maxWidth=700&width=1640)
    
    
## 使用到的API列表

在本教程中，需要调用云文档和消息与群组业务域的 API 列表：

### 云文档

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** |
| --- | --- | --- |
| <md-text type="field-name" >[获取空间根目录](/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta)<br>`GET` /open-apis/drive/explorer/v2/root_folder/meta<br>> 获取云空间的根目录<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[获取文件夹下的文档清单](/document/ukTMukTMukTM/uEjNzUjLxYzM14SM2MTN)<br>`GET` /open-apis/drive/explorer/v2/folder/:folderToken/children<br>> 获取文件夹内的文档清单，也包括文件夹<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[获取文档元数据](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/meta/batch_query)<br>`POST` /open-apis/drive/v1/metas/batch_query<br>> 根据 Token 获取各类文档的元数据<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive.metadata:readonly" desc="查看云空间中文件元数据" support_app_types="custom,isv" tags="">查看云空间中文件元数据</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[复制文件](/document/ukTMukTMukTM/uYTNzUjL2UzM14iN1MTN)<br>`POST` /open-apis/drive/explorer/v2/file/copy/files/:fileToken<br>> 将文件复制到用户云空间的其他文件夹中<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[增加权限](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create)<br>`POST` /open-apis/drive/v1/permissions/:token/members<br>> 该接口用于根据 filetoken 给用户增加文档的权限<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[添加知识空间成员](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-member/create)<br>`POST` /open-apis/wiki/v2/spaces/:space_id/members<br>> 添加知识空间成员或管理员<br></md-text> | <md-perm name="wiki:wiki" desc="查看、编辑和管理知识库" support_app_types="custom,isv" tags="">查看、编辑和管理知识库</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[添加已有云文档至知识库](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/move_docs_to_wiki)<br>`POST` /open-apis/wiki/v2/spaces/:space_id/nodes/move_docs_to_wiki<br>> 该接口允许添加已有云文档至知识库<br></md-text> | <md-perm name="wiki:wiki" desc="查看、编辑和管理知识库" support_app_types="custom,isv" tags="">查看、编辑和管理知识库</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |


### 消息与群组

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** |
| --- | --- | --- |
| <md-text type="field-name" >[获取用户或机器人所在的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/list)<br>`GET` /open-apis/im/v1/chats<br>> 获取用户或者机器人所在群列表<br></md-text> | <md-perm name="im:chat:read" desc="查看群信息" support_app_types="custom,isv" tags="">查看群信息</md-perm><br><md-perm name="im:chat" desc="获取与更新群组信息" support_app_types="custom,isv" tags="">获取与更新群组信息</md-perm><br><md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm><br><md-perm name="im:chat:readonly" desc="获取群组信息" support_app_types="custom,isv" tags="">获取群组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)<br>`POST` /open-apis/im/v1/messages<br>> 给指定用户或者会话发送消息，支持文本、富文本、卡片、群名片、个人名片、图片、视频、音频、文件、表情包<br></md-text> | <md-perm name="im:message:send_as_bot" desc="以应用的身份发消息" tags="">以应用的身份发消息</md-perm><br><md-perm name="im:message" desc="获取与发送单聊、群组消息" tags="">获取与发送单聊、群组消息</md-perm><br><md-perm name="im:message:send" desc="发送消息V2" support_app_types="custom,isv" tags="history,offline">发送消息V2</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |


