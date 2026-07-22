---
document_id: '7026663890609422342'
directory_id: '7021842990278164485'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-card/overview
breadcrumb:
- Server API
- Messaging
- Message card
- Resource introduction
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:38Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-card/overview
---

# 资源介绍
## 资源定义

消息卡片是Lark的一种可以承载丰富的图文内容和交互行为的消息类型。
通过消息卡片，你可以：
- 使用消息卡片提供的富文本样式、图文布局模块，发送样式精美的卡片，比如醒目的通知、图文并茂的文章列表等，让重要的信息更好地触达用户。
- 使用消息卡片提供的交互组件，使用户只需在卡片上点击一下就能提交信息，从而无需离开聊天会话，就能快捷完成 OA 审批、投票统计、报警处理等系统操作。

<br>
实现以上功能无需客户端开发经验，Lark已经为卡片消息定义了结构化的组件与样式，通过服务端一段 JSON 描述，即可构造样式精美、可交互的消息卡片。[消息卡片详细介绍>>](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)


## 字段说明

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| &nbsp;<md-text type="field-name" >content</md-text> | <md-text type="field-type" >string</md-text> | 是 | 消息内容为JSON格式的卡片结构转义成String，卡片结构各字段说明请参考[卡片结构介绍](/document/ukTMukTMukTM/uEjNwUjLxYDM14SM2ATN) |


### 数据示例
以下示例代码可拷贝至[消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder?from=howtoguide)中编辑使用。

```json 
{
  "config": {
    "wide_screen_mode": true
  },
  "header": {
    "title": {
      "tag": "plain_text",
      "content": "你有一个休假申请待审批"
    }
  },
  "elements": [
    {
      "tag": "div",
      "fields": [
        {
          "is_short": true,
          "text": {
            "tag": "lark_md",
            "content": "**申请人**\n王晓磊"
          }
        },
        {
          "is_short": true,
          "text": {
            "tag": "lark_md",
            "content": "**休假类型：**\n年假"
          }
        },
        {
          "is_short": false,
          "text": {
            "tag": "lark_md",
            "content": ""
          }
        },
        {
          "is_short": false,
          "text": {
            "tag": "lark_md",
            "content": "**时间：**\n2020-4-8 至 2020-4-10（共3天）"
          }
        },
        {
          "is_short": false,
          "text": {
            "tag": "lark_md",
            "content": ""
          }
        },
        {
          "is_short": true,
          "text": {
            "tag": "lark_md",
            "content": "**备注**\n因家中有急事，需往返老家，故请假"
          }
        }
      ]
    },
    {
      "tag": "hr"
    },
    {
      "tag": "action",
      "layout": "bisected",
      "actions": [
        {
          "tag": "button",
          "text": {
            "tag": "plain_text",
            "content": "批准"
          },
          "type": "primary",
          "value": {
            "chosen": "approve"
          }
        },
        {
          "tag": "button",
          "text": {
            "tag": "plain_text",
            "content": "拒绝"
          },
          "type": "primary",
          "value": {
            "chosen": "decline"
          }
        }
      ]
    }
  ]
}
``` 

### 发送消息卡片示例
通过调用 [发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)接口，向某个群组发送上图示例的审批消息卡片，cURL示例代码如下：
```bash
curl --location --request POST 'https://open.larksuite.com/open-apis/im/v1/messages?receive_id_type=chat_id' \
--header 'Authorization: Bearer t-XXX' \
--header 'Content-Type: application/json; charset=utf-8' \
--data-raw '{
    "receive_id": "oc_84983ff6516d731e5b5f68d4ea2e1da5",
    "content": "{\"config\":{\"wide_screen_mode\":true},\"header\":{\"title\":{\"tag\":\"plain_text\",\"content\":\"你有一个休假申请待审批\"}},\"elements\":[{\"tag\":\"div\",\"fields\":[{\"is_short\":true,\"text\":{\"tag\":\"lark_md\",\"content\":\"**申请人**\\n王晓磊\"}},{\"is_short\":true,\"text\":{\"tag\":\"lark_md\",\"content\":\"**休假类型：**\\n年假\"}},{\"is_short\":false,\"text\":{\"tag\":\"lark_md\",\"content\":\"\"}},{\"is_short\":false,\"text\":{\"tag\":\"lark_md\",\"content\":\"**时间：**\\n2020-4-8 至 2020-4-10（共3天）\"}},{\"is_short\":false,\"text\":{\"tag\":\"lark_md\",\"content\":\"\"}},{\"is_short\":true,\"text\":{\"tag\":\"lark_md\",\"content\":\"**备注**\\n因家中有急事，需往返老家，故请假\"}}]},{\"tag\":\"hr\"},{\"tag\":\"action\",\"layout\":\"bisected\",\"actions\":[{\"tag\":\"button\",\"text\":{\"tag\":\"plain_text\",\"content\":\"批准\"},\"type\":\"primary\",\"value\":{\"chosen\":\"approve\"}},{\"tag\":\"button\",\"text\":{\"tag\":\"plain_text\",\"content\":\"拒绝\"},\"type\":\"primary\",\"value\":{\"chosen\":\"decline\"}}]}]}",
    "msg_type": "interactive"
}'
``` 
### 卡片预览

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/67f0e373b1d3c090099cc673b00ff5c0_jRoSHwBEgC.png?lazyload=true&width=1366&height=678)
