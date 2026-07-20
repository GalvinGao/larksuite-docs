---
document_id: '6967331173082185733'
directory_id: '6907567266537603073'
title: 内容模块
full_path: /ukTMukTMukTM/uMjNwUjLzYDM14yM2ATN
breadcrumb:
- Developer Guides
- Deprecated Guides
- Message Card
- Content Module
document_type: GuideDocumentType
updated_at: 2024-07-24T08:05:56Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMjNwUjLzYDM14yM2ATN
---

# 内容模块

## 模块介绍

内容模块以文本内容为主体，同时可以选择组合图片、按钮等交互组件，实现内容混排的效果。

模块标签为 `div` , 可以单独通过 `text` 或 `fields` 来展示文本内容，也可以配合一个 `image` 元素或一个 `button`, `overflow`, `selectMenu`, `datePicker` 等互动元素增加内容的丰富性。

在[消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder?from=cotentmodule) 中已经预置好了这些模块元素组合，你可以使用工具简化卡片搭建过程。


## 字段定义

| 字段  | 必须 | 类型   | 取值                                                     | 说明                              |
| ----- | ---- | ------ | -------------------------------------------------------- | --------------------------------- |
| `tag`   | 是   | String | `div`                                                    | 模块标签                          |
| `text`  | 是   | Struct | [text](/document/ukTMukTMukTM/uUzNwUjL1cDM14SN3ATN) | 单个文本展示，和`fields`至少要有一个 |
| `fields` | 否   | Struct | [field数组](/document/ukTMukTMukTM/uYzNwUjL2cDM14iN3ATN)                                             | 多个文本展示，和text至少要有一个  |
| `extra` | 否   | Struct | 元素的结构体  | 附加的元素展示在文本内容右侧。<br>可附加的元素包括[image](/document/ukTMukTMukTM/uAzNwUjLwcDM14CM3ATN)、[button](/document/ukTMukTMukTM/uEzNwUjLxcDM14SM3ATN)、[selectMenu](/document/ukTMukTMukTM/uIzNwUjLycDM14iM3ATN)、[overflow](/document/ukTMukTMukTM/uMzNwUjLzcDM14yM3ATN)、[datePicker](/document/ukTMukTMukTM/uQzNwUjL0cDM14CN3ATN)

## 设置富文本格式
消息卡片中支持 **加粗**、*斜体*、~~删除线~~、[超链接](https://larksuite.com)、@人、@所有人 等富文本格式。<br>

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3d8c3fd7a43a09fb2c549f736a6bb18b_fRgQkBJjpc.png)
你可以参考 [使用Markdown标签构造卡片内容](/document/ukTMukTMukTM/uADOwUjLwgDM14CM4ATN) ，使用markdown标签，为卡片文本配置富文本格式。





## 模块组合效果示例
在`extra`中组合`image`元素后，图文混排的效果示意如下：

![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/72eb3f60158cc1b05324c6b105ddf6c8.png)


**示例JSON：**<br>

```json
{
    "chat_id": "667*************032",
    "msg_type": "interactive",
    "card": {
        "elements": [
            {
                "tag": "div",
                "text": {
                    "tag": "plain_text",
                    "content": "Content module"
                },
                "fields": [
                    {
                        "is_short": false,
                        "text": {
                            "tag": "lark_md",
                            "content": "**module:**\nContent module（div）"
                        }
                    },
                    {
                        "is_short": false,
                        "text": {
                            "tag": "lark_md",
                            "content": "**function:**\nNew function"
                        }
                    }
                ],
                "extra": {
                    "tag": "img",
                    "img_key": "f32******************5cf7",
                    "alt": {
                        "tag": "plain_text",
                        "content": "alt_content"
                    }
                }
            }
        ]
    }
}
```



