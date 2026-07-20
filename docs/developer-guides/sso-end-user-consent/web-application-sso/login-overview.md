---
document_id: '7347620348471705606'
directory_id: '7073083801460457478'
title: 登录流程概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/authen-v1/login-overview
breadcrumb:
- Developer Guides
- SSO&End User Consent
- Web Application SSO
- Login Overview
document_type: GuideDocumentType
updated_at: 2025-06-11T03:14:51Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/login-overview
---

# 登录流程概述
网页应用登录是基于 [OAuth 2.0](https://oauth.net/2/) 标准协议实现的，通过Lark账号授权登录第三方应用的能力。


## 准备工作
在使用网页应用登录能力之前，首先你需要阅读 [概述](/document/home/app-types-introduction/overview)，并依据文档中的指引在 [开放平台开发者后台](https://open.larksuite.com/app) 创建一个应用，并获得应用的 AppID 和 AppSecret。

接下来，需要选中【安全设置】一栏，并将接收授权码的页面地址添加到右侧的【重定向URL】。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/70816649b673ce88728f4afee04749e0_6jzVde6Cct.png?height=755&lazyload=true&width=1451)

## 整体流程

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/98aa43798f7168f54c91924f90e7e567_olUcc25AH5.png?height=964&lazyload=true&width=962)

### 第一步：获取 code
[接口文档：获取登录授权码code](/document/common-capabilities/sso/api/obtain-oauth-code)

### 第二步：获取 user_access_token
[接口文档：获取 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/authentication-management/access-token/get-user-access-token)

### 第三步：获取用户身份信息
[接口文档：获取用户身份信息](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/user_info/get)

### 第四步：刷新 user_access_token
[接口文档：刷新 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/authentication-management/access-token/refresh-user-access-token)
