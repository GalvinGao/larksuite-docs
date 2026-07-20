---
document_id: '7376928275937509382'
directory_id: '7002892512470597638'
title: 话题介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/thread-introduction
breadcrumb:
- Server API
- Messaging
- Message management
- Thread Introduction
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/thread-introduction
---

# 话题介绍

## 什么是“话题”?
:::note
了解更多：[如何使用话题回复？](https://www.larksuite.com/hc/zh-CN/articles/622032823229-reply-to-messages-in-a-thread)
:::

你可以点击消息菜单中的“创建话题”，为群组中的某条消息创建话题并回复，话题下的所有讨论都将聚合在该话题回复中，不再打扰群内其他成员。
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2e1e620e1362e8c1b051a046566a36c5_N06YJnXYfj.png?height=720&lazyload=true&width=1086)
 -   **聚焦讨论：** 沟通和讨论变得更高效聚焦，你也可以在话题中充分了解上下文信息，无需辛苦爬楼翻找。
 -   **减少打扰：** 不关心讨论主题的成员不会收到通知提醒，有效减少打扰，提升信息流转效率。

| 消息形式         | 示例           | 特点        |
| --------- | --------------- | -------   |
|话题 |![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/443309e26ad66e57cfce9069c75bb170_75pIWPsrqe.png?height=626&lazyload=true&width=1166) | 所有消息讨论聚合在该话题的回复中 | 
|消息 |![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/deb299c394f067749ee982d839635f4d_snqfBeI19r.png?height=702&lazyload=true&width=1264) | 每条消息平铺展示 | 






## 什么是“话题模式”群、有哪些应用场景？

**“话题模式”** **群**：如果你希望创建一个“产品反馈收集群”、“销售经验分享群”、“工单信息同步群”来保持相关人员信息同步，可以把普通群升级为“话题模式群”，群里的所有消息都会以话题的方式发送出来，所有回复只能以话题方式回复。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/073930ee5c85993745540558619626d8_Qspj5Ssaws.png?height=1750&lazyload=true&width=1852)

**应用场景**：
- 在工单管理场景中，可以通过话题模式群沉淀每个工单的信息讨论。每个话题对应一个工单信息，可以通过接口将工单信息发送到话题模式群中，工单的上下文讨论围绕话题展开。
- 同理，在项目管理、内容管理、用户反馈收集讨论等场景中，通过话题管理每一个项目、内容、反馈信息可以让讨论更聚焦。

## 话题 ID （thread_id）说明
每一个话题均有一个唯一的话题 ID（thread_id)。`thread_id` 的格式是以 omt_ 开头的字符串，例如：omt_d4be107c616a。 通过 `thread_id`，可以对话题进行操作，如获取话题中的所有消息，转发话题等。

<br>

**如何获取 thread_id ？**

`thread_id` 可通过消息资源获取，若消息资源中存在`thread_id`字段，则说明其是一条话题中的消息；
- **方式一**：在话题模式对话群中，调用 [发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)、[回复消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/reply)、[转发消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/forward) 等接口，从响应的结果中获取 `thread_id` 字段；
- 
- **方式二**：在消息模式对话群中，调用[回复消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/reply)接口，填写参数  `"reply_in_thread": "true"`， 从响应的结果中获取 `thread_id` 字段；

- **方式三**：监听[接收消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/receive)事件，若消息为话题消息，可从事件体中获得message实体的 `thread_id` 字段；


- **方式四**：调用 [获取指定消息的内容](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/get)、[获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list) 接口，若消息为话题消息，可从响应体中获取 `thread_id` 字段；

  
<br>
## “话题”相关的接口能力都有哪些？

  
1. **将群设为话题模式群**
    - 创建群时同步将群设为话题模式：调用[创建群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create)接口时，填写参数`"group_message_type": "thread"`；
    - 将已有的对话群设为话题模式：调用[更新群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/update)接口时，填写参数`"group_message_type": "thread"`；
2. **查询群消息形式**
    - 调用[获取群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/get)接口，响应体中`group_message_type`为`thread`即为话题模式群；若未返回该字段，说明该群聊暂不支持设置群消息形式；
3.  **以话题的方式进行回复（创建话题）**
    - 调用[回复消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/reply)接口时，填写参数`"reply_in_thread": "true"`；
4.  **转发、合并转发**
    - 转发一个话题到其他会话里：调用[转发话题](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/thread/forward)接口；
    - 将一条消息转发到话题里：调用[转发消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/forward)接口时，填写参数`"receive_id_type"： "thread_id"`；
    - 合并转发多条消息到话题里：调用[合并转发消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/merge_forward)时，填写参数`"receive_id_type"： "thread_id"`；
5. **查询话题信息：**
    
    - 获取话题中的所有消息：调用[获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list)接口，填写参数`"container_id_type": "thread"`并在`container_id`中填写话题 ID（thread_id），即可分页获取话题中的所有消息；
    - 获取话题模式群中的所有消息：
        1. 调用[获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list)接口，填写参数`"container_id_type": "chat"`并在`container_id`中填写群 ID（chat_id），即可分页获取群聊中的所有根消息；
        2. 根据 a 中获取到的每一条话题根消息（消息资源中存在`thread_id`字段），再次调用[获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list)接口，填写参数`"container_id_type": "thread"`并在`container_id`中填写话题 ID（thread_id），即可分页获取该话题中的所有消息；
6. **事件通知：**
    - 当群的模式发生改变时通知我：订阅“群配置修改”事件，可以监听到群消息形式（消息形式 & 话题形式）是否发生改变。


## 常见问题
1. **话题可见性**

	若群聊关闭了“新成员可查看历史消息”且话题为操作者进入群聊前创建的，则该话题需要操作者被动订阅才可见，如群内其他用户在话题中@操作者。

2. **话题群和话题模式群的区别**
	::: note
	推荐使用话题模式群。
	:::

	话题群是 `chat_mode` 为 `topic` 的群组，而话题模式群是指群消息形式`group_message_type`为 `thread`、`chat_mode` 为 `group` 的普通对话群。 前者创建后仅支持发送话题，而后者支持在“对话消息”和“话题消息”两种形式间切换。 [创建群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create)接口仅支持创建普通对话群，在设置时可指定群消息模式。














