---
document_id: '7233612551991590918'
directory_id: '7199928167142244357'
title: 使用到的API列表
full_path: /home/sales-statistics-base-on-spreadsheets/list-of-apis
breadcrumb:
- Home
- Sales statistics base on spreadsheets
- List of APIs
document_type: GuideDocumentType
updated_at: 2023-05-16T03:12:31Z
source_url: https://open.larksuite.com/document/home/sales-statistics-base-on-spreadsheets/list-of-apis
---

# 使用到的API列表

在当前场景中，需要调用云文档业务域的 API 列表：
- 云文档
  - [获取空间根目录](/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta)
  - [创建表格](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create)
  - [获取表格元数据](/document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN)
  - [向多个范围写入数据](/document/ukTMukTMukTM/uEjMzUjLxIzM14SMyMTN)
  - [批量设置单元格样式](/document/ukTMukTMukTM/uAzMzUjLwMzM14CMzMTN)
  - [增加权限](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create)

## 云文档

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** |
| --- | --- | --- |
| <md-text type="field-name" >[获取空间根目录](/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta)<br>`GET` /open-apis/drive/explorer/v2/root_folder/meta<br>> 获取云空间的根目录<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[创建表格](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create)<br>`POST` /open-apis/sheets/v3/spreadsheets<br>> 使用该接口可以在指定的目录下创建在线表格<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[获取表格元数据](/document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN)<br>`GET` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/metainfo<br>> 该接口用于根据 spreadsheetToken 获取表格元数据<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[向多个范围写入数据](/document/ukTMukTMukTM/uEjMzUjLxIzM14SMyMTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_update<br>> 该接口用于根据 spreadsheetToken 和 range 向多个范围写入数据<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[批量设置单元格样式](/document/ukTMukTMukTM/uAzMzUjLwMzM14CMzMTN)<br>`PUT` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/styles_batch_update<br>> 该接口用于根据 spreadsheetToken、range 批量更新单元格样式<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[增加权限](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create)<br>`POST` /open-apis/drive/v1/permissions/:token/members<br>> 该接口用于根据 filetoken 给用户增加文档的权限<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |


