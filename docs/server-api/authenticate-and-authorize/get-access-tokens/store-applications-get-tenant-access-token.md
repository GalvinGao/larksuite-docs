---
document_id: '6999168249243385862'
directory_id: '7262980135904346117'
title: 商店应用获取 tenant_access_token
full_path: /ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token
breadcrumb:
- Server API
- Authenticate and Authorize
- Get Access Tokens
- Store applications get tenant_access_token
document_type: GuideDocumentType
updated_at: 2023-11-13T06:25:42Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token
---

# 获取 tenant_access_token（应用商店应用）

应用商店应用通过此接口获取 tenant_access_token，调用接口获取企业资源时，需要使用 tenant_access_token 作为授权凭证。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=auth&version=v3&resource=auth&method=tenant_access_token)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="tip">
token 有效期为 2 小时，在此期间调用该接口 token 不会改变。当 token 有效期小于 30 分的时候，再次请求获取 token 的时候，会生成一个新的 token，与此同时老的 token 依然有效。
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/auth/v3/tenant_access_token |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | 无 |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >app_access_token</md-text> | <md-text type="field-type" >string</md-text> | 是 | 应用授权凭证<br>**示例值**："a-32bd8551db2f081cbfd26293f27516390b9feb04" |
| <md-text type="field-name" >tenant_key</md-text> | <md-text type="field-type" >string</md-text> | 是 | 企业标识，两种获取方式：<br>1. 企业开通应用时由平台方推送给应用，具体可参考[首次启用应用](/document/uAjLw4CM/ukTMukTMukTM/application-v6/event/app-first-enabled)<br>2. 用户登录时返回，具体可参考[获取用户身份访问凭证](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/get-)<br>**示例值**："73658811060f175d" |




### 请求体示例

```json
{
    "app_access_token": "a-32bd8551db2f081cbfd26293f27516390b9feb04",
    "tenant_key": "73658811060f175d"
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >tenant_access_token</md-text> | <md-text type="field-type" >string</md-text> | 访问 token |
| <md-text type="field-name" >expire</md-text> | <md-text type="field-type" >int</md-text> | tenant_access_token 过期时间，单位: 秒 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "tenant_access_token": "t-caecc734c2e3328a62489fe0648c4b98779515d3",
    "expire": 7140
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


