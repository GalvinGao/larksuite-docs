---
document_id: '7026663896463523845'
directory_id: '7136113559830396933'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-info/intro
breadcrumb:
- Server API
- Group Chat
- Group management
- Resource introduction
document_type: GuideDocumentType
updated_at: 2024-06-05T08:09:03Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/im-v1/chat/chat-info/intro
---

# 资源介绍
## 资源定义
Lark中的聊天群组。

## 字段说明
:::note
以下字段并非所有类型的群组都具备，部分群组可能缺失某些字段
:::
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
	&nbsp;<md-text type="field-name" >chat_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	群 ID，详情参见[群ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >avatar</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	群头像 URL
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	群名称
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >description</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	群描述
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >i18n_names</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >i18n_names</md-text>
	</md-td>
	<md-td>
	群国际化名称
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >zh_cn</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	中文名
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >en_us</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	英文名
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ja_jp</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	日文名
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
	群主 ID，ID值与 owner_id_type 对应。

不同 ID 的说明参见 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)。

当群主是机器人时，该字段不返回
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
	群主 ID 对应的ID类型，取值为：`open_id`、`user_id`、`union_id`其中之一。不同 ID 的说明参见 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)。

当群主是机器人时，该字段不返回
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >add_member_permission</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	拉 用户或机器人 入群权限

**可选值有**：
- `only_owner`：仅群主和管理员
- `all_members`：所有成员
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >share_card_permission</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	群分享权限

**可选值有**：
- `allowed`：允许
- `not_allowed`：不允许
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >at_all_permission</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	at 所有人权限

**可选值有**：
- `only_owner`：仅群主和管理员
- `all_members`：所有成员
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >edit_permission</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	群编辑权限

**可选值有**：
- `only_owner`：仅群主和管理员
- `all_members`：所有成员
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >chat_mode</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	群模式

**可选值有**：
- `group`：群组
- `topic`：话题群
- `p2p`：单聊
	</md-td>
</md-tr>
    
    
<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >group_message_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	群消息形式

**可选值有**：
- `chat`：对话消息
- `thread`：话题消息
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >chat_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	群类型

**可选值有**：
- `private`：私有群
- `public`：公开群
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >chat_tag</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	群标签，如有多个，则按照下列顺序返回第一个

**可选值有**：
- `inner`：内部群
- `tenant`：公司群
- `department`：部门群
- `edu`：教育群
- `meeting`：会议群
- `customer_service`：客服群
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >external</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	是否是外部群
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >tenant_key</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	租户在Lark上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >join_message_visibility</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	入群消息可见性

**可选值有**：
- `only_owner`：仅群主和管理员可见
- `all_members`：所有成员可见
- `not_anyone`：任何人均不可见
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >leave_message_visibility</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	出群消息可见性

**可选值有**：
- `only_owner`：仅群主和管理员可见
- `all_members`：所有成员可见
- `not_anyone`：任何人均不可见
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >membership_approval</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	加群审批

**可选值有**：
- `no_approval_required`：无需审批
- `approval_required`：需要审批
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&nbsp;<md-text type="field-name" >moderation_permission</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	发言权限

**可选值有**：
- `only_owner`：仅群主和管理员
- `all_members`：所有成员
- `moderator_list`：指定群成员
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::


### 数据示例
```json
{
        "chat_id": "oc_a0553eda9014c201e6969b478895c230",
        "avatar": "https://p3-lark-file.byteimg.com/img/lark-avatar-staging/default-avatar_44ae0ca3-e140-494b-956f-78091e348435~100x100.jpg",
        "name": "测试群名称",
        "description": "测试群描述",
        "i18n_names": {
            "zh_cn": "群聊",
            "en_us": "group chat",
            "ja_jp": "グループチャット"
        },
        "owner_id": "4d7a3c6g",
        "owner_id_type": "user_id",
        "add_member_permission": "all members",
        "share_card_permission": "allowed",
        "at_all_permission": "all members",
        "edit_permission": "all members",
        "group_message_type": "chat",
        "chat_mode": "group",
        "chat_type": "private",
        "chat_tag": "inner",
        "external": false,
        "tenant_key": "736588c9260f175e",
        "join_message_visibility": "all_members",
        "leave_message_visibility": "all_members",
        "membership_approval": "no_approval_required",
        "moderation_permission": "all_members",
}
```


## 群 ID 说明
参见文档 [群ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
