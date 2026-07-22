---
document_id: '7263040419905437702'
directory_id: '6907567269107777538'
title: 配置应用数据权限
full_path: /home/introduction-to-scope-and-authorization/configure-app-data-permissions
breadcrumb:
- Server API
- API Call Guide
- Calling Process
- Configure app data permissions
document_type: GuideDocumentType
updated_at: 2023-08-03T12:15:39Z
source_url: https://open.larksuite.com/document/home/introduction-to-scope-and-authorization/configure-app-data-permissions
---

# 配置应用数据权限

应用的数据权限是指以应用身份访问业务资源时可获取的数据范围。当应用申请了部分 API 的权限（例如，通讯录）后，还需要配置相应的数据权限并提交审核。待审核通过后权限生效，才可以成功调用 API 获取数据，否则调用 API 时会返回权限错误。

## 数据权限介绍

企业自建应用和商店应用均支持配置数据权限，你可以通过本章节了解各类应用的数据权限详情以及配置方式。

### 自建应用的数据权限

自建应用支持配置 **通讯录** 的数据权限。

| 数据权限类型 | 权限描述 | 管理方式 |
| --- | --- | --- |
| 通讯录权限范围 | 以应用身份调用通讯录 API 时，应用可以获取到的通讯录数据范围。例如，以应用身份调用通讯录 API 查询用户 A 的信息时，需要应用具备用户 A 的数据权限。<br>默认情况下，通讯录权限范围与应用的[可用范围](/document/home/introduction-to-scope-and-authorization/availability)一致。 | - 方式一：由应用所有者、管理员或者开发角色在应用内配置通讯录权限范围。<br>- 方式二：由企业管理员在管理后台配置指定应用的通讯录权限范围。 |


:::note
部门、用户等组织架构的数据对企业而言均为敏感数据。应用一旦授权了相应的数据权限，即可增删改查这些敏感数据，所以当应用开发者配置数据权限后，需要发布应用并等待应用审核人员通过审核，方可生效数据权限。
:::

### 商店应用的数据权限

商店应用仅支持企业管理员配置 **通讯录** 的数据权限。应用开发者可以调用[获取通讯录授权范围](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/list)查看应用的数据权限。

| 数据权限类型 | 权限描述 | 管理方式 |
| --- | --- | --- |
| 通讯录权限范围 | 以应用身份调用通讯录 API 时，应用可以获取到的通讯录数据范围。例如，以应用身份调用通讯录 API 查询用户 A 的信息时，需要应用具备用户 A 的数据权限。 | 由企业管理员在管理后台配置指定应用的通讯录权限范围。 |


## 企业自建应用

如果你开发的应用涉及以应用身份调用通讯录 API，则你需要配置相应的数据权限。

:::note
- 数据权限需要搭配 API 权限才可以生效，即在配置数据权限之前，你需要确保已为应用申请了相应的 API 权限，具体操作参见[申请 API 权限](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)。

- 关于通讯录权限范围的更多概念介绍，参见[权限范围详解](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)。


- 如果你使用应用的[测试企业与人员](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)能力，将应用切换为测试版本进行开发联调，则应用默认具有全员范围的通讯录权限。
:::


### 操作步骤

1. 登录[开发者后台](https://open.larksuite.com/app)。找到并进入指定应用详情页。

2. 在左侧导航栏，选择 **开发配置** > **权限管理**。


1. 在 **通讯录权限范围** 区域，点击 **配置**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/171da13b9a6bf9bd378a0ec6a1d2ea44_2kxeUdXhzG.png?height=564&lazyload=true&maxWidth=600&width=1070)

2. 配置权限范围，并点击 **确认**。
    
    默认情况下，通讯录的数据权限范围为 **与应用的可用范围一致**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c5734db999a1c5d6fbcc61a700ea5749_rWqNbnEtVA.png?height=684&lazyload=true&maxWidth=600&width=1866)

    - **与应用的可用范围一致**：通讯录权限范围与应用的可用范围一致。关于应用可用范围的详细介绍，参见[配置应用可用范围](/document/home/introduction-to-scope-and-authorization/availability)。
    
    - **部分成员**：选择该范围后，你还需要手动选择部门或成员，以确认通讯录权限范围。
    
    - **全部成员**：企业内所有成员的通讯录权限。

:::note
企业管理员可以在[管理后台](https://www.larksuite.com/admin) 的工作台中找到指定应用，修改应用的通讯录权限范围。
:::

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fb40ff142abb6e027b312434af12183b_9ZzHyRQcM6.png?height=698&lazyload=true&maxWidth=600&width=2324)




## 商店应用

商店应用仅支持企业管理员在[管理后台](https://www.larksuite.com/admin)的工作台中找到指定应用，修改应用的通讯录权限范围。

:::note
应用开发者可以调用[获取通讯录授权范围](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/list)查看应用的数据权限。
:::

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/093d937d12197ace5d05c9d65de881f6_Ft065pZ5SZ.png?height=738&lazyload=true&maxWidth=600&width=1968)
