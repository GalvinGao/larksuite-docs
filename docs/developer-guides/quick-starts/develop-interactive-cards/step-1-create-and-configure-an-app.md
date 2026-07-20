---
document_id: '7275897728244334598'
directory_id: '7273792780344868870'
title: 步骤一：创建并配置应用
full_path: /home/quickly-develop-interactive-cards/step-one-create-and-configure-the-application
breadcrumb:
- Developer Guides
- Quick Starts
- Develop Interactive Cards
- 'Step 1: Create and configure an app'
document_type: GuideDocumentType
updated_at: 2023-09-07T07:07:50Z
source_url: https://open.larksuite.com/document/home/quickly-develop-interactive-cards/step-one-create-and-configure-the-application
---

# 步骤一：创建并配置应用

在本步骤，你将创建一个测试应用，并为应用开启机器人能力及 OpenAPI 权限，为后续通过机器人应用发起审批操作做准备。

## 一、创建测试应用

在本步骤，你将创建一个测试版的企业自建应用。

### 操作步骤

1. 登录 [Lark开发者后台](https://open.larksuite.com/app)。

2. 单击 **创建企业自建应用**，然后填写应用 **名称** 和 **应用描述**，单击 **创建**。
	
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6ccb65d95f5ae9d4bce7f93d8881ff90_Tsp5wDYYlh.png?height=1404&lazyload=true&maxWidth=500&width=1190)

3. 在左侧导航栏单击进入 **测试企业和人员** 页面，单击 **创建测试企业**，填写 **测试企业名称**、**手机号**、**验证码**，单击 **确认创建**。
    
::: note
为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了[测试企业与人员](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)功能。在开发阶段，推荐开发者使用测试版应用，此版本中涉及的**权限和配置变更都会直接生效，无需管理员审核**，客户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。
:::

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dc74dca16da2445b0996f0d00e8584af_xi72GSo1ES.png?height=600&lazyload=true&maxWidth=500&width=1192)
    

4. 在创建完成的测试企业操作栏，单击 **关联应用**，即可为当前应用自动生成测试版本。
     
     ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4202be5e53167a7c40376eb6487b6e08_9b9b7ErrGL.png?height=660&lazyload=true&maxWidth=500&width=2252)
     
5. 左侧导航栏，单击应用名称右侧的切换图标并选择 **切换至测试版本。**
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5f1f18e8ea79283471f74d880fdce927_EX8Khle2XB.png?height=870&lazyload=true&maxWidth=500&width=2114)

## 二、开通应用权限

通过本步骤您将为创建的测试应用开启机器人能力并开通对应的应用权限，用于后续的 OpenAPI 调用。

### 操作步骤

1. 在 **添加应用能力** 页面，添加 **机器人** 功能。
   
   ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dc088bcaf885ec3d7da93e5e28cd17ff_8VoGdV92kI.png?height=694&lazyload=true&maxWidth=500&width=1696)
   
2. 在 **权限管理** 页面，搜索并开通以下权限：

	- 以应用的身份发消息
	
    - 读取用户发给机器人的单聊消息
    
    你可以直接将以下权限 Keys 粘贴到权限搜索框，点击 批量开通 权限。
    
    ```
    im:message:send_as_bot,im:message.p2p_msg:readonly
    ```
  
   	 ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/46487433df8c6890dfa98b1cb7996cb4_fTI7fwQ0i5.png?height=1268&lazyload=true&maxWidth=600&width=2866)
