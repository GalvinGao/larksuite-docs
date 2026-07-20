---
document_id: '7074952334765719558'
directory_id: '7073442394955612165'
title: 订阅用户进群事件并发送
full_path: /home/event-based-messaging/subscribe-to-user-group-entry-events-and-send
breadcrumb:
- Home
- Auto Send Welcome Messages
- Subscribe to user group entry events and send
document_type: GuideDocumentType
updated_at: 2023-05-16T03:11:49Z
source_url: https://open.larksuite.com/document/home/event-based-messaging/subscribe-to-user-group-entry-events-and-send
---

# 订阅用户进群事件并发送

**订阅用户进群事件**

在开放平台后台找到“添加事件”（需要配置请求网址URL）

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2e31830581644a7616ce5f1aaea43f40_b7UTIAh8g0.png?lazyload=true&width=2798&height=1512)


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b68ba483fbdb5d9e8b905eff3f47337e_Gu219dxho6.png?lazyload=true&width=1640&height=921)
这样我们就可以收到用户进群的事件推送了（需要机器人在群里）。

**识别用户进群事件**

因为可能还有其他事件推送，所以我们需要找到用户进群事件的特征，从而识别出来。参考[事件订阅文档](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-member-user/events/added)。

根据文档，用户进群事件的关键特征就是```header.event_type ``` 为 ```im.chat.member.user.added_v1```。

**找到事件中所需的信息，用来构造消息**

比如，用户进群事件里，我们需要找到谁进群了，然后构造一个消息来at进群的同学。还需要找到群id，用来向该群发消息。还是根据上面的文档，我们可以找到```event.users ```为进群的用户列表，```event.chat_id ```为群 id。

**向群里发送上面构造的消息**
根据[发送消息文档](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)
将内容构造为指定格式并调用接口发出。

整体逻辑用 js 代码描述如下：

```javascript 
const axios = require('axios')
// 假设data为回调过来的事件体原始内容
async function group_member_greet(data) {
    if (data.header.event_type !== 'im.chat.member.user.added_v1') {
        // 不是用户进群事件，忽略
        return
    }
    const users_info = data.event.users.map(x=>`<at id=${x.user_id.open_id}></at>`).join("")
    const card = `{
  "header": {
    "title": {
      "tag": "lark_md",
      "i18n": {
        "zh_cn": "欢迎新同学"
      }
    }
  },
  "i18n_elements": {
    "zh_cn": [
      {
        "tag": "div",
        "fields": [
          {
            "is_short": false,
            "text": {
              "tag": "lark_md",
              "content": "亲爱的${users_info}，欢迎入群！👏🏻"
            }
          }
        ]
      }
    ]
  }
}`
    try {
        const resp = await axios.post("https://open.larksuite.com/open-apis/im/v1/messages?receive_id_type=chat_id", {
            receive_id: "${data.event.chat_id}",
            msg_type: "interactive",
            content: "${JSON.stringify({card})}",)
        }
        return {
            code: resp.data.code,
            msg: resp.data.msg
        }
    } catch(e) {
        return {
            code: -1,
            msg: "${e}"
        }
    } 
}
``` 


|     |   | *powered by guilin* |
| :-------- | -------:|  -------: |
