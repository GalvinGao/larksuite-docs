---
document_id: '7138319837503586309'
directory_id: '7137982153342402566'
title: Pin 消息
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/pin/create
breadcrumb:
- Server API
- Messaging
- Pin
- Pin a message
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:08:55Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/pin/create
---

# Pin 消息

Pin 一条指定的消息。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=pin&method=create)

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
- 需要开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)  
- Pin消息时，机器人必须在对应的群组中
- 若消息已经被Pin，返回该Pin的操作信息
- 不能Pin一条对操作者不可见的消息
- 对同一条消息的Pin操作不能超过==5 QPS==
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/im/v1/pins |
| HTTP Method | POST |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="im:message" desc="获取与发送单聊、群组消息" support_app_types="custom,isv" tags="">获取与发送单聊、群组消息</md-perm><br><md-perm name="im:message.pins:write_only" desc="添加、 取消 Pin 消息" support_app_types="custom,isv" tags="">添加、 取消 Pin 消息</md-perm><br><md-perm name="im:message:send_as_bot" desc="以应用的身份发消息" support_app_types="custom,isv" tags="">以应用的身份发消息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




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
	<md-text type="field-name" >message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	待Pin的消息ID，详情参见[消息ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)

**示例值**："om_dc13264520392913993dd051dba21dcf"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "message_id": "om_dc13264520392913993dd051dba21dcf"
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
	<md-text type="field-name" >pin</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >pin</md-text>
	</md-dt-td>
	<md-dt-td>
	Pin的操作信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Pin的消息ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >chat_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Pin消息所在的群聊ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >operator_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Pin的操作人ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >operator_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Pin的操作人ID类型。当Pin的操作人为用户时，为==open_id==；当Pin的操作人为机器人时，为==app_id==
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >create_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Pin的创建时间（毫秒级时间戳）
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
        "pin": {
            "message_id": "om_dc13264520392913993dd051dba21dcf",
            "chat_id": "oc_a0553eda9014c201e6969b478895c230",
            "operator_id": "ou_7d8a6e6df7621556ce0d21922b676706ccs",
            "operator_id_type": "open_id",
            "create_time": "1615380573211"
        }
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 230001 | Your request contains an invalid request parameter. | 参数错误，请根据接口返回的错误信息并参考文档检查输入参数。 |
| 400 | 230002 | The bot can not be outside the group. | 机器人不在对应群组中。 |
| 400 | 230006 | Bot ability is not activated. | [机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)未启用 。可在[开发者后台](https://open.larksuite.com/app) -> 应用能力 -> 添加应用能力页面添加机器人功能，发布新版本后生效。 |
| 400 | 230111 | Action unavailable as the message will self-destruct soon. | 消息将过期，无法进行操作。 |
| 400 | 232009 | Your request specifies a chat which has already been dissolved. | 群组已被解散。 |
| 400 | 230011 | The message is recalled. | 消息已被撤回，不支持该操作。 |
| 400 | 230013 | Bot has NO availability to this user. | 机器人对用户没有[可用性](/document/home/introduction-to-scope-and-authorization/availability)。可在[开发者后台](https://open.larksuite.com/app) -> 应用发布 -> 版本管理与发布 -> 创建版本页面编辑应用的可用范围，发布新版本后生效。 |
| 400 | 230027 | Lack of necessary permissions. | 暂不支持在外部群中进行本操作。 |
| 400 | 230045 | The chat not exist. | 消息所属的群聊不存在，请检查群聊是否已解散。 |
| 400 | 230046 | No Permission to Pin/Unpin messages in the chat. | 该群设置仅群主和群管理员可Pin/Unpin消息，请检查操作者在群内的身份。 |
| 400 | 230047 | Pin/Unpin message trigger message_id limit. | 对同一条消息Pin/Unpin触发限流策略，请降低请求速度，稍后再试。 |
| 400 | 230050 | The message is invisible to the operator. | 该消息对于操作者不可见，无法进行本操作。 |
| 400 | 230054 | This operation is not supported for this message type. | 该消息类型不支持本操作，详情参见[消息与群组部分API增加不支持的消息类型校验<br>](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/unsupported-message-type-verification)。 |


:::note
其他服务端错误码参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)。
:::


