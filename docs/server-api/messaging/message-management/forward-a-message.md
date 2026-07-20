---
document_id: '7242510573728235526'
directory_id: '7002892512470597638'
title: 转发消息
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/forward
breadcrumb:
- Server API
- Messaging
- Message management
- Forward a message
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:08:17Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/forward
---

# 转发消息

向用户、群聊或话题转发一条指定消息。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=message&method=forward)

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
- 向用户转发消息，需要机器人对用户有[可用性](/document/home/introduction-to-scope-and-authorization/availability)
- 向群组转发消息，需要机器人在群组中
- 对于要转发的消息与转发到的对象，本接口有以下限制：
     - 不支持红包、投票、语音、日程转让等消息类型
    - 不支持再次转发合并转发消息中的子消息（含有==upper_message_id==字段的消息）
-  为避免对用户造成打扰，向同一用户发送消息的限频为 ==5 QPS==，向同一群组发送消息的限频为群内机器人共享 ==5 QPS==
</md-alert>
:::



## 请求
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/im/v1/messages/:message_id/forward</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    <md-tr>
      <md-th>接口频率限制</md-th>
      <md-td>[1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom,isv"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
            
      </md-th>
      <md-td>
            <md-perm name="im:message" desc="获取与发送单聊、群组消息" support_app_types="custom,isv" tags="">获取与发送单聊、群组消息</md-perm>
            <md-perm name="im:message:send_as_bot" desc="以应用的身份发消息" support_app_types="custom,isv" tags="">以应用的身份发消息</md-perm>
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 35%;">名称</md-th>
      <md-th style="width: 13%;">类型</md-th>
       <md-th style="width: 15%;" filters="是,否" >必填</md-th>
      <md-th  style="width: 37%;">描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"

[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)

</md-td>
</md-tr>
<md-tr>
<md-td>Content-Type</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
</md-tbody>
</md-table>
:::



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
	<md-text type="field-name" >message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	要转发的消息ID，详情参见[消息ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)

**示例值**："om_dc13264520392913993dd051dba21dcf"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 查询参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;" >描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >receive_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	消息接收者id类型 open_id/user_id/union_id/email/chat_id/thread_id

**示例值**：open_id

**可选值有**：
<md-enum>
<md-enum-item key="open_id" >标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)</md-enum-item>
<md-enum-item key="user_id" >标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)</md-enum-item>
<md-enum-item key="union_id" >标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)</md-enum-item>
<md-enum-item key="email" >以用户的真实邮箱来标识用户。</md-enum-item>
<md-enum-item key="chat_id" >以群ID来标识群聊。[了解更多：如何获取群ID ](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)</md-enum-item>
<md-enum-item key="thread_id" >以话题ID来标识话题。了解更多：[话题介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/thread-introduction)
</md-enum-item>

**当值为 `user_id`，字段权限要求**：

<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm></md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >uuid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	由开发者生成的唯一字符串序列，用于转发消息请求去重；持有相同uuid的请求在1小时内向同一个目标的转发只可成功一次。

**示例值**：b13g2t38-1jd2-458b-8djf-dtbca5104204

**数据校验规则**：

- 最大长度：`50` 字符
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
	<md-text type="field-name" >receive_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	依据`receive_id_type`的值，填写对应的转发目标的ID

**示例值**："ou_a0553eda9014c201e6969b478895c230"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "receive_id": "ou_a0553eda9014c201e6969b478895c230"
}
</md-code-json>
:::

**cURL请求示例**

```json 
curl --location --request POST 'https://open.larksuite.com/open-apis/im/v1/messages/om_dc13264520392913993dd051dba21dcf/forward?receive_id_type=open_id' \
--header 'Authorization: Bearer Bearer t-XXX' \
--header 'Content-Type: application/json; charset=utf-8' \
--data-raw '{
    "receive_id": "ou_a0553eda9014c201e6969b478895c230"
}'
```

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
	<md-text type="field-type" >message</md-text>
	</md-dt-td>
	<md-dt-td>
	\-
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	消息id，说明参见：[消息ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >root_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	根消息id，用于回复消息场景，说明参见：[消息ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >parent_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	父消息的id，用于回复消息场景，说明参见：[消息ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >thread_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	消息所属的话题 ID（不返回说明该消息非话题消息），说明参见：[话题介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/thread-introduction)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >msg_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等，类型定义请参考[接收消息Content](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/events/message_content)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >create_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	消息生成的时间戳（毫秒）
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >update_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	消息更新的时间戳（毫秒）
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >deleted</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	消息是否被撤回
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >updated</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	消息是否被更新
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >chat_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	所属的群
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >sender</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >sender</md-text>
	</md-dt-td>
	<md-dt-td>
	发送者，可以是用户或应用
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	该字段标识发送者的id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	该字段标识发送者的id类型

**可选值有：**
- `open_id`
- `app_id`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >sender_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	该字段标识发送者的类型

**可选值有：**
- `user`: 用户
- `app`: 应用
- `anonymous`: 匿名
- `unknown`: 未知
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >tenant_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	为租户在Lark上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >body</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >message_body</md-text>
	</md-dt-td>
	<md-dt-td>
	消息内容
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	消息内容，json结构序列化后的字符串。不同msg_type对应不同内容。消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等，类型定义请参考：[接收消息内容](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/events/message_content)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >mentions</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >mention\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	被@的用户或机器人的id列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	被@的用户或机器人的序号。例如，第3个被@到的成员，值为“@_user_3”
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	被@的用户或者机器人的open_id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	被@的用户或机器人 id 类型，目前仅支持 `open_id` ([什么是 Open ID？](/document/home/user-identity-introduction/open-id))
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	被@的用户或机器人的姓名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >tenant_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	为租户在Lark上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >upper_message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	合并转发消息中，上一层级的消息id message_id，说明参见：[消息ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
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
        "message_id": "om_dc13264520392913993dd051dba21dcf",
        "root_id": "om_40eb06e7b84dc71c03e009ad3c754195",
        "parent_id": "om_d4be107c616aed9c1da8ed8068570a9f",
        "msg_type": "text",
        "create_time": "1615380573411",
        "update_time": "1615380573411",
        "deleted": false,
        "updated": false,
        "chat_id": "oc_5ad11d72b830411d72b836c20",
        "sender": {
            "id": "cli_9f427eec54ae901b",
            "id_type": "app_id",
            "sender_type": "app",
            "tenant_key": "736588c9260f175e"
        },
        "body": {
            "content": "{\"text\":\"test content\"}"
        }
    }
}
</md-code-json>
:::



### 错误码
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">HTTP状态码</md-th>
            <md-th style="width: 15%;">错误码</md-th>
            <md-th style="width: 30%;">描述</md-th>
            <md-th style="width: 30%;">排查建议</md-th>
        </md-tr>
    </md-thead>
  <md-tbody>

<md-tr>
  <md-td>400</md-td>
  <md-td>230001</md-td>
  <md-td>Your request contains an invalid request parameter.</md-td>
  <md-td>参数错误，请根据接口返回的错误信息并参考文档检查输入参数。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230002</md-td>
  <md-td>The bot can not be outside the group.</md-td>
  <md-td>机器人不在对应群组中。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230006</md-td>
  <md-td>Bot ability is not activated.</md-td>
  <md-td>[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)未启用 。可在[开发者后台](https://open.larksuite.com/app) -> 应用能力 -> 添加应用能力页面添加机器人功能，发布新版本后生效。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230013</md-td>
  <md-td>Bot has NO availability to this user.</md-td>
  <md-td>机器人对用户没有[可用性](/document/home/introduction-to-scope-and-authorization/availability)。可在[开发者后台](https://open.larksuite.com/app) -> 应用发布 -> 版本管理与发布 -> 创建版本页面编辑应用的可用范围，发布新版本后生效。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230018</md-td>
  <md-td>These operations are NOT allowed at current group settings.</md-td>
  <md-td>当前操作被群设置禁止，请检查群设置或联系群管理员。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230019</md-td>
  <md-td>The thread does NOT exist.</md-td>
  <md-td>要转发到的话题不存在，请检查 thread_id 是否正确。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230020</md-td>
  <md-td>This operation triggers the frequency limit.</md-td>
  <md-td>当前操作触发限频，请降低请求频率。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230027</md-td>
  <md-td>Lack of necessary permissions.</md-td>
  <md-td>暂不支持在外部群中进行本操作。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230029</md-td>
  <md-td>User has resigned.</md-td>
  <md-td>用户已离职。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230034</md-td>
  <md-td>The receive_id is invalid.</md-td>
  <md-td>请求参数中的receive_id不合法，请检查。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230035</md-td>
  <md-td>Send Message Permission deny.</md-td>
  <md-td>没有发送消息的权限，请排查群是否已开启禁言，或受到租户维度沟通权限的管控。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230036</md-td>
  <md-td>Tenant crypt key has been deleted.</md-td>
  <md-td>租户加密密钥已被删除，请联系企业管理员。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230038</md-td>
  <md-td>Cross tenant p2p chat operate forbid.</md-td>
  <md-td>跨租户的单聊不允许通过本接口发送消息。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230049</md-td>
  <md-td>The message is being sent.</md-td>
  <md-td>消息正在发送中，请稍后。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230050</md-td>
  <md-td>The message is invisible to the operator.</md-td>
  <md-td>该消息对于操作者不可见，无法进行本操作。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230053</md-td>
  <md-td>The user has stopped the bot from sending messages.</md-td>
  <md-td>用户已设置不再接收机器人消息，无法主动给用户发送单聊消息。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230061</md-td>
  <md-td>The message type does not support forwarding.</md-td>
  <md-td>系统消息、红包、投票、语音、日程转让等消息类型不支持转发。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230062</md-td>
  <md-td>No permission to forward to third-party encryption group.</md-td>
  <md-td>没有权限转发到第三方加密群。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230063</md-td>
  <md-td>The chat_id of group to forward is invalid.</md-td>
  <md-td>要转发到的群是无效的，请检查chat_id对应的群组是否存在。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230064</md-td>
  <md-td>The message to be forwarded is invalid.</md-td>
  <md-td>要转发的消息都是无效的，如不存在、已被撤回、对操作者不可见等。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230065</md-td>
  <md-td>The message to be forwarded has been recalled.</md-td>
  <md-td>要转发的消息都已被撤回。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230066</md-td>
  <md-td>The message to be forwarded is in a secret group, forwarding is not supported.</md-td>
  <md-td>不支持转发密聊群中的消息。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230070</md-td>
  <md-td>Forwarding of messages in restricted mode is not allowed.</md-td>
  <md-td>不支持转发保密消息，请检查消息是否已被设置为保密或群组开启了防泄密模式。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230074</md-td>
  <md-td>The target thread is invisible to the operator.</md-td>
  <md-td>要转发到的话题对于操作者不可见。 若群聊关闭了“新成员可查看历史消息”且此话题为操作者进入群聊前创建的，则该话题需要操作者被动订阅才可见，如其他用户在话题中@操作者。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230110</md-td>
  <md-td>Action unavailable as the message has been deleted.</md-td>
  <md-td>消息已被删除.</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232009</md-td>
  <md-td>Your request specifies a chat which has already been dissolved.</md-td>
  <md-td>群组已被解散。</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




