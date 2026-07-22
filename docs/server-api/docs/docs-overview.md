---
document_id: '7073817463670554630'
directory_id: '6907567269107531778'
title: 云文档概述
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-overview
breadcrumb:
- Server API
- Docs
- Docs Overview
document_type: GuideDocumentType
updated_at: 2022-03-11T12:20:31Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-overview
---

# 云文档概述


## 业务介绍

云文档是Lark在线文档、电子表格、多维表格、知识库、云盘等产品的统称。你可以调用云文档的相关接口，完成以下操作：
- 上传、下载文件

- 创建、编辑各类在线文档

- 管理你所有的文件和文件夹、知识库



###  接入流程

|  | 步骤 | 介绍 |
| --- | --- | --- |
| 1 | 创建一个应用 | - 如需创建企业自建应用，可参考 [自建应用的开发流程](/document/home/introduction-to-custom-app-development/self-built-application-development-process)<br>-   如需创建应用商店应用，可参考 [开发和上架应用商店应用](/document/uMzNwEjLzcDMx4yM3ATM/uYzNwEjL2cDMx4iN3ATM) |
| 2 | 调用API，对云文档进行操作 | 调用API前，你需要先获取访问凭证并开启对应的权限，详情参见 [如何调用服务端API](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)<br>你还可以在Postman工具中快速调试这些API：[![Run in Postman](https://run.pstmn.io/button.svg?lazyload=true&width=128&height=32)](https://god.gw.postman.com/run-collection/17195890-6fd42609-12b3-409b-8cbb-73656cf9d805?action=collection%2Ffork&collection-url=entityId%3D17195890-6fd42609-12b3-409b-8cbb-73656cf9d805%26entityType%3Dcollection%26workspaceId%3D55edcb9c-bbfe-45f5-a74a-efb882fe5384#?env%5Bfeishu-demo%5D=W3sia2V5IjoiYmFzZVVybCIsInZhbHVlIjoib3Blbi5mZWlzaHUuY24iLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN0b3JlX2FwcF9pZCIsInZhbHVlIjoiY2xpX2EwYTUwODBjODRiODUwMTMiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN0b3JlX2FwcF9zZWNyZXQiLCJ2YWx1ZSI6InpFb0xLNHBjZE45VXBsNUpDOGNjcGZORlI3Q1FpbmNhIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJhcHBfdGlja2V0IiwidmFsdWUiOiI3YTU1NjFlODdiMjkzYjFjOTEyZWM1NTQ2MDVjNDFlOWZhMjZkYzJmIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJhcHBfYWNjZXNzX3Rva2VuIiwidmFsdWUiOiJhLWNlOTJjZTNhMmRjNmM2ZjQzYTVjNzM2YmRlMzAxM2FkYzdlZGM2MzQiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRlbmFudF9rZXkiLCJ2YWx1ZSI6IjczNjU4OGM5MjYwZjE3NWQiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRlbmFudF9hY2Nlc3NfdG9rZW4iLCJ2YWx1ZSI6InQtMmQ0OWY4ZjMyOTY2YTEzYmMzN2ZiMWJkZWFmZTBkNDdhMjAwZDZkZiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiU1RBVEUiLCJ2YWx1ZSI6IjExIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJSRURJUkVDVF9VUkkiLCJ2YWx1ZSI6Imh0dHBzJTNBJTJGJTJGd3d3LmJhaWR1LmNvbSUyRiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidXNlcl9hY2Nlc3NfdG9rZW4iLCJ2YWx1ZSI6InUtMDJvYmhid01DSEl5c2ZhNzFWVGNUZCIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2lkIiwidmFsdWUiOiJjbGlfYTA3ZmM0ZDVhMmY5NTAwYyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX3NlY3JldCIsInZhbHVlIjoiVlpBcFd0ZXc2UUdHQm1SbmxJNTF2aEZtbUU0bkJScmwiLCJlbmFibGVkIjp0cnVlfV0=)。使用方法参见[Postman模版使用说明](/document/tools-and-resources/postman-collection-data-manual) |
| 3 | 监听事件，获知云文档的变化 | 监听事件前，你需要先申请相应的权限，详情参见 [事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM) |





## 资源介绍

云文档业务域以“资源”为中心进行开放，资源的关系图如下：

![Frame 12.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4abef565f7dad55059945afb2a895f23_LwEvLXgSp2.png?lazyload=true&width=1640&height=757)


:::html

资源的定义如下：

| 资源 | 资源定义 |
| --- | --- |
| [文件夹](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction) | 用于管理文件和其它文件夹的容器。 |
| [文件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction) | 各种类型的文件的统称，泛指云空间内所有的文件。 |
| [文档](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-doc-overview) | Lark在线文档。 |
| [电子表格](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/guide/sheets-faq) | Lark电子表格。 |
| [评论](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment/list) | Lark在线文档中的评论。 |



:::
