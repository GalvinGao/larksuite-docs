---
document_id: '7074952334765686790'
directory_id: '7073442394955612165'
title: 消息卡片参考
full_path: /home/event-based-messaging/message-card-reference
breadcrumb:
- Home
- Auto Send Welcome Messages
- Message card reference
document_type: GuideDocumentType
updated_at: 2023-05-16T03:11:49Z
source_url: https://open.larksuite.com/document/home/event-based-messaging/message-card-reference
---

# 消息卡片参考

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/935794365cad91bc06687e514babdff3_UnuOUaVWNS.png)

**消息卡片模板：**

```json 
{
  "config": {
    "wide_screen_mode": true
  },
  "header": {
    "template": "orange",
    "title": {
      "content": " 🎈 欢迎新同学加入团队！ 🎈 ",
      "tag": "plain_text"
    }
  },
  "i18n_elements": {
    "zh_cn": [
      {
        "alt": {
          "content": "",
          "tag": "plain_text"
        },
        "img_key": "img_v2_aeb0744b-22e9-4ffd-8d94-d18035f8858g",
        "tag": "img"
      },
      {
        "tag": "div",
        "text": {
          "content": "亲爱的<at email=test@email.com></at>，欢迎加入大家庭！\n\n加入团队后，完成以下事情，有助于你更快融入团队：\n1. 请在群内做个简单的自我介绍，帮助大家更好地了解你\n2. 想要快速了解团队的业务和职责吗？单独约团队同学1 on 1，让大家给你介绍吧\n3. 你可能会需要的重要资料如下: \n3.1 [团队周报集合](https://open.larksuite.com/)\n3.2 部门 leader 的[ 双月OKR](https://open.larksuite.com/)\n3.3 向产品经理推荐：[产品设计原则](https://open.larksuite.com/)",
          "tag": "lark_md"
        }
      },
      {
        "tag": "hr"
      },
      {
        "extra": {
          "tag": "button",
          "text": {
            "content": "🙋 立刻领取",
            "tag": "lark_md"
          },
          "type": "primary",
          "url": "https://larksuite.com"
        },
        "tag": "div",
        "text": {
          "content": "**🎁 领取入职套件**\n地点：15 楼行政中心\n时间：9:00 - 18:00\n",
          "tag": "lark_md"
        }
      },
      {
        "tag": "hr"
      },
      {
        "extra": {
          "tag": "button",
          "text": {
            "content": "👀 点击查看",
            "tag": "lark_md"
          },
          "type": "primary",
          "url": "https://open.larksuite.com/?lang=zh-CN"
        },
        "tag": "div",
        "text": {
          "content": "**😘 收到的问候**\n王凡、刘子涵、陈怡君......向你打招呼啦",
          "tag": "lark_md"
        }
      }
    ]
  }
}
``` 


快速生成指定样式的消息卡片，试试[消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder?from=ttrlbot2)，更有丰富模板可供直接使用。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6331e35b7a18cf4e7c89fe5dff8f560d_nbFTTFSHxD.png)
