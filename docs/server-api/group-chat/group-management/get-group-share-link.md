---
document_id: '7198482854850265094'
directory_id: '7136113559830396933'
title: 获取群分享链接
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/link
breadcrumb:
- Server API
- Group Chat
- Group management
- Get group share link
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:09:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/link
---

# 获取群分享链接

获取指定群的分享链接。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=chat&method=link)

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
注意事项:
- 应用需要开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
 - access_token所对应的 **机器人** 或 **授权用户** 必须在`chat_id`参数指定的群组中
- 单聊、密聊、团队群不支持分享群链接
- 当Bot被停用或Bot退出群组时，Bot生成的群链接也将停用
- 当群聊开启了 ==仅群主和群管理员可添加群成员/分享群== 设置时，仅**群主**和**群管理员**可以获取群分享链接
- 获取内部群分享链接时，操作者须与群组在同一租户下
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/im/v1/chats/:chat_id/link |
| HTTP Method | POST |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="im:chat" desc="获取与更新群组信息" support_app_types="custom,isv" tags="">获取与更新群组信息</md-perm><br><md-perm name="im:chat:read" desc="查看群信息" support_app_types="custom,isv" tags="">查看群信息</md-perm><br><md-perm name="im:chat:readonly" desc="获取群组信息" support_app_types="custom,isv" tags="">获取群组信息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




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
	待获取分享链接的群ID，详情参见[群ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)

**注意**：单聊、密聊、团队群不支持分享群链接

**示例值**："oc_a0553eda9014c201e6969b478895c230"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 请求体

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >validity_period</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	群分享链接有效时长，可选值week、year、permanently，分别表示7天、1年以及永久有效

**示例值**："week"

**可选值有**：
<md-enum>
<md-enum-item key="week" >有效期7天</md-enum-item>
<md-enum-item key="year" >有效期1年</md-enum-item>
<md-enum-item key="permanently" >永久有效</md-enum-item>
</md-enum>

**默认值**：`week`
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "validity_period": "week"
}
</md-code-json>
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


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >share_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群分享链接
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >expire_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	分享链接过期时间戳（秒级）
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >is_permanent</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	分享链接是否永久有效
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
    "msg": "success",
    "data": {
        "share_link": "https://applink.larksuite.com/client/chat/chatter/add_by_link?link_token=3nf8789-4rfx-427d-a6bf-ed1d2df348aabd",
        "expire_time": "1609296809",
        "is_permanent": false
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 232001 | Your request contains an invalid request parameter. | 参数错误，参考本文档检查输入参数。 |
| 400 | 232006 | Your request specifies a chat_id which is invalid. | 无效的 chat_id，请检查chat_id是否正确。 |
| 400 | 232009 | Your request specifies a chat which has already been dissolved. | 群组已被解散。 |
| 400 | 232010 | Operator and chat can NOT be in different tenants. | 操作者和被操作的群组应该在同一租户下。 |
| 400 | 232011 | Operator can NOT be out of the chat. | 操作者需要在群组中。 |
| 400 | 232019 | The request has been rate limited. | 触发群限流，请控制请求的速度，详情参见[频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)。 |
| 400 | 232025 | Bot ability is not activated. | 应用需要开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。 |
| 400 | 232033 | The operator or invited bots does NOT have the authority to manage external chats without the scope. | 没有权限操作外部群。 |
| 400 | 232034 | The app is unavailable or inactivate in the tenant. | 应用在该租户下未安装或未启用。 |
| 400 | 232061 | Secret chat cannot be share link. | 密聊不支持分享群链接。 |
| 400 | 232062 | P2P chat cannot be share link. | 单聊不支持分享群链接。 |
| 400 | 232063 | Team cannot be share link. | 团队群不支持分享群链接。 |
| 400 | 232064 | The operator is not a group owner or administrator, no permission to share chat link. | 该群开启了“仅群主和群管理员可添加群成员/分享群”，请检查操作者在群中的身份。 |
| 400 | 232065 | The User/Bot can NOT be found. | 请检查机器人和用户是否存在。 |





