---
document_id: '7069302182251823109'
directory_id: '7069296428547686405'
title: 消息卡片相关
full_path: /ugTN1YjL4UTN24CO1UjN/uczN1YjL3cTN24yN3UjN
breadcrumb:
- Developer Guides
- FAQ
- Server-side develop
- Questions on Message Card
document_type: GuideDocumentType
updated_at: 2022-03-16T12:42:07Z
source_url: https://open.larksuite.com/document/ugTN1YjL4UTN24CO1UjN/uczN1YjL3cTN24yN3UjN
---

# 消息卡片相关
**1. 发送、编写消息卡片的代码在哪里添加？**

答：如果是机器人发送消息卡片，参考[开发机器人](/document/home/develop-a-bot-in-5-minutes/create-an-app) 和 [如何发送消息卡片](/document/ukTMukTMukTM/uYTNwUjL2UDM14iN1ATN)。如果是小程序发送消息卡片，可以在小程序主体代码中向 [sendMessageCard](/document/uYjL24iN/uUjN5UjL1YTO14SN2kTN) API发送请求来发送消息卡片，参考[小程序demo](/document/uYjL24iN/uYDM04iNwQjL2ADN)。
<br>
<br>

**2. 消息卡片设置card_link或者按钮的跳转链接后，如何能在Lark内部打开，而不是通过浏览器打开？**

答：只有 appLink 可在应用内打开，其余链接都是在外部打开。
<br>
<br>

**3. 消息卡片样式是否支持自定 HTML 语法？**

答：消息卡片不支持自定 HTML 语法，建议按照开放平台官方的消息卡片语法说明进行使用。
<br>
<br>

**4. 消息卡片能批量发给多人吗？**

答：支持。API参考[批量发送消息](/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM)。<br>
注意：<br>
- 请求参数中`msg_type`为`interactive`，`content` 取值为`card:{}`结构体
- 批量发送的消息卡片不支持更新、不支持回传交互

<br>
<br>

**5. 消息卡片的按钮中可以添加 emoji 表情吗？**

答：消息卡片的按钮支持以直接复制 emoji 的方式添加表情。查看[emoji表情](https://apps.timwhitlock.info/emoji/tables/unicode#emoji-modal)。
<br>
<br>


**6. 消息卡片是否支持@单个成员？**

答：支持的，语法参看 [使用markdown标签](/document/ukTMukTMukTM/uADOwUjLwgDM14CM4ATN)
<br>
<br>

**7.消息卡片回调超时时间是多少?**

答：卡片回调超时时间为 **3s**，不支持自定义。如果业务方在 3s 内无法返回可以先返回{}，然后使用在回调中的 token [异步更新卡片](/document/ukTMukTMukTM/uMDO1YjLzgTN24yM4UjN)。
<br>
<br>

**8. 消息卡片发送时报链接 scheme 不在白名单？**

答：目前不支持不合法的URL，仅支持 ==http== 和 ==https==
