---
document_id: '6967261389551353861'
directory_id: '6907567266537472001'
title: 获取图片
full_path: /ukTMukTMukTM/uYzN5QjL2cTO04iN3kDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Message
- Images
- Get Image
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:15Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYzN5QjL2cTO04iN3kDN
---

# 获取图片

根据图片的image_key获取图片内容


:::html
<md-alert type="warn">
需要启用机器人能力；机器人只能获取自己上传的图片。
</md-alert>
:::


## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/image/v4/get?image_key=24383920-9321-4ecd-8b33-bf8ce74e84c8 |
| HTTP Method | GET |


### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant-desc">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 查询参数
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
image_key | string| 必填 | 图片的key | |img_fdeb9536-8ec4-485f-988b-6ebd338ad47g|

## 响应
### 响应体
返回图片的二进制数据流
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
