---
document_id: '7026663896463310853'
directory_id: '7002892512470597638'
title: 发送消息
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create
breadcrumb:
- Server API
- Messaging
- Message management
- Send message
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:08:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create
---

# 发送消息

给指定用户或者会话发送消息，支持文本、富文本、可交互的[消息卡片](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)、群名片、个人名片、图片、视频、音频、文件、表情包。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=message&method=create)

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
- 给用户发送消息，需要机器人对用户有[可用性](/document/home/introduction-to-scope-and-authorization/availability)
- 给群组发送消息，需要机器人在群组中
- 为避免对用户造成打扰，向同一用户发送消息的限频为 ==5 QPS==，向同一群组发送消息的限频为群内机器人共享 ==5 QPS==
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
      <md-td>https://open.larksuite.com/open-apis/im/v1/messages</md-td>
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
            <md-perm name="im:message:send" desc="发送消息V2" support_app_types="custom,isv" tags="history">发送消息V2</md-perm>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            字段权限要求
      </md-th>
      <md-td>
        <md-alert type="tip" icon="none">
        该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请
        </md-alert>
        <md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
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
	消息接收者id类型 open_id/user_id/union_id/email/chat_id

**示例值**：open_id

**可选值有**：
<md-enum>
<md-enum-item key="open_id" >标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)</md-enum-item>
<md-enum-item key="union_id" >标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)</md-enum-item>
<md-enum-item key="user_id" >标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)。</md-enum-item>
<md-enum-item key="email" >以用户的真实邮箱来标识用户。</md-enum-item>
<md-enum-item key="chat_id" >以群ID来标识群聊。[了解更多：如何获取群ID ](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)</md-enum-item></md-enum-item>
</md-enum>

**当值为 `user_id`，字段权限要求**：
<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
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
	消息接收者的ID，ID类型应与查询参数==receive_id_type== 对应；推荐使用 OpenID，获取方式可参考文档[如何获取 Open ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)

**示例值**："ou_7d8a6e6df7621556ce0d21922b676706ccs"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >msg_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等，类型定义请参考[发送消息内容](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json)

**示例值**："text"
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
	是
	</md-dt-td>
	<md-dt-td>
	消息内容，JSON结构序列化后的字符串。不同msg_type对应不同内容，具体格式说明参考：[发送消息内容](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json)

**注意：**
- JSON字符串需进行转义，如换行符转义后为`\\n`
- 文本消息请求体最大不能超过150KB
- 卡片及富文本消息请求体最大不能超过30KB

**示例值**："`{\"text\":\"test content\"}`"
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
	由开发者生成的唯一字符串序列，用于发送消息请求去重；持有相同uuid的请求1小时内至多成功发送一条消息

**示例值**："选填，每次调用前请更换，如a0d69e20-1dd1-458b-k525-dfeca4015204"

**数据校验规则**：

- 最大长度：`50` 字符
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "receive_id": "ou_7d8a6e6df7621556ce0d21922b676706ccs",
    "msg_type": "text",
    "content": "{\"text\":\"test content\"}",
    "uuid": "选填，每次调用前请更换，如a0d69e20-1dd1-458b-k525-dfeca4015204"
}
</md-code-json>
:::

**cURL请求示例**

```json 
curl --location --request POST 'https://open.larksuite.com/open-apis/im/v1/messages?receive_id_type=chat_id' \
--header 'Authorization: Bearer t-XXX' \
--header 'Content-Type: application/json; charset=utf-8' \
--data-raw '{
    "receive_id": "oc_84983ff6516d731e5b5f68d4ea2e1da5",
    "msg_type": "text",
    "content": "{\"text\":\"test content\"}"
}'
```

**Python请求示例**
```
import json
import requests

def send():
    url = "https://open.larksuite.com/open-apis/im/v1/messages"
    params = {"receive_id_type":"chat_id"}
    msg = "text content"
    msgContent = {
        "text": msg,
    }
    req = {
        "receive_id": "oc_xxx", # chat id
        "msg_type": "text",
        "content": json.dumps(msgContent)
    }
    payload = json.dumps(req)
    headers = {
        'Authorization': 'Bearer xxx', # your access token
        'Content-Type': 'application/json'
    }
    response = requests.request("POST", url, params=params, headers=headers, data=payload)
    print(response.headers['X-Tt-Logid']) # for debug or oncall
    print(response.content) # Print Response

if __name__ == '__main__':
    send()
```

Golang请参考[机器人自动拉群报警教程（含 Go 示例代码）](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/determine-the-api-and-event-to-call)

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
	消息所属的话题 ID
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
	消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等，类型定义请参考[接收消息内容](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/events/message_content)
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
        "thread_id": "omt_d4be107c616a",
        "msg_type": "card",
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
            "content": "{\"text\":\"@_user_1 test content\"}"
        },
        "mentions": [
            {
                "key": "@_user_1",
                "id": "ou_155184d1e73cbfb8973e5a9e698e74f2",
                "id_type": "open_id",
                "name": "Tom",
                "tenant_key": "736588c9260f175e"
            }
        ],
        "upper_message_id": "om_40eb06e7b84dc71c03e009ad3c754195"
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
  <md-td>230015</md-td>
  <md-td>P2P chat can NOT be shared.</md-td>
  <md-td>私聊会话不允许被分享。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230017</md-td>
  <md-td>Bot is NOT the owner of the resource.</md-td>
  <md-td>机器人不是资源的拥有者。</md-td>
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
  <md-td>The topic does NOT exist.</md-td>
  <md-td>当前话题不存在。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230020</md-td>
  <md-td>This operation triggers the frequency limit.</md-td>
  <md-td>当前操作触发限频，请降低请求频率。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230022</md-td>
  <md-td>The content of the message contains sensitive information.</md-td>
  <md-td>消息包含敏感信息，请检查消息内容。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230025</md-td>
  <md-td>The length of the message content reaches its limit.</md-td>
  <md-td>消息体长度超出限制。文本消息最大不能超过150KB、卡片及富文本消息最大不能超过30KB。需要注意：
- 若使用卡片模板（template_id）发送消息，实际大小也包含模板对应的卡片数据大小。
- 若消息中包含大量样式标签，会使实际消息体长度大于您输入的请求体长度。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230027</md-td>
  <md-td>Lack of necessary permissions.</md-td>
  <md-td>无权进行本操作，可能的原因有：
1. 缺少相应权限，请根据错误信息进行排查；
2. 未检查到用户授权信息；
3. 暂不支持在外部群中进行本操作。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230099</md-td>
  <md-td>Failed to create card content.</md-td>
  <md-td>创建卡片失败。如有报错信息可根据报错信息排查解决；如无报错信息则需自行检查构造的卡片内容是否有问题。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230028</md-td>
  <md-td>The messages do NOT pass the audit.</md-td>
  <md-td>消息DLP审查未通过，当消息内容中含有明文电话号码、明文个人邮箱等内容时可能会触发该错误；请根据接口返回的错误信息检查消息内容。</md-td>
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
  <md-td>没有发送消息的权限，请排查群是否已开启禁言、被对方屏蔽或受到租户维度沟通权限的管控。</md-td>
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
  <md-td>230053</md-td>
  <md-td>The user has stopped the bot from sending messages.</md-td>
  <md-td>用户已设置不再接收机器人消息，无法主动给用户发送单聊消息。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230054</md-td>
  <md-td>This type of message is unavailable in the connection group.</md-td>
  <md-td>私有互通群不支持发送特定的消息类型。如卡片消息。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230055</md-td>
  <md-td>The type of file upload does not match the type of message being sent.</md-td>
  <md-td>文件上传时选择的类型与发送的消息类型不匹配。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>230075</md-td>
  <md-td>Sending encrypted messages is not supported.</md-td>
  <md-td>不支持向密聊、密盾聊中发送消息。</md-td>
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

其他未列出的错误码请参见[服务端通用错误码](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)。


