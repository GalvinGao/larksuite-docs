---
document_id: '7074952334765768710'
directory_id: '7073442394955644933'
title: 步骤一：创建并配置应用
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/turn-on-app-permissions
breadcrumb:
- Home
- Bot Auto Sends Group Alarm
- 'Step 1: Create and configure the application'
document_type: GuideDocumentType
updated_at: 2024-07-15T06:21:00Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-development-tutorial/turn-on-app-permissions
---

# 步骤一：创建并配置应用

通过本步骤您将创建一个测试应用，并开通应用权限，用于后续调用 API 以及配置事件订阅。

## 操作步骤

1. 登录[Lark开发者后台](https://open.larksuite.com/app)。

2. 在开发者后台首页，单击 **创建企业自建应用**，填写应用名称、描述以及图标信息，然后单击 **创建**。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8024a7e2fd42054b4653d54fc884ae54_JxjdJMFLmA.png?height=1526&lazyload=true&maxWidth=600&width=2512)
    
3. 关联测试企业，生成测试版本的应用。

    1. 在应用详情页左侧导航栏，进入 **测试企业和人员** 页面，并在页面右上角单击 **创建测试企业**。
    

        为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了[测试企业与人员功能](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)。在开发阶段，推荐开发者使用测试版应用，此**版本中涉及的权限和配置变更都会直接生效，无需管理员审核**，客户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。

        
    2. 在 **创建测试企业** 对话框，填写 **测试企业名称**、**手机号**、**验证码**，并单击 **确认创建**。
    
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/85af43ae4f1337a78e80d3608c590449_ju0AuXRoSM.png?height=1378&lazyload=true&maxWidth=600&width=3572)

    3. 创建测试企业后，在 **操作** 列，单击 **关联应用**。
    
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/341586fdf85d2297f0eb9ef2e85a1b09_HMbUOdhjsW.png?height=552&lazyload=true&maxWidth=600&width=2950)

    4. 测试企业关联应用后，在页面顶部切换企业应用为测试版应用。
       
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5d934d17429ce3722de3fafa4ae4356e_peZyNBbGSz.png?height=804&lazyload=true&maxWidth=600&width=3576)

4. 在 **应用能力** > **添加应用能力** 页面的 **按能力添加** 页签，找到 **机器人** 卡片，并点击 **添加**。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a390ebab8733900fb719fb713443e054_ZPOnpibcby.png?height=926&lazyload=true&maxWidth=600&width=2010)

5. 在 **权限管理** 页面，为应用开通以下 **API 权限**。

	- `im:chat:create`：创建群
	- `im:chat:read`：查看群信息
	- `im:chat:update`：更新群信息
	- `im:message:send_as_bot`：以应用的身份发消息
	- `im:resource`：获取与上传图片或文件资源
	- `im:message:readonly`：获取单聊、群组消息
	- `im:message.group_msg`：获取群组中所有消息（敏感权限）
	- `im:message.group_at_msg:readonly`：接收群聊中@机器人消息事件
	- `im:message.p2p_msg:readonly`：读取用户发给机器人的单聊消息


	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e0a325c0eb2564dfa2828e2d0440b8ef_rU3SXdIos2.png?height=712&lazyload=true&maxWidth=600&width=1968)
