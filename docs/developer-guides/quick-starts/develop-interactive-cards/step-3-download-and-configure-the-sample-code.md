---
document_id: '7275897728244416518'
directory_id: '7273792780344868870'
title: 步骤三：下载并配置项目
full_path: /home/quickly-develop-interactive-cards/step-3-download-and-configure-the-project
breadcrumb:
- Developer Guides
- Quick Starts
- Develop Interactive Cards
- 'Step 3: Download and configure the sample code'
document_type: GuideDocumentType
updated_at: 2023-09-07T02:00:02Z
source_url: https://open.larksuite.com/document/home/quickly-develop-interactive-cards/step-3-download-and-configure-the-project
---

# 步骤三：下载并配置项目

在本步骤，你将下载并配置教程提供的示例代码。

## 操作步骤

1. 执行以下命令，下载[示例代码](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/79ef9303773712f3c5880f482c8f1dbe_uoV2JdMJV8.zip)。
     
     ```
     curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/79ef9303773712f3c5880f482c8f1dbe_uoV2JdMJV8.zip -o MESSAGE-CARD.zip
     ```

2. 下载完成后，使用`unzip`命令进行解压。
	
    Windows 用户可以直接使用解压缩工具进行解压。
    ```
    unzip message-card.zip
    ```

3. 在示例代码所在目录，执行以下命令，进入 **message-card** 目录。
    
    ```
    cd message-card
    ```
    
4. 配置项目中的变量值。
    
    1. 将 **appId**、**appSecret**、**verificationToken** 的参数值修改为实际的应用凭证参数值。
    
    	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bee69112c9f3849de173af2efff8aa98_3v5bC15kxp.png?height=1198&lazyload=true&maxWidth=600&width=1316)
    
   	 	应用的 **appId** 和 **appSecret** 可以在 [开发者后台](https://open.larksuite.com/app) 的 **凭证与基础信息** 页查看。
        
    	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dea8ebe8a5c72bdc531fb34da19633a0_8ssNUJ2ow7.png?height=462&lazyload=true&maxWidth=500&width=2280)
        
    	应用的 **Verification Token** 可以在 [开发者后台](https://open.larksuite.com/app) 的 **事件订阅** 页查看。
        
    	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4976a41a472358665cb26a3926258f93_YNGw65DoFY.png?height=942&lazyload=true&maxWidth=500&width=2080)
        
    2. 将 templateID 的参数值分别修改为实际的消息卡片 ID 。
    
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2ac967978c11f7a41e0aa83c1b6b54da_tWCtZqgXHr.png?height=1600&lazyload=true&maxWidth=600&width=2912)
        
        消息卡片 ID 可以在 [消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder) 中 **我的卡片** 栏中直接点击 **复制卡片 ID**，也可以在卡片预览区域点击复制卡片 ID。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e2a19268f718e362c67e23a2951590fe_xUlgMrejFw.png?height=1148&lazyload=true&maxWidth=500&width=1668)
