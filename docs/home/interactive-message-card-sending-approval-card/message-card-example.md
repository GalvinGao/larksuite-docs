---
document_id: '7074952334766063622'
directory_id: '7073442394955595781'
title: 消息卡片参考
full_path: /home/interactive-message-card-sending/message-card-reference
breadcrumb:
- Home
- Interactive message card sending (approval card)
- Message card example
document_type: GuideDocumentType
updated_at: 2023-05-16T03:12:02Z
source_url: https://open.larksuite.com/document/home/interactive-message-card-sending/message-card-reference
---

# 消息卡片参考


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ddbc1612cf3e96ec6de149bf095ef0de_p32aQNJZZ4.png?lazyload=true&width=1640&height=800)

**消息卡片模板：**

```json
{
  "elements": [
    {
      "tag": "markdown",
      "content": "工单号：[#223874](https://open.larksuite.com/)\n**当前进行的节点**\n全量发布：<at id=all></at>",
      "href": {
        "urlVal": {
          "url": "https://www.larksuite.com",
          "android_url": "https://developer.android.com/",
          "ios_url": "lark://msgcard/unsupported_action",
          "pc_url": "https://www.larksuite.com"
        }
      }
    },
    {
      "tag": "hr"
    },
    {
      "fields": [
        {
          "is_short": true,
          "text": {
            "content": "**👤 提交人：**\n<at email=test@email.com></at>",
            "tag": "lark_md"
          }
        },
        {
          "is_short": true,
          "text": {
            "content": "**📄 审批类型：**\n发布审核",
            "tag": "lark_md"
          }
        }
      ],
      "tag": "div"
    },
    {
      "fields": [
        {
          "is_short": true,
          "text": {
            "content": "**🗂️ 工单来源：**\n线上系统",
            "tag": "lark_md"
          }
        },
        {
          "is_short": true,
          "text": {
            "content": "**📚 留言内容：**\n工作需要，麻烦通过一下，谢谢！",
            "tag": "lark_md"
          }
        },
        {
          "is_short": false,
          "text": {
            "content": "",
            "tag": "lark_md"
          }
        },
        {
          "is_short": true,
          "text": {
            "content": "**📅 提交时间：**\n2021年5月25日 17:21:14",
            "tag": "lark_md"
          }
        },
        {
          "is_short": true,
          "text": {
            "content": "**🕙 已等待时长：**\n24时32分45秒",
            "tag": "lark_md"
          }
        }
      ],
      "tag": "div"
    },
    {
      "tag": "action",
      "actions": [
        {
          "tag": "button",
          "text": {
            "tag": "plain_text",
            "content": "同意"
          },
          "type": "primary"
        },
        {
          "tag": "button",
          "text": {
            "tag": "plain_text",
            "content": "拒绝"
          },
          "type": "danger"
        }
      ]
    }
  ],
  "header": {
    "template": "orange",
    "title": {
      "content": "❗️你有一份待审批文件",
      "tag": "plain_text"
    }
  }
}
```

快速生成指定样式的消息卡片，试试[消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder?from=ttrlbot4)，更有丰富模板可供直接使用。





![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/aa6cf955ebea72044e8ffacc1112c7c3_Y0yA9A0EK2.png?lazyload=true&width=1640&height=696)
