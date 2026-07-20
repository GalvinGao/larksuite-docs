---
document_id: '7273741028051107846'
directory_id: '7270783571969998854'
title: 简介
full_path: /home/quickly-develop-a-web-app-nodejs/introduction
breadcrumb:
- Developer Guides
- Quick Starts
- Develop H5 (Node.js)
- Introduction
document_type: GuideDocumentType
updated_at: 2025-04-07T02:41:23Z
source_url: https://open.larksuite.com/document/home/quickly-develop-a-web-app-nodejs/introduction
---

# 简介

本教程将基于Lark开放平台的能力快速实现一个网页应用，包括应用的免登、鉴权和 JSAPI 的使用。通过教程您可以了解企业自建网页应用从开发到发布的完整流程、理解免登和鉴权的基本原理。

## 什么是网页应用？

Lark客户端网页（Web）应用指的是用 H5 方式开发，可以运行在Lark客户端内的应用。网页应用可以调用丰富的Lark客户端开放接口（客户端 API，也称为 JSAPI），包含手机系统功能和通讯录、云文档等Lark客户端功能，也可以享受到客户端侧的性能优化，使你的网页应用能够接近原生体验。更多信息，请参见[网页应用简介](/document/uYjL24iN/uMTMuMTMuMTM/introduction)。

## 操作流程

本教程涉及的操作流程如下图所示：

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4864490662078a3c0484b4bae8e48ac3_bfCE6tihfB.png?height=410&lazyload=true&maxWidth=600&width=1294)

## 实现效果

通过示例代码构建的网页应用，包含查询设备信息、展示示例选项卡、展示示例图片的能力。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9d348fdae8f6de9874aa83fb05c2345a_QESLvn9Ijv.PNG?height=2532&lazyload=true&maxWidth=300&width=1170)

## 使用到的 API 列表

### JSAPI

- 免登实现使用到的 JSAPI
	- [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) 获取网页应用免登授权码（ 登录预授权码，LarkV6.9.0版本前使用 [requestAuthCode](/document/uYjL24iN/uUzMuUzMuUzM/20220308) ）。
    
- 鉴权实现使用到的 JSAPI

    - window.h5sdk.config：对使用到的JSAPI集合进行鉴权。
    
- JSAPI 使用示范（鉴权通过后可使用）用到的 JSAPI

    - [getSystemInfo](/document/uYjL24iN/uQjNx4CN2EjL0YTM)：获取系统信息。
    
    - [showActionSheet](/document/uYjL24iN/ukDNy4SO0IjL5QjM)：显示操作菜单。
    
    - [previewImage](/document/uYjL24iN/uMDOx4yM4EjLzgTM)：预览一组图片。
    
    - [showToast](/document/uYjL24iN/ugzMy4COzIjL4MjM)：显示灰色背景的消息提示框。

### OpenAPI

- 免登实现使用到的OpenAPI

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

<md-text type="field-name" >[获取 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/access_token/create) 

   `POST` /open-apis/authen/v1/access_token
  
  </md-text>

</md-td>
  
<md-td>

无

</md-td>
  
<md-td>

<md-tag mode="inline" type="token-app">app_access_token</md-tag>

</md-td>
  
</md-tr>
  
</md-tbody>
  
</md-table>
:::

- 鉴权实现使用到的 OpenAPI

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

<md-text type="field-name" >[自建应用获取 tenant_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal) 

   `POST` /open-apis/auth/v3/tenant_access_token/internal
  
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
  
<md-text type="field-name" >[JSAPI 临时授权凭证](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/h5_js_sdk/authorization) 

   `POST` open-apis/jssdk/ticket/get
  
  </md-text>

</md-td>
  
<md-td>
	无

</md-td>
  
<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>
  
</md-tr>

<md-tr>
  
</md-tbody>
  
</md-table>
:::
