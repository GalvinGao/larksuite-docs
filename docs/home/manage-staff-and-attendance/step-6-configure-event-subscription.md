---
document_id: '7275897728244072454'
directory_id: '7273792780344950790'
title: 步骤六：配置事件订阅
full_path: /home/quick-start-of-personnel-and-attendance-management-system/step-6-configure-event-subscription
breadcrumb:
- Home
- Manage Staff and Attendance
- 'Step 6: Configure event subscription'
document_type: GuideDocumentType
updated_at: 2023-09-08T08:38:10Z
source_url: https://open.larksuite.com/document/home/quick-start-of-personnel-and-attendance-management-system/step-6-configure-event-subscription
---

# 步骤六：配置事件订阅

机器人接收的消息会以回调事件请求的形式，通过 POST 请求送达到服务端处理。本地服务启动后，回调事件无法请求到内网，需配置公网请求 URL。

:::note
本教程为了方便实现，使用了反向代理工具 [localtunnel](https://www.npmjs.com/package/localtunnel) 完成内网穿透，暴露本地服务的公网访问入口。**该工具仅适用于开发测试阶段，不可用于生产环境，使用前请确认是否符合所在公司网络安全政策。**
:::

## 操作步骤

1. 运行以下命令获得 **公网** **URL**。
    

    示例代码中用 3002 端口监听事件订阅。

    
    ```
    npx localtunnel --port 3002
    ```
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/eab99dc349d84e69d60e06c4f0073cfa_jgTxkH9mt9.png?height=190&lazyload=true&maxWidth=600&width=870)
    
2. 在 **事件订阅** 页面，配置 **请求网址 URL**，填入的请求网址为上一步使用代理工具生成的 `公网 URL+/webhook/event` 。


    保存请求网址 URL 时会请求到后端服务，**请求期间需保证本地服务为启动状态**。
    
	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ffabaf74e11f7c721845b64a419443cc_BtbqD3YO3X.png?height=922&lazyload=true&maxWidth=600&width=1804)
        
3. 在 **机器人** 页面，配置 **消息卡片请求网址** ，填入的请求网址为上一步使用代理工具生成的 `公网 URL+/webhook/message-card`。


    保存请求网址 URL 时会请求到后端服务，**请求期间需保证服务为启动状态**。

    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/be5080d3e47b3430ab0d22ebb52d6bd8_QwIM14CTof.png?height=798&lazyload=true&maxWidth=600&width=2896)
    
4. 在 **事件订阅** 页面，单击 **添加事件**，搜索并添加以下事件。

    - 审批通过
    - [员工离职](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/events/deleted)
    - [员工入职](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/events/created)
    - [接收消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/receive)
    
:::note
监听事件需要开通对应的权限，为了成功订阅上述事件，需要在 **权限管理** 页面，为应用开通上述事件所需权限。为了方便演示，在本教程中，已经在[步骤一](/document/home/quick-start-of-personnel-and-attendance-management-system/step-1-create-and-configure-an-application)中统一开通了所需权限。
:::
    
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f9f50c2901efb1966fd034c78fb9b3a3_P81Ovzlo5X.png?height=1180&lazyload=true&maxWidth=600&width=2898)
