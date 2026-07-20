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
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td><md-version>V3.19.0+</md-version></md-td>
      <md-td><md-version>V3.19.0+</md-version></md-td>
      <md-td><md-version>V3.19.0+</md-version></md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/send-msg-card/send-msg-card" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> </md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                shouldChooseChat
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                false
            </md-td>
            <md-td>
                是否在选择会话页面中发送卡片，如果该字段设置为true则会跳转到选择会话页面并进行后续操作来完成卡片的发送。
<md-alert type="tip" icon="none">
- Android/iOS端 [V3.42.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC端 [V3.43.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- 该参数的优先级高于 openChatIDs
- shouldChooseChat 和 openChatIDs两者必填一个
- 若shouldChooseChat取值为false，则在小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 Lark V6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) ) ；网页应用[鉴权](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)后即可调用
</md-alert>

**示例值**：true
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                chooseChatParams
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                若 `shouldChooseChat` 输入为true，则可以定制选择会话的入参，参考选择会话[chooseChat](/document/uYjL24iN/uMTN3QjLzUzN04yM1cDN)。
<md-alert type="tip" icon="none">
- Android/iOS端 [V3.42.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC端 [V3.43.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>
              
**示例值**：

{"allowCreateGroup":true,"multiSelect":true,"externalChat":false,"confirmTitle":"testTitle"}
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                openChatIDs
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                会话的[open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)列表。

<md-alert type="tip" icon="none">最大数量10</md-alert>
              
**示例值**：["chatID1","chatID2"]
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                cardContent
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                消息卡片内容，参考卡片结构：[消息卡片参考](/document/home/mass-messaging-to-designated-departments/message-card-reference)。

**示例值**：

{"config":{"wide_screen_mode":true},"header":{"title":{"tag":"plain_text","content":"this is header"}},"elements":[{module-1},{module-2},{module-3},......]}
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                withAdditionalMessage
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                false
            </md-td>
            <md-td>
                若 `withAdditionalMessage` 输入为true，则在发送卡片时开启发送附带留言的能力。

**示例值**：true

<md-alert type="tip" icon="none">
Lark[V4.10.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>  
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

`success`返回对象的扩展属性：
:::html
<md-table>
 <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
  <md-tbody>
    <md-tr>
        <md-td>
            errMsg
        </md-td>
        <md-td>
            string
        </md-td>
        <md-td>
            错误消息。
        </md-td>
    </md-tr>
    <md-tr>
        <md-td>
            sendCardInfo
        </md-td>
        <md-td>
            string[]
        </md-td>
        <md-td>
            发送消息卡片的message信息（只有消息发送动作产生时才会返回该字段；例如没有传`openChatIDs`，用户取消发送，解析卡片内容失败等情况都不会返回该字段）。 
<md-alert type="tip" icon="none">
Lark[V4.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>
        </md-td>
    </md-tr>
    <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    status
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                发送消息的状态码，0表示发送成功。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    openChatId
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    openMessageId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                 [open_message_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
            </md-td>
        </md-tr>
  </md-tbody>
</md-table>
:::


`fail`返回对象的扩展属性：

:::html
<md-table>
    <md-thead>
      	
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
      	<md-tr>
            <md-td>
                errMsg
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                错误消息。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                errCode
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                错误码。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                sendCardInfo
            </md-td>
            <md-td>
                object[]
            </md-td>
            <md-td>
                发送消息卡片的message信息（只有消息发送动作产生时才会返回该字段；例如没有传`openChatIDs`，用户取消发送，解析卡片内容失败等情况都不会返回该字段）。
<md-alert type="tip" icon="none">
Lark[V4.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>
            </md-td>
        </md-tr>
      	<md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    status
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                发送消息的状态码，0表示发送成功。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    openChatId
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    openMessageId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                [open_message_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                failedOpenChatIDs
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                发送失败的会话[open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)列表。
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                additionalMessageInfo
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                发送消息卡片的留言信息返回结果（只有发送留言失败的时候才会返回该信息，发送留言成功时不会返回该信息）。
<md-alert type="tip" icon="none">
Lark[V4.10.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>
            </md-td>
        </md-tr>
      	<md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    status
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                发送消息的状态码，0表示发送成功。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    openChatId
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    message
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                [open_message_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


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


:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">错误码</md-th>
      <md-th style="width: 40%;">描述</md-th>
      <md-th style="width: 40%;">排查建议</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>`-1`</md-td>
      <md-td>`openChatIDs`为空</md-td>
      <md-td>参数错误，请检查参数后重试。</md-td>
   	</md-tr>
    <md-tr>
      <md-td>`-2`</md-td>
      <md-td>`openChatIDs`数量超过10</md-td>
      <md-td>参数错误，请检查参数后重试。</md-td>
   	</md-tr>
    <md-tr>
      <md-td>`-3`</md-td>
      <md-td>`cardContent`为空</md-td>
      <md-td>参数错误，请检查参数后重试。</md-td>
   	</md-tr>
    <md-tr>
      <md-td>`-4`</md-td>
      <md-td>`cardContent`格式化失败</md-td>
      <md-td>消息卡片格式错误，请检查后重试。</md-td>
   	</md-tr>
    <md-tr>
      <md-td>`-5`</md-td>
      <md-td>发送失败</md-td>
      <md-td>内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。</md-td>
   	</md-tr>
   	<md-tr>
      <md-td>`-6`</md-td>
      <md-td>用户取消发送</md-td>
      <md-td></md-td>
   	</md-tr>
    <md-tr>
      <md-td>`-7`</md-td>
      <md-td>其他错误</md-td>
      <md-td>内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。</md-td>
   	</md-tr>
    <md-tr>
      <md-td>`-8`</md-td>
      <md-td>`openChatIDs`中的参数错误</md-td>
      <md-td>请检查是不是[open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)类型（如果是[open_id](/document/home/user-identity-introduction/open-id)将会导致此错误）。</md-td>
   	</md-tr>
   	<md-tr>
      <md-td>`-9`</md-td>
      <md-td>未知错误</md-td>
      <md-td>未知错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。</md-td>
   	</md-tr>
    <md-tr>
      <md-td>`-10`</md-td>
      <md-td>选择会话失败</md-td>
      <md-td>内部错误，请检查会话id是否正确，并请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。</md-td>
   	</md-tr>
    <md-tr>
      <md-td>`42406`</md-td>
      <md-td>发送留言失败</md-td>
      <md-td>内部错误，请检查参数是否正确，并请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。</md-td>
   	</md-tr>
   </md-tbody>
</md-table>
:::
