---
document_id: '7265156745869508613'
directory_id: '7280804871254589446'
title: 机器人菜单使用说明
full_path: /uAjLw4CM/ukTMukTMukTM/bot-v3/bot-customized-menu
breadcrumb:
- Developer Guides
- Develop Bots
- Bot customized menu
document_type: GuideDocumentType
updated_at: 2023-08-09T07:48:59Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/bot-v3/bot-customized-menu
---

# 机器人菜单使用说明
## 功能概述
通过为你的机器人配置自定义菜单，你可以将应用的常用入口固定在机器人聊天的输入框上。

终端用户在使用机器人时，可以用更自然的方式了解到机器人能做什么，并快捷跳转到应用的常用功能页上。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d0b6117e506b698e11266440e0b75d10_fPVUAMgyAM.gif?height=1080&lazyload=true&width=1920)

## 配置方法
在[Lark开发者后台](https://open.larksuite.com/app)为你的应用开启机器人能力后，如下图所示，即可通过可视化的方式完成机器人菜单配置。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/688308c3d89dde32b177dff3f164c3f7_P7PbUcHMWX.gif?height=974&lazyload=true&width=1504)

:::note
**你需要注意的是**：
* **精简菜单文案**：机器人菜单可展示的文案空间有限，建议你精简菜单文案，避免在小屏设备上无法展示完整的内容。
* **菜单项需发布后生效**：在保存机器人菜单配置后，你需要继续提交应用版本发布，配置才可生效。菜单在设备上生效可能有延迟，版本发布成功后，**请稍候5分钟**再查看菜单效果。
* **菜单的个数限制**：一个机器人可配置**最多3个**一级菜单入口，每个一级菜单下可配置**最多5个**二级菜单。
* **可用的客户端版本**：终端用户需要升级Lark客户端至 **5.27及以上版本** 才可见机器人菜单。请注意检查并更新你的Lark至最新版本。
:::

## 进阶用法
除了在菜单上配置普通的链接跳转外，如果你有一定的服务端开发基础，还可以进阶配置菜单交互：
-  **配置 Applink 跳转Lark的功能**：你可以在菜单链接中配置applink，实现例如打开小程序、打开群聊等Lark客户端内的功能跳转。[Applink支持的协议说明](/document/uAjLw4CM/uYjL24iN/applink-protocol/applink-introduction/applink-application)
-  **配置机器人菜单点击事件，作为逻辑触发器**：你可以订阅“机器人自定义菜单事件”，并在机器人菜单中配置用户点击菜单项后的回传参数信息。<br>在接收到用户的点击事件后，你可以通过服务端openAPI调用的方式响应用户操作，例如：在聊天里为用户发送使用帮助卡片。


| 操作 | 示意图 |
| --- | --- |
| **Step1**: 订阅机器人菜单的点击事件<br>事件订阅的基本配置过程可参考[事件订阅流程](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)。 | <img src=//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4dec6c8a870b063daf7ce0f0b2aa55e4_zk9HtN4vrw.png style="width: 95%"> |
| **Step2**：配置点击事件ID参数，用于识别用户具体点击了哪个菜单项 | <img src=//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fbbe13bf884555fd4e068de1abe94304_cDzwBQNYje.png style="width: 95%"> |



用户点击菜单项后，你的服务端将接收到如下规格的事件回调。你可以基于这个事件触发下一步操作，例如：给用户发送一条消息卡片以响应用户的点击操作。



**事件体示例：**

详细的事件说明文档可参考[机器人自定义菜单事件](/document/uAjLw4CM/ukTMukTMukTM/application-v6/bot/events/menu)。

```json 
 {
    "schema": "2.0",
    "header": {
        "event_id": "5e3702a84e847582be8db7fb73283c02",//事件ID
        "event_type": "application.bot.menu_v6",//事件类型标识，application.bot.menu_v6 表示这是用户点击机器人菜单触发的事件
        "create_time": "1608725989000",//事件创建时间戳（单位：毫秒）
        "token": "rvaYgkND1GOiu5MM0E1rncYC6PLtF7JV",//事件 Token
        "app_id": "cli_9f5343c580712544",//应用 ID
        "tenant_key": "2ca1d211f64f6438"//应用所属的租户 Key
    },
    "event": {
        "operator": {
            "operator_name": "张三",//操作人名称
            "operator_id": { ////操作人的用户ID
                "union_id": "on_8ed6aa67826108097d9ee143816345",
                "user_id": "e33ggbyz",
                "open_id": "ou_84aad35d084aa403a838cf73ee18467"
            }
        },
        "event_key": "menu2022********67dfc8b885136",//菜单事件的唯一标识,透传在开发者后台配置的自定义事件ID
        "timestamp": "1669364458"//用户点击菜单时间戳（单位：秒）
    }
}
``` 

## 反馈建议
如你在使用过程中，对机器人菜单有任何产品建议和反馈，欢迎点击链接加入 [Lark开发者互助交流群](https://applink.larksuite.com/client/chat/chatter/add_by_link?link_token=1dclf18c-e1c4-4d87-b0da-98bc7bde71cq) 发帖。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3761071914618705fbcb5bcdbab85d26_X7KDSMOM3J.png?height=760&lazyload=true&width=2880)
