---
document_id: '6967331173081399301'
directory_id: '7122028361538797574'
title: 取消订阅审批事件
full_path: /ukTMukTMukTM/ugDOyUjL4gjM14CO4ITN
breadcrumb:
- Server API
- Approval
- Approval（history version）
- v2
- Lark native approval
- Cancel a Subscription to an Approvals Event
document_type: GuideDocumentType
updated_at: 2022-07-20T09:38:56Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugDOyUjL4gjM14CO4ITN
---

# 取消订阅审批事件
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/unsubscribe)
</md-alert>
:::
取消订阅 approval_code 后，无法再收到该审批定义对应实例的事件通知。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://www.larksuite.com/approval/openapi/v2/subscription/unsubscribe |
| HTTP Method | POST |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体

| 名称         | 类型           | 必须        | 说明        |
| --------- | --------------- | -------   | --------- |
|approval_code | string | 是 |  审批定义唯一标识 |

### 请求体示例

```json
{
	"approval_code":"7C468A54-8745-2245-9675-08B7C63E7A85"
}
````

## 响应

### 响应体
| 参数         |类型         |必须  | 说明        |
| --------- | ----------|----- | --------- |
|code |int |是 |错误码，非0表示失败 |
|msg | string |是| 返回码的描述|
### 响应体示例

```json
{
    "code":0,
    "msg":"success"
}
```
