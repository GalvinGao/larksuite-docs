---
document_id: '7198482854850314246'
directory_id: '7047337547797905414'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/overview-of-message-reaction-resources
breadcrumb:
- Server API
- Messaging
- Message reaction
- Resource introduction
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:54Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/overview-of-message-reaction-resources
---

# 资源介绍


## 资源定义
一条消息的表情回复。

## 字段说明
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: $$$im.v1.message.reaction.method.create.response.body.table.param-column.width$$$;">名称</md-dt-th>
      <md-dt-th style="width: $$$im.v1.message.reaction.method.create.response.body.table.type-column.width$$$;">类型</md-dt-th>
      <md-dt-th style="width: $$$im.v1.message.reaction.method.create.response.body.table.desc-column.width$$$;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >reaction_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	reaction资源ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >operator</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >operator</md-text>
	</md-dt-td>
	<md-dt-td>
		添加reaction的操作人
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >operator_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
		操作人ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >operator_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	操作人身份，用户或应用

**可选值有**：
<md-enum>
<md-enum-item key="app" >"app"</md-enum-item>
<md-enum-item key="user" >"user"</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >action_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
		reaction动作的的unix timestamp(单位:ms)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >reaction_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >emoji</md-text>
	</md-dt-td>
	<md-dt-td>
	reaction资源类型
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >emoji_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	emoji类型，请参考[emoji类型列举](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/emojis-introduce)
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::

### 数据示例
```json
{
    "reaction_id": "ZCaCIjUBVVWSrm5L-3ZTw*************sNa8dHVplEzzSfJVUVLMLcS_",
    "operator": {
        "operator_id": "ou_ff0b7ba35fb********67dfc8b885136",
        "operator_type": "app/user"
    },
    "action_time": "1626086391570",
    "reaction_type": {
        "emoji_type": "SMILE"
    }
}
```
