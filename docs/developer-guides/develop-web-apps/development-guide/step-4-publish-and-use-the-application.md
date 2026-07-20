---
document_id: '7199617266244403206'
directory_id: '7130175417072861190'
title: 步骤四：发布与使用应用
full_path: /uYjL24iN/uMTMuMTMuMTM/development-guide/step-4
breadcrumb:
- Developer Guides
- Develop Web Apps
- Development Guide
- 'Step 4: Publish and use the application'
document_type: GuideDocumentType
updated_at: 2024-10-21T03:46:53Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMuMTMuMTM/development-guide/step-4
---

# 步骤四：发布与使用应用

完成应用的开发后，即可正式发布应用。已发布的应用可以通过Lark客户端访问使用。

## 发布应用

1. 登录[开发者后台](https://open.larksuite.com/app)。

2. 找到并进入需要发布的自建应用详情页。

3. 在左侧导航栏，选择 **应用发布** > **版本管理与发布**。

4. 在页面右上角，点击 **创建版本**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f46ddf4844d17ffefa5b0de51c4ebc73_bW2sI1hz0w.png?height=558&lazyload=true&maxWidth=600&width=2834)

5. 在 **版本详情** 页面，依次配置版本号、应用能力、可用性状态等属性，并点击 **保存**。
    
    - **应用版本号**：自定义版本号，格式示例：1.0.0。
    
    - **默认能力**：设置移动端和 PC 端的默认应用能力，此处需要选择 **网页应用**。
    
    - **更新说明**：当前应用版本的更新说明，将展示在更新日志中。
    
    - **可用范围**：自定义当前应用的可用范围，仅可用范围内的用户可以搜索并使用该应用。具体说明参见[配置应用可用范围](/document/home/introduction-to-scope-and-authorization/availability)。
    
    - **申请理由**：当前版本的申请理由，用于帮助审核人员了解应用相关附加信息。

6. 点击 **申请线上发布**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bd537581018397b72e7186f702044f0d_KR9DMVyoeP.png?height=702&lazyload=true&maxWidth=600&width=2324)
    
    等待应用审核通过后，即成功发布应用。

## 使用应用

当应用成功发布后，在应用可用范围内的用户即可通过Lark客户端使用应用。本章节以[快速开发网页应用](/document/home/integrating-web-apps-in-5-minutes/create-app-and-configuration)开发教程的示例项目为例，介绍不同的使用应用方式。

:::note
由于示例项目生成的访问地址是临时内网访问地址，因此需要确保移动端、桌面端和项目服务器同处一个局域网中。
:::

### 方式一

打开Lark移动端，在 **工作台** 中搜索并打开应用。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b62bf1ed8b7c6775425bdff991cae1b9_ubcbFnAOoH.png?height=1400&lazyload=true&maxWidth=280&width=670)

### 方式二

通过[Lark开发者工具](/document/uYjL24iN/ucDOzYjL3gzM24yN4MjN)访问示例项目，并通过二维码功能在Lark移动端打开使用应用。

1. 运行示例项目，获取临时访问地址。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c7c6735b81b416ea5a028391c73eb376_c4npnXqTY3.png?height=308&lazyload=true&maxWidth=600&width=2108)

2. 在同一局域网中，打开Lark开发者工具，添加一个网页项目。

3. 在网页项目中添加临时访问地址。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dcfcac4e8ea62a816b57e88ca6a638e1_tgLrHOPskV.png?height=962&lazyload=true&maxWidth=600&width=1908)

4. 打开预览功能内提供的移动端二维码。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/83f0aef39d4323a5e66597020fef4b9a_PhBkvcNTSf.png?height=1244&lazyload=true&maxWidth=600&width=2672)

5. 使用Lark移动端的扫一扫功能，扫描二维码使用应用。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b62bf1ed8b7c6775425bdff991cae1b9_ubcbFnAOoH.png?height=1400&lazyload=true&maxWidth=280&width=670)

### 方式三

打开Lark桌面端，在 **工作台** 中搜索并打开应用。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/275b3220f3bc163e9bf76712ed867823_L3MCKRhYlk.png?height=1200&lazyload=true&maxWidth=600&width=2200)
