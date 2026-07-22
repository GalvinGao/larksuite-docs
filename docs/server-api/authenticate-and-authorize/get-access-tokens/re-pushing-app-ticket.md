---
document_id: '6999168249243402246'
directory_id: '7262980135904346117'
title: 重新推送 app_ticket
full_path: /ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_ticket_resend
breadcrumb:
- Server API
- Authenticate and Authorize
- Get Access Tokens
- Re-pushing app_ticket
document_type: GuideDocumentType
updated_at: 2022-08-30T03:23:33Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_ticket_resend
---

# 重新推送 app_ticket

Lark每隔 1 小时会给应用推送一次最新的 app_ticket，应用也可以主动调用此接口，触发Lark进行及时的重新推送。（该接口并不能直接获取app_ticket，而是触发事件推送）{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=auth&version=v3&resource=auth&method=app_ticket_resend)

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

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/auth/v3/app_ticket/resend |
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
| <md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 应用唯一标识，创建应用后获得<br>**示例值**："cli_slkdjalasdkjasd" |
| <md-text type="field-name" >app_secret</md-text> | <md-text type="field-type" >string</md-text> | 是 | 应用秘钥，创建应用后获得<br>**示例值**："dskLLdkasdjlasdKK" |




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
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success"
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


