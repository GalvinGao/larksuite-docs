---
document_id: '7156039415050993669'
directory_id: '7083440791243243525'
title: 步骤一：创建并配置应用
full_path: /home/calendar-event-sync/determine-which-api-needs-to-be-called
breadcrumb:
- Home
- Auto-fetch Calendar Events
- 'Step 1: Create and configure the application'
document_type: GuideDocumentType
updated_at: 2024-12-10T11:29:05Z
source_url: https://open.larksuite.com/document/home/calendar-event-sync/determine-which-api-needs-to-be-called
---

# 步骤一：创建并配置应用

通过本步骤您将创建一个测试应用，并开通应用权限，用于后续调用 API 以及配置事件订阅。

## 操作步骤

1. 登录[Lark开发者后台](https://open.larksuite.com/app)。

2. 在开发者后台首页，单击 **创建企业自建应用**，填写应用名称、描述以及图标信息，然后单击 **创建**。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e2650ad0e70391058e4d6ae540f1479d_EVeSrt9mm5.png?height=1394&lazyload=true&maxWidth=400&width=1162)

3. 关联测试企业，生成测试版本的应用。
    
    1. 在应用详情页左侧导航栏，进入 **测试企业和人员** 页面，并在页面右上角单击 **创建测试企业**。
        

        为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了[测试企业与人员功能](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)。在开发阶段，推荐开发者使用测试版应用，此**版本中涉及的权限和配置变更都会直接生效，无需管理员审核**，客户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。

        
    2. 在 **创建测试企业** 对话框，填写 **测试企业名称**、**手机号**、**验证码**，并单击 **确认创建**。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/85af43ae4f1337a78e80d3608c590449_sjd24PDf1K.png?height=1378&lazyload=true&maxWidth=600&width=3572)

    3. 创建测试企业后，在 **操作** 列，单击 **关联应用**。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/341586fdf85d2297f0eb9ef2e85a1b09_uprtuYUEqP.png?height=552&lazyload=true&maxWidth=600&width=2950)

    4. 测试企业关联应用后，在页面顶部切换企业应用为测试版应用。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5d934d17429ce3722de3fafa4ae4356e_PcrdD0Vs71.png?height=804&lazyload=true&maxWidth=600&width=3576)

4. 在 **应用能力** > **添加应用能力** 页面的 **按能力添加** 页签，找到 **机器人** 卡片，并点击 **添加**。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a5319501e30869ff10a202706946067a_ICVWILGdmr.png?height=900&lazyload=true&maxWidth=600&width=1682)

5. 在 **开发配置** > **权限管理** 页面，为应用添加权限。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e8398afc679d69da273ad4f1023c8d5b_eeUX6T6Xd4.png?height=1250&lazyload=true&maxWidth=600&width=2872)

    你需要查找并开通以下权限：
    
    - 获取日历、日程及忙闲信息
    - 更新日历及日程信息
    - 获取用户 user ID
    
    你可以直接将以下权限 Keys 粘贴到权限搜索框，确认所选权限无问题后，点击 **批量开通** 权限。
    
    ```
    calendar:calendar:readonly,calendar:calendar,contact:user.employee_id:readonly
    ```
6. 配置事件订阅。
    
    1. 在应用左侧导航栏，点击 **事件与回调**。  
    2. 在 **事件配置** 页签，点击 **订阅方式** 右侧的编辑图标。
    3. 填写 **请求地址**。
        
        关于请求地址的配置说明，参见[配置请求地址](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/request-url-configuration-case)。
      
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3e5610c131fdc0425c174681fea7f2aa_mIs4Inwg3U.png?height=870&lazyload=true&maxWidth=600&width=1614)
        
    4. 配置请求地址后，在 **已添加事件** 区域，点击 **添加事件**，查找并添加 **日历变更**v2.0 和 **日程变更**v2.0 事件。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9973a96f6c149dd27b74b7095b99f670_tIIoeFGle5.png?height=820&lazyload=true&maxWidth=600&width=2184)
