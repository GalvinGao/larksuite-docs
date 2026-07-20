---
document_id: '7073461364316225542'
directory_id: '7073442394955563013'
title: 订阅用户消息事件并发送
full_path: /home/interactive-session-based-robot/subscribe-to-user-group-entry-events-and-send
breadcrumb:
- Home
- Interactive session-based robot
- Subscribe to user-message events and send messages
document_type: GuideDocumentType
updated_at: 2023-05-16T03:11:55Z
source_url: https://open.larksuite.com/document/home/interactive-session-based-robot/subscribe-to-user-group-entry-events-and-send
---

# 订阅用户消息事件并发送

**订阅用户消息事件**

在开放平台后台找到“添加事件”

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2e31830581644a7616ce5f1aaea43f40_0aSGTmg63I.png?lazyload=true&width=2798&height=1512)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/364572b0ce980cb8d680946096d00e7f_BhHScjIvfq.png?lazyload=true&width=1640&height=992)

添加完成，我们就可以收到用户发送的消息了。同时，将根据应用具备的权限，判断可推送的信息：
当具备获取群组中所有消息 权限时，可接收与机器人单聊会话中，发送给机器人的所有消息 当具备获取群组中用户@机器人的消息 权限，可接收机器人所在群聊中 @ 机器人的消息

**识别消息事件**

因为可能还有其他事件推送，所以我们需要找到用户发送消息事件的特征，从而识别出来。开放平台的事件订阅文档里写的很清楚：

[接收消息 - 服务端文档 - Lark开放平台](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/receive)

根据文档，发消息事件的关键特征就是`header.event_type`为`im.message.receive_v1`。

**找到用户发的内容，用来进行业务逻辑处理**

根据上述文档，用户发送的内容在`event.message.content`，是一个序列化的JSON字符串，我们需要反序列化后再使用。回复用户消息需要使用原消息的message id，在`event.message.message_id`，后面会用到。

**执行业务逻辑，获取结果**

比如“小查”是将用户的内容作为关键词进行内网搜索查询，返回命中的结果内容。

**使用结果构造消息内容，回复给用户**

根据[回复消息文档](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/reply)

将上述结果内容构造为消息格式，发送给用户。

整体逻辑用js代码描述如下：



```javascript 
const axios = require('axios')
// 假设data为回调过来的事件体原始内容
function keywords_search(data) {
    if (data.header.event_type !== 'im.message.receive_v1') {
        return
    }
    // 获取消息内容和message id
    const {content, message_id} = data.event.message
    // 反序列化，得到用户输入的内容
    const {text} = JSON.parse(content)
    // 这里是自定义业务逻辑处理
    const resp = await custom_business_logic(text)
    if (resp.code !== 0) {
        return resp
    }
    // 拿到业务处理结果
    const result = resp.data
    const card = `{
  "header": {
    "title": {
      "tag": "lark_md",
      "i18n": {
        "zh_cn": "🔍 抓手"
      }
    },
    "template": "blue"
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
              "content": "result",
            }
          }
        ]
      }
    ]
  }
}`
    try {
        const resp = await axios.post("https://open.larksuite.com/open-apis/im/v1/messages/${message_id}/reply", {
            msg_type: "interactive",
            content: "${JSON.stringify({card})}",
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
