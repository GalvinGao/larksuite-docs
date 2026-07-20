---
document_id: '7074952334765670406'
directory_id: '7073442394955644933'
title: 步骤三：创建群聊并发送告警
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/the-robot-sends-an-alarm-notification
breadcrumb:
- Home
- Bot Auto Sends Group Alarm
- 'Step 3: Create a group chat and send an alert'
document_type: GuideDocumentType
updated_at: 2023-09-21T03:01:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/the-robot-sends-an-alarm-notification
---

# 步骤三：创建群聊并发送告警

本步骤通过运行 `/quick_start/robot/main.py` 文件，实现机器人的自动建群、拉人入群、发送告警卡片。并启动本地服务，监听卡片和事件回调。

代码实现所使用的 API 如下：

- [获取 tenant_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal)

- [创建群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create)

- [上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)

- [发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)

## 操作步骤

1. 进入 `/quick_start/robot/im.py` 文件，修改创建群时邀请的群成员 OpenID。

:::note
用户 OpenID 的获取方式，参见[通过手机号或邮箱获取用户 ID](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch_get_id)。
:::
        
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b0d5765ab07ccb98feddfe2ba24427b3_8Rcw1AQfWN.png?height=870&lazyload=true&maxWidth=600&width=2452)
        
        
2. 运行 `/quick_start/robot/main.py` 文件。

:::note
你需要保存回显信息中打印的会话 ID（`chat_id`），后续在获取群历史消息中需要使用该 ID。
:::

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/145f4f2ba135ccfcc8e51674c2b53ed0_051hVNzsuW.png?height=1416&lazyload=true&maxWidth=600&width=2756)
 
3. 登录Lark客户端，查看已创建的群与告警消息。
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f14c4a9e60c9d1798d67d6d13abf7932_UWslwubbAs.png?height=1496&lazyload=true&maxWidth=600&width=2130)
