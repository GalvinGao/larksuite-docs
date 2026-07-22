---
document_id: '6967331158355427334'
directory_id: '6907567266537013249'
title: 发送消息卡片
full_path: /ukTMukTMukTM/uYTNwUjL2UDM14iN1ATN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Message
- Send Message
- Send Message Card
document_type: GuideDocumentType
updated_at: 2022-03-13T12:47:11Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYTNwUjL2UDM14iN1ATN
---

# 发送消息卡片


给指定用户或者会话发送消息卡片，其中会话包括私聊会话和群会话。<br>
:::html
<md-alert type="warn">
需要启用机器人能力；私聊会话时机器人需要拥有对用户的可见性，群会话需要机器人在群里
</md-alert>
:::

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/message/v4/send/ |
| HTTP Method | POST |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 以应用的身份发消息 </md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |



### 请求体

| 参数| 类型   | 必须 | 说明  |默认值|实例 |                                                   
| - | - | - | - | - | - |
open_id <br>user_id <br> email <br> chat_id  | string | 是 | 给用户发私聊消息，只需要填 open_id、email、user_id 中的一个即可，向群里发消息使用群的 chat_id（可通过[获取群列表接口](/document/ukTMukTMukTM/uITO5QjLykTO04iM5kDN)获取）。服务端依次读取字段的顺序为 chat_id > open_id > user_id > email   ( user_id 对应V3接口的 employee_id , chat_id 对应V3的 open_chat_id )||ou_5ad573a6411d72b8305fda3a9c15c70e|
| msg_type                                          | string | 是   | 消息的类型，此处固定填 "interactive" ||interactive|
| card|object | 是   | 消息卡片的描述内容，具体参考[卡片结构](/document/ukTMukTMukTM/ugTNwUjL4UDM14CO1ATN) |
| root_id                                              | string | 否   | 需要回复的消息的open_message_id||om_40eb06e7b84dc71c03e009ad3c754195| 
| update_multi                                     | bool | 否   | 控制卡片是否是共享卡片(所有用户共享同一张消息卡片），默认为 false，流程参考[交互模块](/document/ukTMukTMukTM/uYjNwUjL2YDM14iN2ATN) ||true|


### 请求体示例

```json
{
   "chat_id": "oc_abcdefg1234567890",
   "msg_type": "interactive",
   "root_id":"om_4*********************ad8",
   "update_multi":false,
   "card": {
        // card content
    }
}
```
## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码描述|
data | - | -
&emsp;∟message_id |string| 消息 ID

### 响应体示例
```json
{
    "code": 0,
    "msg": "ok",
    "data":{
       "message_id": "om_92eb70a7120ga8c3ca56a12ee9ba7ca2"
    }
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
