---
document_id: '7075603703986044934'
directory_id: '6920532209304993793'
title: 通讯录概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/resources
breadcrumb:
- Server API
- Contacts
- Contacts overview
document_type: GuideDocumentType
updated_at: 2022-03-27T14:05:06Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/resources
---

# 通讯录概述


## 业务介绍

在开放平台的设计中，通讯录可理解为企业的组织架构，包括企业部门信息，企业人员信息等，我们提供了一系列安全、可靠的 API，来方便你对通讯录信息进行操作。通过通讯录 API，你可以实现多种功能，例如：

- 查看企业的组织架构

- 为企业创建新的用户

- 修改企业中已有用户的基本属性

- 维护用户和部门的关联关系

- 维护用户和用户组的关联关系




###  接入流程

|  | 步骤 | 介绍 |
| --- | --- | --- |
| 1 | 创建一个应用 | - 如需创建企业自建应用，可参考 [自建应用的开发流程](/document/home/introduction-to-custom-app-development/self-built-application-development-process)<br>-   如需创建应用商店应用，可参考 [开发和上架应用商店应用](/document/uMzNwEjLzcDMx4yM3ATM/uYzNwEjL2cDMx4iN3ATM) |
| 2 | 调用API，对通讯录进行操作 | 调用API前，你需要先获取访问凭证并开启对应的权限，详情参见 [如何调用服务端API](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)<br>你还可以在Postman工具中快速调试这些API：[![Run in Postman](https://run.pstmn.io/button.svg?lazyload=true&width=128&height=32)](https://god.gw.postman.com/run-collection/17195890-6fd42609-12b3-409b-8cbb-73656cf9d805?action=collection%2Ffork&collection-url=entityId%3D17195890-6fd42609-12b3-409b-8cbb-73656cf9d805%26entityType%3Dcollection%26workspaceId%3D55edcb9c-bbfe-45f5-a74a-efb882fe5384#?env%5Bfeishu-demo%5D=W3sia2V5IjoiYmFzZVVybCIsInZhbHVlIjoib3Blbi5mZWlzaHUuY24iLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN0b3JlX2FwcF9pZCIsInZhbHVlIjoiY2xpX2EwYTUwODBjODRiODUwMTMiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN0b3JlX2FwcF9zZWNyZXQiLCJ2YWx1ZSI6InpFb0xLNHBjZE45VXBsNUpDOGNjcGZORlI3Q1FpbmNhIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJhcHBfdGlja2V0IiwidmFsdWUiOiI3YTU1NjFlODdiMjkzYjFjOTEyZWM1NTQ2MDVjNDFlOWZhMjZkYzJmIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJhcHBfYWNjZXNzX3Rva2VuIiwidmFsdWUiOiJhLWNlOTJjZTNhMmRjNmM2ZjQzYTVjNzM2YmRlMzAxM2FkYzdlZGM2MzQiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRlbmFudF9rZXkiLCJ2YWx1ZSI6IjczNjU4OGM5MjYwZjE3NWQiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRlbmFudF9hY2Nlc3NfdG9rZW4iLCJ2YWx1ZSI6InQtMmQ0OWY4ZjMyOTY2YTEzYmMzN2ZiMWJkZWFmZTBkNDdhMjAwZDZkZiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiU1RBVEUiLCJ2YWx1ZSI6IjExIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJSRURJUkVDVF9VUkkiLCJ2YWx1ZSI6Imh0dHBzJTNBJTJGJTJGd3d3LmJhaWR1LmNvbSUyRiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidXNlcl9hY2Nlc3NfdG9rZW4iLCJ2YWx1ZSI6InUtMDJvYmhid01DSEl5c2ZhNzFWVGNUZCIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2lkIiwidmFsdWUiOiJjbGlfYTA3ZmM0ZDVhMmY5NTAwYyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX3NlY3JldCIsInZhbHVlIjoiVlpBcFd0ZXc2UUdHQm1SbmxJNTF2aEZtbUU0bkJScmwiLCJlbmFibGVkIjp0cnVlfV0=)。使用方法参见[Postman模版使用说明](/document/tools-and-resources/postman-collection-data-manual) |
| 3 | 监听事件，获知通讯录的变化 | 监听事件前，你需要先申请相应的权限，详情参见 [事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM) |


:::warning
  **为了隐私安全考虑，通讯录相关API设置了额外的权限控制要求：**
- 应用通过API能够查询或修改的企业部门和人员范围 受到[企业管理员](https://www.larksuite.com/hc/zh-CN/articles/360049067822)配置的通讯录范围限制，你的应用仅能操作已获得授权的通讯录范围，详情请参见[通讯录权限范围说明](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)；

- 商店应用无法修改用户所在企业的通讯录信息。
:::



## 资源介绍

通讯录业务域以“资源”为中心进行开放，资源的关系图如下：





![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cef36972572508d94a05ddff52ac5a2f_HBL9GJfb7A.png?lazyload=true&width=1640&height=1284)
:::html

资源的定义如下：

| 资源 | 资源定义 |
| --- | --- |
| [用户](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/field-overview) | Lark某个企业里的一个用户。 |
| [部门](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview) | 对应Lark某个企业里的组织架构树上的一个节点。 |
| [用户组](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/overview) | 部门之外的另一种成员圈定方式，用户组可关联用户。分为[静态用户组](https://www.larksuite.com/hc/zh-CN/articles/360049067479)和[动态用户组](https://www.larksuite.com/hc/zh-CN/articles/360049067874) |
| [人员类型](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/overview) | 一种特殊的用户属性字段，用于标记用户的身份类型。默认值为：正式、实习、外包、劳务、顾问。企业可基于自身管理诉求自定义该字段的枚举值，每个用户只能引用一个枚举值。 |
| [自定义用户字段](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/custom_attr/overview) | 由企业自定义的用户字段，用于不同企业基于自身管理诉求灵活表征用户信息，自定义字段由[企业管理员](https://www.larksuite.com/hc/zh-CN/articles/360049067822)在[企业管理后台 - 组织架构 - 成员字段管理](http://www.larksuite.com/admin/contacts/employee-field-new/custom) 里进行创建、更新操作，可以通过接口可获取企业的所有自定义字段。<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/916ed86bcfe8cf6413a201bd6426f273_Cp6kDjtCKO.png?lazyload=true&width=1640&height=774)<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a780f8cf26717b120edc647bab48cdb9_hDqwt3VBsd.png?lazyload=true&width=1640&height=937) |
| [通讯录权限范围](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/overview) | 应用通过通讯录接口可以操作的组织架构数据范围，由[企业管理员](https://www.larksuite.com/hc/zh-CN/articles/360049067822)在 [企业管理后台 - 工作台 - 应用管理](http://www.larksuite.com/admin/appCenter/manage) 给不同应用设置。<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8ba4dbecb02390a47a3e418932fd1aa5_4oMArG3vdq.png?lazyload=true&width=1640&height=1074)<br>![6e868e1d-299f-43ce-9078-ba273827b10c (1).png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f63dd6f78e6b2488e02a8ce78680aafe_m5zuNEK1Ol.png?lazyload=true&width=1277&height=436) |



:::

以下将详细介绍每个资源的字段、方法、事件。

### 资源：用户 User
查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/field-overview)

#### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[创建用户](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/create)<br>`POST` /open-apis/contact/v3/users<br>> 向通讯录创建一个用户，可以理解为员工入职<br></md-text> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[获取单个用户信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/get)</md-text><br>`GET` /open-apis/contact/v3/users/:user_id<br>> 获取通讯录中单个用户的信息 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user" >user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取部门直属用户列表](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/find_by_department)</md-text><br>`GET` /open-apis/contact/v3/users/find_by_department<br>> 获取部门下直属用户列表 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门组织架构信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[修改用户部分信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/patch)</md-text><br>`PATCH` /open-apis/contact/v3/users/:user_id<br>>更新通讯录中用户的字段，未传递的参数不会更新 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新用户基本信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[更新用户所有信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/update)</md-text><br>`PUT` /open-apis/contact/v3/users/:user_id<br>>更新通讯录中用户的字段 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[删除用户](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/delete)</md-text><br>`DELETE` /open-apis/contact/v3/users/:user_id<br>>向通讯录删除一个用户信息，可以理解为员工离职 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |



#### 事件列表

| **[事件 (Event)](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 触发时机 | 权限要求（满足任一） | 事件类型 | 商店 | 自建 |
| --- | --- | --- | --- | --- | --- |
| <md-text type="field-name" >[员工信息被修改](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/events/updated)</md-text> | 员工信息被修改时 | contact.user.updated_v3 | **✓** | **✓** |  |
| <md-text type="field-name" >[员工入职](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/events/created)</md-text> | 员工入职时 | contact.user.created_v3 | **✓** | **✓** |  |
| <md-text type="field-name" >[员工离职](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/events/deleted)</md-text> | 员工离职时 | contact.user.deleted_v3 | **✓** | **✓** |  |


### 资源：部门 Department

查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview)

#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[创建部门](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/create)</md-text><br>`POST` /open-apis/contact/v3/departments<br>>向通讯录中创建部门 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[获取单个部门信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/get)</md-text><br>`GET` /open-apis/contact/v3/departments/:department_id<br>>向通讯录获取单个部门信息 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | <md-tag ype="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取子部门列表](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/children)</md-text><br>`GET` open-apis/contact/v3/departments/:department_id/children<br>>获取当前部门子部门列表 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门组织架构信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取父部门信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/parent)</md-text><br>`GET` /open-apis/contact/v3/departments/parent<br>>递归获取部门父部门的信息 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门组织架构信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[搜索部门](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/search)</md-text><br>`POST` /open-apis/contact/v3/departments/search<br>>通过关键词查询可见的部门数据 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | <md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[修改部门部分信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/patch)</md-text><br>`PATCH` /open-apis/contact/v3/departments/:department_id<br>>更新通讯录中部门的信息中的任一个字段 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[更新部门所有信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/update)</md-text><br>`PUT` /open-apis/contact/v3/departments/:department_id<br>>用于更新当前部门所有信息 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[删除部门](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/delete)</md-text><br>`DELETE` /open-apis/contact/v3/departments/:department_id<br>>向通讯录中删除部门 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |


#### 事件列表

| **[事件 (Event)](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 触发时机 | 权限要求（满足任一） | 事件类型 | 商店 | 自建 |
| --- | --- | --- | --- | --- | --- |
| <md-text type="field-name" >[部门信息被修改](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/events/updated)</md-text> | 部门信息被修改时 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | contact.department.updated_v3 | **✓** | **✓** |
| <md-text type="field-name" >[部门被创建](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/events/created)</md-text> | 部门被创建时 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | contact.department.created_v3 | **✓** | **✓** |
| <md-text type="field-name" >[部门被删除](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/events/deleted)</md-text> | 部门被删除时 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | contact.department.deleted_v3 | **✓** | **✓** |


### 资源：用户组 User Group

查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/overview)

#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[创建用户组](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/create)</md-text><br>`POST` /open-apis/contact/v3/group<br>>新增用户组 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">更新用户组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[更新用户组](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/patch)</md-text><br>`PATCH` /open-apis/contact/v3/group/:group_id<br>>更新某个用户组信息 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">更新用户组信息</md-perm> | <md-tag ype="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[删除用户组](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/delete)</md-text><br>`DELETE` /open-apis/contact/v3/group/:group_id<br>>删除某个用户组 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">更新用户组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[查询用户组](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/get)</md-text><br>`GET` /open-apis/contact/v3/group/:group_id<br>>获取某个用户组信息 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">获取用户组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[查询用户组列表](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/simplelist)</md-text><br>`GET` /open-apis/contact/v3/group/simplelist<br>>查询组织的用户组列表 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">获取用户组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[添加用户组成员](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/add)</md-text><br>`POST` /open-apis/contact/v3/group/:group_id/member/add<br>>向某个用户组中添加成员 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">更新用户组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[移除用户组成员](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/remove)</md-text><br>`PUT` /open-apis/contact/v3/group/:group_id/member/remove<br>>用于移除用户组中的成员 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">更新用户组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[查询用户组成员列表](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/simplelist)</md-text><br>`GET` /open-apis/contact/v3/group/:group_id/member/simplelist<br>>获取用户组中的成员列表 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">获取用户组信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |




### 资源：人员类型 Employee Type

查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/overview)

#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[查询人员类型](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/list)</md-text><br>`GET` /open-apis/contact/v3/employee_type_enums<br>>获取员工的人员类型 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新人员类型](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/update)</md-text><br>`PUT` /open-apis/contact/v3/employee_type_enums/:enum_id<br>>更新自定义人员类型 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[删除人员类型](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/delete)</md-text><br>`DELETE` /open-apis/contact/v3/employee_type_enums/:enum_id<br>>删除自定义人员枚举 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |
| <md-text type="field-name" >[新增人员类型](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/create)</md-text><br>`POST` /open-apis/contact/v3/employee_type_enums<br>>新增自定义人员类型 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **X** | **✓** |


#### 事件列表

| **[事件 (Event)](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 触发时机 | 权限要求（满足任一） | 事件类型 | 商店 | 自建 |
| --- | --- | --- | --- | --- | --- |
| <md-text type="field-name" >[新建人员类型事件](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/events/created)</md-text> | 人员类型被创建时 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | contact.employee_type_enum.created_v3 | **✓** | **✓** |
| <md-text type="field-name" >[启用人员类型事件](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/events/actived)</md-text> | 人员类型被启用时 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | contact.employee_type_enum.actived_v3 | **✓** | **✓** |
| <md-text type="field-name" >[停用人员类型事件](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/events/deactivated)</md-text> | 人员类型被停用时 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | contact.employee_type_enum.deactivated_v3 | **✓** | **✓** |
| <md-text type="field-name" >[修改人员类型名称事件](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/events/updated)</md-text> | 人员类型被修改时 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | contact.employee_type_enum.updated_v3 | **✓** | **✓** |
| <md-text type="field-name" >[删除人员类型事件](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/events/deleted)</md-text> | 人员类型被删除时 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | contact.employee_type_enum.deleted_v3 | **✓** | **✓** |



### 资源：自定义用户字段 Custom user fields

查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/custom_attr/overview)

#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)**（选择其一） | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[获取企业自定义用户字段](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/custom_attr/list)</md-text><br>`GET` /open-apis/contact/v3/custom_attrs<br>>获取企业自定义的用户字段配置信息 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> | **✓** | **✓** |


### 资源：通讯录权限范围 Contact Scope

查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/overview)

#### 事件列表

| **[事件 (Event)](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 触发时机 | 权限要求（满足任一） | 事件类型 | 商店 | 自建 |
| --- | --- | --- | --- | --- | --- |
| <md-text type="field-name" >[通讯录范围权限被变更](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/events/updated)</md-text> | 通讯录范围权限发生变更时 | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> | contact.scope.updated_v3 | **✓** | **✓** |

