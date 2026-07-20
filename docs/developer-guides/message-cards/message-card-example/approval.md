---
document_id: '6967331158355542022'
directory_id: '6956151028698791942'
title: 卡片模板-审批卡片
full_path: /ukTMukTMukTM/uYzM3QjL2MzN04iNzcDN/sample-code/approval
breadcrumb:
- Developer Guides
- Message cards
- Message Card Example
- Approval
document_type: GuideDocumentType
updated_at: 2024-07-24T08:05:48Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYzM3QjL2MzN04iNzcDN/sample-code/approval
---

# 卡片模板-审批卡片
💡注意：
本示例代码只提供**静态的卡片样式**示例，提交审批结果的功能需要你进一步通过**回传交互提交用户操作信息、更新消息卡片给用户反馈**来实现。详细开发说明可以参考文档：[交互模块](/document/ukTMukTMukTM/uYjNwUjL2YDM14iN2ATN)
##  审批封面
###  效果

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/67f0e373b1d3c090099cc673b00ff5c0_jRoSHwBEgC.png?lazyload=true&width=1366&height=678)
###  示例代码
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

##  审批完成
###  效果
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/47ed20de7ce4be1bc474e311b889b723_MdtKuKHe5D.png?lazyload=true&width=1280&height=593)

###  示例代码
以下示例代码可拷贝至[消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder?from=openplantform_mcb_entrance)中编辑使用。

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
      "tag": "note",
      "elements": [
        {
          "tag": "img",
          "img_key": "img_b79d3e29-5ffe-4897-822a-9743fe9f7b1g",
          "alt": {
            "tag": "plain_text",
            "content": "图片"
          }
        },
        {
          "tag": "plain_text",
          "content": "已同意休假申请"
        }
      ]
    }
  ]
}
``` 




