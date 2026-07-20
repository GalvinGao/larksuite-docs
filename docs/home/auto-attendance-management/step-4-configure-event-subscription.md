---
document_id: '7275897728244563974'
directory_id: '7273792780344918022'
title: 步骤四：配置事件订阅
full_path: /home/automatic-attendance-management-based-on-approval/step-4-configure-the-event
breadcrumb:
- Home
- Auto Attendance Management
- 'Step 4: Configure event subscription'
document_type: GuideDocumentType
updated_at: 2023-09-07T01:59:46Z
source_url: https://open.larksuite.com/document/home/automatic-attendance-management-based-on-approval/step-4-configure-the-event
---

# 步骤四：配置事件订阅

机器人接收的消息会以回调事件请求的形式，通过 POST 请求送达到服务端处理。本地服务启动后，回调事件无法请求到内网，需配置公网请求 URL。


  
## 操作步骤

:::note
本教程为了方便实现，使用了反向代理工具（[localtunnel](https://www.npmjs.com/package/localtunnel)）完成内网穿透，暴露本地服务的公网访问入口。
**该工具仅适用于开发测试阶段，不可用于生产环境，使用前请确认是否符合所在公司网络安全政策**。
:::


1. 运行以下命令获得公网 URL。

    ```Plain Text
    npx localtunnel --port 3000
    ```

	成功运行结果如下图：

    ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/d66501d3b188ce3a6c5c8219037fd68b.png?height=150&lazyload=true&maxWidth=600&width=1112)

2. 在 **事件订阅** 页面，配置 **请求网址** **URL**，填写使用代理工具生成的公网域名 + `/webhook/event`。

:::note
保存请求网址 URL 及发送消息给机器人，都会请求到后端服务，**请求期间需保证服务为启动状态**。
:::
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3f709007448965686475ceefbd1941f4_IssSBkiQgM.png?height=864&lazyload=true&maxWidth=600&width=1694)


3. 单击添加事件，搜索并添加 **审批通过** 和 **接收消息** 事件。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/83fed4ee1b7869255a185ab1fcdac735_5Ojp7yFL5D.png?height=990&lazyload=true&maxWidth=600&width=1842)


