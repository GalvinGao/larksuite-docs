---
document_id: '7275897728244645894'
directory_id: '7273792780344934406'
title: 步骤一：创建测试应用
full_path: /home/quick-access-to-contact-api/step-1-create-a-test-application
breadcrumb:
- Home
- Quickly Integrate to Contacts
- 'Step 1: Create a test app'
document_type: GuideDocumentType
updated_at: 2024-06-05T08:53:58Z
source_url: https://open.larksuite.com/document/home/quick-access-to-contact-api/step-1-create-a-test-application
---

# 步骤一：创建测试应用


在使用LarkOpenAPI之前，你必须先创建一个应用。本文介绍如何创建应用，并配置其对应的测试应用。

## 一、创建应用

1. 登录[Lark开发者后台](https://open.larksuite.com/app)。

2. 在开发者后台首页，单击 **创建企业自建应用**，然后填写应用名称和应用描述，最后单击 **创建**。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/87f3be570ff473e78170ce195babfa27_a6aNyvbIZf.png?height=1380&lazyload=true&maxWidth=600&width=1178)

3. 在左侧导航栏单击进入 **测试企业和人员** 页面，单击 **创建测试企业**，填写 **测试企业名称**、**手机号**、**验证码**，单击 **确认创建**。

:::note
为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了 [测试企业与人员功能](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions) 。在开发阶段，推荐开发者使用测试版应用，此**版本中涉及的权限和配置变更都会直接生效，无需管理员审核**，客户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。
:::
  
![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/dc74dca16da2445b0996f0d00e8584af.png?height=600&lazyload=true&maxWidth=400&width=1192)
  
4. 在创建完成的测试企业操作栏，单击 **关联应用**，即可为当前应用自动生成测试版本。
  
    ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/67f2fc2eecb4c5637d9ea7ce9236789a.png?height=660&lazyload=true&maxWidth=600&width=2252)

5. 在左侧导航栏，单击应用名称右侧的切换图标并选择 **切换至测试版本**。
  

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/601ca1cb5f01606d647da57fb63f3f0b_qhAiJMu2Uc.png?height=890&lazyload=true&maxWidth=600&width=2164)
              
## 二、开通应用权限
              
在**权限管理**页面，搜索并开通以下权限。
* **contact:contact**：更新通讯录
* **contact:contact.base:readonly**：获取通讯录基本信息
