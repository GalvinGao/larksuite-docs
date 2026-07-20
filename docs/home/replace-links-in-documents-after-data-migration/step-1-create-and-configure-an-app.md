---
document_id: '7233612551992164358'
directory_id: '7199928167142178821'
title: 步骤一：创建并配置应用
full_path: /home/replace-links-in-documents-after-data-migration/list-of-apis
breadcrumb:
- Home
- Replace links in documents after data migration
- 'Step 1: Create and configure an App'
document_type: GuideDocumentType
updated_at: 2023-05-16T06:13:17Z
source_url: https://open.larksuite.com/document/home/replace-links-in-documents-after-data-migration/list-of-apis
---

# 步骤一：创建并配置应用

在本步骤，你将创建一个测试应用，并获取应用凭证信息，用于调用服务端接口。
## 步骤一：创建测试应用
:::html
<md-td>
1. 登录[Lark开发者后台](https://open.larksuite.com/app)。
2. 在开发者后台首页，单击 **创建企业自建应用**，然后填写应用名称和应用描述，单击 **创建**。
 
  	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/7d4b05f2e242981f57e3ff9c04166f3c.png?lazyload=true&width=2552&height=1736" style="width:70%"/>
  
3. 在左侧导航栏单击进入 **测试企业和人员** 页面，单击 **创建测试企业**，填写 **测试企业名称**、**手机号**、**验证码**，单击 **确认创建**。
  
    :::note
    为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了[测试企业与人员功能](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)。在开发阶段，推荐开发者使用测试版应用，此**版本中涉及的权限和配置变更都会直接生效，无需管理员审核**，客户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。
    :::
  
  	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dc74dca16da2445b0996f0d00e8584af_P0P8gaPatc.png?lazyload=true&width=1192&height=600" style="width:40%"/>
  
4. 在创建完成的测试企业操作栏，单击 **关联应用**，即可为当前应用自动生成测试版本。
  
  	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4202be5e53167a7c40376eb6487b6e08.png?lazyload=true&width=2252&height=660" style="width:70%"/>
  
5. 左侧导航栏，单击应用名称右侧的切换图标并选择 **切换至测试版本**。
  
  	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/8623c8ddd59ef32854159c689c464b5a.png?lazyload=true&width=1398&height=672" style="width:70%"/>
</md-td>
:::
## 步骤二：配置应用权限

在**权限管理**页面，搜索并开通以下权限，
 * **docx:document**：创建及编辑新版文档。
 * **drive:drive**：查看、评论、编辑和管理云空间中所有文件。

:::html
<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/caf7d215e9b9d0e1d2889ac9ebd01a81.png?lazyload=true&width=2704&height=1118" style="width:70%"/>
:::



