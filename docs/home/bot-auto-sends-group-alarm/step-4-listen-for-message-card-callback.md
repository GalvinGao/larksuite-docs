---
document_id: '7074952334766129158'
directory_id: '7073442394955644933'
title: 步骤四：监听消息卡片回调
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/monitor-and-interact-with-messages-in-the-group
breadcrumb:
- Home
- Bot Auto Sends Group Alarm
- 'Step 4: Listen for message card callback'
document_type: GuideDocumentType
updated_at: 2024-03-28T04:00:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/monitor-and-interact-with-messages-in-the-group
---

# 步骤四：监听消息卡片回调

在上一步骤中，已启动了本地服务并监听 **7777** 端口，本步骤中需要将本机地址配置到消息卡片回调地址，然后进行卡片交互。

:::warning
若本机无公网地址，需使用反向代理工具（[ngrok](https://ngrok.com/download)）完成内网穿透，暴露本地服务的公网访问入口。使用方式可参考[配置事件订阅](/document/home/develop-a-bot-in-5-minutes/step-5-configure-event-subscription)。**该工具仅适用于开发测试阶段，不可用于生产环境，使用前请确认是否符合所在公司网络安全政策。**
:::


代码实现所使用的能力如下：

- [卡片回调地址](/document/ukTMukTMukTM/uYzMxEjL2MTMx4iNzETM)

- [获取 tenant_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal)

- [获取群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/get)

- [更新群信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/update)

## 操作步骤

1. 登录[开发者后台](https://open.larksuite.com/app)， 然后进入应用详情页。
       
2. 进入 **机器人** 页面，点击 **机器人配置** 后方的编辑按钮。
    
3. 在 **消息卡片请求网址** 下方的输入框中，填写的你卡片回调地址，并验证、保存。
	
    地址格式为 `URL/card`，示例值：`https://www.example.com/card`。
    
	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/07419612cb5fac4fe89b137686fa289c_CXqxa8v56F.png?height=868&lazyload=true&maxWidth=600&width=2354)

4. 配置完成后，返回Lark客户端，在群组内的消息卡片中，点击 **跟进处理** 按钮。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fba608bfe6ab034ec53d0737e2f90a3e_pRb1rxXoqt.png?height=1498&lazyload=true&maxWidth=600&width=2116)

	卡片和群名称会修改为 `跟进中`。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dc8d242c2b3313f022fb435e96bdbd71_UMBsFrvosl.png?height=1506&lazyload=true&maxWidth=600&width=2148)
