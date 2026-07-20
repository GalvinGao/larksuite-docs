---
document_id: '7275897728244006918'
directory_id: '7273792780344918022'
title: 步骤一：创建测试应用
full_path: /home/automatic-attendance-management-based-on-approval/step-1-create-a-test-application
breadcrumb:
- Home
- Auto Attendance Management
- 'Step 1: Create a test app'
document_type: GuideDocumentType
updated_at: 2024-07-15T06:20:44Z
source_url: https://open.larksuite.com/document/home/automatic-attendance-management-based-on-approval/step-1-create-a-test-application
---

# 步骤一：创建测试应用

在本步骤，你将创建一个测试应用，并为应用开启机器人能力，用于通过机器人发起请假和加班申请。

## 一、创建测试应用

1. 登录[Lark开发者后台](https://open.larksuite.com/app)。

2. 在开发者后台首页，单击 **创建企业自建应用**，然后填写应用名称和应用描述，单击 **创建**。

	![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3cd27a9711ff938a20139b879fecf4ca_0bGLlnnGXc.png?height=1182&lazyload=true&maxWidth=600&width=1928)

3. 在左侧导航栏单击进入 **测试企业和人员** 页面，单击 **创建测试企业**，填写 **测试企业名称**、**手机号**、**验证码**，单击 **确认创建**。

:::note
为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了[测试企业与人员](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)功能。在开发阶段，推荐开发者使用测试版应用，此版本中涉及的**权限和配置变更都会直接生效，无需管理员审核**，客户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。
:::

![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/dc74dca16da2445b0996f0d00e8584af.png?height=600&lazyload=true&maxWidth=400&width=1192)

4. 在创建完成的测试企业操作栏，单击 **关联应用**，即可为当前应用自动生成测试版本。

    ![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4202be5e53167a7c40376eb6487b6e08.png?height=660&lazyload=true&maxWidth=600&width=2252)

5. 左侧导航栏，单击应用名称右侧的切换图标并选择 **切换至测试版本**。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bb414dd26bdf1415b00280110df2425f_RQB7f2f3VU.png?height=914&lazyload=true&maxWidth=600&width=2170)
    
    
## 二、开通应用权限
1. 在 **添加应用能力** 页面，添加 **机器人** 功能。

    ![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6da24de6717f58e43c470eb98ce16747_ZtjrCN59em.png?height=682&lazyload=true&maxWidth=600&width=1702)
  
2. 在 **权限管理** 页面，开通以下 API 权限。

   * **approval:approval**：查看、创建、更新、删除审批应用相关信息
   * **approval:approval:readonly**：访问审批应用
   * **attendance:task**：写入打卡数据
   * **calendar:calendar**：更新日历及日程信息
   * **calendar:calendar:readonly**：获取日历、日程及忙闲信息
   * **calendar:timeoff**：创建或删除请假日程
   * **im:message:send_as_bot**：以应用的身份发消息
   * **im:message.p2p_msg:readonly**：读取用户发给机器人的单聊消息
    
	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/263a30f6320010abfe0a3d360f06c12b_G5CysE02RI.png?height=698&lazyload=true&maxWidth=600&width=1952)


