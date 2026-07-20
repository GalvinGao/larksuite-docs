---
document_id: '7275897728244318214'
directory_id: '7273792780344885254'
title: 步骤一：创建并配置应用
full_path: /home/quick-access-to-base/step-1-create-and-configure-an-application
breadcrumb:
- Home
- Quickly Integrate to Base
- 'Step 1: Create and configure an application'
document_type: GuideDocumentType
updated_at: 2023-09-07T01:59:54Z
source_url: https://open.larksuite.com/document/home/quick-access-to-base/step-1-create-and-configure-an-application
---

# 步骤一：创建并配置应用

通过本步骤你将创建一个测试应用，并开通应用权限，用于后续调用 OpenAPI。

## 操作步骤

1. 登录[Lark开发者后台](https://open.larksuite.com/app)。

2. 在开发者后台首页，单击 **创建企业自建应用**，填写应用名称、描述以及图标信息，然后单击 **创建**。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/774c215ba2855a6ba5d319cbca2f6426_usHzCJfXv5.png?height=1116&lazyload=true&maxWidth=600&width=964)

3. 在应用详情页左侧导航栏，进入 **测试企业和人员** 页面，并在页面右上角单击 **创建测试企业**。

:::note
为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了[测试企业与人员功能](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)。在开发阶段，推荐开发者使用测试版应用，此**版本中涉及的权限和配置变更都会直接生效，无需管理员审核**，客户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。
:::
    
4. 在 **创建测试企业** 对话框，填写 **测试企业名称**、**手机号**、**验证码**，并单击 **确认创建**。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/85af43ae4f1337a78e80d3608c590449_JyCVCQrMKh.png?height=1378&lazyload=true&maxWidth=600&width=3572)

5. 创建测试企业后，在 **操作** 列，单击 **关联应用**。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/341586fdf85d2297f0eb9ef2e85a1b09_Td28olGi2k.png?height=552&lazyload=true&maxWidth=600&width=2950)

6. 测试企业关联应用后，在页面顶部切换企业应用为测试版应用。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5d934d17429ce3722de3fafa4ae4356e_w6ZVAbDjr5.png?height=804&lazyload=true&maxWidth=600&width=3576)

7. 开通应用权限。
   	
   本教程中使用到的多维表格 OpenAPI 需要 **查看、评论、编辑和管理多维表格** 权限。关于不同 OpenAPI 所需权限说明，请参见[权限列表](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/scope-list)。

    1. 在应用详情页左侧导航栏，单击 **权限管理**。

    2. 在 **权限配置** 页面左侧列表，单击 **云文档**，找到 **查看、评论、编辑和管理多维表格** 权限，并在 **操作** 列单击 **开通权限**。

        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ce355ed78c01ae11c8926fb455fe21a1_vW5MQDqPW9.png?height=704&lazyload=true&maxWidth=600&width=2978)
