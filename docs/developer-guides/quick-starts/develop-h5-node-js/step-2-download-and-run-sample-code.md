---
document_id: '7273741028050993158'
directory_id: '7270783571969998854'
title: 步骤二：下载并运行示例代码
full_path: /home/quickly-develop-a-web-app-nodejs/step-2-download-and-run-sample-code
breadcrumb:
- Developer Guides
- Quick Starts
- Develop H5 (Node.js)
- 'Step 2: Download and run sample code'
document_type: GuideDocumentType
updated_at: 2023-09-01T09:50:58Z
source_url: https://open.larksuite.com/document/home/quickly-develop-a-web-app-nodejs/step-2-download-and-run-sample-code
---

# 步骤二：下载并运行示例代码

在本步骤中，您将下载并运行教程提供的示例代码。示例代码需要本地环境已安装 Node.js。

## 操作步骤

1. 执行以下命令，下载[示例代码](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bbf019442bbbac7005994185be306e22_ZqyFMZqgeX.zip)至本地。
    
   ```bash
   curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bbf019442bbbac7005994185be306e22_ZqyFMZqgeX.zip -o web_app_quick_start.zip
   ```
   
2. 解压示例代码（web_app_quick_start.zip）。

3. 在示例代码所在目录，执行以下命令安装依赖。

   ```bash
   cd web_app_quick_start/
   npm install
   ```
   
4. 运行以下命令，配置应用。
   
   ```bash
   npm run config
   ```
   
   根据命令行回显信息，依次输入应用的 App ID、App Secret、API Port（默认值 8989）。
   
   ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f55bb8e2fc07550c45946fb590377666_vo2fLLz9Ka.png?height=348&lazyload=true&maxWidth=500&width=1198)
   
   应用凭证 App ID 和 App Secret 获取方式：
   
    1. 登录[Lark开发者后台](https://open.larksuite.com/app)。
    
    2. 进入应用详情页，在左侧导航栏，单击 **凭证与基础信息**。
    
    3. 在 **应用凭证** 区域，获取并保存 **App ID** 和 **App Secret**。
       
       ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f7f89950be7e57c2760a8b5b1f5e17c9_fCD9ruySAG.png?height=524&lazyload=true&maxWidth=600&width=3594)

5. 运行以下命令，启动本地服务。
   
   ```bash
   npm run start
   ```
   
   服务启动需要一定时间，请耐心等待。成功启动后回显信息如下图所示。
   
   ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d0f880a8aa7dd1d1ca5e27bcf6a161ce_0pinEJbCJx.png?height=446&lazyload=true&maxWidth=400&width=964)
   
:::note
您需要保存 **On Your Network** 对应的域名信息，后续用于配置网页应用。
:::
