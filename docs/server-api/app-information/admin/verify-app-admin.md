---
document_id: '6971799133842391045'
directory_id: '6907567266537291777'
title: 校验应用管理员
full_path: /ukTMukTMukTM/uITN1EjLyUTNx4iM1UTM
breadcrumb:
- Server API
- App Information
- Admin
- Verify App Admin
document_type: GuideDocumentType
updated_at: 2022-03-08T06:13:24Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uITN1EjLyUTNx4iM1UTM
---

# 获取某个用户是否有应用管理权限

该接口用于查询用户是否为应用管理员。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/application/v3/is_user_admin |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="admin:app.admin:check" desc="校验用户是否为应用管理员" support_app_types="custom,isv" tags="">校验用户是否为应用管理员</md-perm><br><md-perm name="admin:app.admin:readonly" desc="获取应用管理员 ID、管理范围等信息" support_app_types="custom" tags="">获取应用管理员 ID、管理范围等信息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求参数
|参数|类型|必须|说明|
|--|-----|--|----|
|open_id|string|否|用户 open_id，open_id 和 employee_id 两个参数必须包含其一，若同时传入取 open_id|
|employee_id|string|否|用户 employee_id（同通讯录 v3 版本中的 user_id），open_id 和 employee_id 两个参数必须包含其一，若同时传入取 open_id|

## 响应

### 响应体
|参数|说明|
|--|--|
|code|返回码，非 0 表示失败|
|msg|返回码的描述|
|data|返回的业务信息|
|&emsp;∟is_app_admin|用户是否为管理员，true 为是，false 为否|

### 响应示例
```json
{ 
    "code": 0, 
    "msg": "ok", 
    "data": { 
        "is_app_admin": false
    } 
}
```
