---
document_id: '6967331173081432069'
directory_id: '7073444337254891525'
title: image
full_path: /ukTMukTMukTM/uAzNwUjLwcDM14CM3ATN
breadcrumb:
- Developer Guides
- Deprecated Guides
- Message Card
- Add card interaction
- Embedded non-interactive elements
- Image
document_type: GuideDocumentType
updated_at: 2022-03-13T12:47:47Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uAzNwUjLwcDM14CM3ATN
---

# image

**元素介绍：**<br>

​    **作为图片元素被使用**

​   `image`属于内容元素的一种，可用于内容块的`extra`字段和备注块的`elements`字段。


**字段定义：**<br>

| 字段    | 必须 | 类型   | 取值     | 说明          |
| ------- | ---- | ------ | -------- | ------------- |
| tag     | 是   | String | "img"    | 元素标签     |
| img_key | 是   | String |          | 图片资源，获取方式：[上传图片](/document/ukTMukTMukTM/uEDO04SM4QjLxgDN)      |
| alt     | 是   | Struct |[text](/document/ukTMukTMukTM/uUzNwUjL1cDM14SN3ATN) | 图片hover说明 |
| preview   | 否   | Bool |true / false| 点击后是否放大图片，缺省为true。在配置 [card_link](/document/ukTMukTMukTM/uYDN1UjL2QTN14iN0UTN) 后可设置为false，使用户点击卡片上的图片也能响应card_link链接跳转            |


**效果样例：**<br>


![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/dc7a626c83c15fd3c1362e28e2c72d01.png)



**JSON样例：**<br>

```json
{
    "chat_id": "667*****************032",
    "msg_type": "interactive",
    "card": {
        "elements": [
            {
                "tag": "div",
                "text": {
                    "tag": "plain_text",
                    "content": "image element"
                },
                "extra": {
                    "tag": "img",
                    "img_key": "8ef***************************952",
                    "alt": {
                        "tag": "plain_text",
                        "content": "hover图片后的tips文案"
                    }
                }
            },
            {
                "tag": "hr"
            },
            {
                "tag": "note",
                "elements": [
                    {
                        "tag": "img",
                        "img_key": "f32**************************cf7",
                        "alt": {
                            "tag": "plain_text",
                            "content": "hover图片后的tips文案"
                        }
                    },
                    {
                        "tag": "plain_text",
                        "content": "note"
                    }
                ]
            }
        ]
    }
}
```



**注意事项：**<br>

- 放置在内容模块的extra的图片尺寸固定为64*64
- 放置在备注模块的elements的图片尺寸固定为16*16
