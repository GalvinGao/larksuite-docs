---
document_id: '7026663896463392773'
directory_id: '7002892512470728710'
title: 步骤二：创建小程序项目
full_path: /home/develop-a-gadget-in-5-minutes/create-a-gadget-project
breadcrumb:
- Home
- Quickly develop a gadget
- 'Step2: Create a gadget project'
document_type: GuideDocumentType
updated_at: 2024-02-08T06:42:18Z
source_url: https://open.larksuite.com/document/home/develop-a-gadget-in-5-minutes/create-a-gadget-project
---

# 步骤二：创建小程序项目

在本步骤，你将创建一个小程序项目并完成项目基本配置。

## 步骤一：登录Lark开发者工具

1. 打开Lark开发者工具。

2. 单击左下角 **登录** 按钮，选择登录环境为 **Lark**，单击 **登录**。

3. 在弹出的登录页面中，登录你的Lark账号。

	你可以选择使用Lark移动端扫码登录，或使用手机号、邮箱进行登录。


4. 登录成功后，单击左下角用户名，选择 **切换团队**。

:::note
你登录时的所在企业（团队） 必须与开发者后台创建自建应用的所在企业（团队） 一致。否则会导致应用关联不上，预览和调试失败。
:::

## 步骤二：创建小程序项目
:::html
<md-td>
1. 在Lark开发者工具左侧目录中选择 **小程序**，单击 **`+`** 创建项目。

	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a3922873393ed42d68711d2c3a38df9c_spKQ7ewgy5.png?lazyload=true&width=1744&height=1204" style="width:70%"/>

2. 在 **选择项目模板** 页面，选择**空白模板**，单击 **下一步** 。

	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/21a538aa874781ddb4700565ae80f0a3_FxYCc54n2I.png?lazyload=true&width=1744&height=1204" style="width:70%"/>

3. 在 **填写项目信息** 页面，填写项目名称和项目路径。

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/7d3aa144b120d389999759454cbd4243.png?lazyload=true&width=1744&height=1204" style="width:70%"/>

	创建成功后的项目如下图所示。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8b04d7306ae6be445b189c88d8e4d940_oyJqeOz50w.png?height=1122&lazyload=true&maxWidth=600&width=1904)

4. 打开`project.config.json`文件，修改 **appid** 的值为已创建的测试版应用的App ID。


	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/723a7c7ac6fb35d2e8492cb2b8d895f5_ULpUP39Tlu.png?height=1152&lazyload=true&maxWidth=600&width=1750)

5. 打开`app.json`文件，修改 **navigationBarTitleText** 的值为`MiniAppDemo`。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4c04adda0920cc57936534b7050bba02_jFwq3dhEyy.png?height=532&lazyload=true&maxWidth=600&width=1576)
</md-td>
:::
:::note
更多小程序项目配置说明，可参考[项目配置](/document/uYjL24iN/uEzMzUjLxMzM14SMzMTN/gadget-project-configuration)。
:::

