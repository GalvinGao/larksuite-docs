---
document_id: '7026663896463572997'
directory_id: '7280827644190162950'
title: 企业自建应用开发流程
full_path: /home/introduction-to-custom-app-development/self-built-application-development-process
breadcrumb:
- Developer Guides
- Develop Process
- Custom app development process
document_type: GuideDocumentType
updated_at: 2023-09-19T09:37:05Z
source_url: https://open.larksuite.com/document/home/introduction-to-custom-app-development/self-built-application-development-process
---

# 企业自建应用开发流程

企业自建应用是由企业内部人员或企业授权的开发人员进行开发，在企业内发布上线，并供内部人员使用的应用。Lark上的企业或组织可以基于Lark套件的开放能力，自主开发自建应用，以满足办公场景中的个性化需求。该类应用无需Lark团队审核，由企业租户管理员审核通过后即可使用。
  
更多介绍，可参见[应用类型概述](/document/home/app-types-introduction/overview)。

## 开发流程

企业自建应用的开发流程如下图所示，本文将按照该流程依次介绍相关操作。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cedba481c85fb858be48c8c8ad68fff7_g97Rx9fIpO.png?height=700&lazyload=true&width=2558)

## 步骤一：成为Lark开发者

在Lark开放平台上开发应用前，你必须先成为Lark的企业用户。你可以通过以下任一方式成为企业用户：

- **创建企业**
   
  如果你是企业管理员，可以进入[Lark官网](https://www.larksuite.com/)，点击右上角的 **免费试用**，注册并创建企业账号。创建完成后，你将默认成为企业管理员，并拥有邀请企业成员、管理企业组织架构等权限。详情参见[注册企业账号](https://www.larksuite.com/hc/zh-CN/articles/360047181633-%E6%B3%A8%E5%86%8C%E8%B4%A6%E5%8F%B7)。

- **加入企业**
  
  如果你是企业普通成员，并且所在的企业已使用Lark，可以联系企业管理员，获取任意方式的邀请（例如：企业链接、企业二维码、企业邀请码、手机短信），在Lark中加入所在企业。详情参见[加入企业](https://www.larksuite.com/hc/zh-CN/articles/360043257294-%E5%8A%A0%E5%85%A5%E4%BC%81%E4%B8%9A)。
  
:::note
如何邀请企业其他成员加入企业：
- 如果你是企业普通成员，但拥有邀请成员的权限，可参见[如何邀请企业内部成员加入Lark](https://www.larksuite.com/hc/zh-CN/articles/360048488020-%E4%BC%81%E4%B8%9A%E6%88%90%E5%91%98%E9%82%80%E8%AF%B7%E5%86%85%E9%83%A8%E5%85%B6%E4%BB%96%E6%88%90%E5%91%98%E5%8A%A0%E5%85%A5%E4%BC%81%E4%B8%9A)。
- 如果你是企业管理员，可参见 [使用企业二维码加入企业](https://www.larksuite.com/hc/zh-CN/articles/360041418693-%E4%BD%BF%E7%94%A8%E4%BC%81%E4%B8%9A%E4%BA%8C%E7%BB%B4%E7%A0%81%E5%8A%A0%E5%85%A5%E4%BC%81%E4%B8%9A)、[使用企业邀请码加入企业](https://www.larksuite.com/hc/zh-CN/articles/360040931394-%E4%BD%BF%E7%94%A8%E4%BC%81%E4%B8%9A%E9%82%80%E8%AF%B7%E7%A0%81%E5%8A%A0%E5%85%A5%E4%BC%81%E4%B8%9A)。
:::

## 步骤二：创建企业自建应用

1. 在 [开发者后台](https://open.larksuite.com/app)，点击 **创建企业自建应用**。
   
  	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8e5db6fefb0bc08af5b4f5ef391f6d7d_gGgrQpYryy.png?height=1108&lazyload=true&maxWidth=600&width=2196)

2. 填写应用基本信息，包括应用名称、应用描述、应用图标，并点击 **创建**。

	![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6e8d7fc5286acdb6d11b7cedb35b7f31_aIDJsyjZrd.png?height=1322&lazyload=true&maxWidth=450&width=1194)
    
## 步骤三：配置应用

本章节介绍在应用详情页内，支持的应用配置操作。

### **开启应用能力**

Lark应用支持四种基础的应用能力：**小程序、机器人、网页、小组件**。你可以根据需要选择对应的产品形态单独开启一种能力，也可以多种能力组合开启。不同能力的适用场景不同，详情请参见 [应用能力简介](/document/home/app-types-introduction/robots-web-applications-and-mini-programs)。

1. 在[开发者后台](https://open.larksuite.com/app)中，进入指定应用的详情页。

2. 在左侧导航栏，进入 **应用能力** > **添加应用能力** 页面。
   
   在该页面中，选择添加应用能力。

	![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ad86ac9b63cb652728df7d3267233c08_txxby6ZQ1I.png?height=1212&lazyload=true&maxWidth=600&width=1882)

:::note
开启应用能力后，需完成能力的必填项配置，并且需要新增应用版本，待新版本的应用审核发布后，已开启的能力才能在线上生效。
:::

### **开通应用权限**

为 OpenAPI 接口调用或 SDK 的接口调用配置所需要的权限。如下图所示，调用各 API 前需开通的权限列表可参考具体的 API 文档说明。关于如何开通所需权限，可参见[申请 API 权限](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)。
  
![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e83a597c27d65bd9c587afd3695606e9_J94ujRJgQV.png?height=748&lazyload=true&maxWidth=600&width=1688)
  
### 国际化配置（可选）

企业自建应用支持多语言配置，你可根据实际需要选择配置。

1. 在[开发者后台](https://open.larksuite.com/app)中，进入指定应用的详情页。

2. 在左侧导航栏，进入 **基础信息** > **凭证与基础信息** 页面。

3. 在 **国际化配置** 区域，为应用配置多语言的名称与描述信息。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/761b24df44f8e771af6f1d65219a0a32_apPxDA4V2X.png?height=1104&lazyload=true&maxWidth=600&width=1658)

在应用的基础信息中完成国际化配置后，你可以在已开启的应用能力（小程序、网页应用、机器人、小组件）中，进行国际化配置。各能力对应的国际化配置如下表所示。

:::note
应用的基础信息国际化配置，是应用能力国际化配置的前提条件。例如，基础信息国际化配置了 **简体中文** 与 **英文**，则相应的应用能力国际化配置只能配置 **简体中文** 与 **英文**。
:::

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width:20%">应用能力</md-th>
<md-th style="width:40%">配置方式</md-th>
<md-th style="width:40%">效果示例</md-th>
</md-tr>
</md-thead>
<md-tbody>

<md-tr>
<md-td>小程序</md-td>
<md-td>在应用详情页的 **应用能力** > **小程序** 页面中，添加任一 **小程序的扩展场景**。
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/24a44b4f08a5612bb18cf44b2cde3987_9bk9AbFyIH.png?height=658&lazyload=true&width=1292)

以小程序的 **聊天框“+”菜单** 扩展场景为例，国际化配置图示如下。
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/710ca786dfc6f6ccc0f4fb0c0040f9a8_Ouv6Ev4XMX.png?height=1146&lazyload=true&width=1738)
</md-td>
<md-td>例如设置的应用介绍为`Test Gadget`，则在Lark客户端对应的菜单中，应用信息如下图所示。
  
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9f894b17eaa8a7cd929f6423d4f2ee38_fLK3C2yqnn.png?height=1488&lazyload=true&width=1824)

如果应用未配置某种语言，则在切换Lark客户端语言后：

- 应用默认显示英文。
- 由于无法配置小程序的多语言，所以也不会显示对应语言的应用介绍。
</md-td>
</md-tr>

<md-tr>
<md-td>网页应用</md-td>
<md-td>在应用详情页的 **应用能力** > **网页应用** 页面中，添加任一 **网页应用的扩展场景**。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/852a910a76bb22c8f0698916c4f61b2c_cLqVqz0d9e.png?height=644&lazyload=true&width=1196)

以网页应用的 **聊天框“+”菜单** 扩展场景为例，国际化配置图示如下。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/188deb8e843341ee39fc6a64c5065114_FOHxaECHqv.png?height=1146&lazyload=true&width=1712)
</md-td>
<md-td>例如设置的应用介绍为`Test Web app`，则在Lark客户端搜索结果如下图所示。
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/913df22e7a053d0b5e724c4294d23776_cFRMEdVqGi.png?height=1488&lazyload=true&width=1820)
  
如果应用未配置某种语言，则在切换Lark客户端语言后：
  
- 应用默认显示英文。

- 由于无法配置网页应用的多语言，所以也不会显示对应语言的应用介绍。</md-td>
</md-tr>

<md-tr>
<md-td>机器人</md-td>
<md-td>
- 配置一：在应用详情页的 **应用能力** > **机器人** 页面中编辑 **机器人配置**，在 **如何开始使用** 区域配置国际化。
  
	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/177a2b23ce53bceabe8a9914ca68e359_iWRPaXCVk2.png?lazyload=true&width=1718&height=924)

- 配置二：在应用详情页的 **应用能力** > **机器人** 页面中开启 **机器人自定义菜单**，在 **主菜单配置** 区域中，**名称** 支持国际化配置。
  
	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/15e1dfcb84dddf64e5ebf3af656d7b2d_jQ4eX5QKyl.png?height=1178&lazyload=true&width=1562)</md-td>
<md-td>例如：

- 在 **机器人配置** 中，为机器人添加描述`This is a test bot.`。

- 在 **机器人自定义菜单** 中，配置主菜单名称为`Test Bot`。则示例配置如下图所示。
  
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/68dd3b1455ced6bd03269275dcc883b9_BcHPVz6ypq.png?height=1536&lazyload=true&width=1848)

如果应用未配置某种语言，则在切换Lark客户端语言后，将默认展示英文。
</md-td>
</md-tr>

<md-tr>
<md-td>小组件</md-td>
<md-td>小组件分为 **工作台小组件**、**云文档小组件**、**多维表格记录视图**，各组件的国际化配置入口相同。
  
以 **工作台小组件** 为例，在应用详情页的 **应用能力** > **工作台小组件** 页面中，编辑 **基础信息**。在 **基础信息** 中，**小组件名称**、**小组件介绍** 支持国际化配置。
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cfe4a102869347a7995a5996af57560a_nnp6sH3wru.png?height=918&lazyload=true&width=1716)
  
<md-alert type="tip">
本文介绍的小组件国际化配置仅适用于非标准小组件。如果你的小组件适配类型为标准小组件，则需要通过代码进行国际化配置，详情参见[配置参考](/document/uAjLw4CM/uYjL24iN/block/block-frame/config)。
</md-alert>  
</md-td>
<md-td>
例如，小组件基础信息中配置的小组件名称为`App Demo`，则在Lark客户端的工作台内小组件信息如下图所示。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/632b39fc2379f8ee0d27c141bbf08a39_gqqyJPA39Z.png?height=356&lazyload=true&width=902)
  
如果应用未配置某种语言，则在切换Lark客户端语言后，将默认展示英文。
</md-td>
</md-tr>

</md-tbody>
</md-table>
:::
  
  
### 成员管理（可选）

1. 在[开发者后台](https://open.larksuite.com/app)进入指定应用详情页。

2. 在左侧导航栏选择 **基础信息** > **成员管理**。

3. 点击 **添加协作人员**，在弹出框中输入需要添加的用户信息（名字或邮箱地址），选择角色类型后，点击 **确认添加**。

:::note
不同角色拥有不同的操作权限，可查看页面提供的权限列表信息。
:::
  
![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/28b29865f90fd62765fb6a00495f5f17_90yLeeMC0c.png?height=1336&lazyload=true&maxWidth=600&width=2822)
  
### 事件订阅（可选）

事件订阅功能可以满足应用及时响应Lark事件变更的需求。在使用事件订阅功能时，需要先配置请求地址，并添加订阅事件。后续当事件发生时，Lark开放平台会以 HTTP POST 请求的方式将事件内容以 JSON 格式推送给请求地址。关于事件订阅流程，可参见[事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)。

- 请求地址配置图示。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/acc69a24151aa64551233c90a237b594_lc8CYUyubh.png?height=882&lazyload=true&maxWidth=600&width=1652)

- 添加事件图示。

	![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dfe623e3f4b5239d3aff6cb2b5aa1206_RUs4c7NDde.png?height=514&lazyload=true&maxWidth=600&width=838)
  
## 步骤四：开发应用 

你可以参考以下各应用能力的快速入门文档，了解应用开发流程及示例，从而完成应用功能的开发。

* [小程序开发流程](/document/home/develop-a-gadget-in-5-minutes/create-a-custom-app)

* [网页接入流程](/document/home/integrating-web-apps-in-5-minutes/create-app-and-configuration)

* [机器人开发流程](/document/home/develop-a-bot-in-5-minutes/create-an-app)

* [小组件开发流程](/document/uAjLw4CM/uYjL24iN/block/quick-start)
   
## 步骤五：测试应用

为了满足开发测试阶段频繁变更配置的需求，Lark开放平台提供了 **测试企业和人员** 功能。该功能可将应用分为正式版本和测试版本，使用测试版本时，应用相关的权限与配置变更均会直接生效，无需管理员审核，同时你也可以使用测试企业的人员账号进行测试。关于测试应用的更多介绍与操作说明，可参见[测试企业与人员](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)。

## 步骤六：发布应用

在开发自建应用的过程中，当应用的 **基本信息**、**权限范围**、**应用功能** 和 **事件订阅** 等信息发生变更时，都需要发布新的应用版本，并且通过企业租户管理员的审核后才能生效。相关说明参见[自建应用审核指南](https://www.larksuite.com/hc/zh-CN/articles/360048488346-%E8%87%AA%E5%BB%BA%E5%BA%94%E7%94%A8%E5%8F%91%E7%89%88%E5%AE%A1%E6%A0%B8%E6%8C%87%E5%8D%97)。

1. 在[开发者后台](https://open.larksuite.com/app)进入需要发布的应用详情页。

2. 在左侧导航栏，选择 **应用发布** > **版本管理与发布**，并点击 **创建版本**。

	![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0a289b7bb5285af544d14f88d6c5beb2_rD2viQriPT.png?height=1016&lazyload=true&maxWidth=600&width=2810)

3. 在 **版本详情** 页面完善并核对应用版本信息，点击 **保存**，并点击 **申请线上发布**。
	
    等待企业租户管理员通过审核后，应用即可发布上线。

:::note
在开发及测试阶段，你可以在版本发布时控制有限的应用可用范围，避免开发中的版本被普通用户访问和使用。当完成正常的开发流程，应用进入到正式版本的发布阶段后，可以将应用的可用性范围调整为正式的配置。后续待租户管理员完成上架审核后，应用就可以被普通用户使用了。关于应用可用范围的详细说明，参见[配置应用可用范围](/document/home/introduction-to-scope-and-authorization/availability)。
:::
  
  
## 步骤七：应用运营及运维

应用发布上线后，你可以通过[开发者后台](https://open.larksuite.com/app)提供的以下功能，进行应用的线上运营和维护，精准提升应用品质与体验。
  
* **日志检索**
	
    日志检索工具可以帮助你查找到应用调用 **服务端 API、客户端 API**，以及 **事件推送** 的详细日志信息。使用此工具将大幅降低与用户的沟通成本，提升问题排查效率。若出现无法解决的问题，一键点击即可将日志信息转发给Lark客服，以获得帮助。更多介绍，参见[日志检索](/document/tools-and-resources/open-api-log-query)。



* **用户反馈**
	
    用户反馈功能是提升产品质量的重要途径。通过该功能，用户能够便捷反馈应用使用过程中遇到的问题，你在获取应用的线上反馈后能够快速定位问题，进而推动产品的迭代优化。更多介绍，参见[用户反馈](/document/tools-and-resources/userfeedback)。
     
    * 作为Lark应用的普通用户，每当遇到产品功能 bug、体验不佳或对操作方法产生疑问时，只需点击 **反馈** 按钮，即可向应用开发者传递自己的声音。
    
    * 作为Lark应用的开发者，只需进入[开发者后台](https://open.larksuite.com/app)应用详情页的 **用户反馈** 面板，即可根据时间和类型查看用户反馈，轻松定位问题，及时解决 bug，为用户提供更优质的服务体验。
  

 
## 相关文档

* [应用鉴权](/document/home/introduction-to-scope-and-authorization/overview)

* [应用审核规则](https://www.larksuite.com/hc/zh-CN/articles/360046527434-%E5%BA%94%E7%94%A8%E5%AE%A1%E6%A0%B8%E7%9A%84%E8%B4%9F%E8%B4%A3%E4%BA%BA%E6%98%AF%E8%B0%81)
