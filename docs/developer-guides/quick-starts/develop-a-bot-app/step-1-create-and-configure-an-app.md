---
document_id: '7280787361268678662'
directory_id: '7002892512470794246'
title: 步骤一：创建并配置应用
full_path: /home/develop-a-bot-in-5-minutes/step-1-create-app-and-enable-robot-capabilities
breadcrumb:
- Developer Guides
- Quick Starts
- Develop a Bot App
- 'Step 1: Create and configure an app'
document_type: GuideDocumentType
updated_at: 2024-02-08T03:49:33Z
source_url: https://open.larksuite.com/document/home/develop-a-bot-in-5-minutes/step-1-create-app-and-enable-robot-capabilities
---

# 步骤一：创建并配置应用

:::html
<md-td>
1. 在 [开发者后台](https://open.larksuite.com/app)，创建 **企业自建应用**，填写**名称**及**应用描述**，点击 **创建。**

  	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/396cce462bf6c487c47bcd1defb176d1_BJe8ZwFxG7.png?height=707&lazyload=true&maxWidth=400&width=600)


2. 在应用列表中，点击应用名称或应用图标进入应用详情页。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d1982dfe7c3898420a5f2d204e8eed99_frSj87mRzO.png?height=570&lazyload=true&maxWidth=600&width=1964)

3. 在左侧导航栏点击切换至 **添加应用能力** 页面，点击 **机器人** 下方的 **添加能力** 按钮。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5fde03fb710ac2454a5badb3918406d5_gVmS4vZvkJ.png?height=706&lazyload=true&maxWidth=600&width=1738)

4. 在左侧导航栏点击进入 **测试企业和人员** 页面，点击 **创建测试企业**，填写 **测试企业名称**、**手机号**、**验证码**，点击 **确认创建**。

	为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了[测试企业与人员功能](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)。在开发阶段，推荐开发者使用测试版应用，此**版本中涉及的权限和配置变更都会直接生效，无需管理员审核**，客户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。


	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dc74dca16da2445b0996f0d00e8584af_owd3xwapw7.png?lazyload=true&width=1192&height=600" style="width:50%" />

  
5. 在创建完成的测试企业操作栏，点击 **关联应用**，即可为当前应用自动生成测试版本。

   <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f3e9f47e3c48f93315c1cb6c0ef6973d_LW8YaM8Vcz.png?lazyload=true&width=2218&height=790" style="width:70%" />

6. 在左侧导航栏，点击应用名称右侧的切换图标并选择 **切换至测试版本**。

   <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1a214c6c9025733258ef537370036844_WxaLU00J73.png?lazyload=true&width=1484&height=754" style="width:70%" />

7. 配置应用权限。
  
	1. 查询 [接受消息事件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/receive) 依赖的权限清单，根据推送消息的场景，确定需要开通的应用权限。
    
    	本示例中所需的权限如下图所示：
    
   		 <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/acfac4110f1ac2937db5e4fbfea30bd2_7za7tRKlG2.png?lazyload=true&width=1804&height=1274" style="width:70%" />
  
	2. 在左侧导航栏点击切换至 **权限管理** 页面，搜索需要的权限配置，并开通权限 `获取用户发给机器人的单聊消息`。
  
    	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e436081ea0e41897ff548746fbf31b3f_FGUtYbX92N.png?lazyload=true&width=2124&height=514" style="width:70%" /> 
    
	3. 搜索并开通`获取与发送单聊、群组消息`权限，使机器人拥有发送消息的能力。
  
        <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/12cb48369617257d28ad331de1de0ca4_RFE1ya7v1g.png?lazyload=true&width=2032&height=720" style="width:70%" />
</md-td>
:::
