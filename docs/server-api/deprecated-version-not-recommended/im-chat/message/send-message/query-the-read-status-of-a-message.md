---
document_id: '6967261316286791685'
directory_id: '6907567266537013249'
title: 查询消息已读状态
full_path: /ukTMukTMukTM/ukTM2UjL5EjN14SOxYTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Message
- Send Message
- Query the Read Status of a Message
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:13Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukTM2UjL5EjN14SOxYTN
---

# 查询消息已读状态

查询消息已读状态，只能查询最近七天机器人自身发送消息的已读信息。
:::html
<md-alert type="warn">
需要启用机器人能力
</md-alert>
:::

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/message/v4/read_info/ |
| HTTP Method | POST |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |



### 请求体
|参数:w=15%|类型:w=10%|必填/选填:w=10%|说明:w=35%|默认值|实例:w=20%  |
|--|--|--|--|--|--  |
|message_id | string | 是 | 需要查询的消息id ||om_4d9bed57fa51660c96fcda6238d0e84e|

### 请求体示例

```json
{
   "message_id": "om_4d9bed57fa51660c96fcda6238d0e84e"
}
```

## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码描述|
|data | - | - |
|&emsp;∟read_users | list  | 已读的用户信息 |
|&emsp;&emsp;∟open_id | string  | 用户open_id |
|&emsp;&emsp;∟user_id | string  | 用户id，ISV应用没有这个字段 |
|&emsp;&emsp;∟timestamp | string  | 消息读取时间 |

### 响应体示例
```json
{
    "code": 0,
    "data": {
        "read_users": [
            {
                "open_id": "ou_18eac85d35a26f989317ad4f02e8bbbb",
                "timestamp": "1570697776",
                "user_id": "ca51d83b"
            }
        ]
    },
    "msg": "ok"
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)




