---
document_id: '7031483915610439685'
directory_id: '6921376028544499713'
title: 概述
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction
breadcrumb:
- Server API
- Docs
- Space
- Overview
document_type: GuideDocumentType
updated_at: 2022-10-08T09:37:24Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction
---

# 云空间概述

云空间是便捷管理知识资源的企业云盘。在Lark所有文档都集中存储在云端，支持多格式文件的高速上传、下载及预览，并通过电脑、手机随时随地查看、编辑、共享和协作，打造触手可及的知识智库。

| 资源 | 资源定义 |
| --- | --- |
| **文件夹** | 用于管理文件和其它文件夹的容器。 |
| **文件** | 各种类型的文件的统称，泛指云空间内所有的文件。 |




## 资源：文件夹 Folder
文件夹是用于管理文件和其它文件夹的容器。每个文件夹都有唯一 token 作为标识。
:::note
由于线上资源存在新老规范，文件夹 token 在部分接口中的命名可能为  folder_token, token, folderToken，在调用时请仔细阅读接口文档，避免因命名问题导致报错。
:::

##  字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >folder_token</md-text> | <md-text type="field-type" >string</md-text> | 一个文件夹的唯一标识。  <br><br>由于线上资源存在新老规范，文件夹 token 在部分接口中的命名可能为  folder_token, token, folderToken，在调用时请仔细阅读接口文档，避免因命名问题导致报错。<br><br>**示例值**："fldcnK0sP9zb1TejQsaN0S54cHc"<br>**字段权限要求（任选其一）**：<br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly	" desc="查看、评论和下载云空间中所有文件" tags="">查看、评论和下载云空间中所有文件</md-perm> |
| <md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 文件夹的名称。 |




### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[新建文件夹](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/create_folder)<br>`POST` /open-apis/drive/v1/files/create_folder<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user" >user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取空间根目录](/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta)</md-text><br>`GET` /open-apis/drive/explorer/v2/root_folder/meta<br>> 获取云空间的根目录 | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user" >user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取文件夹元信息](/document/ukTMukTMukTM/uAjNzUjLwYzM14CM2MTN)</md-text><br>`GET` /open-apis/drive/explorer/v2/folder/:folderToken/meta<br>> 获取文件夹的元信息 | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取文件夹下的文档清单](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list)</md-text><br>`GET` /open-apis/drive/v1/files | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |

  
  
## 资源：文件 Files
文件是各种类型的文件的统称，泛指云空间内所有的文件。每个文件都有唯一 token 作为标识。


##  字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >file_token</md-text> | <md-text type="field-type" >string</md-text> | 一个文件的唯一标识。  <br><br>**示例值**："boxcnK7G8kasZRac70Wo50y6NGh"<br>**字段权限要求（任选其一）**：<br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly	" desc="查看、评论和下载云空间中所有文件" tags="">查看、评论和下载云空间中所有文件</md-perm> |
| <md-text type="field-name" >file_name</md-text> | <md-text type="field-type" >string</md-text> | 文件的名称。 |
| <md-text type="field-name" >parent_node</md-text> | <md-text type="field-type" >string</md-text> | 文件所在文件夹的 token。 |




### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/create)</md-text><br>`POST` /open-apis/drive/v1/files/upload_all<br>> 用于上传 20M 以内的文件 | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm> | <md-tag ype="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[分片上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/multipart-upload-file-/introduction)</md-text><br>`POST` /open-apis/drive/v1/files/upload_prepare<br>`POST` /open-apis/drive/v1/files/upload_part<br>`POST` /open-apis/drive/v1/files/upload_finish<br>>上传较大文件（>20M）时，推荐使用分片上传 | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm> | <md-tag ype="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[下载文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/download)</md-text><br>`GET` /open-apis/drive/v1/files/:file_token/download | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[复制文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/copy)</md-text><br>`POST` /open-apis/drive/v1/files/:file_token/copy | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[移动文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/move)</md-text><br>`POST` /open-apis/drive/v1/files/:file_token/move | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[删除文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete)</md-text><br>`DELETE` /open-apis/drive/v1/files/:file_token | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取文件元数据](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/meta/batch_query)</md-text><br>`POST` /open-apis/drive/v1/metas/batch_query<br>>支持批量获取文件的元数据 | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |

