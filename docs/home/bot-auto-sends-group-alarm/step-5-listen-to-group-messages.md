---
document_id: '7074952334765850630'
directory_id: '7073442394955644933'
title: 步骤五：监听群消息
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/modify-the-group-name-after-solving-the-problem
breadcrumb:
- Home
- Bot Auto Sends Group Alarm
- 'Step 5: Listen to group messages'
document_type: GuideDocumentType
updated_at: 2023-09-21T03:01:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/modify-the-group-name-after-solving-the-problem
---

# 步骤五：监听群消息

本步骤中需要将本机地址配置到事件回调地址，并订阅消息事件，以实现机器人监听群消息。

代码实现所使用的能力如下：

- [接收消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/receive)

- [获取 tenant_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal)

- [获取群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/get)

- [更新群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/update)

- [发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)


## 操作步骤

1. 登录[开发者后台](https://open.larksuite.com/app)，然后进入应用详情页。
       
2. 进入 **事件订阅** 页面，点击 **请求地址配置** 后方的编辑按钮。
    
3. 填写回调地址，并保存。
	
    地址格式为 `URL/event`，示例值：`https://www.example.com/event`。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0f771df6f3e8af4620397fc860193b17_oWVwPECAvs.png?height=1080&lazyload=true&maxWidth=600&width=2308)

4. 在页面下方的 **已添加事件** 区域，点击 **添加事件**，并添加 **接收消息** v2.0。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c0c4a975383b68274494af4733cd65db_0eeK3K6ngu.png?height=1172&lazyload=true&maxWidth=600&width=2234)

5. 配置完成后，返回Lark客户端，在群聊中 @ 机器人，并回复 `/solve`。
    
	你可以查看到机器人自动回复消息，并自动修改群名称。
    
	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/96fb3612d25cda48f28e29f848f112ba_OhUshibI9E.png?height=1504&lazyload=true&maxWidth=600&width=2118)
