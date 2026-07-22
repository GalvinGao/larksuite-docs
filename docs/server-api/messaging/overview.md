---
document_id: '7075603703986061318'
directory_id: '6924949741370621980'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/introduction
breadcrumb:
- Server API
- Messaging
- Overview
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:00Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/introduction
---

# 概述
## 业务介绍
Lark提供了一系列即时通讯场景的API。通过创建[Lark机器人](/document/ukTMukTMukTM/uATM04CMxQjLwEDN)并调用消息与群组API，你可以实现多种功能，例如：

- 通过Lark发送不同类型的消息，如文本、富文本、图片、文件、互动[消息卡片](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)、视频、音频和表情符号。
- 管理Lark群组，例如创建群组、添加用户或机器人到群组。


更多应用场景可参考客户案例：
- 华住：[从通知到高效协同，Lark统一告警方案让服务保障坚如磐石](https://open.larksuite.com/solutions/detail/alert)
- 蔚来：[当项目管理遇见Lark，协同沉淀更便捷](https://open.larksuite.com/solutions/detail/project)


###  接入流程

|  | 步骤 | 介绍 |
| --- | --- | --- |
| 1 | 创建一个应用 | - 如需创建企业自建应用，可参考 [自建应用的开发流程](/document/home/introduction-to-custom-app-development/self-built-application-development-process)<br>-   如需创建应用商店应用，可参考 [开发和上架应用商店应用](/document/uMzNwEjLzcDMx4yM3ATM/uYzNwEjL2cDMx4iN3ATM) |
| 2 | 调用API，对消息或群组进行操作 | 调用API前，你需要先获取访问凭证并开启对应的权限，详情参见 [如何调用服务端API](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)<br>你还可以在 [API 调试台![API cn.svg](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/90b013d1fc7c191381d3edd6cf147707_3VtIkELbfg.svg?height=24&lazyload=true&width=87)](https://open.larksuite.com/api-explorer?from=guide)中快速调试这些 API ，使用方法参见[API 调试台使用指南](/document/tools-and-resources/api-explorer-guide)。 |
| 3 | 监听事件，获知消息或群组的变化 | 监听事件前，你需要先申请相应的权限，详情参见 [事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM) |



### 开发教程

| 快速入门场景教学 |  |
| --- | --- |
| [机器人自动拉群报警](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/introduction)<br>![14机器人自动拉群报警.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/69e9999ad015bca8e79c50f1229fbc61_pBAn6j9TCi.png?height=400&lazyload=true&width=752) | [向指定部门进行消息群发](/document/home/mass-messaging-to-designated-departments/introduction)<br>![13将企业组织架构同步到Larkcn.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e14bc2fd84ca3974597db8e371b92cca_WqZ1usQENO.png?height=400&lazyload=true&width=752) |
| [新人入群欢迎机器人](/document/home/event-based-messaging/introduction)<br>![13将企业组织架构同步到Larkcn.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8c385d02b353ac4d5569a37b81156f77_7cEJornbRS.png?height=400&lazyload=true&width=752) | [基于会话的互动机器人](/document/home/interactive-session-based-robot/introduction)<br>![13将企业组织架构同步到Larkcn.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e2d3245e6c8b63b1b231d7fcc0b955d3_zoPIf8ryhR.png?height=400&lazyload=true&width=752) |
| [互动型消息卡片发送（审批卡片）](/document/home/interactive-message-card-sending/introduction)<br>![13将企业组织架构同步到Larkcn.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/45fe39340b4da08a5be99f35690a4972_JKOfgkOxJS.png?height=400&lazyload=true&width=752) |  |



- 发送不同类型的Lark消息。如文本、富文本、图片、文件、可交互的[消息卡片](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)、视频、音频、表情等；
- 进行Lark群组管理。如创建群组、拉用户或机器人入群。
## 资源介绍

消息与群组业务域以“资源”为中心进行开放，资源的关系图如下：

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/61926a6c9825a7d40b22e022ede774bb_GC1fDpDAOl.png?height=1854&lazyload=true&width=1640)


上图中 [用户 User](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/field-overview) 资源 属于通讯录业务范围，可打开 [通讯录文档](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/resources#3b13f0a0) 查看详情。

消息与群组 的相关资源定义如下：

| 资源 | 资源定义 |
| --- | --- |
| [消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro) | 通过IM发送的一段信息，类型包括文本、富文本、卡片、群名片、个人名片、图片、视频、文件等。 |
| [消息 - 图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/introduction#504f9b2a) | 承载图片信息的静态资源。 |
| [消息 - 文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/introduction#c70e0434) | 承载文件信息的静态资源。 |
| [消息 - 消息卡片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-card/overview) | 可以承载丰富的图文内容和交互行为的消息类型。 |
| [消息 - 表情回复](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/overview-of-message-reaction-resources) | Lark消息的表情回复。 |
| [群信息](/document/uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-info/intro) | Lark中的聊天群组。 |
| [群组 - 群成员](/document/uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-announcement/intro) | 群组内成员（包括用户和机器人）的集合，用于描述群组和成员的关系。 |
| [群组 - 群公告](/document/uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-announcement/intro) | 群组中的公告文档。 |


以下将详细介绍每个资源的字段、方法、事件。
>  “商店”代表应用商店应用，“自建”代表企业自建应用，请参考[应用类型说明](/document/home/app-types-introduction/overview)。

### 资源：消息
查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro)

#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)<br>`POST` /open-apis/im/v1/messages<br>> 给指定用户或者会话发送消息，支持文本、富文本、卡片、群名片、个人名片、图片、视频、音频、文件、表情包。<br></md-text> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用的身份发消息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[批量发送消息](/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM)</md-text><br>`POST` /open-apis/message/v4/batch_send/<br>> 给多个用户或者多个部门发送消息。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">给多个用户批量发消息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">给一个或多个部门的成员批量发消息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[回复消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/reply)</md-text><br>`POST` /open-apis/im/v1/messages/:message_id/reply<br>> 回复指定消息，支持文本、富文本、卡片、群名片、个人名片、图片、视频、文件等多种消息类型。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用的身份发消息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[撤回消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/delete)</md-text><br>`DELETE` /open-apis/im/v1/messages/:message_id<br>>机器人撤回机器人自己发送的消息或群主撤回群内消息。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">撤回消息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[查询消息已读信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/read_users)</md-text><br>`GET` /open-apis/im/v1/messages/:message_id<br>>查询消息的已读信息。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取单聊、群组消息</md-perm></md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list)</md-text><br>`GET` /open-apis/im/v1/messages<br>>获取会话（包括单聊、群组）的历史消息。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取单聊、群组消息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取消息中的资源文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-resource/get)</md-text><br>`GET` /open-apis/im/v1/messages/:message_id/resources/:file_key<br>>获取消息中的资源文件，包括音频，视频，图片和文件，暂不支持表情包资源下载。当前仅支持 100M 以内的资源文件的下载。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取单聊、群组消息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取指定消息的内容](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/get)</md-text><br>`GET` /open-apis/im/v1/messages/:message_id<br>>通过 message_id 查询消息内容 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取单聊、群组消息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |



#### 事件列表

| **[事件 (Event)](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 触发时机 | 权限要求（满足任一） | 事件类型 | 商店 | 自建 |
| --- | --- | --- | --- | --- | --- |
| <md-text type="field-name" >[接收消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/receive)</md-text> | 机器人接收到用户发送的消息后触发此事件。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">接收群聊中@机器人消息事件</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">读取用户发给机器人的单聊消息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取群组中所有消息</md-perm> | im.message.receive_v1 | **✓** | **✓** |
| <md-text type="field-name" >[消息已读](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/message_read)</md-text> | 用户阅读机器人发送的单聊消息后触发此事件。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取单聊、群组消息</md-perm> | im.message.message_read_v1 | **✓** | **✓** |

### 资源：消息 - 图片
#### 资源字段及示例

| **名称** | 类型 | **描述** |
| --- | --- | --- |
| image_key | string | 图片的key |

```json
{
    "image_key": "img_v2_b99741-7628-4abd-aad0-b881e4db83ig"
}
```
  
#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)<br>`POST` /open-apis/im/v1/images<br>> 上传图片接口，可以上传 JPEG、PNG、WEBP 格式图片。<br></md-text> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取与上传图片或文件资源 </md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[下载图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/get)</md-text><br>`GET` /open-apis/im/v1/images/:image_key<br>> 下载图片资源，只能下载应用自己上传且图片类型为message的图片。 | 无 | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |


### 资源：消息 - 文件

| **名称** | 类型 | **描述** |
| --- | --- | --- |
| file_key | string | 文件的key |

```json
{
    "file_key": "file_456a92d6-c6ea-4de4-ac3f-7afcf44ac78g"
}
```
#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/create)<br>`POST` /open-apis/im/v1/files<br>> 上传文件，可以上传视频，音频和常见的文件类型。<br></md-text> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取与上传图片或文件资源 </md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[下载文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/get)</md-text><br>`GET` /open-apis/im/v1/files/:file_key<br>> 下载文件接口，只能下载应用自己上传的文件。 | 无 | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |



### 资源：消息 - 消息卡片
查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-card/overview)

#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[更新应用发送的消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/patch)<br>`PATCH` /open-apis/im/v1/messages/:message_id<br>> 更新应用已发送的消息卡片内容。<br></md-text> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新消息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用的身份发消息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[消息卡片延迟更新](/document/ukTMukTMukTM/uMDO1YjLzgTN24yM4UjN)</md-text><br>`POST` /open-apis/interactive/v1/card/update<br>> 用于用户交互完成后延后更新消息卡片 | 无 | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[发送临时卡片消息](/document/ukTMukTMukTM/uETOyYjLxkjM24SM5IjN)</md-text><br>`POST` /open-apis/ephemeral/v1/send<br>> 用于机器人在群会话中发送指定用户可见的消息卡片。 | 无 | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[删除临时消息](/document/ukTMukTMukTM/uITOyYjLykjM24iM5IjN)</md-text><br>`POST` /open-apis/ephemeral/v1/delete<br>> 在群会话中删除指定用户的临时消息卡片<br>临时卡片消息可以通过该接口进行显式删除，临时卡片消息删除后将不会在该设备上留下任何痕迹。 | 无 | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |

  
### 资源：消息 - 表情回复
查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/overview-of-message-reaction-resources)

#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[添加消息表情回复](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/create)<br>`POST` /open-apis/im/v1/messages/:message_id/reactions<br>> 给指定消息添加指定类型的表情回复。<br></md-text> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">发送、删除消息表情回复</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取消息表情回复](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/list)</md-text><br>`GET` /open-apis/im/v1/messages/:message_id/reactions<br>> 获取指定消息的特定类型表情回复列表。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看消息表情回复</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取单聊、群组消息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[删除消息表情回复](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/delete)</md-text><br>`DELETE` /open-apis/im/v1/messages/:message_id/reactions/:reaction_id<br>> 删除指定消息的表情回复。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">发送、删除消息表情回复</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |

  

#### 事件列表

| **[事件 (Event)](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 触发时机 | 权限要求（满足任一） | 事件类型 | 商店 | 自建 |
| --- | --- | --- | --- | --- | --- |
| <md-text type="field-name" >[删除消息表情回复](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/events/deleted)</md-text> | 消息被删除某一个表情回复后触发此事件。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看消息表情回复</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取单聊、群组消息</md-perm> | im.message.reaction.deleted_v1 | **✓** | **✓** |
| <md-text type="field-name" >[新增消息表情回复](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/events/created)</md-text> | 消息被添加某一个表情回复后触发此事件。 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">查看消息表情回复</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取单聊、群组消息</md-perm> | im.message.reaction.created_v1 | **✓** | **✓** |

  



