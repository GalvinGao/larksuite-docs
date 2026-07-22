---
document_id: '7026663890609487878'
directory_id: '7002892512470745094'
title: 用户或机器人主动加入群聊
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/me_join
breadcrumb:
- Server API
- Group Chat
- Group member
- Users or bots join a group voluntarily
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:09:13Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/me_join
---

# 用户或机器人主动加入群聊

用户或机器人主动加入群聊。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=chat.members&method=me_join)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="tip">
注意事项：
- 应用需要开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
- 目前仅支持加入公开群
- 操作内部群时，操作者须与群组在同一租户下
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/im/v1/chats/:chat_id/members/me_join |
| HTTP Method | PATCH |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="im:chat" desc="获取与更新群组信息" support_app_types="custom,isv" tags="">获取与更新群组信息</md-perm><br><md-perm name="im:chat.members:write_only" desc="添加、移除群成员" support_app_types="custom,isv" tags="">添加、移除群成员</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




### 路径参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >chat_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群 ID，详情参见[群ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)

**注意**：
- 仅支持公开（Public）群类型
- 对于已认证企业的Lark的群人数默认上限：普通群5000人，会议群3000人，话题群5000人

**示例值**："oc_a0553eda9014c201e6969b478895c230"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





## 响应





### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	错误码，非 0 表示失败
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >msg</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	错误描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >data</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >\-</md-text>
	</md-dt-td>
	<md-dt-td>
	\-
	</md-dt-td>
</md-dt-tr>


  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "data": {},
    "msg": "ok"
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 232004 | Such an app does NOT exist. | 作为操作者的 app_id 不存在，请联系[技术支持](https://applink.larksuite.com/TLJsX982)。 |
| 400 | 232006 | Your request specifies a chat_id which is invalid. | 无效的 chat_id，请检查chat_id是否正确。 |
| 400 | 232008 | Your request specifies a chat whose type is NOT supported currently. | 不支持的群模式（chat_mode）或群类型（chat_type）。 |
| 400 | 232009 | Your request specifies a chat which has already been dissolved. | 群组已被解散。 |
| 400 | 232010 | Operator and chat can NOT be in different tenants. | 操作者和被操作的群组应该在同一租户下。 |
| 400 | 232013 | You have reached the limit of maximum number of members a chat can have. | 加入群组时，群成员数量已达到上限。对于已认证企业的Lark的群人数默认上限：普通群5000人，会议群3000人，话题群5000人。 |
| 400 | 232019 | The request has been rate limited. | 触发群限流，请控制请求的速度，详情参见[频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)。 |
| 400 | 232025 | Bot ability is not activated. | 应用需要开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。 |
| 400 | 232027 | There are no valid members in the ID list specified in your request. | 成员ID列表为空或不存在有效的成员。 |
| 400 | 232034 | The app is unavailable or inactivated by the tenant. | 应用在本租户下未安装或未启用。 |
| 400 | 232075 | The public group needs to apply to join, and users or bots are not allowed to actively join the group. | 公开群开启进群验证，不能通过API入群。 |





