---
document_id: '6967261389551370245'
directory_id: '6907567266536570881'
title: 事件
full_path: /ukTMukTMukTM/uMTNxYjLzUTM24yM1EjN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Message
- Events
document_type: GuideDocumentType
updated_at: 2021-05-28T08:52:33Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMTNxYjLzUTM24yM1EjN
---

# 机器人和消息会话事件

## 机器人进群
机器人被邀请加入群聊时触发此事件。   

- 依赖条件：应用必须开启了[机器人能力](/document/home/develop-a-bot-in-5-minutes/create-an-app)。

**回调示例：**
```json
{ 
    "ts": "1502199207.7171419", //  事件发送的时间，一般近似于事件发生的时间。 
    "uuid": "bc447199585340d1f3728d26b1c0297a",  // 事件的唯一标识
    "token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
    "type": "event_callback", // 此事件此处始终为event_callback
    "event": { 
        "app_id": "cli_9c8609450f78d102", 
        "chat_i18n_names": { // 群名称国际化字段 
            "en_us": "英文标题", 
            "zh_cn": "中文标题"
        }, 
        "chat_name": "群名称",
        "chat_owner_employee_id": "ca51d83b",// 群主的employee_id（即“用户ID”。如果群主是机器人则没有这个字段，仅企业自建应用返回） 
        "chat_owner_name": "xxx", // 群主姓名
        "chat_owner_open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb", // 群主的open_id 
        "open_chat_id": "oc_e520983d3e4f5ec7306069bffe672aa3",  // 群聊的id
        "operator_employee_id": "ca51d83b", // 操作者的emplolyee_id ，仅企业自建应用返回
        "operator_name": "yyy", // 操作者姓名 
        "operator_open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb",//操作者的open_id 
        "owner_is_bot": false, //群主是否是机器人 
        "tenant_key": "736588c9260f175d",  // 企业标识
        "type": "add_bot" // 事件类型
    }
}

```

## 机器人被移出群
机器人被从群聊中移除时触发此事件。

- 依赖条件：应用必须开启了[机器人能力](/document/home/develop-a-bot-in-5-minutes/create-an-app)。

**回调示例：**
```json
{ 
    "ts": "1502199207.7171419", //  事件发送的时间，一般近似于事件发生的时间。 
    "uuid": "bc447199585340d1f3728d26b1c0297a",  // 事件的唯一标识
    "token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
    "type": "event_callback", // 此事件此处始终为event_callback
    "event": { 
        "app_id": "cli_9c8609450f78d102", 
        "chat_i18n_names": { // 群名称国际化字段 
            "en_us": "英文标题", 
            "zh_cn": "中文标题"
        }, 
        "chat_name": "群名称", 
        "chat_owner_employee_id": "ca51d83b",// 群主的employee_id（即“用户ID”。如果群主是机器人则没有这个字段，仅企业自建应用返回） 
        "chat_owner_name": "xxx", // 群主姓名
        "chat_owner_open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb", // 群主的open_id 
        "open_chat_id": "oc_e520983d3e4f5ec7306069bffe672aa3",  // 群聊的id
        "operator_employee_id": "ca51d83b", // 操作者的emplolyee_id ，仅企业自建应用返回
        "operator_name": "yyy", // 操作者姓名 
        "operator_open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb",//操作者的open_id 
        "owner_is_bot": false, //群主是否是机器人 
        "tenant_key": "736588c9260f175d",  // 企业标识
        "type": "remove_bot" // 移除机器人：remove_bot
        
    }
}
```

## 用户和机器人的会话首次被创建 
首次会话是用户了解应用的重要机会，你可以发送操作说明、配置地址来指导用户开始使用你的应用。

:::warnning
如果是应用商店应用，请务必确保订阅并响应此事件。
:::


**回调示例：**
```json
{ 
    "ts": "1502199207.7171419", //  事件发送的时间，一般近似于事件发生的时间。 
    "uuid": "bc447199585340d1f3728d26b1c0297a",  // 事件的唯一标识
    "token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
    "type": "event_callback", // 此事件此处始终为event_callback
    "event": { 
        "app_id": "cli_9c8609450f78d102",
        "chat_id": "oc_26b66a5eb603162b849f91bcd8815b20", //机器人和用户的会话id
        "operator": { // 会话的发起人。可能是用户，也可能是机器人。
            "open_id": "ou_2d2c0399b53d06fd195bb393cd1e38f2" // 员工对此应用的唯一标识，同一员工对不同应用的open_id不同
            "user_id": "gfa21d92"  // 即“用户ID”，仅企业自建应用会返回 
        },
        "tenant_key": "736588c9260f175c",  // 企业标识 
        "type": "p2p_chat_create",  // 事件类型
        "user": {  // 会话的用户
            "name": "张三",
            "open_id": "ou_7dede290d6a27698b969a7fd70ca53da",  // 员工对此应用的唯一标识，同一员工对不同应用的open_id不同
            "user_id": "gfa21d92" // 即“用户ID”，仅企业自建应用会返回 
        }
    }
}
```


## 接收消息
当用户发送消息给机器人或在群聊中@机器人时触发此事件。

- 依赖权限：==获取用户发给机器人的私聊消息== 、 ==获取群聊中用户 @ 机器人的消息==。开启了相应的权限才能获取到相应的消息。
- 其他条件：应用必须开启了[机器人能力](/document/home/develop-a-bot-in-5-minutes/create-an-app)。

### 文本消息 

**回调示例：**
```json
{ 
    "uuid": "41b5f371157e3d5341b38b20396e77e3", 
    "token": "2g7als3DgPW6Xp1xEpmcvgVhQG621bFY",//校验Token 
    "ts": "1550038209.428520",  //时间戳 
    "type": "event_callback",//事件回调此处固定为event_callback 
    "event": { 
        "type": "message", // 事件类型 
        "app_id": "cli_xxx", 
        "tenant_key": "xxx", //企业标识 
        "root_id": "", 
        "parent_id": "", 
        "open_chat_id": "oc_5ce6d572455d361153b7cb51da133945", 
        "chat_type": "private", //私聊private，群聊group 
        "msg_type": "text",    //消息类型 
        "open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb", 
        "employee_id": "xxx", // 即“用户ID”，仅企业自建应用会返回 
        "union_id": "xxx", 
        "open_message_id": "om_36686ee62209da697d8775375d0c8e88", 
        "is_mention": false, 
        "text": "<at open_id="xxx">@小助手</at> 消息内容 <at open_id="yyy">@张三</at>",      // 消息文本，可能包含被@的人/机器人。
        "text_without_at_bot":"消息内容 <at open_id="yyy">@张三</at>" //消息内容，会过滤掉at你的机器人的内容 
   } 
} 
```
### 富文本和 post 消息 

**回调示例：**
```json
{ 
    "uuid": "bc447199585340d1f3728d26b1c0297a", 
    "token": "2g7als3DgPW6Xp1xEpmcvgVhQG621bFY", 
    "ts": "1550032496.527917", 
    "type": "event_callback", 
    "event": { 
        "type": "message", 
        "app_id": "cli_xxx", 
        "tenant_key": "xxx", //企业标识 
        "root_id": "", 
        "parent_id": "", 
        "open_chat_id": "oc_5ce6d572455d361153b7cb51da133945",//发消息的open_chat_id 
        "chat_type": "private", //私聊private，群聊group
        "msg_type": "post",// rich_text和post消息这里统一都是post 
        "open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb",//发消息的用户open_id 
        "employee_id": "xxx", // 即“用户ID”，仅企业自建应用会返回 
        "union_id": "xxx",//发消息的用户union_id 
        "open_message_id": "om_a81fa00ee467b1084ffd80b197b58f4b",//消息id 
        "is_mention": false, 
        "text": "\u003cp\u003e测试1212\u003c/p\u003e\u003cfigure\u003e\u003cimg src=\"http://p4.pstatp.com/origin/daff000d4967d033705b\" origin-width=\"2456\" origin-height=\"2458\"/\u003e\u003c/figure\u003e",//消息内容 
        "text_without_at_bot":"消息内容",//消息内容，会过滤掉at你的机器人的内容 
        "title": "测试" ,//消息标题 
        "image_keys": [ //富文本里面的图片的keys
           "1867eac8-8006-40be-8549-b7beae0d3c4a",
           "434593af-5269-4db4-8b94-65c6dfd4f35e"
         ],
  } 
} 
```
### 图片image消息

**回调示例：** 
```json
{ 
    "uuid": "c58e17e9e84824a48e51a562cf969fb3", 
    "token": "2g7als3DgPW6Xp1xEpmcvgVhQG621bFY", 
    "ts": "1550038110.478493", 
    "type": "event_callback", 
    "event": { 
        "type": "message", 
        "app_id": "cli_xxx", 
        "tenant_key": "xxx", //企业标识 
        "root_id": "", 
        "parent_id": "", 
        "open_chat_id": "oc_5ce6d572455d361153b7cb51da133945", 
        "chat_type": "private", //私聊private，群聊group
        "msg_type": "image", //图片消息 
        "image_height" :"300",
        "image_width" :"300",
        "open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb", 
        "employee_id": "xxx", // 即“用户ID”，仅企业自建应用会返回 
        "union_id": "xxx", 
        "open_message_id": "om_340057d660022bf141eb470859c6114c", 
        "is_mention": false, 
        "image_key": "cd1ce282-94d1-4154-a326-121b07e4721e", // image_key，获取图片内容请查/ssl:ttdoc/ukTMukTMukTM/uYzN5QjL2cTO04iN3kDN

  } 
} 
```
### 文件file消息

**回调示例：** 
```json
{
    "uuid": "8bbb4b5ba77c316991c41fa9ef0ced8b",
    "token": "ZYcIUHYh8lmZen6jStLgvcXAXbqn2tle",
    "ts": "1576725930.795192",
    "type": "event_callback",
    "event": {
        "app_id": "cli_xxx",
        "tenant_key": "xxx",
        "chat_type": "private", //私聊private，群聊group
        "is_mention": false,
        "msg_type": "file",
        "open_chat_id": "oc_a1e061372f7745a543dsd5h3d6d2349a",
        "open_id": "ou_2b940d169b7a4a0c76633984b08ced73",
        "employee_id": "xxx", // 即“用户ID”，仅企业自建应用会返回 
        "union_id": "xxx",
        "open_message_id": "om_d85c4sd7b209tbb98g693a958bc7185f",
        "parent_id": "",
        "root_id": "",
        "type": "message",
        "file_key": "file_b4do9r9b-3526-4bc4-a568-65fe3695b05g"
  }
}
```
### 合并转发消息内容 

**回调示例：**
```json
{ 
    "uuid": "d465df13905458e361ff39af85d96d09", 
    "token": "2g7als3DgPW6Xp1xEpmcvgVhQG621bFY", 
    "ts": "1550111453.764646", 
    "type": "event_callback", 
    "event": { 
        "type": "message", 
        "app_id": "cli_xxx", 
        "tenant_key": "xxx", //企业标识    
        "root_id": "", 
        "parent_id": "", 
        "open_chat_id": "oc_5ce6d572455d361153b7cb51da133945", 
        "msg_type": "merge_forward", 
        "open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb", 
        "employee_id": "xxx", // 即“用户ID”，仅企业自建应用会返回 
        "union_id": "xxx",
        "open_message_id": "om_b3961b120d67259e7495d8eb69488189", 
        "is_mention": false, 
        "chat_type": "private", //私聊private，群聊group
        "user": "6610187460791558158", 
        "msg_list": [ 
        { 
            "root_id": "", 
            "parent_id": "", 
            "open_chat_id": "oc_b74c59c68d0f2d0ac65846272499d651", 
            "msg_type": "image", 
            "open_id": "", 
            "open_message_id": "be1000265b014075a869134b20c87633", 
            "is_mention": false, 
            "image_key": "99295878-5e85-41a3-bb00-0ad63b5b156d", 
            "image_url": "https://oapi-staging.zjurl.cn/open-apis/api/v2/file/f/99295878-5e85-41a3-bb00-0ad63b5b156d", 
            "create_time": 1550044148
       }, 
       { 
            "root_id": "", 
            "parent_id": "", 
            "open_chat_id": "oc_b74c59c68d0f2d0ac65846272499d651", 
            "msg_type": "text", 
            "open_id": "", 
            "open_message_id": "om_a96c620f2aa036e3c08abebaec7f09d1", 
            "is_mention": false, 
            "text": "文本1", 
            "create_time": 1550044749
       }, 
       { 
            "root_id": "", 
            "parent_id": "", 
            "open_chat_id": "oc_b74c59c68d0f2d0ac65846272499d651", 
            "msg_type": "post", 
            "open_id": "", 
            "open_message_id": "om_5d5b1732aa9b997dff23d63146427bb2", 
            "is_mention": false, 
            "text": "\u003cp\u003e富文本内容\u003c/p\u003e\u003cfigure\u003e\u003cimg src=\"http://p2.pstatp.com/origin/dad90010d9c8fc72f0b0\" origin-width=\"888\" origin-height=\"888\"/\u003e\u003c/figure\u003e", 
            "title": "富文本", 
            "create_time": 1550044772
       } 
       ] 
   } 
}
```

## 消息已读 
用户阅读机器人发送的消息后触发此事件。


**回调示例：**
```json
{
    "ts": "1502199207.7171419", //  事件发送的时间，一般近似于事件发生的时间。 
    "uuid": "bc447199585340d1f3728d26b1c0297a",  // 事件的唯一标识
    "token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
    "type": "event_callback", // 此事件此处始终为event_callback
	"event": {
		"app_id": "cli_9c8609450f78d102",
		"open_chat_id": "oc_e520983d3e4f5ec7306069bffe672aa3",
		"open_id": "ou_2d2c0399b53d06fd195bb393cd1e38f2",              
		"open_message_ids": ["om_dc13264520392913993dd051dba21dcf"],   // 已读消息列表
		"tenant_key": "xxx",
		"type": "message_read"
	}
}
```




