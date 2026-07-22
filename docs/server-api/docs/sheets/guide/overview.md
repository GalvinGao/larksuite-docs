---
document_id: '6967331158355836934'
directory_id: '6956134701804322822'
title: 电子表格概述
full_path: /ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview
breadcrumb:
- Server API
- Docs
- Sheets
- Guide
- Overview
document_type: GuideDocumentType
updated_at: 2022-03-28T13:28:11Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview
---

# 电子表格概述

支持通过调用表格 API，完成自动创建表格、读取或编辑工作表等操作。  


# 名词解释

## 表格（Spreadsheet）

表格是承载数据的容器，也提供数据处理和呈现的功能。一个表格可能包含一个或者多个工作表，我们在处理数据时，都是针对某个工作表进行操作的。
:::note
每篇文档都有一个 spreadsheetToken 作为唯一标识，每一个工作表都有一个 sheetId 作为唯一标识。
:::

## spreadsheetToken 以及 sheetId

spreadsheetToken 是一个表格的唯一标识，你可以通过以下任一方式获取一个表格的 spreadsheetToken：  

- 通过表格的 URL 获取：https://sample.larksuite.com/sheets/==shtcnmBA\*****yGehy8==
- 通过 [获取文件夹下文档清单 API](/document/ukTMukTMukTM/uEjNzUjLxYzM14SM2MTN) 的返回值获取表格的 spreadsheetToken

<br>
<br>
sheetId 是一个工作表的唯一标识，你可以通过以下任一方式获取一个工作表的 sheetId：
- 通过表格的 URL 获取：https://sample.larksuite.com/sheets/shtcnmBA\*****yGehy8?sheet===0b\**12==
- 通过 [获取表格元数据 API](/document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN) 的返回值获取工作表的 sheetId

:::note
几乎所有的表格操作方法，需要传入 spreadsheetToken 来指定要操作的表格。
:::

## Range
Range 描述工作表的某个范围。在数据读写中，能帮助用户过滤数据的操作范围。

range 的描述方式为 ==\<sheetId>!<开始位置>:<结束位置>== ，共有 4 种描述方法，分别为：
- \<sheetId>!<开始单元格>:<结束单元格>  
如：0b\**12!A1:B5 就表示 0b\**12 这个工作表中 A1:B5 的区域，如下图所示：
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/76498891d78bff326b0bcffe43427fa9_fUFkCG77Vw.png?lazyload=true&width=722&height=484)

- \<sheetId>!<开始列>:<结束列>，如：0b\**12!A:B
- \<sheetId>!<开始单元格>:<结束列>，如：0b\**12!A1:B
- \<sheetId>，区域留空，如：0b\**12，代表这个表格中非空的最大行列范围内的数据  


在使用「读取单个范围」或「读取多个范围」等读取表格数据的接口时，上述几种描述方式，表示这个工作表中 A1:B5 的区域，如下图所示：
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dca12da354cb99d01c2893283893ae2a_Y9COFAZsea.png?lazyload=true&width=856&height=576)

# 方法集合

## 表格
包含表格创建、获取和更新相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[创建表格](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create)<br>`POST` /open-apis/sheets/v3/spreadsheets<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取表格元数据](/document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN)<br>`GET` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/metainfo<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">查看、评论和导出电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新表格属性](/document/ukTMukTMukTM/ucTMzUjL3EzM14yNxMTN)<br>`PUT` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/properties<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


## 工作表
包含工作表操作和更新属性相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[更新工作表属性](/document/ukTMukTMukTM/ugjMzUjL4IzM14COyMTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[操作工作表](/document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


## 工作表 - 条件格式
包含工作表中条件格式创建、删除、获取和更新相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[创建条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-set)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_create<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-get)<br>`GET` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">查看、评论和导出电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-update)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_update<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[删除条件格式](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-delete)<br>`DELETE` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_delete<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


## 工作表 - 筛选
包含工作表筛选创建、删除、获取和更新相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[获取筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/get)<br>`GET` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet:readonly.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive:readonly.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[创建筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/create)<br>`POST` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/update)<br>`PUT` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[删除筛选](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/delete)<br>`DELETE` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


## 工作表 - 筛选视图
包含工作表筛选视图创建、删除、获取和更新相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[删除筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/delete)<br>`DELETE` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/patch)<br>`PATCH` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[查询筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/query)<br>`GET` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/query<br></md-text> | <md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet:readonly.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive:readonly.desc.desc$$$</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/get)<br>`GET` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet:readonly.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive:readonly.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[创建筛选视图](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/create)<br>`POST` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


## 筛选视图 - 筛选条件
包含筛选视图的筛选条件创建、删除、获取、更新和查询相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[删除筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/delete)<br>`DELETE` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/update)<br>`PUT` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[查询筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/query)<br>`GET` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/query<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet:readonly.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive:readonly.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/get)<br>`GET` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id<br></md-text> | <md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive:readonly.desc.desc$$$</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet:readonly.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[创建筛选条件](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/create)<br>`POST` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions<br></md-text> | <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


## 工作表 - 行列
包含工作表行列增加、插入、删除、移动和更新相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[删除行列](/document/ukTMukTMukTM/ucjMzUjL3IzM14yNyMTN)<br>`DELETE` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新行列](/document/ukTMukTMukTM/uYjMzUjL2IzM14iNyMTN)<br>`PUT` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[增加行列](/document/ukTMukTMukTM/uUjMzUjL1IzM14SNyMTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[插入行列](/document/ukTMukTMukTM/uQjMzUjL0IzM14CNyMTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/insert_dimension_range<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[移动行列](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/move_dimension)<br>`POST` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/move_dimension<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.drive:drive.desc.desc$$$</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">$$$sheets.v3.scope.sheets.v3.scope.sheets:spreadsheet.desc.desc$$$</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


 
## 行列 - 保护范围
包含行列的保护范围增加、删除、获取和修改相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[增加保护范围](/document/ukTMukTMukTM/ugDNzUjL4QzM14CO0MTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_dimension<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取保护范围](/document/ukTMukTMukTM/uQTM5YjL0ETO24CNxkjN)<br>`GET` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_get<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[修改保护范围](/document/ukTMukTMukTM/uUTM5YjL1ETO24SNxkjN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_update<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[删除保护范围](/document/ukTMukTMukTM/uYTM5YjL2ETO24iNxkjN)<br>`DELETE` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_del<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


## 工作表 - 数据
包含工作表中单元格数据写入与读取，插入图片，样式设置，单元格合并与拆分，内容查找和替换相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[插入数据](/document/ukTMukTMukTM/uIjMzUjLyIzM14iMyMTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_prepend<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[追加数据](/document/ukTMukTMukTM/uMjMzUjLzIzM14yMyMTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_append<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[读取单个范围](/document/ukTMukTMukTM/ugTMzUjL4EzM14COxMTN)<br>`GET` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values/:range<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">查看、评论和导出电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[读取多个范围](/document/ukTMukTMukTM/ukTMzUjL5EzM14SOxMTN)<br>`GET` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_get<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">查看、评论和导出电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[向单个范围写入数据](/document/ukTMukTMukTM/uAjMzUjLwIzM14CMyMTN)<br>`PUT` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[向多个范围写入数据](/document/ukTMukTMukTM/uEjMzUjLxIzM14SMyMTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_update<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[设置单元格样式 ](/document/ukTMukTMukTM/ukjMzUjL5IzM14SOyMTN)<br>`PUT` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/style<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[批量设置单元格样式 ](/document/ukTMukTMukTM/uAzMzUjLwMzM14CMzMTN)<br>`PUT` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/styles_batch_update<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[合并单元格](/document/ukTMukTMukTM/ukDNzUjL5QzM14SO0MTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/merge_cells<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[拆分单元格](/document/ukTMukTMukTM/uATNzUjLwUzM14CM1MTN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/unmerge_cells<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[写入图片](/document/ukTMukTMukTM/uUDNxYjL1QTM24SN0EjN)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_image<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[查找单元格](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/find)<br>`POST` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/find<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">查看、评论和导出电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[替换单元格](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/replace)<br>`POST` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/replace<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


## 工作表 - 浮动图片
包含工作表浮动图片创建、删除、查询、获取和更新相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[创建浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/create)<br>`POST` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[获取浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/get)<br>`GET` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">查看、评论和导出电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[查询浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/query)<br>`GET` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/query<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">查看、评论和导出电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/patch)<br>`PATCH` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[删除浮动图片](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/delete)<br>`DELETE` /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


## 工作表 - 数据校验
包含工作表数据验证设置、删除、查询和更新相关接口。
### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** | 商店 | 自建 |
| --- | --- | --- | --- | --- |
| <md-text type="field-name" >[设置下拉列表](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/set-dropdown)<br>`POST` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[删除下拉列表设置](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/delete-datavalidation)<br>`DELETE` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[更新下拉列表设置](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/update-datavalidation)<br>`PUT` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation/:sheetId/:dataValidationId<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |
| <md-text type="field-name" >[查询下拉列表设置](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/query-datavalidation)<br>`GET` /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation<br></md-text> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> | **✓** | **✓** |


