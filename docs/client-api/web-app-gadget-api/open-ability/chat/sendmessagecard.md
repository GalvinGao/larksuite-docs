---
document_id: '6967331158355050502'
directory_id: '6907567269107597314'
title: sendMessageCard
full_path: /uYjL24iN/uUjN5UjL1YTO14SN2kTN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- sendMessageCard
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:27Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN5UjL1YTO14SN2kTN
---

# sendMessageCard(Object object)


发送[消息卡片](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)到指定会话。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.19.0+</md-version> | <md-version>V3.19.0+</md-version> | <md-version>V3.19.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/send-msg-card/send-msg-card" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| shouldChooseChat | boolean | 否 | false | 是否在选择会话页面中发送卡片，如果该字段设置为true则会跳转到选择会话页面并进行后续操作来完成卡片的发送。<br><md-alert type="tip" icon="none"><br>- Android/iOS端 [V3.42.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC端 [V3.43.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- 该参数的优先级高于 openChatIDs<br>- shouldChooseChat 和 openChatIDs两者必填一个<br>- 若shouldChooseChat取值为false，则在小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 Lark V6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) ) ；网页应用[鉴权](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)后即可调用<br></md-alert><br>**示例值**：true |
| chooseChatParams | object | 否 |  | 若 `shouldChooseChat` 输入为true，则可以定制选择会话的入参，参考选择会话[chooseChat](/document/uYjL24iN/uMTN3QjLzUzN04yM1cDN)。<br><md-alert type="tip" icon="none"><br>- Android/iOS端 [V3.42.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC端 [V3.43.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert><br>**示例值**：<br>{"allowCreateGroup":true,"multiSelect":true,"externalChat":false,"confirmTitle":"testTitle"} |
| openChatIDs | string[] | 否 |  | 会话的[open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)列表。<br><md-alert type="tip" icon="none">最大数量10</md-alert><br>**示例值**：["chatID1","chatID2"] |
| cardContent | object | 否 |  | 消息卡片内容，参考卡片结构：[消息卡片参考](/document/home/mass-messaging-to-designated-departments/message-card-reference)。<br>**示例值**：<br>{"config":{"wide_screen_mode":true},"header":{"title":{"tag":"plain_text","content":"this is header"}},"elements":[{module-1},{module-2},{module-3},......]} |
| withAdditionalMessage | boolean | 否 | false | 若 `withAdditionalMessage` 输入为true，则在发送卡片时开启发送附带留言的能力。<br>**示例值**：true<br><md-alert type="tip" icon="none"><br>Lark[V4.10.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| errMsg | string | 错误消息。 |
| sendCardInfo | string[] | 发送消息卡片的message信息（只有消息发送动作产生时才会返回该字段；例如没有传`openChatIDs`，用户取消发送，解析卡片内容失败等情况都不会返回该字段）。<br><md-alert type="tip" icon="none"><br>Lark[V4.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>status<br></md-text> | number | 发送消息的状态码，0表示发送成功。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>openChatId<br></md-text> | number | [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description) |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>openMessageId<br></md-text> | string | [open_message_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2) |



`fail`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| errMsg | string | 错误消息。 |
| errCode | number | 错误码。 |
| sendCardInfo | object[] | 发送消息卡片的message信息（只有消息发送动作产生时才会返回该字段；例如没有传`openChatIDs`，用户取消发送，解析卡片内容失败等情况都不会返回该字段）。<br><md-alert type="tip" icon="none"><br>Lark[V4.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>status<br></md-text> | number | 发送消息的状态码，0表示发送成功。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>openChatId<br></md-text> | number | [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description) |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>openMessageId<br></md-text> | string | [open_message_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2) |
| failedOpenChatIDs | string[] | 发送失败的会话[open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)列表。 |
| additionalMessageInfo | string[] | 发送消息卡片的留言信息返回结果（只有发送留言失败的时候才会返回该信息，发送留言成功时不会返回该信息）。<br><md-alert type="tip" icon="none"><br>Lark[V4.10.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>status<br></md-text> | number | 发送消息的状态码，0表示发送成功。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>openChatId<br></md-text> | number | [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description) |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>message<br></md-text> | string | [open_message_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2) |



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/send-msg-card/send-msg-card" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
	<md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.sendMessageCard({
    "shouldChooseChat": true,
    "chooseChatParams": {},
    "openChatIDs": [
        "687485********10402"
    ],
    "triggerCode": "testCode",
    "cardContent": {
        "msg_type": "interactive",
        "update_multi": false,
        "card": {
            "elements": [
                {
                    "tag": "div",
                    "text": {
                        "tag": "plain_text",
                        "content": "Content module"
                    }
                }
            ]
        }
    },
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`sendMessageCard fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "sendCardInfo": [
        {
            "openMessageId": "om_925327317135d242832d17fbfab96064",
            "openChatId": "oc_a8a2a40a61589ec720f709bd45a2a9eb",
            "status": 0
        }
    ],
    "errMsg": "sendMessageCard:ok"
}
```

`fail`返回对象示例：
```json
{
    "errCode": -4,
    "errMsg": "sendMessageCard:fail"
}
``` 


## 错误码

fail 返回对象中会包含[errCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，代表错误码，具体错误码列表参见：


| 错误码 | 描述 | 排查建议 |
| --- | --- | --- |
| `-1` | `openChatIDs`为空 | 参数错误，请检查参数后重试。 |
| `-2` | `openChatIDs`数量超过10 | 参数错误，请检查参数后重试。 |
| `-3` | `cardContent`为空 | 参数错误，请检查参数后重试。 |
| `-4` | `cardContent`格式化失败 | 消息卡片格式错误，请检查后重试。 |
| `-5` | 发送失败 | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。 |
| `-6` | 用户取消发送 |  |
| `-7` | 其他错误 | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。 |
| `-8` | `openChatIDs`中的参数错误 | 请检查是不是[open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)类型（如果是[open_id](/document/home/user-identity-introduction/open-id)将会导致此错误）。 |
| `-9` | 未知错误 | 未知错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。 |
| `-10` | 选择会话失败 | 内部错误，请检查会话id是否正确，并请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。 |
| `42406` | 发送留言失败 | 内部错误，请检查参数是否正确，并请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。 |

