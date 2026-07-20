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
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: 40%;">名称</md-th>
      <md-th style="width: 20%;">类型</md-th>
      <md-th style="width: 30%;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >items</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >list_member\[\]</md-text>
	</md-td>
	<md-td>
	member 列表
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >member_id_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	成员的用户 ID 类型，取值为：`open_id`、`user_id`、`union_id`其中之一。
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >member_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	成员的用户ID，ID值与 member_id_type 对应。

不同 ID 的说明参见 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	名字
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::

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
