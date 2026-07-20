---
document_id: '7073823165957603334'
directory_id: '7073460768595378181'
title: 打开机器人会话
full_path: /uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-a-bot
breadcrumb:
- Developer Guides
- AppLink Protocol
- Supported protocol
- Open a bot
document_type: GuideDocumentType
updated_at: 2022-03-11T16:42:53Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-a-bot
---

# 打开机器人会话
::: note 
从Lark 3.40.0 版本开始支持。
:::
## 使用场景
打开一个机器人的聊天窗。
如果访问用户没有机器人的可用性，将看到相关的引导提示。

## 协议
`https://applink.larksuite.com/client/bot/open`

##  参数

| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|**appId** |    是      | 机器人的appId | 

##  使用示例
打开一个机器人的聊天页面

`https://applink.larksuite.com/client/bot/open?appId=cli_9c21a4767c305107`
