---
document_id: '6971799133842309125'
directory_id: '6907567266537291777'
title: 获取应用管理员管理范围
full_path: /ukTMukTMukTM/uMzN3QjLzczN04yM3cDN
breadcrumb:
- Server API
- App Information
- Admin
- Obtain an App Admin’s Management Permissions
document_type: GuideDocumentType
updated_at: 2022-03-08T06:13:27Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMzN3QjLzczN04yM3cDN
---

# 获取应用管理员的管理范围
该接口用于获取应用管理员的管理范围，即该应用管理员能够管理哪些部门。  

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v1/user/admin_scope/get |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="admin:app.admin:readonly" desc="获取应用管理员 ID、管理范围等信息" support_app_types="custom" tags="">获取应用管理员 ID、管理范围等信息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求参数
|参数|类型|必须|说明|
|-|-|-|-|-|
|employee_id、open_id|string|是|支持通过 open_id 或者 employee_id 查询，不支持混合两种 ID 进行查询，其中 employee_id 同通讯录 v3 版本中的 user_id|

## 响应
### 响应体
|参数|说明|
|-|-|
|code|返回码，非 0 表示失败|
|msg|返回码的描述|
|data|返回业务数据|
|&emsp;∟is_all|是否管理所有部门|
|&emsp;∟department_list|管理的部门列表，当 is_all 为 true 时，不返回该字段|

### 响应示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "is_all":false,
        "departments_list":[
            "od-a140b4eeb892b90a0ab3e616fc2054d6",
            "od-a140b4eeb892b90a0ab3e616fc205477"
        ]
    }
}
```
