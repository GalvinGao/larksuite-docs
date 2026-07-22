---
document_id: '7182545664369278982'
directory_id: '7280804871254589446'
title: 机器人概述
full_path: /uAjLw4CM/ukTMukTMukTM/bot-v3/bot-overview
breadcrumb:
- Developer Guides
- Develop Bots
- Bot overview
document_type: GuideDocumentType
updated_at: 2024-02-05T02:53:44Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/bot-v3/bot-overview
---

# 机器人介绍
# 一、功能概述

机器人 ( bot ) 是一种自动化的程序，可以用拟人化的身份自动推送消息，或在聊天里与你进行简单的交互。

机器人能力的优势：
- **嵌入式的体验**：可以在聊天中通过消息完成内容的触达、信息收集等操作。借助机器人能力，你可以将企业系统集成进Lark，在Lark内获得一站式的系统使用体验。
- **开发成本相对较低**：只需要服务端开发，就能实现内容呈现友好、可进行互动的机器人。并且一次开发后，可以被企业内的其他成员轻松使用。
- **支持丰富的消息类型**：你不仅可以用机器人发送文本、图片、文件消息，还能进一步发送呈现样式更友好、支持互动的消息卡片，使推送内容更好地触达用户。完整的消息类型参考：[发送消息content说明](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json)

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/78822c483c64bb5b3606cbd6b9712a57_Ou9gs61KSb.png?height=972&lazyload=true&maxWidth=600&width=1600)

**以下提供一些常见机器人场景的开发教程，供你参考：**

| 快速入门场景教学 |  |
| --- | --- |
| [机器人自动拉群报警](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/introduction)<br>![14机器人自动拉群报警.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/69e9999ad015bca8e79c50f1229fbc61_pBAn6j9TCi.png?height=400&lazyload=true&width=752) | [向指定部门进行消息群发](/document/home/mass-messaging-to-designated-departments/introduction)<br>![13将企业组织架构同步到Larkcn.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e14bc2fd84ca3974597db8e371b92cca_WqZ1usQENO.png?height=400&lazyload=true&width=752) |
| [新人入群欢迎机器人](/document/home/event-based-messaging/introduction)<br>![13将企业组织架构同步到Larkcn.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8c385d02b353ac4d5569a37b81156f77_7cEJornbRS.png?height=400&lazyload=true&width=752) | [基于会话的互动机器人](/document/home/interactive-session-based-robot/introduction)<br>![13将企业组织架构同步到Larkcn.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e2d3245e6c8b63b1b231d7fcc0b955d3_zoPIf8ryhR.png?height=400&lazyload=true&width=752) |
| [互动型消息卡片发送（审批卡片）](/document/home/interactive-message-card-sending/introduction)<br>![13将企业组织架构同步到Larkcn.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/45fe39340b4da08a5be99f35690a4972_JKOfgkOxJS.png?height=400&lazyload=true&width=752) |  |



# 二、机器人的开发类型

## 应用机器人

应用机器人需要在 [开发者后台](https://open.larksuite.com/app) 中创建，申请发布并经过租户的应用管理员审核通过后，即可使用。

在应用发布范围内的用户，可以直接与你创建的机器人单聊，或在 **群设置>群机器人** 面板中将这个机器人添加进群聊使用。

应用机器人受本租户的应用管理员管控。在应用管理员进行权限审核后，应用机器人可以调用Lark丰富的开放接口，获取、使用用户和租户资源。

你可以参考教程：[开发一个可以互动的监控报警机器人](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/introduction)，了解开发一个可直接在聊天里互动的机器人的基本开发过程。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7ccf8484e4be6c42a00eee2bcb4c1145_MOL54aproq.png?height=1068&lazyload=true&maxWidth=600&width=1590)

## 自定义机器人

自定义机器人是一种只能在当前群聊中使用的机器人。你可以在 **群设置>群机器人** 面板中快捷创建一个自定义机器人，无需经过租户管理员审核，即可在当前群聊中通过调用webhook地址的方式完成消息推送。

由于自定义机器人无需经过租户管理员审核即可使用，**使用上的便捷性也限制了其使用范围**：

- 只能在被添加的群聊内使用，不能与机器人单聊
- 只能对群聊进行单向的消息推送，不能调用Lark丰富的开放接口，获取任何用户、租户信息

详细的自定义机器人配置过程参看[自定义机器人指南](/document/ukTMukTMukTM/ucTM5YjL3ETO24yNxkjN)。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e75b7fb58778fecda4c7f2f2e0f798d9_4Al8Uany56.png?height=662&lazyload=true&maxWidth=600&width=1538)

## 能力对比

自定义机器人配置过程简单，但使用场景有比较大的局限，只能满足基本的群消息推送的场景需求。

应用机器人虽然需要经过发版审核，但可以在应用管理员授权的前提下，调用Lark丰富的开放接口，完成更多的互动场景。

* 如果你只需要临时性地在群聊中完成比较固定的消息推送，建议使用简单的**自定义机器人**；
* 如果你想将外部系统集成进Lark，让机器人能进行互动、群管理，建议你使用**应用机器人**。

详细的能力对比如下表：
| 能力         | 自定义机器人           | 机器人应用        | 
| --------- | --------------- | -------   | 
|添加进外部群 | ✅ | ❌ | 
|向所在群推送消息 | ✅ | ✅ | 
|[实现包含链接跳转的消息卡片](/document/ukTMukTMukTM/uYjNwUjL2YDM14iN2ATN#7f69ddbb) | ✅ | ✅ | 
|[实现将点击操作提交到服务端的消息卡片](/document/ukTMukTMukTM/uYjNwUjL2YDM14iN2ATN#49904b71) | ❌ | ✅ | 
|[响应用户@机器人的消息](/document/home/interactive-session-based-robot/introduction) | ❌ | ✅ | 
|[向用户发送单聊消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create) | ❌ | ✅ | 
|[创建群、管理群、获取群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create) | ❌ | ✅ | 
|[访问通讯录、管理云文档等其他丰富的开放能力](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM) | ❌ | ✅ | 


# 三、如何使用已开发的机器人
你创建的“机器人”上线后，用户可参考[机器人使用指南](/document/ukTMukTMukTM/uATM04CMxQjLwEDN)，学习如何使用你的机器人：

* 用户将自己可见的机器人添加到群里，和群成员互动
* 向可见范围内的用户主动推送消息

