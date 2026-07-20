---
document_id: '7275897728244039686'
directory_id: '7273792780344934406'
title: 步骤三：配置事件订阅请求地址
full_path: /home/quick-access-to-contact-api/step-3-configure-the-event-subscription-request-address
breadcrumb:
- Home
- Quickly Integrate to Contacts
- 'Step 3: Configure event subscription'
document_type: GuideDocumentType
updated_at: 2023-09-07T02:00:18Z
source_url: https://open.larksuite.com/document/home/quick-access-to-contact-api/step-3-configure-the-event-subscription-request-address
---

# 步骤三：配置事件订阅请求地址

服务端接收的消息会以回调事件请求的形式，通过 POST 请求送达到服务端处理。本地服务启动后，回调事件无法请求到内网，需配置公网请求 URL。

## 操作步骤
:::note
本教程为了方便实现，使用了反向代理工具（[localtunnel](https://www.npmjs.com/package/localtunnel)）完成内网穿透，暴露本地服务的公网访问入口。**该工具仅适用于开发测试阶段，不可用于生产环境，使用前请确认是否符合所在公司网络安全政策。**
:::

1. 运行以下命令获得公网 URL。

    ```PowerShell
    npx localtunnel --port 3001
    ```

  	成功运行的结果如下图：
    
    ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/6ba14823023f7827def9c29788cb8f35.png?height=174&lazyload=true&maxWidth=600&width=1088)

2. 在 **事件订阅** 页面，配置 **请求网址** **URL**，填写使用代理工具生成的公网域名 + `/webhook/event`。

:::note
保存请求网址 URL 及发送事件消息给服务端，都会请求到后端服务，**请求期间需保证服务为启动状态**。
:::

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d0201da8108b944e16dd96aa0ab005d1_9UlQOsgvSD.png?height=892&lazyload=true&maxWidth=600&width=1640)


3. 单击添加事件，搜索并添加 **员工入职** 和 **员工离职** 事件。

:::note
监听事件也需要开通对应的权限，在本教程中，为了接收 **员工入职** 和 **员工离职** 事件，需要开通 **以应用身份读取通讯录** 权限。本教程为了演示方便，已经在步骤一中统一开通了所需权限。
:::

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0ae2ab2789100b15073ca18d13718370_HwpQzMbvnm.png?height=596&lazyload=true&maxWidth=600&width=1766)
