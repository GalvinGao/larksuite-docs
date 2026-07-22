---
document_id: '7070902815602966534'
directory_id: '6975751873563574278'
title: 修改用户设置
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//rule/user-setting-modify
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Rule
- Modify User Settings
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:18Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//rule/user-setting-modify
---

# 修改用户设置
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/modify)
</md-alert>
:::
修改授权内员工的用户设置信息，包括人脸照片文件 ID。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_settings/modify |
| HTTP Method | POST |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限，开启其中任意一项权限即可调用</md-tooltip> | <md-perm >写入打卡管理规则</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用 access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户类型<br>**可选值有**：<br>- `employee_id`： 员工 ID<br>- `employee_no`： 员工工号 |


### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >user_setting</md-text> | <md-text type="field-type" >user_setting</md-text> | 否 | 用户信息 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >face_key</md-text> | <md-text type="field-type" >string</md-text> | 是 | 人脸照片 key（通过文件上传接口得到） |



### 请求体示例

```json
{
    "user_setting": {
        "user_id": "61gc44e1",
        "face_key": "1013d6cb59555a26ff3e5f721342a2a7"
    }
}
```

## 响应


### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_setting</md-text> | <md-text type="field-type" >user_setting</md-text> | 用户设置 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >face_key</md-text> | <md-text type="field-type" >string</md-text> | 人脸照片 key |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "user_setting": {
            "user_id": "61gc44e1",
            "face_key": "1013d6cb59555a26ff3e5f721342a2a7",
        }
    }
}
```

### 错误码
|HTTP 状态码|错误码|描述|排查建议|
|---|---|---|---|
|400|1220001|参数错误|请检查参数是否符合要求|
|400|1220002|租户不存在|请检查 tenant_access_token 是否正确|
|400|1220004|用户不存在或没有权限|请检查用户 ID 是否正确|
|400|1220005|没有权限|请前往考勤管理后台检查数据权限范围|
|500|1225000|系统错误|详见错误信息|
|500|1227000|管理服务系统错误|详见错误信息|



