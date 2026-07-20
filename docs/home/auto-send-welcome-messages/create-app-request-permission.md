---
document_id: '7074952334765785094'
directory_id: '7073442394955612165'
title: 创建应用&申请权限
full_path: /home/event-based-messaging/create-app-request-permission
breadcrumb:
- Home
- Auto Send Welcome Messages
- Create app & request permission
document_type: GuideDocumentType
updated_at: 2023-05-16T03:11:45Z
source_url: https://open.larksuite.com/document/home/event-based-messaging/create-app-request-permission
---

# 创建应用 & 申请权限
**登录开放平台，创建应用**
进入[Lark开放平台 > 开发者后台](https://open.larksuite.com/app/)，登录Lark账号后，创建一个名为 **迎新机器人** 的应用。

在企业自建应用列表中，可以看到刚刚创建好的应用。点击刚刚创建的机器人应用，可以进入应用详情界面

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1fe5e3189f35afe79e8b18582ef88b2d_3Ocw90G5Fh.png?lazyload=true&width=2230&height=1520)

(图 1：应用详情页)


应用详情界面的**凭证与基础信息**一栏里，可以查询到应用凭证，也就是 **App ID** 和 **App Secret**

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/718d5e31d2ba17eebc84f42afe8c6ac1_HUCx7N1tNB.png?lazyload=true&width=2784&height=1168)

(图 2：查看应用 ID 信息)

接着在**添加应用能力**栏，添加**机器人**能力

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9980ec117246161f94d36784a9136027_IEipaCFscJ.png?lazyload=true&width=2686&height=1376)

(图 3：启用机器人能力)

然后选择**事件订阅**一栏，在这里可以查看到应用的 **Verification Token**

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d43ca2850d7280175e9eeb3a651ed856_VGYShkZFGB.png?lazyload=true&width=2320&height=1288)

(图 4：验证 Token)

到现在为止，你已经开启了应用的机器人能力，并获得了应用的 App ID、App Secret 和 VerificationToken 了, 此时就可以开始进行后端服务代码的编写了。
<br>
<br>


**需要申请的权限：**

消息权限
- 获取与发送单聊、群组消息

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/03c0152c0d3dfd41414b2599be784056_fHjbEYsEw5.png?lazyload=true&width=2746&height=1610)
由于应用申请权限需要租户管理员审批，为了提升应用开发调试效率，可以尝试使用[测试企业与人员功能](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)。 
