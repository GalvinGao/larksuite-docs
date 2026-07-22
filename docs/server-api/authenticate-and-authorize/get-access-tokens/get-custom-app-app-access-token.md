---
document_id: '6999168249243336710'
directory_id: '7262980135904346117'
title: 自建应用获取 app_access_token
full_path: /ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token_internal
breadcrumb:
- Server API
- Authenticate and Authorize
- Get Access Tokens
- Get custom app app_access_token
document_type: GuideDocumentType
updated_at: 2023-11-13T06:25:42Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token_internal
---

# 自建应用获取 app_access_token

自建应用通过此接口获取`app_access_token`。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=auth&version=v3&resource=app_access_token&method=internal)



:::note
**说明：** `app_access_token` 的最大有效期是 2 小时。如果在有效期小于 30 分钟的情况下，调用本接口，会返回一个新的 `app_access_token`，这会同时存在两个有效的 `app_access_token`。
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/auth/v3/app_access_token/internal |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | 无 |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 应用唯一标识，创建应用后获得。有关`app_id` 的详细介绍。请参考[通用参数](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology)介绍<br>**示例值：** "cli_slkdjalasdkjasd" |
| <md-text type="field-name" >app_secret</md-text> | <md-text type="field-type" >string</md-text> | 是 | 应用秘钥，创建应用后获得。有关 `app_secret` 的详细介绍，请参考[通用参数](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology)介绍<br>**示例值：** "dskLLdkasdjlasdKK" |




### 请求体示例

```json
{
    "app_id": "cli_slkdjalasdkjasd",
    "app_secret": "dskLLdkasdjlasdKK"
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 取值表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >app_access_token</md-text> | <md-text type="field-type" >string</md-text> | 应用访问凭证 |
| <md-text type="field-name" >expire</md-text> | <md-text type="field-type" >int</md-text> | `app_access_token` 的过期时间，单位为秒 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "app_access_token": "a-6U1SbDiM6XIH2DcTCPyeub",
    "expire": 7140
}
```

### 错误码
有关错误码的详细介绍，请参考[通用错误码](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)介绍。

