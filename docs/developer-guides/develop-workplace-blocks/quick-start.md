---
document_id: '7180269945547292677'
directory_id: '7179507279661744134'
title: 快速开始
full_path: /uAjLw4CM/uYjL24iN/block/quick-start
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Quick start
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:23Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/quick-start
---

# 快速开始

小组件应用的消费生命周期可以分成 **创建、开发、发布、使用** 四个阶段，如下图所示。接下来本文档将详细介绍这四个阶段。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/250684a3984d5a69935fee9de678af83_odKGTgkL5y.png?lazyload=true&width=1640&height=1001)

## 创建小组件
1. **创建应用。** 小组件属于开放平台的一种应用能力，因此在开始创建小组件之前，你需要先有一个Lark应用。如果你还没有创建过Lark应用，请参考[开发流程与工具介绍](/document/home/introduction-to-custom-app-development/self-built-application-development-process)快速创建一个应用。

2. **创建小组件。** 进入应用详情，在应用功能中选择小组件，点击右侧新建小组件，此时小组件已创建完成。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/971b290f02201ae9244700fa881e06f5_b3yN9j3ZvI.png?lazyload=true&width=2880&height=1120)

3. **获取 BlockTypeID。** 创建小组件完成后会自动进入小组件详情页面，点击右侧复制图标，复制小组件 BlockTypeID。


	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/eac1d6be25a6507c3199c599bea6eaf8_dJ2hlfsldS.png?lazyload=true&width=2220&height=686)
:::html
<md-alert type="tip">
 创建小组件的目的是为了获取 BlockTypeID，用于小组件开发。未上传开发包前，小组件信息无法保存，因此创建完小组件后请勿直接填写配置信息，避免重复操作。
</md-alert>
:::

## 开发小组件
1. **开发工具。** 小组件的开发需使用Lark开发者工具，若你还没有安装过，请参考[Lark开发者工具概述](/document/uYjL24iN/ucDOzYjL3gzM24yN4MjN) 进行安装 。

2. **登录Lark开发者工具。** 打开开发者工具，点击左下角的**登录**按钮，选择登录环境 **Lark** ，通过扫码或输入账号密码完成登录。



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/56a559a308435180fa803d9f61541ac1_XsSbunBvso.png?lazyload=true&width=1740&height=1200)

3. **创建小组件项目。** 登录后，依次点击左侧**扩展项目**、**小组件**。然后点击右侧的**加号菜单**，选择你倾向的模版，进入填写**项目信息页**。按需填入对应项目信息，点击**新建**，完成小组件项目的创建。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/086465f9612d2a86d95e455727844b6b_9ZBRfIgdVg.png?lazyload=true&width=1734&height=1180)

4. **开发、调试小组件。** 小组件项目创建完成后，会自动进入开发界面，可以对小组件进行开发、调试、编译、预览等，具体操作方式请参考 [Lark开发者工具主界面介绍](/document/uYjL24iN/uEzMzUjLxMzM14SMzMTN/feishu-developer-tools-ide)。

5. **上传小组件。** 完成小组件的开发后，点击右上角的上传，根据弹窗确认小组件项目的相关信息，确认无误后，点击上传，上传成功后，按照弹窗指引前往开发者后台进行设置。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f07809279ff2cc58a2a0061afb93cf10_mZYO7CrXig.png?lazyload=true&width=2340&height=1586)





## 发布小组件

1. **选择上传的小组件版本。** 完成小组件项目的上传后，前往开发者后台小组件详情页，选择本次需要发布的小组件版本

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fdb6cfe29bf04ce6999a5b4c664efd45_VGPsJbU7vh.png?lazyload=true&width=2468&height=418)

2. **配置小组件。** 依次填入小组件信息，完成配置后，点击保存。

3. **发布小组件。**  保存完小组件信息后，进入应用**版本管理与发布**，填写版本信息，提交企业管理员审核，管理员审核通过后，一个小组件即发布生效。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/42410e147b75e06e6d9aa614131bb18c_09FSIurDtU.png?lazyload=true&width=2872&height=310)


## 使用小组件 

 小组件在不同宿主中的使用入口和方式存在较大差异，详情请前往 [宿主场景](/document/uAjLw4CM/uYjL24iN/block/guide/hosting-scenario-introduction/workplace) 目录选择对应宿主文档进行查看。














