---
document_id: '7275897728244350982'
directory_id: '7273792780344934406'
title: 简介
full_path: /home/quick-access-to-contact-api/introduction
breadcrumb:
- Home
- Quickly Integrate to Contacts
- Introduction
document_type: GuideDocumentType
updated_at: 2023-09-07T02:00:10Z
source_url: https://open.larksuite.com/document/home/quick-access-to-contact-api/introduction
---

# 简介

本教程介绍如何使用Lark开放平台通讯录开放能力，完成对部门的增删改查操作，并监听员工的入职、离职事件。通过本教程你可以快速了解如何使用 OpenAPI 操作通讯录，以及处理员工入职、离职事件。

## 流程简介

本文涉及的操作流程如下图所示：

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/59ee310275cb59f1ba2988be60e97100_JbUaXpTz1n.png?height=208&lazyload=true&width=733)

## 实现效果

- 通过运行示例代码实现对部门的增删改查操作。

  :::html
  <md-video src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fbde84dcb693208cfc57fac841a159d4_Yb932ksVwI.mp4" />
  :::
<br>
- 通过事件订阅的方式监听员工入职、离职事件。

    :::html
    <md-video src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3fac79e17553af7ff7e8aa40e446f04d_2l4KJvmSL1.mp4" />
    :::


## 使用到的API列表

### 通讯录

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** |
| --- | --- | --- |
| [创建部门](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/create)<br>`POST` /open-apis/contact/v3/departments<br>> 用于向通讯录中创建部门 | <md-perm name="contact:contact" desc="更新通讯录" support_app_types="custom" tags="">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| [获取子部门列表](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/children)<br>`GET` /open-apis/contact/v3/departments/:department_id/children<br>> 通过部门ID获取部门的子部门列表 | <md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| [获取单个部门信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/get)<br>`GET` /open-apis/contact/v3/departments/:department_id<br>> 该接口用于向通讯录获取单个部门信息 | <md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| [修改部门部分信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/patch)<br>`PATCH` /open-apis/contact/v3/departments/:department_id<br>> 该接口用于更新通讯录中部门的信息 | <md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |


### 事件

| **[事件](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 权限要求 |
| --- | --- |
| [员工入职](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/events/created)<br>`contact.user.created_v3`<br>> 通过该事件订阅员工入职 | <md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm> |
| [员工离职](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/events/deleted)<br>`contact.user.deleted_v3`<br>> 通过该事件订阅员工离职 | <md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm> |

