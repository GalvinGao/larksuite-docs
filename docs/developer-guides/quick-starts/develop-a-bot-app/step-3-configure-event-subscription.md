---
document_id: '7280787361268645894'
directory_id: '7002892512470794246'
title: 步骤三：配置事件订阅
full_path: /home/develop-a-bot-in-5-minutes/step-5-configure-event-subscription
breadcrumb:
- Developer Guides
- Quick Starts
- Develop a Bot App
- 'Step 3: Configure event subscription'
document_type: GuideDocumentType
updated_at: 2024-02-08T03:49:36Z
source_url: https://open.larksuite.com/document/home/develop-a-bot-in-5-minutes/step-5-configure-event-subscription
---

# 步骤三：配置事件订阅

  
机器人接收的消息会以回调事件请求的形式，通过 POST 请求送达到服务端处理。本地服务启动后，回调事件无法请求到内网，需配置公网请求 URL。

:::warning
本教程为了方便实现，使用了反向代理工具（[ngrok](https://ngrok.com/download)）完成内网穿透，暴露本地服务的公网访问入口。**该工具仅适用于开发测试阶段，不可用于生产环境，使用前请确认是否符合所在公司网络安全政策。**
:::

:::html
<md-td>
1. 注册并安装 [ngrok](https://ngrok.com/download)，按照官方指引完成安装。
2. 在个人的 [dashboard 页面](https://dashboard.ngrok.com/get-started/your-authtoken) 中，获取 Authtoken。
  
    <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3a6d48a86bbd55342a81f80e43561204_DQ45y4wKim.png?lazyload=true&width=2480&height=624" style="width:70%" />
  
3. 运行以下命令获得公网 URL。
  
    ```
    ngrok authtoken "token" // token需替换为实际值
    ngrok http 3000
    ```
  
    成功运行结果如下图：
  
    <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8defec8701727380a1818b152681e206_iWbpgbJZ5E.png?lazyload=true&width=2490&height=882" style="width:70%" />
  
4. 在 **事件订阅** 页面，配置 **请求网址** **URL**，填写使用 ngrok 工具生成的公网域名。
  

    保存请求网址 URL 及发送消息给机器人，都会请求到后端服务，**请求期间需保证服务为启动状态**。


	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ad3902b6401df261fa26fad6a1f6ea84_rlLN7IXW7n.png?height=1012&lazyload=true&maxWidth=600&width=2486)

  
5. 在左侧导航栏点击切换至 **事件订阅** 页面，点击**添加事件**，勾选并确认添加 `接收消息` 事件，完成订阅。

	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cdf1990c402747051635f4db27a00051_hcevuSG5F5.png?lazyload=true&width=1674&height=1026" style="width:70%" />


    未开通所需的应用权限前，所添加的事件无法被后端服务监听到。

  
	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3b2f274c0f604d221154d6b36041a2ab_GqxWkp4V1u.png?lazyload=true&width=1274&height=774" style="width:70%" />

</md-td>
:::
