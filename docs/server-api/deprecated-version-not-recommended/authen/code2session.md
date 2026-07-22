---
document_id: '6966503476859895814'
directory_id: '7345445193132883974'
title: 获取 user_access_token（小程序）
full_path: /uYjL24iN/ukjM04SOyQjL5IDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- authen
- code2session
document_type: GuideDocumentType
updated_at: 2024-03-18T08:47:01Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjM04SOyQjL5IDN
---

# code2session
通过 [login](/document/uYjL24iN/uYzMuYzMuYzM)接口获取到登录凭证`code`后，开发者可以通过服务器发送请求的方式获取 session_key 和 用户信息
:::html
<md-alert type="warn">
本接口应在服务端调用，调用方法参见 [如何调用服务端API](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/mina/v2/tokenLoginValidate |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | 无 |


### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-app">app_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体
参数 | 类型 | 必填 | 说明 
-- | -- | -- | -- 
code | string | 是 | [登录](/document/uYjL24iN/uYzMuYzMuYzM)时获取的 code

### 请求体示例
```json
{
     "code": "2ef0bb04e272d274"
}
```

## 响应
### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的[Open ID](/document/home/user-identity-introduction/open-id)，用于在同一个应用中对用户进行标识 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >employee_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的[User ID](/document/home/user-identity-introduction/user-id)，在职员工在企业内的唯一标识<br>**仅当开通以下权限后 返回该字段**：<br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >session_key</md-text> | <md-text type="field-type" >string</md-text> | 会话密钥 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >tenant_key</md-text> | <md-text type="field-type" >string</md-text> | 用户所在企业唯一标识 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >access_token</md-text> | <md-text type="field-type" >string</md-text> | [user_access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)，用户身份访问凭证 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >expires_in</md-text> | <md-text type="field-type" >int</md-text> | [user_access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)过期时间戳 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >refresh_token</md-text> | <md-text type="field-type" >string</md-text> | 刷新用户 access_token 时使用的 token，过期时间为30天 |


### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
    	"open_id": "ou_194fcfc5e4b78db556a040ff5e42c0",
    	"employee_id":"6c486g",
    	"union_id":"ou_be2c1742f2bb189469bdd33f0b1516",
    	"session_key": "e3aeb7df000c835365c630dac91bcf",
    	"tenant_key":"2c5914ac018f97",
    	"access_token":"u-tpwcnx2XzIcq8yHyJ6KL",
    	"expires_in":1565512680,
    	"refresh_token":"ur-W9dGvBJyVtwZmrwh0vBn"
    }
}
```

### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 200 | 10202 | access token invalid | 检查 access_token 是否过期 |
| 200 | 10213 | code appid not match | 1. 获取code的应用与该接口的应用必须是同一个应用，请确认是否跨应用调用<br>2. code 仅能使用一次，请确认是否重复使用或者过期 |
| 200 | 10228 | user to app has no visibility | 当前用户没有可见性 |




## 已知问题
返回的data中会包含一个union_id，该参数已废弃，与开放平台中常用的 **Union ID** 不是一个概念，请勿使用；如有需要，可通过[获取单个用户信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/get)获取
:::html
