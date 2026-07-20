---
document_id: '7298247881122955269'
directory_id: '7199928167142227973'
title: 步骤一：创建并配置应用
full_path: /home/management-weekly-report-based-docs/step-1-create-and-configure-an-application
breadcrumb:
- Home
- Manage Weekly Report with Docs
- 'Step 1: Create and configure an application'
document_type: GuideDocumentType
updated_at: 2024-07-15T06:21:09Z
source_url: https://open.larksuite.com/document/home/management-weekly-report-based-docs/step-1-create-and-configure-an-application
---

# 步骤一：创建并配置应用

在本步骤，你将创建一个测试应用，并获取应用凭证信息，用于调用服务端接口。
:::html
<md-td>
## 步骤一：创建测试应用

1. 登录[Lark开发者后台](https://open.larksuite.com/app)。

2. 在开发者后台首页，单击 **创建企业自建应用**，然后填写应用名称和应用描述，单击 **创建**。
  
	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5125302cc33d571373a53f1be4c7afc5_MCa27HCmLp.png?lazyload=true&width=2552&height=1736" style="width:60%"/>

3. 在左侧导航栏单击进入 **测试企业和人员** 页面，单击 **创建测试企业**，填写 **测试企业名称**、**手机号**、**验证码**，单击 **确认创建**。
    

 	为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了[测试企业与人员](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)功能。在开发阶段，推荐开发者使用测试版应用，此版本中涉及的**权限和配置变更都会直接生效，无需管理员审核**，客户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。

  
	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/dc74dca16da2445b0996f0d00e8584af.png?lazyload=true&width=1192&height=600" style="width:50%"/>

4. 在创建完成的测试企业操作栏，单击 **关联应用**，即可为当前应用自动生成测试版本。
  
	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4202be5e53167a7c40376eb6487b6e08.png?lazyload=true&width=2252&height=660" style="width:70%"/>

5. 左侧导航栏，单击应用名称右侧的切换图标并选择 **切换至测试版本**。
  
	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b1d98c83a1e4d818a36a79c3f27bd4fb_qmQFMvu4Gi.png?lazyload=true&width=1398&height=672" style="width:70%"/>
</md-td>
:::
## 步骤二：添加应用权限
:::html
<md-td>
1. 单击进入**添加应用能力**页面，添加**机器人**功能。

    开启机器人能力之后，才可以使用机器人发送消息。

	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/96eea87355b3ad284e4ac3ba216b461e_EbX8sFR8Zx.png?lazyload=true&width=2432&height=1580" style="width:70%"/>

2. 在左侧导航栏选择 **权限管理**，在 **API 权限** 页面开通以下权限。
   
     * **drive:drive**：查看、评论、编辑和管理云空间中所有文件
     * **wiki:wiki**：查看、编辑和管理知识库
     * **im:chat:read**：查看群信息
     * **im:message:send_as_bot**：以应用的身份发消息
  
	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d17312b80fcc965c1bb4895c512ccb80_w8QXschGgh.png?lazyload=true&width=2650&height=1640" style="width:70%"/>

3. 单击 **安全设置** ，添加 **重定向** **URL** 为 `http://127.0.0.1:3000`。


    后续需要知识库管理账号在浏览器访问该地址，添加周报管理应用为知识库成员，不进行此操作应用无法将周报文档归档到知识库。

  
	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/451b0c1c4989bd6ec62874ddd98bad06_dQdGt4Cwld.png?lazyload=true&width=2558&height=1162" style="width:70%"/>
</md-td>
:::
