---
document_id: '6963854764274991109'
directory_id: '6907567269107777538'
title: 流程概述
full_path: /ukTMukTMukTM/uITNz4iM1MjLyUzM
breadcrumb:
- Server API
- API Call Guide
- Calling Process
- Process overview
document_type: GuideDocumentType
updated_at: 2024-02-18T03:23:32Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uITNz4iM1MjLyUzM
---

# 流程概述

Lark开放平台提供的 API 遵循 RESTful 风格，请求 URL 的格式为：

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/379b0797ca0cd9ed086efce6fd4d0b46_umwPiqSjkc.png?height=256&lazyload=true&maxWidth=750&width=2044)

调用服务端 API 的流程如下图所示。

![API概述zh.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5ca2db50685db2af90c99d46a7ccb87f_Z0QZjf9ahO.png?height=214&lazyload=true&maxWidth=800&width=2276)

- 创建应用。在[开发者后台](https://open.larksuite.com/app)，根据实际需求，创建自建应用或者商店应用。

:::note
仅拥有 ISV 资质的用户可以创建商店应用。有关 ISV 的详细介绍，请参考[如何入驻Lark开放平台](/document/uMzNwEjLzcDMx4yM3ATM/uUzNwEjL1cDMx4SN3ATM)。
:::
    
- [获取访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)。Lark开放平台设置了多种访问凭证（也称为 `access_token`），不同的访问凭证代表了不同的资源访问权限。调用 API 时，需要在 HTTP Header 中携带访问凭证，以便获取权限范围内的资源信息。   
 
- [申请 API 权限](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)。如果要调用 API，需要先获取接口调用权限；如果涉及到访问敏感字段，还需获取访问敏感字段的权限。

- [（可选）配置应用数据权限](/document/home/introduction-to-scope-and-authorization/configure-app-data-permissions)。当应用申请了部分 API 的权限（例如，通讯录、Lark人事企业版）后，还需要配置相应的数据权限并提交审核。待审核通过后权限生效，才可以成功调用 API 获取数据，否则调用 API 时会返回权限错误。

- [（可选）设置 IP 白名单](/document/ukTMukTMukTM/ucTMxYjL3ETM24yNxEjN)。为了提升应用的安全性，可以为应用设置 IP 白名单。仅当源 IP 在白名单内时，Lark开放平台才会响应，否则请求将被拒绝。

- [调用 API](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/get-)。完成上述步骤后，便可以调用 API 了。你可以参考 API 文档，了解 API 的具体功能。
