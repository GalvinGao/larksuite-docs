---
document_id: '7072724425749168133'
directory_id: '7186908899420831749'
title: 原生审批接入指南
full_path: /ukTMukTMukTM/uIjN4UjLyYDO14iM2gTN
breadcrumb:
- Server API
- Approval
- Access guide
- Native approval access guide
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:22Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uIjN4UjLyYDO14iM2gTN
---

# 原生审批接入指南
## 导语
	跟随本教程，可以完成一个简单审批 API 流程的体验，帮助你更快速的完成审批 API 的接入工作。
	使用工具：Postman
## Guideline
### 流程
通过开放平台进行审批 API 接入的流程如下：

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8bee22d27fe816069a05a440d06a41e1_gSVlwjkGUb.png?lazyload=true&width=759&height=511)

### 准备工作
首先，要在[【Lark开放平台】](https://open.larksuite.com/)创建一个应用。
1.进入Lark开放平台—开发者后台点击“创建自建应用”。

![7c4b58ef72c68298270ac5850904d7f5_U0zs2GLJrD.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7c4b58ef72c68298270ac5850904d7f5_F9eAInmd7o.png?lazyload=true&width=1280&height=337)

2.填写应用信息，应用名称为 Contact Test

![a2e57acf59d6fcb627a99c4bf7b22d40_YGqMtBc30n.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a2e57acf59d6fcb627a99c4bf7b22d40_KaTX7YvAzP.png?lazyload=true&width=1640&height=650)

3.创建成功后，可以看到 Contact Test 应用被添加到企业自建应用目录中，点击进入应用详情页可以看到新创建的应用。

![9b5cf58551a3e7a1589c80ac1fbb2746_tJs6tA7zCw.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9b5cf58551a3e7a1589c80ac1fbb2746_9v9qS6v2I9.png?lazyload=true&width=1280&height=199)


具体应用如何使用及权限开通问题请参阅开放平台文档[【如何开发企业自建应用】](/document/home/introduction-to-custom-app-development/self-built-application-development-process)
       
### 正式开始
1. 创建审批定义

创建前请先阅读[【Lark审批应用使用说明】](https://www.larksuite.com/hc/zh-CN/articles/360034502513) ，及[【审批应用开发指南】](/document/ukTMukTMukTM/ukDNyUjL5QjM14SO0ITN)以对审批有一个初步的了解，然后联系企业管理员在[【审批后台】](https://www.larksuite.com/approval/admin/approvalList?devMode=on)创建所需要的审批定义。

在这里我们以一个新的包含数字和图片两个控件的新表单为例，审批人选择先由指定成员审批再由发起人自选。





![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6477b8cd8a6592b232c3500b684440f2_eo5AEknSUQ.png?lazyload=true&width=1640&height=1287)


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/864750d2507a80269a392f330a5f8ea4_EKW0DVjbMw.png?lazyload=true&width=1640&height=1292)


2. 获得 Approval Code

在[【审批后台】](https://www.larksuite.com/approval/admin/approvalList?devMode=on)找到我们的审批定义，点击编辑按钮，即可在地址栏找到这个审批对应的 Approval Code。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/27541c7951c3d554f22a11bba9fb8598_vwjx0WJZMB.png?lazyload=true&width=1640&height=169)



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b44136cd880892bea7920cdf2a6b04ad_GZr3TBSFlD.png?lazyload=true&width=1640&height=1160)

此时在地址栏的最后输入参数 **&devMode=on** 即可进入开发者模式，在开发者模式下可以指定表单控件和流程节点的自定义 ID，方便之后的开发工作，具体的编辑页面如下图。

如果设置了自定义 ID，则可以通过自定义 ID 来指定一个唯一的控件或者一个唯一的节点。在一个审批定义中自定义 ID 不可重复。



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f13b66e18a1e1c1966de9f95e860b113_OiYFbvWdDo.png?lazyload=true&width=1640&height=1322)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a9cb0f1968b9472d870d46df7621b50e_9pc1kx9dhh.png?lazyload=true&width=1640&height=1445)

3. 获得开放平台Token

在开放平台找到刚才发布通过了的应用，我们可以找到这个应用对应的 App ID 和 App Secret。

根据开放平台[【获取 token】](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal)这个文档取得你的 APP 对应的 token，token 有效期为 2 小时，过期后需要重新获取。接下来我们使用的全部都是 tenant_access_token。

在Postman 中先设置请求头，在审批 API 中请求头都是一样的。箭头位置填写刚才获得的 tenant_access_token。



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/18e142f75ba23d5e473f189dfb267646_hnOvTmkBZS.png?lazyload=true&width=1640&height=1356)

4. 查看审批定义

通过[【查看审批定义】](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get)接口，我们可以通过 API 查询到我们刚才创建的审批定义。

使用刚才的 Approval Code 来试一下：在 Postman 的地址栏中填入请求地址，请求方法为 POST，并在 Body 中选取 raw，以 json 格式填写所需参数，approval_code 处填写刚才第二步获得的 Approval Code 即可。



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e1f10efca0f14bf12bfb738e11c7e5b0_PlV1iQFiGi.png?lazyload=true&width=1640&height=1356)
5. 上传文件

如果我们想要通过 API 使用刚才创建的定义来创建一个新的审批实例，则需要先进行文件上传，因为我们在审批定义中含有一个图片控件。

下面我们来通过[【上传文件】](/document/ukTMukTMukTM/uUDOyUjL1gjM14SN4ITN)API 先进行文件的上传：在 Postman 的地址栏中填入请求地址，请求方法为 POST，并在 Body 中选取 form-data，填写相关字段，并在 content 这一栏中选择 File，在 value 处选择一个本地文件。



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fc1db84818951aa8aa2756d17ccf4283_XVOJoI8H7g.png?lazyload=true&width=1640&height=1356)

上传成功后我们会收到如下回应，请记住其中的 code，在之后的实例创建过程中会用到这个 code。



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3059b8c0baffc1da04d4e915a8e386e9_vgJiJe6qcY.png?lazyload=true&width=1640&height=1356)

6. 创建审批实例（获取审批实例的instance_code）

接下来我们正式开始创建实例，请先阅读[【创建审批实例】](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/create)文档来熟悉一下请求 body。

这里展示的是在刚才创建的审批定义下，创建应审批实例的请求 body：在 Postman 的地址栏中填入请求地址，请求方法为 POST，并在 Body 中选取 raw，以 json 格式填写所需参数，approval_code 处填写刚才第二步获得的 Approval Code，user_id 和 department_id 填写申请发起人的信息，node_approver_user_id_list 和form 字段按照文档描述填写即可，这个例子中涉及到【发起人自选】概念，具体描述可以查看[【审批应用开发指南】](/document/ukTMukTMukTM/ukDNyUjL5QjM14SO0ITN)。

这里也可以用到我们在第 2 步设置的自定义表单 ID 和节点 ID 来进行创建，比如说我们指定了数字控件的自定义 ID 为『number』，发起人自选节点的ID为『node1』，则可以将下面的请求中『form』字段里的数字控件默认 ID 替换为『number』，将『node_approver_user_id_list』字段里的『855XXX』这个发起人自选节点的默认 ID 替换为『node1』。

需要注意的是，审批定义在修改并发布后不能保证原有控件的默认 ID 不变，但是自定义 ID 会保证不随着定义的修改而变动，只要这个控件没有被删除或者删除后重新添加并保证自定义 ID 一致，那可以一直使用自定义 ID 访问到这个控件。所以我们更推荐使用自定义 ID 来进行开发。




![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/441873bc5e384a9d89b0d9e5e52ba85f_zK88szBMOq.png?lazyload=true&width=1640&height=1356)
创建成功以后，我们会收到一个如下所示返回值：



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1d20fed777e792413a3e9f8d47652d64_nRQ1HkVsjI.png?lazyload=true&width=1640&height=1356)

其中，instance_code 则是我们创建的这个实例对应的 code。

现在我们可以在[【审批前台】](https://applink.larksuite.com/client/mini_program/open?appId=cli_9c7cc8a9a9edd105&path=pages%2Fapproval-list%2Findex%3FselectIndex%3D3)的『我发起的』列表页看到刚才我们通过 API 发起的审批实例（如果你的 user_id 和 department_id 填的是自己的话）。

7. 获取审批实例详情(获取审批任务的id)

现在我们可以通过实例的 ID 来查看审批实例的详情了，我们来通过 API 查看一下刚才创建的实例：在 Postman 的地址栏中填入请求地址，请求方法为 POST，并在 Body 中选取 raw，以 json 格式填写所需参数， instance_code 处填写刚才第 6 步中返回的 code 即可。




![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b1c2f8b662ba7b25a1af5eb3a01c9be3_luAnLtyQvV.png?lazyload=true&width=1640&height=1357)
其中返回的数据里task_list里的id即为审批任务的id，后续的审批任务相关操作可以使用这个id

8. 订阅事件和取消

订阅事件的含义可以查看[【审批事件监听开发指南】](/document/ukTMukTMukTM/ugDNyUjL4QjM14CO0ITN)，订阅成功后即可在应用所填请求网址收到审批消息。

订阅是以审批定义的维度进行订阅，订阅后该审批定义下所有事件都会推送。

这里展示的是你的应用订阅刚才创建的审批定义的请求：在 Postman 的地址栏中填入请求地址，请求方法为 POST，并在 Body 中选取 raw，以 json 格式填写所需参数，approval_code 处填写刚才第二步获得的 Approval Code。




![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/63466d21462e0a53d1964a2d0838fadb_PpEdVyJ3at.png?lazyload=true&width=1640&height=1356)

订阅成功后可以随时取消订阅。

这里展示的是你的应用取消刚才的订阅请求：在 Postman 的地址栏中填入请求地址，请求方法为 POST，并在 Body 中选取 raw，以 json 格式填写所需参数，approval_code 处填写刚才第二步获得的 Approval Code。



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/63466d21462e0a53d1964a2d0838fadb_YAPrcdEG42.png?lazyload=true&width=1640&height=1356)

## 结语

好了，本次的 guideline 到此结束，这里只展示了审批 API 的基础应用，我们还提供了一些其他 API，例如[【审批操作 API】](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/approve)和[【批量获取审批实例 ID】](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/list)等。如果有更多的需求欢迎查阅[【开放平台审批 API 文档】](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get)。





