---
document_id: '7026663896463933445'
directory_id: '7012181176438243333'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-announcement/intro
breadcrumb:
- Server API
- Group Chat
- Group announcement
- Resource introduction
document_type: GuideDocumentType
updated_at: 2024-06-05T08:09:17Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-announcement/intro
---

# 资源介绍
## 资源定义
群公告是群组中的公告文档。群公告采用Lark云文档承载，每个群组只有一个群公告。

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
	&nbsp;<md-text type="field-name" >content</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	云文档序列化信息
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >revision</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文档当前版本号 纯数字
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >create_time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文档生成的时间戳（秒）
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >update_time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文档更新的时间戳（秒）
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >owner_id_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文档所有者的 ID 类型

- 如果所有者是用户，取值为`open_id` `user_id` `union_id` 其中之一，不同 ID 的说明参见 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)
- 如果所有者是机器人，为机器人应用的 `app_id`，详情参见[通用参数](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology)


**可选值有：**
- `user_id`：以 user_id 来识别用户
- `union_id`：以 union_id 来识别用户
- `open_id`：以 open_id 来识别用户
- `app_id`：以 app_id 来识别机器人应用
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >owner_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文档所有者 ID，ID 值与owner_id_type 中的ID类型对应
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >modifier_id_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文档最新修改者 id 类型

 - 如果修改者是用户，取值为`open_id` `user_id` `union_id` 其中之一，不同 ID 的说明参见 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)
- 如果修改者是机器人，为机器人应用的 `app_id`，详情参见[通用参数](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology)

**可选值有：**
- `user_id`：以 user_id 来识别用户
- `union_id`：以 union_id 来识别用户
- `open_id`：以 open_id 来识别用户
- `app_id`：以 app_id 来识别应用
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >modifier_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文档最新修改者 ID，ID 值与 modifier_id_type 中的ID类型对应
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::
### 数据示例
```json
{
        "content": "{\"title\":{\"elements\":[{\"type\":\"textRun\",\"textRun\":{\"text\":\"announcement\",\"style\":{},\"location\":{\"zoneId\":\"0\",\"startIndex\":0,\"endIndex\":12}}}],\"location\":{\"zoneId\":\"0\",\"startIndex\":0,\"endIndex\":12},\"lineId\":\"4m10Rp\"},\"body\":{\"blocks\":[{\"type\":\"paragraph\",\"paragraph\":{\"elements\":[{\"type\":\"textRun\",\"textRun\":{\"text\":\"Announcement.\",\"style\":{},\"location\":{\"zoneId\":\"0\",\"startIndex\":13,\"endIndex\":26}}}],\"location\":{\"zoneId\":\"0\",\"startIndex\":13,\"endIndex\":26},\"lineId\":\"5VYRPT\"}},{\"type\":\"paragraph\",\"paragraph\":{\"elements\":[],\"location\":{\"zoneId\":\"0\",\"startIndex\":27,\"endIndex\":27}}}]}}",
        "revision": "12",
        "create_time": "1609296809",
        "update_time": "1609296809",
        "owner_id_type": "open_id",
        "owner_id": "ou_7d8a6e6df7621556ce0d21922b676706ccs",
        "modifier_id_type": "open_id",
        "modifier_id": "ou_7d8a6e6df7621556ce0d21922b676706ccs"
}
```
