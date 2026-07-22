---
document_id: '7026663896463900677'
directory_id: '7002892512470745094'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-member/intro
breadcrumb:
- Server API
- Group Chat
- Group member
- Resource introduction
document_type: GuideDocumentType
updated_at: 2024-06-05T08:09:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-member/intro
---

# 资源介绍
## 资源定义
群成员是群组内成员（包括用户和机器人）的集合，用于描述群组和成员的关系。

## 字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| &nbsp;<md-text type="field-name" >items</md-text> | <md-text type="field-type" >list_member\[\]</md-text> | member 列表 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >member_id_type</md-text> | <md-text type="field-type" >string</md-text> | 成员的用户 ID 类型，取值为：`open_id`、`user_id`、`union_id`其中之一。 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >member_id</md-text> | <md-text type="field-type" >string</md-text> | 成员的用户ID，ID值与 member_id_type 对应。<br>不同 ID 的说明参见 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction) |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 名字 |


### 数据示例
```json
{
        "items": [
            {
                "member_id_type": "user_id",
                "member_id": "4d7a3c6g",
                "name": "张三"
            }
        ]
}
```
