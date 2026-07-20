---
document_id: '7070902815603163142'
directory_id: '6975751873563574278'
title: 批量查询用户设置
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//rule/batch-query-user-settings
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Rule
- Batch Query the User Settings
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:16Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//rule/batch-query-user-settings
---

# 批量查询用户设置
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/query)
</md-alert>
:::
批量查询授权内员工的用户设置信息，包括人脸照片文件 ID、人脸照片更新时间。


## 请求
|基本||
|---|---|
|HTTP URL|https://open.larksuite.com/open-apis/attendance/v1/user_settings/query|
|HTTP Method|GET|
|HTTP Content-Type|application/json; charset=utf-8|
|凭证要求|tenant_access_token|
|权限要求|打卡数据导出|
### 头部
key|value
--|--
Authorization|Bearer tenant_access_token
Content-Type|application/json
### 查询参数
|名称|类型|必填|描述|
|---|---|---|---|
|employee_type|string|是|请求体中的 user_ids 的员工工号类型，可用值：【employee_id（员工的 employeeId），employee_no（员工工号）】，示例值：“employee_id”|

### 请求体
|名称|类型|必填|描述|
|---|---|---|---|
|user_ids|string\[\]|是|employee_no 或 employee_id 列表，长度不超过 100|
### 请求体示例
```json
{
    "user_ids": [
        "abd754f7"
    ]
}
```
## 响应
### 响应体
|名称|类型|描述|
|---|---|---|
|code|int|错误码，非 0 表示失败|
|msg|string|错误描述|
|data|-|-|
|&emsp;∟user_settings|user_setting\[\]|用户设置信息列表|
|&emsp;&emsp;∟user_id|string|员工工号|
|&emsp;&emsp;∟face_key|string|人脸照片文件 ID|
|&emsp;&emsp;∟face_key_update_time|string|人脸照片更新时间，精确到秒的时间戳|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "user_settings": [
          {
            "face_key": "xxxxxb306842b1c189bc5212eefxxxxx",
            "face_key_update_time": "1625681917",
            "user_id": "abd754f7"
          }
        ]
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
|500|1227500|组织架构服务系统错误|详见错误信息|
