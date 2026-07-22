---
document_id: '7233334510161969157'
directory_id: '7199928167142211589'
title: 简介
full_path: /home/agile-project-cycle-management-based-on-bitable/introduction
breadcrumb:
- Home
- Agile project cycle management based on Bitable
- Introduction
document_type: GuideDocumentType
updated_at: 2023-05-17T03:03:27Z
source_url: https://open.larksuite.com/document/home/agile-project-cycle-management-based-on-bitable/introduction
---

# 简介

本教程介绍如何使用多维表格进行敏捷项目管理，通过日历开放能力将项目版本迭代周期创建为公开日历，供全团队订阅。
## 流程简介
本教程将按照以下流程调用Lark开放接口，实现多维表格中的项目版本迭代周期时间创建为公共日历。
:::html
<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/929f74f0d46a430dadf9c62b953db4dd_NZtgwMVVRX.png?lazyload=true&width=918&height=896" style="width:60%"/>
:::
## 实现效果
* 多维表格中的数据表数据如下：
  ![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1f78a79b38ac482dca91414a608861cf_crbfUj0MC1.png?lazyload=true&width=1640&height=603)
* 创建出的**项目版本迭代周期**公开日历，效果如下：
  ![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5bf17814814b631ae292e05221618116_MffNfp8VWz.png?lazyload=true&width=1640&height=727)
## 使用到的API列表

### 登录

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** |
| --- | --- | --- |
| <md-text type="field-name" >[获取 tenant_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal)<br>`GET` /open-apis/auth/v3/tenant_access_token/internal<br>> 获得访问其他接口需要用到的访问凭证<br></md-text> |  |  |


### 云文档

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** |
| --- | --- | --- |
| <md-text type="field-name" >[列出记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/list)<br>`GET` /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records<br>> 列出多维表格数据表中的详细记录<br></md-text> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm><br><md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv" tags="">查看、评论和导出多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[更新多条记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_update)<br>`POST` /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_update<br>> 更新数据表中的多条记录<br></md-text> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[新增多条记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_create)<br>`POST` /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_create<br>> 在数据表中新增多条记录<br></md-text> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[新增数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/create)<br>`POST` /open-apis/bitable/v1/apps/:app_token/tables<br>> 新增一个数据表<br></md-text> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[列出字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/list)<br>`GET` /open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields<br>> 获取数据表的所有字段<br></md-text> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm><br><md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv" tags="">查看、评论和导出多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[新增字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/create)<br>`POST` /open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields<br>> 在数据表中新增一个字段<br></md-text> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[更新字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/update)<br>`PUT` /open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields/:field_id<br>> 在数据表中更新一个字段<br></md-text> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |


### 日历

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求（满足任一） | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）** |
| --- | --- | --- |
| <md-text type="field-name" >[创建日历](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/create)<br>`POST` /open-apis/calendar/v4/calendars<br>> 创建一个新日历<br></md-text> | <md-perm name="calendar:calendar" desc="更新日历及日程信息" support_app_types="custom,isv" tags="">更新日历及<br>日程信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |
| <md-text type="field-name" >[创建日程](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/create)<br>`POST` /open-apis/calendar/v4/calendars/:calendar_id/events<br>> 身份由 Header Authorization 的 Token 类型决定。<br></md-text> | <md-perm name="calendar:calendar" desc="更新日历及日程信息" support_app_types="custom,isv" tags="">更新日历及<br>日程信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag><br><md-tag type="token-user">user_access_token</md-tag> |

