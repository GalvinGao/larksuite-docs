---
document_id: '7074952334765735942'
directory_id: '7073442394955579397'
title: 简介
full_path: /home/quickly-create-a-login-free-web-app/introduction
breadcrumb:
- Home
- No-login for H5
- Introduction
document_type: GuideDocumentType
updated_at: 2024-03-18T08:46:30Z
source_url: https://open.larksuite.com/document/home/quickly-create-a-login-free-web-app/introduction
---

# 简介

本文介绍如何实现网页应用在Lark客户端内的免登操作，在用户打开网页应用时，弹出授权登录页面，授权通过后直接完成登录操作。通过本教程你可以了解到网页应用授权登录的完整流程。

## 免登流程

网页应用免登为网页应用提供了获取当前登录用户的Lark身份的能力，网页应用免登流程如下：

* 步骤一：[获取用户登录预授权码](/document/common-capabilities/sso/api/obtain-oauth-code)。
* 步骤二：[使用预授权码获取user_access_token](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/oidc-access_token/create)。
* 步骤三：[获取用户信息并完成登录](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/user_info/get)。
* 步骤四：[刷新已过期的user_access_token](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/oidc-refresh_access_token/create)。

![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/b28b526728d7352249e55a89ee2b940d.png?height=964&lazyload=true&width=962)

## 实现效果

按照本教程操作最终可以实现如下图的示意效果。

![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/bb01c432766002315d88eee0d64fc40f.png?height=622&lazyload=true&width=1280)

