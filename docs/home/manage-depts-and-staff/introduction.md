---
document_id: '7275897728244252678'
directory_id: '7273792780344901638'
title: 简介
full_path: /home/quick-access-to-base/department-personnel-management-based-on-web-app/overview
breadcrumb:
- Home
- Manage Depts and Staff
- Introduction
document_type: GuideDocumentType
updated_at: 2023-09-07T07:07:58Z
source_url: https://open.larksuite.com/document/home/quick-access-to-base/department-personnel-management-based-on-web-app/overview
---

# 简介

本文介绍如何快速开发一个网页应用，该应用基于Lark开放平台的 OpenAPI 和应用[免登](/document/uYjL24iN/uMTMuMTMuMTM/development-guide/step-3)功能，实现一个人员部门管理的 [网页应用](/document/uYjL24iN/uMTMuMTMuMTM/introduction)。通过该教程，您可以了解开发网页应用的流程，以及如何实现应用的免登流程。

## 操作流程

本文涉及的操作流程如下图所示：

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4476362f4f9f6413200590b7ae376b14_mFvtzvEprw.png?height=208&lazyload=true&width=492)

## 实现效果

通过运行代码示例，访问部门人员管理应用。

:::html
<md-video src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/55b4f5dc4db8513984e8470547c3abbd_pS2bPp8pb7.mp4" poster="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3ec9cdeada8d8a22fbdac461160bb289_rTi5OaYq1R.png?lazyload=true&width=3438&height=1536" width="80%"/>
:::

## 使用到的 OpenAPI 列表

:::html
<md-table>

<md-thead>

<tr>

<md-th style="width: 50%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 30%;">权限要求（满足任一）</md-th>

<md-th style="width: 20%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）**</md-td></md-th>


</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>
  
<md-text type="field-name" >[自建应用获取 app_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token_internal) 

   `POST` open-apis/auth/v3/app_access_token/internal
  
  </md-text>

</md-td>
  
<md-td>
	无

</md-td>
  
<md-td>

	无

</md-td>
  
</md-tr>
  
<md-tr>

<md-td>

<md-text type="field-name" >[获取 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/access_token/create)</md-text>
   
`POST` open-apis/authen/v1/access_token

</md-td>

<md-td>
	
  	无

</md-td>

<md-td>

<md-tag type="token-app">app_access_token</md-tag>

</md-td>

</md-tr>
  
<md-tr>

<md-td>

<md-text type="field-name" >[获取登录用户信息](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/user_info/get) </md-text>
  
`GET` open-apis/authen/v1/user_info

</md-td>

<md-td>

<md-perm name="contact:user.employee:readonly" desc="获取用户受雇信息" support_app_types="custom,isv" tags="">获取用户受雇信息</md-perm>
<md-perm name="contact:user.email:readonly" desc="获取用户邮箱信息" support_app_types="custom" tags="">获取用户邮箱信息</md-perm>
<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
<md-perm name="contact:user.phone:readonly" desc="获取用户手机号" support_app_types="custom" tags="">获取用户手机号</md-perm>

</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

</md-tr>
