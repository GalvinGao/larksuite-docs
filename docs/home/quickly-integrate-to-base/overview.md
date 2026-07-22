---
document_id: '7275897728244301830'
directory_id: '7273792780344885254'
title: 简介
full_path: /home/quick-access-to-base/preparation
breadcrumb:
- Home
- Quickly Integrate to Base
- Overview
document_type: GuideDocumentType
updated_at: 2023-09-19T09:37:43Z
source_url: https://open.larksuite.com/document/home/quick-access-to-base/preparation
---

# 简介

本文介绍如何使用 OpenAPI 管理Lark云文档的多维表格。通过本教程你可以了解包括创建、写入、查看以及导出等多维表格 OpenAPI 的调用方式。

## 什么是多维表格？

多维表格是Lark云文档下的一个产品。一篇多维表格可以由多个数据表组成，便于你统计业务数据、提升工作效率。多维表格可以理解成一个应用（App），它可以独立存在，也可以作为一个模块集成在Lark文档或Lark表格中。更多信息参见[多维表格概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/bitable-overview)。

## 操作流程

本文涉及的操作流程如下图所示：


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/816fb452132c64e9ec509b8a8999e5a6_ux1AFbfFeS.png?height=208&lazyload=true&width=492)


## 实现效果

通过代码示例调用多维表格 OpenAPI，创建多维表格，并对表格进行创建、写入、删除以及导出等操作。

:::html
<md-video src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/95a4384f08c25102664b890b1488672a_gEGtxBJKU3.mov" poster="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e631899631811de74809e018f3ba2e4f_8oPFCWza5F.png?lazyload=true&width=3570&height=1888?lazyload=true&width=1356&height=660" width="80%"/>
:::

## 使用到的 OpenAPI 列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** |
| --- | --- | --- |
| <md-text type="field-name" >[列出记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/list)<br>`GET` /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records<br></md-text> | <md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" tags="">查看、评论和导出多维表格</md-perm><br><md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" tags="">查看、评论、编辑和管理多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user" >user_access_token</md-tag> |
| <md-text type="field-name" >[创建多维表格](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/create)</md-text><br>`POST` /open-apis/bitable/v1/apps | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" tags="">查看、评论、编辑和管理多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[新增多条记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_create)</md-text><br>`POST` /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_create | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" tags="">查看、评论、编辑和管理多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[新增数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/create)</md-text><br>`POST` /open-apis/bitable/v1/apps/:app_token/tables | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" tags="">查看、评论、编辑和管理多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user" >user_access_token</md-tag> |
| <md-text type="field-name" >[删除多条记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_delete)</md-text><br>`POST` /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_delete | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" tags="">查看、评论、编辑和管理多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |

