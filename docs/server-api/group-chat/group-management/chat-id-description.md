---
document_id: '7026663896463360005'
directory_id: '7136113559830396933'
title: 群 ID 说明
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description
breadcrumb:
- Server API
- Group Chat
- Group management
- Chat ID description
document_type: GuideDocumentType
updated_at: 2024-06-05T08:09:07Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description
---

# 群 ID 说明
## 什么是chat_id？
`chat_id`是一个群聊的唯一标识。当创建一个群聊的时候，系统会自动生成该ID。

`chat_id`的格式是由`oc_`开头的字符串，例如：`oc_a0553eda9014c201e6969b478895c230`


## 如何获取群ID？
### 场景1 已经存在的群

1. 创建一个应用，并开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)；
2. 把机器人加入群中；
3. 调用 [获取用户或机器人所在的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/list) 获得`chat_id`;
4. 用上面的`chat_id`作为参数[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)或者[
将用户或机器人拉入群聊](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/create)等。    

### 场景2 机器人新建群 
1. 创建一个应用，并开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)；
2. 用机器人身份建群，调用接口[创建群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create)，从响应结果中获得`chat_id`参数；
3. 用上面的`chat_id`作为参数[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)或者[
将用户或机器人拉入群聊](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/create)等。
