---
document_id: '7073823165957554182'
directory_id: '7073460768595378181'
title: 打开聊天页面
full_path: /uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-a-chat-page
breadcrumb:
- Developer Guides
- AppLink Protocol
- Supported protocol
- Open a chat page
document_type: GuideDocumentType
updated_at: 2022-03-11T16:42:53Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-a-chat-page
---

# 打开聊天页面
::: note 
从Lark 3.9.0 版本开始支持。
:::

## 使用场景
打开一个聊天页面，单聊会话或群聊会话（仅能打开用户已加入的单聊或群聊会话，不会自动进入未加入的群组）。

## 协议
[https://applink.larksuite.com/client/chat/open](https://applink.larksuite.com/client/chat/open)

##  参数




| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|openId | 是         | 用户 openId | 
|openChatId|是         |会话ID，包括单聊会话和群聊会话。示例：oc_41e7bdf4877cfc316136f4ccf6c32613|

::: note
`openId`的获取方式可以参考文档：[如何获得 User ID、Open ID 和 Union ID？](/document/home/user-identity-introduction/how-to-get)
:::
::: note
`openChatId`是以 'oc' 开头的字段，可通过文档 [群ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description) 了解详细信息。

:::
## 使用示例
#### 1. 使用 openId 打开聊天页面

`https://applink.larksuite.com/client/chat/open?openId=1234567890`
#### 2. 使用 openChatId 打开聊天页面

`https://applink.larksuite.com/client/chat/open?openChatId=oc_41e7bdf4877cfc316136f4ccf6c32613`

