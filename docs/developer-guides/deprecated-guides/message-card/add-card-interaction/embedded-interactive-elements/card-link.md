---
document_id: '6967331173081333765'
directory_id: '7073444337254924293'
title: card_link
full_path: /ukTMukTMukTM/uYDN1UjL2QTN14iN0UTN
breadcrumb:
- Developer Guides
- Deprecated Guides
- Message Card
- Add card interaction
- Embedded interactive elements
- Card_link
document_type: GuideDocumentType
updated_at: 2022-03-13T12:48:01Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYDN1UjL2QTN14iN0UTN
---

# 消息卡片 card_link

`card_link`用于指定卡片整体的点击跳转链接，可以配置默认链接，也可以分别配置不同终端的链接，使用例如：

```json
{
    "config": {
        "wide_screen_mode": true
    },
    "card_link": { 
        "url": "https://www.baidu.com",
        "android_url": "https://developer.android.com/",
        "ios_url": "https://developer.apple.com/",
        "pc_url": "https://www.windows.com"
    },
    "header": {
        "title": {
            "tag": "plain_text",
            "content": "this is header"
        }
    },
    "elements": [
        {
            "tag": "div",
            "text": {
                "tag": "plain_text",
                "content": "This is a very very very very very very very long text;"
            }
        },
        {
            "tag": "action",
            "actions": [
                {
                    "tag": "button",
                    "text": {
                        "tag": "plain_text",
                        "content": "Read"
                    },
                    "type": "default"
                }
            ]
        }
    ]
}
```

::: note
如果配置了 ios_href、android_href 或者 pc_href , 则在相应端上优先使用指定的链接
:::

 **展示效果：** 依次分别在 iOS、Android、PC 上点击卡片，会跳转到不同的网址。<br>
![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/425051b2f3af6c9f529cedd5bda2508a.png)

`card_link` 详细可配字段和描述如下:

| 字段         | 必须 | 说明                 |
| ------------ | ---- | -------------------- |
| url         | 是   | 默认的链接地址       |
| pc_url      | 否   | PC 端的链接地址      |
| ios_url     | 否   | iOS 端的链接地址     |
| android_url | 否   | Android 端的链接地址 |
