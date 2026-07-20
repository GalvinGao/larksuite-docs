---
document_id: '7400009101458571270'
directory_id: '7403322981060493318'
title: 消息与群组「更新应用创建群聊的信息」、「读取群信息」等权限点下线
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/platform-updates-/message-and-group-scope-removed
breadcrumb:
- Developer Guides
- Platform Notices
- Platform Updates
- Message and group scope such as "Update the information of groups created by the app", "Read group information" will be sunset
document_type: GuideDocumentType
updated_at: 2024-08-20T03:26:34Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/platform-updates-/message-and-group-scope-removed
---

# 消息与群组「更新应用创建群聊的信息」、「读取群信息」等权限点下线

亲爱的开发者，你好！

我们将于**2024年9月30日**停止「获取用户发给机器人的单聊消息（im:message.p2p_msg）」、「获取用户在群组中@机器人的消息（im:message.group_at_msg）」、「更新应用创建群聊的信息（im:message.groups）」、「读取群信息（im:chat.group_info:readonly）」四个权限点的申请，对于已经获得了此权限点的应用不受影响。

## 下线原因

「获取用户发给机器人的单聊消息（im:message.p2p_msg）」、「获取用户在群组中@机器人的消息（im:message.group_at_msg）」、「更新应用创建群聊的信息（im:message.groups）」、「读取群信息（im:chat.group_info:readonly）」为重复权限，可使用其他权限进行替换。

## 下线后的解决方案

- 原「获取用户发给机器人的单聊消息（im:message.p2p_msg）」权限点控制的接口和事件，可申请权限「读取用户发给机器人的单聊消息（ im:message.p2p_msg:readonly）」。
- 原「获取用户在群组中@机器人的消息（im:message.group_at_msg）」权限点控制的接口和事件，可申请权限「接收群聊中@机器人消息事件（im:message.group_at_msg:readonly）」。
- 原「更新应用创建群聊的信息（im:message.groups）」权限点控制的接口和事件，可申请权限「更新应用所创建群的群信息（im:chat:operate_as_owner）」。
- 原「读取群信息（im:chat.group_info:readonly）」权限点控制的接口和事件，可根据具体接口权限要求申请权限「查看群信息（im:chat:read）」或「查看群成员（im:chat.members:read）」。

## 涉及的接口

- [接收消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/receive)
- [解散群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/delete)
- [更新群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/update)
- [更新群发言权限](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-moderation/update)
- [将用户或机器人拉入群聊](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/create)
- [将用户或机器人移出群聊](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/delete)
- [更新群公告信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/patch)
- [判断用户或机器人是否在群里](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/is_in_chat)
- [搜索对用户或机器人可见的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/search)
- [获取用户或机器人所在的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/list)
- [获取用户所在的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/list)
- [获取群成员列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/get)
- [获取群成员列表](/document/ukTMukTMukTM/uUzMwUjL1MDM14SNzATN)
