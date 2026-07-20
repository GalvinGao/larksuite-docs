---
document_id: '7026663896463245317'
directory_id: '7280791907864559621'
title: 测试企业与人员
full_path: /home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions
breadcrumb:
- Developer Guides
- Develop Process
- Test and Release Apps
- Test companies and users
document_type: GuideDocumentType
updated_at: 2024-11-21T06:27:48Z
source_url: https://open.larksuite.com/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions
---

# 测试企业与人员

  
在开发测试过程中，由于配置变更频繁，开发者会不断重复提交新版本，管理员也不得不频繁审核，这严重影响了工作效率。为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了「**测试企业和人员**」的功能。

## 测试企业功能概述

  
使用测试企业功能，开发者创建的应用将分为「**正式版**」和「**测试版**」。在开发阶段，开发者可以使用测试版应用，此版本中涉及的权限和配置变更都会直接生效，无需管理员审核，用户端的测试也将在测试租户进行。在所有的开发测试完成之后，切换、手动同步到正式版应用，仅提交一次审核即可，大大加速了开发效率，也降低了对管理员的打扰。

使用测试企业与人员功能的开发流程示意如下：

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/aa0fe674b241f419099d9023350a2f8f_JxyuBGM6EY.png?height=1590&lazyload=true&maxWidth=700&width=1640)


## 如何使用测试企业

### 准备工作

在开始使用测试企业之前，你需要先确认当前登录用户与应用的关系。

- 如果当前登录用户是应用的所有者、管理员、开发、运营，可以跳过准备工作，直接进行下一步。应用角色可以登录[开发者后台](https://open.larksuite.com/app)，在应用列表的 **我的角色** 列查看。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f683fe81335312e563f393f1287a730f_iNil0JSP3p.png?height=610&lazyload=true&maxWidth=600&width=2654)
    
- 如果当前登录用户还无法在[开发者后台](https://open.larksuite.com/app)查看到需要测试的应用，则需要先联系应用所有者，让其在应用的 **成员管理** 功能页内，添加协作人员，将当前登录用户添加为某一角色，然后再进行下一步操作。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/29738cb4e2109a6a4fcf90a6422795d2_33jmgHjBLL.png?height=1496&lazyload=true&maxWidth=600&width=2882)

### 1. 创建测试企业

在 **开发者后台** 进入 **应用详情** 页后， 在左侧导航栏选择 **测试企业和人员**，进入此页面即可创建测试企业。

:::warning
- 每个开发者可以创建最多 3 个企业，每个测试企业下最多可以添加 100 名测试人员。
- 此处添加的测试人员只有权限在Lark客户端内使用测试版本应用，无权开发测试版本应用。如果你希望添加某一成员开发测试版本应用，则需要先按照上文 **准备工作** 章节的指引，将该用户添加为应用的协作者，并由该用户自己创建测试企业进行测试。
:::


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/98e32e26ea1e1aabb63a7f8a6c2f8813_Zn4XTg3RNh.png?height=1616&lazyload=true&maxWidth=650&width=2780)

### 2. 将自建应用关联到测试企业

创建测试企业后，你可以将处于开发测试阶段的**应用关联到测试企业。**
- 对于自建应用，在 **测试企业和人员** 页面，点击测试企业列表 **操作** 栏的 **关联应用** 即可。
- 对于 **商店应用** 如何关联测试企业，参考[测试商店应用](/document/uMzNwEjLzcDMx4yM3ATM/uUjMyUjL1IjM14SNyITN)。



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a9700811fc659164cac298dd81e4b9a5_0dOiZPHgJ8.png?height=1188&lazyload=true&maxWidth=650&width=2796)

完成关联之后，在这个测试企业下，会生成一个「**测试版**」应用。

:::warning
- 在应用内，为测试企业点击 **关联应用** 的用户才是对应生成的测试版本应用的所有者。如果你在关联应用时，测试企业已处于关联状态，则需要先移除应用，然后重新关联应用。
- 测试版应用的权限、事件和应用信息等发生变更，将**自动生效，无需再重新创建版本和走审核流程**。
- 对于**自建应用**来说，**测试版和正式版在逻辑上是两个独立的应用**，切换版本时，开发者需及时修改`app_id`和`app_secret`等配置。
:::

### 3. 切换到测试企业并测试应用

点击应用头像旁的切换图标，在展开的选项中点击「**测试版**」即可进入测试企业。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9e9cb563adb68a20107584f1dcfe5627_nMoLdeNXaY.png?height=1194&lazyload=true&maxWidth=650&width=2154)

在开发者后台，你可在应用标签中看到「**正式版**」和「**测试版**」两个标签。**在测试版应用下进行的权限管理等配置修改都将自动生效。**

:::warning
在「**测试版**」上修改的配置并不会自动同步到「**正式版**」中。请在应用配置稳定下来后，手动同步配置，并一次性提交审核。
:::


![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/12747b8c6d415fc67e79b0f0a85e0450_UHzOvw3JO4.png?height=332&lazyload=true&maxWidth=450&width=1016)



## 常见问题

### 为什么有的应用无法创建测试企业？

只有**正式租户下的应用才可以与测试企业相关联**。若当前应用名称下方展示「**测试版本**」标签则代表当前应用是测试应用，此时无法再次创建测试应用或测试企业。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fea1474adbb8fdd02899c9f0795c3e6c_WDYtxLLSL3.png?height=408&lazyload=true&maxWidth=600&width=1410)


### 怎么能解散测试企业或者移除测试人员？

测试企业的创建者登录测试企业的[管理后台](https://www.larksuite.com/admin)进行操作：

- **解散测试企业**

	进入 **企业设置** > **企业信息** 模块，点击 **解散企业**。

    ::: note
    解散企业的操作会在 72 小时后生效。
    :::

	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dfd5b7a88a1323dd2b48c358719a54ce_w0pnBPorgW.png?height=1042&lazyload=true&maxWidth=600&width=1980)
    
- **移除测试人员**

	进入 **组织架构** > **成员与部门** > **成员** 模块，在指定成员列表右侧点击 **···** > **操作离职**，选择成员资源转移方式后点击 **确认离职**。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/37c4d4b282150d216b35bbcb0c68af9f_HeosC7BLjU.png?height=1052&lazyload=true&maxWidth=600&width=2882)

### 创建的测试企业对应的Lark版本是什么？

测试企业的版本不会跟随当前租户的Lark版本变化而变化，固定为未认证的基础版。测试企业仅用于免审测试应用，无问题后还需要切换为正式版应用进行开发、发布。
