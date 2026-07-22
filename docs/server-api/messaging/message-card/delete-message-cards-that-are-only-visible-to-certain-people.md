---
document_id: '6967331173081350149'
directory_id: '7021842990278164485'
title: 删除仅特定人可见的消息卡片
full_path: /ukTMukTMukTM/uITOyYjLykjM24iM5IjN
breadcrumb:
- Server API
- Messaging
- Message card
- Delete message cards that are only visible to certain people
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:50Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uITOyYjLykjM24iM5IjN
---

# 删除仅特定人可见的消息卡片
在群会话中删除仅指定用户可见的临时消息卡片。<br>
临时卡片消息可以通过该接口进行显式删除，临时卡片消息删除后将不会在该设备上留下任何痕迹。
:::html
<md-alert type="warn">
**权限说明** ：需要启用[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)；需要机器人在会话群里。
</md-alert>
:::
## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/ephemeral/v1/delete |
| HTTP Method | POST |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体
| 参数| 类型   | 必须 | 说明  |示例 |                                                   
| - | - | - | - | - | - |
| message_id|string | 是   | 临时消息ID |om_5ad573a6411d72b8305fda3a9c15c70e|
### 请求体示例

```json
{
   "message_id": "om_xxxxxxxxxxxx"
}
```
## 响应
### 响应体示例

```json
{
    "code": 0,
    "msg": "ok"
}
```
### 错误码

| 错误码 | 说明 | 排查建议 |
| --- | --- | --- |
| 18051 | 临时消息已删除 | 该临时消息已被用户删除，无法删除 |


其他通用错误码可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
