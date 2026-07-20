---
document_id: '7026663890609225734'
directory_id: '7002892512470728710'
title: 步骤五：发布小程序
full_path: /home/develop-a-gadget-in-5-minutes/upload-project-and-release
breadcrumb:
- Home
- Quickly develop a gadget
- 'Step 5: Publish the Gadget'
document_type: GuideDocumentType
updated_at: 2024-02-08T06:42:22Z
source_url: https://open.larksuite.com/document/home/develop-a-gadget-in-5-minutes/upload-project-and-release
---

# 步骤五：发布小程序
 
当小程序完成所有测试后，你可以参考本文操作步骤正式将小程序发布上线。

## 步骤一：上传正式版小程序包
:::html
<md-td>

1. 登录[开发者后台](https://open.larksuite.com/app)，进入测试应用详情页。

2. 单击应用名称右侧的切换图标并选择 **切换至正式版本**。

	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/32504d5b09235a22ccc32f37b2dbe24e_5IfITYqUx2.png?lazyload=true&width=1334&height=644" style="width:70%"/>

3. 在正式版应用的**凭证与基础信息**页面，复制应用的**App ID**。

4. 打开Lark开发者工具，单击右上角头像，然后将开发者工具的团队切换为正式团队。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1c0156f2ed0549d3b6bb692b8d554195_RLLPLdKqRB.png?height=782&lazyload=true&maxWidth=600&width=2268)


5. 修改`project.config.json`文件中的**appid**为正式版应用的App ID，然后使用快捷键`Ctrl` + `S`保存。

	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/13a7a495fb2070296c5e1755c01c49c6_cqx8I5o1mb.png?lazyload=true&width=1726&height=748" style="width:70%"/>

6. 单击右上角 **上传**，然后在弹出的页面中选择 **平台** 和 **版本号**，最后单击 **上传** 按钮完成上传。

7. 返回开发者后台，然后选择小程序 **版本号** 和 **最低兼容Lark版本** ，最后单击 **保存**。


## 步骤二：发布小程序

1. 在应用详情页，单击 **版本管理与发布**，然后在版本管理与发布页面单击右上角 **创建版本** 按钮，进入创建版本详情页。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/27c68957f6f19ab7ae2ba761755defe6_TvLjLOMozq.png?height=1034&lazyload=true&maxWidth=600&width=2882)

2. 在创建版本页面下，填写以下信息，填写完成后单击底部 **保存** 按钮。

    * **应用版本号**： 输入当前应用的应用版本号，格式：`x.y.z`。
    * **移动端的默认能力**： 选择移动端默认打开的能力，本文选择小程序。
    * **更新说明**： 此次版本的更新说明。
    * **可用范围**：设置为需要访问和使用当前应用的用户范围。

	<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/f10f2113c530c9780d8937aac02f0ec1.png?lazyload=true&width=1366&height=1532" style="width:70%"/>

3. 保存成功后，单击右上角 **申请线上发布** 按钮。

	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5e4bcf5f74267c94b578061a2898c970_7GNT7eo6Xx.png?lazyload=true&width=2198&height=1010" style="width:70%"/>

4. 应用申请发布后，企业管理员可以在[企业管理后台](https://www.larksuite.cn/admin) > **工作台** > **应用审核** 页面进行审核。

	<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b4cee08c5f21761f5a076b505fcbf70a_lmNhKdlIwR.png?lazyload=true&width=2850&height=838" style="width:70%"/>

5. 审核通过后，打开移动端Lark，进入工作台即可看到已发布成功的小程序应用。

  	:::note
	新发布的版本需要管理员审核通过后 5 分钟左右才会正式生效。
	:::
 
	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d189f8487ca9419b6aaaeb849052bed3_bRucXxrXu7.png?height=1108&lazyload=true&maxWidth=600&width=1138) 

</md-td>
::::
  
  
  
  
