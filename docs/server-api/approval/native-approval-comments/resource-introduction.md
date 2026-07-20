---
document_id: '7139727756257968134'
directory_id: '7122028361538961414'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/overview
breadcrumb:
- Server API
- Approval
- Native approval comments
- Resource introduction
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:54Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/overview
---

# 资源介绍
员工在审批实例中进行的评论或评论回复。

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: ;">名称</md-dt-th>
      <md-dt-th style="width: ;">类型</md-dt-th>
      <md-dt-th style="width: ;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >comment_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	评论ID，如果是编辑、删除一条评论，需要传

**示例值**："7081516627711524883"
	</md-dt-td>
</md-dt-tr>
    
<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >parent_comment_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	父评论ID，如果是回复评论，需要传

**示例值**："7081516627711524883"
	</md-dt-td>
</md-dt-tr>    

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	评论内容，包含艾特人、附件等

**示例值**："{"text":"来自小程序的评论111我带附件中有extra ","files":[{"url":"xxx","fileSize":155149,"title":"9a9fedc5cfb01a4a20c715098.png","type":"image","extra":""}]}"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >at_info_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >comment_at_info\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
	评论中艾特人信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	被艾特人的ID

**示例值**："579fd9c4"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	被艾特人的姓名

**示例值**："张某"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >offset</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	被艾特人在评论中的位置，从0开始

**示例值**："1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >disable_bot</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	disable_bot=true只同步数据，不触发bot

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >extra</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
		附加字段

**示例值**："{\"a\":\"a\"}"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::

## 数据示例
```json
{
    "content": "{\"text\":\"agree\",\"files\":[{\"url\":\"xxx\",\"fileSize\":155149,\"title\":\"9a9fedc5cfb01a4a20c715098.png\",\"type\":\"image\",\"extra\":\"\"}]}",
    "at_info_list": [
        {
            "user_id": "579fd9c4",
            "name": "zhangsan",
            "offset": "1"
        }
    ],
    "parent_comment_id": "7081516627711524883",
    "comment_id": "7081516627711524883",
    "disable_bot": false,
    "extra": "{\"a\":\"a\"}"
}
```

## 用户ID说明
了解user_id，open_id，union_id的区别和用途，参见教程 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)
