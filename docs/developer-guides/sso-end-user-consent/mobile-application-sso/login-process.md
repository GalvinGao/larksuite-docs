---
document_id: '7264784369457790981'
directory_id: '7073083801460473862'
title: 登录流程
full_path: /uAjLw4CM/uYjL24iN/mobile-app/mobile-app-overview
breadcrumb:
- Developer Guides
- SSO&End User Consent
- Mobile Application SSO
- Login Process
document_type: GuideDocumentType
updated_at: 2025-06-11T03:14:47Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/mobile-app/mobile-app-overview
---

# 概述
Lark移动应用登录是基于 [OAuth 2.0](https://oauth.net/2/) 标准协议实现的，通过Lark账号授权登录第三方应用的能力。


## 准备工作
在使用Lark移动应用登录能力之前，首先你需要阅读[概述](/document/home/app-types-introduction/overview)，并依据文档中的指引在[Lark开放平台开发者后台](https://open.larksuite.com/app)创建一个应用，并获得应用的 AppID 和 AppSecret。

接下来，进入【添加应用能力】，在【移动应用登录】模块点击“添加能力”

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cebc0cc7578b10720356b674d4a7edd1_BEpHVOkMdO.png?height=726&lazyload=true&width=1582)


最后，需要在 iOS 应用或者 Android 应用中补全你的应用信息。登录配置不支持商店应用灰度发布。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c365d39092cdb37d2af2685130320798_2gIEJZYhrl.png?height=602&lazyload=true&width=1258)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/19f3b25eb4ddf96fc06b5a7463c51ad9_wLhWQdfWMk.png?height=700&lazyload=true&width=1262)

## 整体流程

![flow-mobile-cn.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/92a39aa7bdb01a761854597a7762b6e9_2ALy6eD0zm.png?height=2052&lazyload=true&width=1988)

### 第一步：获取 code
开发者需要引入Lark提供的 SDK ，并通过 SDK 的提供的 Api 来唤起Lark客户端并进行授权登录，用户点击确认授权之后，Lark客户端会唤起开发者客户端，并携带授权码（code）。

### 第二步：获取 access_token
[接口文档：获取 access_token](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/oidc-access_token/create) 

### 第三步：获取用户身份信息
[接口文档：获取用户身份信息](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/user_info/get) 

### 第四步：刷新已过期的 access_token
[接口文档：刷新已过期的 access_token](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/oidc-refresh_access_token/create)
