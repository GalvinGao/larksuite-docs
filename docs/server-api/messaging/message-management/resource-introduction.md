---
document_id: '7026663890609274886'
directory_id: '7002892512470597638'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro
breadcrumb:
- Server API
- Messaging
- Message management
- Resource introduction
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro
---

# 资源介绍


## 资源定义
Lark聊天中的一条消息。

## 字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| &nbsp;<md-text type="field-name" >message_id</md-text> | <md-text type="field-type" >string</md-text> | 消息id，说明参见：[消息ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2) |
| &nbsp;<md-text type="field-name" >root_id</md-text> | <md-text type="field-type" >string</md-text> | 根消息id，用于回复消息场景，说明参见：[消息ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2) |
| &nbsp;<md-text type="field-name" >parent_id</md-text> | <md-text type="field-type" >string</md-text> | 父消息的id，用于回复消息场景，说明参见：[消息ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2) |
| &nbsp;<md-text type="field-name" >msg_type</md-text> | <md-text type="field-type" >string</md-text> | 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等，类型定义请参考：[发送消息Content](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json) |
| &nbsp;<md-text type="field-name" >create_time</md-text> | <md-text type="field-type" >string</md-text> | 消息生成的时间戳（毫秒） |
| &nbsp;<md-text type="field-name" >update_time</md-text> | <md-text type="field-type" >string</md-text> | 消息更新的时间戳（毫秒） |
| &nbsp;<md-text type="field-name" >deleted</md-text> | <md-text type="field-type" >boolean</md-text> | 消息是否被撤回 |
| &nbsp;<md-text type="field-name" >updated</md-text> | <md-text type="field-type" >boolean</md-text> | 消息是否被更新 |
| &nbsp;<md-text type="field-name" >chat_id</md-text> | <md-text type="field-type" >string</md-text> | 所属的群 |
| &nbsp;<md-text type="field-name" >sender</md-text> | <md-text type="field-type" >sender</md-text> | 发送者，可以是用户或应用 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 该字段标识发送者的id |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >id_type</md-text> | <md-text type="field-type" >string</md-text> | 该字段标识发送者的id类型，如`app_id`、`open_id` |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >sender_type</md-text> | <md-text type="field-type" >string</md-text> | 该字段标识发送者的类型，可选值有`user`， `app` |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >tenant_key</md-text> | <md-text type="field-type" >string</md-text> | 租户在Lark上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识 |
| &nbsp;<md-text type="field-name" >body</md-text> | <md-text type="field-type" >message_body</md-text> | 消息内容 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >content</md-text> | <md-text type="field-type" >string</md-text> | 消息内容，json结构，不同msg_type对应不同内容。消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等，具体格式说明参考：[发送消息Content](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json) |
| &nbsp;<md-text type="field-name" >mentions</md-text> | <md-text type="field-type" >mention\[\]</md-text> | 被@的人或机器人的id列表 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >key</md-text> | <md-text type="field-type" >string</md-text> | 被@的用户或机器人的序号。例如，第3个被@到的成员，值为“@_user_3” |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 被@的用户或者机器人的 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >id_type</md-text> | <md-text type="field-type" >string</md-text> | 被@的用户或机器人 id 类型，如`open_id` ([什么是 Open ID？](/document/home/user-identity-introduction/open-id)) |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 被@的用户或机器人的姓名 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >tenant_key</md-text> | <md-text type="field-type" >string</md-text> | tenant key，为租户在Lark上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识 |
| &nbsp;<md-text type="field-name" >thread_id</md-text> | <md-text type="field-type" >string</md-text> | 消息所属的话题 ID（不返回说明该消息非话题消息），可通过该 ID 实现转发话题、获取话题中的所有消息等，请参见：[话题介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/thread-introduction) |
| &nbsp;<md-text type="field-name" >upper_message_id</md-text> | <md-text type="field-type" >string</md-text> | 合并转发消息中，上一层级的消息id message_id，说明参见：[消息ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2) |


### 数据示例
```json
{
        "message_id": "om_dc13264520392913993dd051dba21dcf",
        "root_id": "om_40eb06e7b84dc71c03e009ad3c754195",
        "parent_id": "om_d4be107c616aed9c1da8ed8068570a9f",
        "msg_type": "card",
        "create_time": "1615380573411",
        "update_time": "1615380573411",
        "deleted": false,
        "updated": false,
        "chat_id": "oc_5ad11d72b830411d72b836c20",
        "sender": {
            "id": "cli_9f427eec54ae901b",
            "id_type": "app_id",
            "sender_type": "app",
            "tenant_key": "736588c9260f175e"
        },
        "body": {
            "content": "{\"text\":\"@_user_1 test content\"}"
        },
        "mentions": [
            {
                "key": "@_user_1",
                "id": "ou_155184d1e73cbfb8973e5a9e698e74f2",
                "id_type": "open_id",
                "name": "Tom",
                "tenant_key": "736588c9260f175e"
            }
        ],
        "thread_id": "omt_16f3c7e1268f1749",
        "upper_message_id": "om_40eb06e7b84dc71c03e009ad3c754195"
}
```

## 消息相关ID说明

### 1、message_id 说明
`message_id` 是一条消息的唯一标识。当发送一条消息时，系统会自动生成唯一ID。
`message_id` 的格式是以om_ 开头的字符串，例如：om_934be5776f5a87239a298af9e74c0f72

<br>
**如何获取message_id?**

- 方式一：调用 [发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create) 接口，从响应的结果中获取 `message_id` 字段；

- 方式二：监听 [接收消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/receive)事件，从事件体中获得message实体的 `message_id` 字段


- 方式三：调用 [获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list) 接口，从响应的结果中获取 `message_id` 字段

### 2、root_id 与 parent_id 说明
- `root_id`：如果某一条消息回复了其他的消息，则`root_id`为回复树上根消息的`message_id`
- `parent_id`: 如果某一条消息回复了另一条消息，则`parent_id`为被回复的这条消息的`message_id`
- 注意：话题群帖子内的回复消息，都是回复根消息，所以`root_id`和`parent_id`都是根消息ID。


<br>
**以下图的消息回复举例：**
- msg2 回复了 msg1，则 msg2 的 `root_id` 和 `parent_id` 均为 msg1 的 `message_id`；
- msg3 回复了msg2，则 msg3的`root_id` 是 msg1 的`message_id`，`parent_id` 是 msg2 的`message_id`

![Group 1321314312.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/08d741da23cd931118c332941e5cc117_wvecM5EteV.png?height=654&lazyload=true&width=1640)
### 3、upper_message_id 说明
`upper_message_id` 是合并转发消息中子消息所在的合并转发消息的 `message_id`。

<br>
**以下图的消息回复举例：**

msg1，msg2，msg3是合并转发消息中的三条子消息，它们的`upper_message` 即为所在的合并转发消息的`message_id`。


![Group 1321314316.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b51867811fc500dbe5ef129f8bd03149_1eOUU1X0E9.png?height=817&lazyload=true&width=1640)
