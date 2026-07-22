---
document_id: '7070902815602688006'
directory_id: '6975751873563574278'
title: 文件上传
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//rule/file_upload
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Rule
- Upload Files
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//rule/file_upload
---

# 文件上传
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/file/upload)
</md-alert>
:::
上传文件并获取文件 ID，可用于“修改用户设置”接口的 face_key 参数。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/files/upload |
| HTTP Method | POST |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限，开启其中任意一项权限即可调用</md-tooltip> | <md-perm>写入打卡管理规则</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用 access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **示例值**："multipart/form-data" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >file_name</md-text> | <md-text type="field-type" >string</md-text> | 是 | 文件名 |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >file</md-text> | <md-text type="field-type" >file</md-text> | 否 | 文件 |




### 请求体示例

```HTTP
Content-Disposition: form-data; name="file"
Content-Type: application/octet-stream
```


## 响应

### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >file</md-text> | <md-text type="field-type" >file</md-text> | 文件 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >file_id</md-text> | <md-text type="field-type" >string</md-text> | 文件 ID |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "file": {
            "file_id": "6b30e7636a38861bbe02869c726a4612"
        }
    }
}
```

### 错误码
|HTTP 状态码|错误码|描述|排查建议|
|---|---|---|---|
|400|1220001|参数错误|请检查参数是否符合要求|
|400|1220002|租户不存在|请检查 tenant_access_token 是否正确|
|500|1225000|系统错误|详见错误信息|
|500|1227000|管理服务系统错误|详见错误信息|


