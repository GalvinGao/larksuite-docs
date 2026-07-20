---
document_id: '7070902815603081222'
directory_id: '6975751873563574278'
title: 文件下载
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//rule/download-file
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Rule
- Download Files
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:16Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//rule/download-file
---

# 文件下载
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/file/download)
</md-alert>
:::
通过文件 ID 下载指定的文件。

## 请求
|基本||
|---|---|
|HTTP URL|https://open.larksuite.com/open-apis/attendance/v1/files/:file_id/download|
|HTTP Method|GET|
|HTTP Content-Type|application/json; charset=utf-8|
|凭证要求|tenant_access_token|
|权限要求|打卡管理规则导出|
### 头部
key|value
--|--
Authorization|Bearer tenant_access_token
Content-Type|application/json
### 路径参数
|名称|类型|必填|描述|
|---|---|---|---|
|file_id|string|是|文件 ID，示例值："xxxxxb306842b1c189bc5212eefxxxxx"|
## 响应
### 响应头
名称|类型|描述|
|---|---|---|
|content-type|string|文件的 MIME|
|content-disposition|string|文件名|
HTTP 状态码为 200 时，表示成功

返回文件二进制流
### 错误码
|HTTP 状态码|错误码|描述|排查建议|
|---|---|---|---|
|400|1220001|参数错误|请检查参数是否符合要求|
|400|1220002|租户不存在|请检查 tenant_access_token 是否正确|
|500|1225000|系统错误|详见错误信息|
|500|1227000|管理服务系统错误|详见错误信息|
