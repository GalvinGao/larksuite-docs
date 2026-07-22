---
document_id: '7233612551992131590'
directory_id: '7199928167142178821'
title: 简介
full_path: /home/replace-links-in-documents-after-data-migration/introduction
breadcrumb:
- Home
- Replace links in documents after data migration
- Introduction
document_type: GuideDocumentType
updated_at: 2023-05-17T03:03:24Z
source_url: https://open.larksuite.com/document/home/replace-links-in-documents-after-data-migration/introduction
---

# 简介

本教程介绍如何使用开放平台云文档能力，在数据迁移场景中将本地文件导入到Lark云文档，并提取Lark云文档中过期的链接，将其修正为导入后的新链接地址。
## 流程简介
本教程将按照以下流程调用Lark开放接口，实现上传文件数据并替换文档中过期的链接。
:::html
<img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/a5e8df1dd27f8276f390e50fbba87329.png?lazyload=true&width=918&height=896" style="width:50%" />
:::

## 实现效果
按照本教程操作最终可以实现如下图的示意效果。
![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/a41c38254880ae2fdc1b250dfad00f51.png?lazyload=true&width=1280&height=821)
## 使用到的API列表
### 鉴权

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** |
| --- | --- | --- |
| <md-text type="field-name" >[获取 tenant_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal)<br>`GET` /open-apis/auth/v3/tenant_access_token/internal<br>> 通过此接口获取 tenant_access_token<br></md-text> |  |  |


### 云文档

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** |
| --- | --- | --- |
| <md-text type="field-name" >[获取空间根目录](/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta)<br>`GET` /open-apis/drive/explorer/v2/root_folder/meta<br>> 获取云空间的根目录<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_all)<br>`POST` /open-apis/drive/v1/files/upload_all<br>> 向云空间指定目录下上传一个小文件<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[创建导入任务](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/create)<br>`POST` /open-apis/drive/v1/import_tasks<br>> 创建导入任务<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[查询导入任务结果](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/get)<br>`GET` /open-apis/drive/v1/import_tasks/:ticket<br>> 根据创建导入任务返回的 ticket 查询导入结果<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[更新云文档权限设置](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public/patch)<br>`PATCH` /open-apis/drive/v1/permissions/:token/public<br>> 该接口用于根据 filetoken 更新云文档的权限设置<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[获取文档所有块](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list)<br>`GET` /open-apis/docx/v1/documents/:document_id/blocks<br>> 获取文档所有块的富文本内容并分页返回<br></md-text> | <md-perm name="docx:document" desc="创建及编辑 DocX 文档" support_app_types="custom,isv" tags="">创建及编辑 DocX 文档</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[更新块](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/patch)<br>`PATCH` /open-apis/docx/v1/documents/:document_id/blocks/:block_id<br>> 更新指定的块<br></md-text> | <md-perm name="docx:document" desc="创建及编辑 DocX 文档" support_app_types="custom,isv" tags="">创建及编辑 DocX 文档</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |


