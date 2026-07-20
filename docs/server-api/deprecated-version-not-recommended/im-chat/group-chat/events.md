---
document_id: '6967261316286758917'
directory_id: '6907567266536652801'
title: 事件
full_path: /ukTMukTMukTM/ukjNxYjL5YTM24SO2EjN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Group Chat
- Events
document_type: GuideDocumentType
updated_at: 2021-05-28T08:52:34Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukjNxYjL5YTM24SO2EjN
---

# 群聊事件

如果你希望订阅机器人进出群、群内有人@机器人事件，请前往[机器人和消息会话事件](/document/ukTMukTMukTM/uMTNxYjLzUTM24yM1EjN)。

## 用户进群、出群事件
用户进群、出群后触发此事件。

* 特殊说明：只有开启机器人能力并且机器人所在的群发生上述变化时才能触发此事件。

事件包括三个类型：
1. 用户进群 - add_user_to_chat
2. 用户出群 - remove_user_from_chat
3. 撤销加人 - revoke_add_user_from_chat

**回调示例：**
```json
{
  "ts": "1502199207.7171419", // 事件发送的时间，一般近似于事件发生的时间。 
  "uuid": "bc447199585340d1f3728d26b1c0297a", // 事件的唯一标识
  "token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
  "type": "event_callback", // 此事件此处始终为event_callback
  "event": {
    "app_id": "cli_9c8609450f78d102",
    "chat_id": "oc_9e9619b938c9571c1c3165681cdaead5", // 群聊的id
    "operator": {
      // 用户进出群的操作人。用户主动退群的话，operator 就是user自己
      "open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb", // 员工对此应用的唯一标识，同一员工对不同应用的open_id不同
      "user_id": "ca51d83b" // 即“用户ID”，仅企业自建应用会返回 
    },
    "tenant_key": "736588c9260f175d",
    "type": "add_user_to_chat", // 事件类型，add_user_to_chat/remove_user_from_chat/revoke_add_user_from_chat
    "users": [{
        "name": "James",
        "open_id": "ou_706adeb944ab1473b9fb3e7da2a40b68",
        "user_id": "51g97a4g"
      },
      {
        "name": "张三",
        "open_id": "ou_7885357f9922aaa34001b190109e0b48",
        "user_id": "6e125386"
      }
    ]
  }
}
```

##  解散群
群聊被解散后触发此事件。

* 特殊说明：只有开启机器人能力并且机器人所在的群被解散时才能触发此事件。

**回调示例：**
```json
{
	"ts": "1502199207.7171419", // 事件发送的时间，一般近似于事件发生的时间。 
	"uuid": "bc447199585340d1f3728d26b1c0297a", // 事件的唯一标识
	"token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
	"type": "event_callback", // 此事件此处始终为event_callback
	"event": {
		"app_id": "cli_9c8609450f78d102",
		"chat_id": "oc_9f2df3c095c9395334bb6e70ced0fa83",
		"operator": {
			"open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb",
			"user_id": "ca51d83b"
		},
		"tenant_key": "736588c9260f175d",
		"type": "chat_disband"
	}
}

```

##  群配置修改
在以下情况下会触发此事件：
|用户行为|after_change/before_change 包括的字段|
|--|--|
|转移群主|owner_user_id（仅自建应用才有）、owner_open_id|
|群成员修改“消息提醒”配置|message_notification|
|群主修改“仅群主可添加群成员”配置|add_member_permission|

* 特殊说明：只有开启机器人能力并且机器人所在的群发生上述变化时才能触发此事件。

**回调示例：**
```json
{
  "ts": "1502199207.7171419", //  事件发送的时间，一般近似于事件发生的时间。 
  "uuid": "bc447199585340d1f3728d26b1c0297a", // 事件的唯一标识
  "token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
  "type": "event_callback", // 此事件此处始终为event_callback
  'event': {
    'tenant_key': '2d520d3b434f175e',
    'type': 'group_setting_update'
    'app_id': 'cli_9e28cb7ba56a100e',
    'chat_id': 'oc_066cad06159f0752fe02c9af8aebfc5a',
    'after_change': {
      // 当群主发生变化时会有下面2个字段。若群主未发生变化，则不会有这2个字段。
      'owner_open_id': 'ou_b5f49f047428e47925599be05e3255f6',
      'owner_user_id': '35g25a59',
      // 当“仅群主可添加群成员”配置变化时有下面的字段。
      'add_member_permission': 'everyone',
      // 当“消息提醒”配置变化时有下面的字段。
	  'message_notification': False
    },
    'before_change': {
      // after_change里有几个字段，这里就有几个与之对应。
    },
    'operator': { // 配置变化的操作者
      'open_id': 'ou_55a50f4c6eba6adbfc8b94803fe78825',
      'user_id': 'g6964gd3'
    }
  }
}
```
