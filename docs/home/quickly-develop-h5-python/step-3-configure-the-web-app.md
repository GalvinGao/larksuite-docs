---
document_id: '7273741028051009542'
directory_id: '7002892512470810630'
title: 步骤三：配置网页应用
full_path: /home/integrating-web-apps-in-5-minutes/step-4-configure-the-home-page-address
breadcrumb:
- Home
- Quickly Develop H5 (Python)
- 'Step 3: Configure the web app'
document_type: GuideDocumentType
updated_at: 2023-09-01T06:36:40Z
source_url: https://open.larksuite.com/document/home/integrating-web-apps-in-5-minutes/step-4-configure-the-home-page-address
---

# 步骤三：配置网页应用

运行示例代码获取应用域名信息后，返回Lark开发者后台配置网页应用。

## 操作步骤

1. 在 [开发者后台](https://open.larksuite.com/app/)，从左侧导航栏点击**添加应用能力**，然后选择 **网页应用** 点击**添加能力**。

2. 在 **网页配置** 中填写 **桌面端主页** 和 **移动端主页**， 填写在步骤二中获取的临时内网访问地址，例如 `http://10.86.120.185:3000` 。

::: warning
正式上线应用时，此处配置主页地址需为**公网地址**。为了快速体验接入流程，本示例中暂时先使用本地环境。
::: 
  
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d397f521373055e77d57b508d1e5b879_Gi06Dj0Gzq.png?height=1560&lazyload=true&maxWidth=600&width=2972)
    
3. 从左侧导航栏点击 **安全设置**，在 **H5可信域名** 中添加需要调用 JSAPI 接口的页面所在域名。

    本示例中配置 H5 可信域名为上述获取的临时内网访问地址，即 `http://10.86.120.185:3000`。


    ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/19f53983ef21c029edb115523b44f9d1_nFzacC8SS5.png?height=1696&lazyload=true&maxWidth=600&width=2964)
