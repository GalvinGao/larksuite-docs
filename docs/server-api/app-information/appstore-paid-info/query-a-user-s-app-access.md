---
document_id: '6967261389552025605'
directory_id: '6907567266540896258'
title: 查询用户是否在应用开通范围
full_path: /ukTMukTMukTM/uATNwUjLwUDM14CM1ATN
breadcrumb:
- Server API
- App Information
- Appstore Paid Info
- Query a User's App Access
document_type: GuideDocumentType
updated_at: 2022-03-08T06:13:43Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uATNwUjLwUDM14CM1ATN
---

#  查询用户是否能使用付费功能

当付费套餐是按人数收费 或者 限制最大使用人数时，开放平台会引导企业管理员设置“付费功能开通范围”。  但是受开通范围限制，部分用户就无法使用对应的付费功能。  可以通过此接口，在付费功能点入口判断是否允许某个用户进入使用。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/pay/v1/paid_scope/check_user |
| HTTP Method | GET |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求参数

| 参数         | 类型           | 必须        | 说明         |
| --------- | --------------- | -------   | ----------- | 
|open_id | string | 否 |用户 open_id，open_id 和 user_id 两个参数必须包含其一，若同时传入取 open_id|
|user_id | string | 否 |用户 user_id，user_id 和 open_id 两个参数必须包含其一，若同时传入取 open_id |

## 响应
### 响应体
| 参数         | 说明           | 
|-|-| 
|code | 返回码，非 0 表示失败 |
|msg | 返回码的描述 |
|data | 返回的业务信息 |
|&emsp;∟status | 用户是否在开通范围中，"valid" -该用户在开通范围中，"not_in_scope"-该用户不在开通范围中，"no_active_license"-企业未购买任何价格方案或价格方案已过期，"exceeds_maximum_limit"-企业当前配置的付费功能开通范围人数超出限制，需提醒管理员调整 |
|&emsp;∟price_plan_id | 租户当前使用的「价格方案ID」，对应开发者后台中「价格方案配置」中的「价格方案」 |
|&emsp;∟is_trial | 是否为试用版本，true-是试用版本；false-非试用版本 |
|&emsp;∟service_stop_time | 租户当前有生效价格方案时表示价格方案的到期时间，为时间unix时间戳 |
### 响应示例
```json
{
    "code": 0， 
    "msg": "success"，
    "data": {
        "status": "valid", 
        "price_plan_id": "price_9daf66c96968c003",
        "is_trial": false,
        "service_stop_time":"1572280518"
    }
} 
```
