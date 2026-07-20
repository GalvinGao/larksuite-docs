---
document_id: '6967331173081202693'
directory_id: '6956151028698791942'
title: 卡片模板-投票卡片
full_path: /ukTMukTMukTM/uYzM3QjL2MzN04iNzcDN/sample-code/vote
breadcrumb:
- Developer Guides
- Message cards
- Message Card Example
- Vote
document_type: GuideDocumentType
updated_at: 2024-07-24T08:05:52Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYzM3QjL2MzN04iNzcDN/sample-code/vote
---

#  卡片模板-投票卡片
💡注意：<br>
本示例代码只提供**静态的卡片样式**示例，投票功能需要你进一步通过消息卡片的[回传交互](/document/ukTMukTMukTM/uYjNwUjL2YDM14iN2ATN#49904b71)来实现。
##  投票封面
###  效果

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7252ef4411c80b017a812f065d9aff49_CLfPG1ZlPU.png)
###  示例代码
以下示例代码可拷贝至[消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder?from=openplantform_mcb_entrance)中编辑使用。

注意：复制到卡片搭建工具后，请先删除卡片中的注释内容（//后的灰字内容），即可正常编辑、预览卡片。

```json 
 {
  "config": {
    "wide_screen_mode": true
  },
  "header": {
    "title": {
      "tag": "plain_text",
      "content": "🥤 下午的奶茶发车了，你要上车么"
    }
  },
  "elements": [
    {
      "tag": "hr"
    },
    {
      "tag": "action",
      "actions": [
        {
          "tag": "button",
          "text": {
            "tag": "plain_text",
            "content": "😍 带我！带我！！"
          },
          "type": "default",
          "value": {
            "chosen": "option1" //在value中自定义用户点击按钮后，向你的服务端提交的内容
          }
        },
        {
          "tag": "button",
          "text": {
            "tag": "plain_text",
            "content": "🤐 告辞…"
          },
          "type": "default",
          "value": {
            "chosen": "option2"
          }
        }
      ]
    },
    {
      "tag": "note", //用这个模块展示备注样式的内容
      "elements": [
        {
          "tag": "plain_text",
          "content": "创建者：王大明 🔐本投票为匿名投票"
        }
      ]
    }
  ]
}
``` 
##  投票结果
###  效果

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b9ceaf6af9f2451997899e5b4f1044a1_7791s8YbVe.png)
### 示例代码
以下示例代码可拷贝至[消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder?from=openplantform_mcb_entrance)中编辑使用。

```json 
 {
  "config": {
    "wide_screen_mode": true
  },
  "header": {
    "title": {
      "tag": "plain_text",
      "content": "🥤 下午的奶茶发车了，你要上车么"
    }
  },
  "elements": [
    {
      "tag": "hr"
    },
    {
      "tag": "div",
      "text": {
        "tag": "lark_md",
        "content": "😍 带我！带我！！"
      }
    },
    {
      "tag": "note",
      "elements": [
        {
          "tag": "img",
          "img_key": "img_79534153-82d8-4699-9083-5a845e65f20g",
          "alt": {
            "tag": "plain_text",
            "content": "图片"
          }
        },
        {
          "tag": "plain_text",
          "content": "1人投票"
        }
      ]
    },
    {
      "tag": "hr"
    },
    {
      "tag": "div",
      "text": {
        "tag": "lark_md",
        "content": "🤐 告辞…"
      }
    },
    {
      "tag": "note",
      "elements": [
        {
          "tag": "plain_text",
          "content": "0人投票"
        }
      ]
    },
    {
      "tag": "div",
      "text": {
        "tag": "lark_md",
        "content": "[🔍 查看完整投票结果](https://www.larksuite.com)"
      }
    }
  ]
}
``` 


