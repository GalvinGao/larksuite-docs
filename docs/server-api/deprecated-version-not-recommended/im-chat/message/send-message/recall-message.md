---
document_id: '6967261316286693381'
directory_id: '6907567266537013249'
title: 消息撤回
full_path: /ukTMukTMukTM/ukjN1UjL5YTN14SO2UTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Message
- Send Message
- Recall Message
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:12Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukjN1UjL5YTN14SO2UTN
---

# 消息撤回
撤回指定消息。

:::html
<md-alert type="warn">
- 调用该接口需要注意：
  - 应用需要启用机器人能力
  - 要撤回的消息发出时间不能超过 24 小时
  - 只能撤回以应用身份发的消息（机器人自己发的消息）
  - **无法撤回** 通过「批量发送消息接口」发送的消息
</md-alert>
:::

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/message/v4/recall/ |
| HTTP Method | POST |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
message_id | string | 是 | 需要撤回的消息id ||om_f32a6454a616f4123b72fd9ecd976c41|

### 请求体示例

```json
{
   "message_id": "om_f32a6454a616f4123b72fd9ecd976c41"
}
```
## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码描述|


### 响应体示例
```json
{
    "code": 0,
    "msg": "ok"
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
