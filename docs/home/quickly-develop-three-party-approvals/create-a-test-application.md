---
document_id: '7139727756258017286'
directory_id: '7137610938320961542'
title: 创建测试应用
full_path: /home/quickly-develop-three-party-approvals/creating-applications-and-requesting-permissions
breadcrumb:
- Home
- Quickly develop three-party approvals
- Create a test application
document_type: GuideDocumentType
updated_at: 2023-05-15T02:36:24Z
source_url: https://open.larksuite.com/document/home/quickly-develop-three-party-approvals/creating-applications-and-requesting-permissions
---

# 创建测试应用

在本步骤你将创建一个测试应用，用于接口调用。

## 步骤一：创建应用
:::html
<md-td>
1. 登录[Lark开发者后台](https://open.larksuite.com/app)。

2. 在开发者后台首页，单击 **创建企业自建应用**，然后填写应用名称和应用描述，最后单击 **创建**。
  
	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c971057edcba9ef4942046411022b4fd_tBWIpPFxs6.png" style="width:40%"/>

3. 应用详情界面的**凭证与基础信息**栏里，可以查询到应用凭证，也就是 App ID 和 App Secret。 

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d83455ca43683cc680c05362cd96075f_1jSvnnCjcv.png" style="width:70%"/>
  
</md-td>
:::
## 步骤二：创建并切换至测试版应用

为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了[测试企业与人员功能](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)。在开发阶段，推荐开发者使用测试版应用，此**版本中涉及的权限和配置变更都会直接生效，无需管理员审核**，客户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。

:::html
<md-td>
1. 在左侧导航栏单击进入 **测试企业和人员** 页面，单击 **创建测试企业**，填写 **测试企业名称**、**手机号**、**验证码**，单击 **确认创建**。

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dc74dca16da2445b0996f0d00e8584af_P0P8gaPatc.png?lazyload=true&width=1192&height=600" style="width:40%"/>

2. 在创建完成的测试企业操作栏，单击 **关联应用**，即可为当前应用自动生成测试版本。

	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0f9208ec71810a84b46eb111266523db_96fe7rEWeS.png?lazyload=true&width=2206&height=658" style="width:70%"/>

3. 在左侧导航栏，单击应用名称右侧的切换图标并选择 **切换至测试版本**。
  
	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e23b21c4d7d3f905a98a4bea78de381a_pSIBTR5E9d.png?lazyload=true&width=1368&height=538" style="width:70%"/>

4. 在测试应用中，单击添加应用能力，然后添加小程序能力。
  
	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/76142837929a1c141d43b3525c36ebe4_TBiq9S0fjh.png?lazyload=true&width=2474&height=1508" style="width:70%"/>
  
</md-td>
:::



## 步骤三：开通权限
:::html
<md-td>
单击**权限管理**切换页面，搜索审批需要的**权限配置**，并申请开通权限。

<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4d327541a96731b9980f912a942e6bb5_rpFRQjzWOE.png?lazyload=true&width=2840&height=1610" style="width:70%"/>

</md-td>
:::
到现在为止，你已经开启了应用的审批权限， 此时就可以开始进行后端服务代码的编写了
