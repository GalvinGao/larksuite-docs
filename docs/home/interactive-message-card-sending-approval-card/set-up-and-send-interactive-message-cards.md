---
document_id: '7074952334766047238'
directory_id: '7073442394955595781'
title: 设置可交互的消息卡片并发送
full_path: /home/interactive-message-card-sending/set-up-interactive-message-cards-and-send-them
breadcrumb:
- Home
- Interactive message card sending (approval card)
- Set up and send interactive message cards
document_type: GuideDocumentType
updated_at: 2023-05-16T03:12:02Z
source_url: https://open.larksuite.com/document/home/interactive-message-card-sending/set-up-interactive-message-cards-and-send-them
---

# 设置可交互的消息卡片并发送

**设置消息卡片请求网址**

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e8a026eec728a22444a1f088706a87f3_SCpoPw8TOJ.png?lazyload=true&width=2792&height=1288)

需要配置外网可访问的服务端回调地址。详细配置方法参考：[请求网址配置示例](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/request-url-configuration-case)

这样，当用户点击卡片上的交互按钮时，会向对应的地址发请求，我们需要在服务端代码里解析请求，做对应的业务逻辑处理，并返回一个新卡片来更新用户的卡片界面即可。

**构造可交互卡片**

比如可交互的按钮的话，可以参考文档

[button - 服务端文档 - Lark开放平台](/document/ukTMukTMukTM/uEzNwUjLxcDM14SM3ATN)

假设我们构造卡片如下：



```json 
{
  "i18n_elements": {
    "zh_cn": [
      {
        "tag": "action",
        "actions": [
          {
            "tag": "button",
            "text": {
              "tag": "plain_text",
              "content": "完成「全量发布」"
            },
            "value": {
              "button_type": "public_publish",
              "button_id": "1626958617426",
            },
            "type": "primary"
          }
        ]
      }
    ]
  }
}
``` 




其中value的值是可以自定义结构的。这里我们设置了`button_type`和`button_id`，前者是为了和其他类型的button区别开来，后者是为了在同一个button type类型中和其他实例区分开来，方便对卡片进行局部刷新。

那么对应的卡片是这样的：




![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7acfbd6a8e5f49157891f08f2fc37fad_iwKYIO993B.png?lazyload=true&width=1640&height=500)

我们需要把这个卡片的JSON代码存起来，以便后续处理用户交互时使用。

**处理可交互卡片的用户操作，返回更新的卡片**

需要参考的文档为

[交互模块 - 服务端文档 - Lark开放平台](/document/ukTMukTMukTM/uYjNwUjL2YDM14iN2ATN#49904b71)

解析请求主要要获取如下内容：

1.  谁点击的卡片。在`data.open_id`中
1.  是哪张卡片。在`data.open_message_id`中
1.  点击的哪个按钮。因为每个按钮的value是我们自己设置的，所以可以通过value来区分同一张卡中的不同按钮。value的数据在`data.action.value`中

获取到内容后，可以根据内容做自定义业务逻辑处理，然后返回更新后的卡片内容。

示例js代码如下：

```javascript
async public_publish_card_process(req, res) {
    const {open_message_id: message_id, action, open_id} = req.body
    const {button_type, msg_id} = action.value
    if (button_type !== 'public_publish') {
        // 其他类型的交互按钮，忽略
        return
    }
    // 调用自定义业务逻辑，如全量发布等
    const resp = await custom_business_logic({open_id, button_type})
    if (resp.code !== 0) {
        // 执行失败，直接返回
        return
    }
    // 获取存储的卡片内容并更新
    let {card} = await get_record("message_card", {message_id})
    card.i18n_elements.zh_cn = [{
        "tag": "div",
        "fields": [{
            "is_short": false,
            "text": {
                "tag": "lark_md",
                "content": "已完成"
            }
        }]
    }]
    // 返回更新后的卡片内容
    res.send(card) 
}
```
|     |   | *powered by guilin* |
| :-------- | -------:|  -------: |
