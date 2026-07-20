---
document_id: '7233232281317687302'
directory_id: '7199928167142260741'
title: 步骤一：创建并配置应用
full_path: /home/todos-daily-reminder-of-weekly-report/list-of-apis
breadcrumb:
- Home
- Todos daily reminder of weekly report in Wiki
- 'Step 1: Create and configure the application'
document_type: GuideDocumentType
updated_at: 2024-07-15T06:21:13Z
source_url: https://open.larksuite.com/document/home/todos-daily-reminder-of-weekly-report/list-of-apis
---

# 步骤一：创建并配置应用

在本步骤，你将创建一个测试版应用，并为应用开通相关的 API 权限及文档阅读权限，为后续调用服务端接口及获取云文档信息做准备。
:::html
<md-td>

## 步骤一：创建测试应用
1. 登录[Lark开发者后台](https://open.larksuite.com/app)。

2. 在开发者后台首页，单击 **创建企业自建应用**，填写应用名称和应用描述，单击 **创建**。

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/7d4b05f2e242981f57e3ff9c04166f3c.png?lazyload=true&width=2552&height=1736" style="width:70%"/>

3. 在左侧导航栏单击进入 **测试企业和人员** 页面，单击 **创建测试企业**，填写 **测试企业名称**、**手机号**、**验证码**，单击 **确认创建**。

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dc74dca16da2445b0996f0d00e8584af_P0P8gaPatc.png?lazyload=true&width=1192&height=600" style="width:40%"/>

4. 在创建完成的测试企业操作栏，单击 **关联应用**，即可为当前应用自动生成测试版本。

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4202be5e53167a7c40376eb6487b6e08.png?lazyload=true&width=2252&height=660" style="width:70%"/>

5. 左侧导航栏，单击应用名称右侧的切换图标并选择 **切换至测试版本**。

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/8623c8ddd59ef32854159c689c464b5a.png?lazyload=true&width=1398&height=672" style="width:70%"/>

## 步骤二：配置应用权限

1. 单击进入**添加应用能力**页面，添加**机器人**功能。

	需要使用机器人能力发送消息到群组。

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/131c64e20014db88431cc33362940279.png?lazyload=true&width=2432&height=1580" style="width:70%"/>

2. 在左侧导航栏选择 **权限管理**，在 **API 权限** 页面开通以下权限。

	* **docs:doc:readonly**：查询、评论和导出文档
	* **wiki:wiki:readonly**：查看知识库
	* **im:chat:read**：查看群信息
	* **im:message:send_as_bot**：以应用身份发消息

	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/90ec16bb8e3a36de712838c66ef082c3_Q88JVkIODy.png" style="width:70%"/>

## 步骤三：为应用开通知识库文档权限

在本步骤你将在周报管理这个知识库中，为已创建的应用开通**可阅读**权限。

1. 登录 [Lark云文档](https://www.larksuite.com/drive/home/) ，进入周报知识库。

2. 进入周报文档的根节点，依次选择`···` > **更多** > **添加文档应用**。

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/d83315a6720ac3e1e2da7fcdd6eeccd4.png?lazyload=true&width=2196&height=1426" style="width:70%"/>

3. 在弹出的对话框中搜索应用名称，然后赋予其 **可阅读** 权限。

    :::note
    访问应用非自有资源时，应用必须拥有该资源的权限，并且需要开启机器人能力。
    :::

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/7569d6aebafbd8cde78c8685953804a3.gif?lazyload=true&width=1808&height=1148" style="width:70%"/>

</md-td>
:::
