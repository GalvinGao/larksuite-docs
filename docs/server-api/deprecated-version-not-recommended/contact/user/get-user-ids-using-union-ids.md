---
document_id: '6965400907875090438'
directory_id: '6907567266541404162'
title: 使用统一 ID 获取用户 ID
full_path: /ukTMukTMukTM/uUTO5UjL1kTO14SN5kTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- User
- Get User IDs Using Union IDs
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:38Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUTO5UjL1kTO14SN5kTN
---

# 使用统一 ID 获取用户 ID
使用统一 ID 获取用户 ID信息。<br>



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/user/v1/union_id/batch_get/list?union_ids=on_94a1ee5551019f18cd73d9f111898cf2&union_ids=on_42f2ef9d07319a4d96fffd7ef5cbfc79 |
| HTTP Method | GET |
| 字段权限要求<br><md-tooltip type="info">接口返回的部分字段受权限控制，开启字段权限才可获取对应字段数据；如无需获取这些字段，则无需开启。</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">根据要获取的字段开启相应权限</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户 user ID</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 查询参数
参数 | 类型 | 必须 | 说明
-- | -- | -- | --
union_ids |string | 是 | 要查询的用户union_id，最多100条
## 响应
### 响应体
参数 | 说明
-- | --
code | 返回码，非 0 表示失败
msg | 对返回码的文本描述
data | 返回内容
&emsp;∟user_infos | 用户信息
&emsp;&emsp;∟open_id | 用户的 open_id
&emsp;&emsp;∟user_id | 用户的 user_id，获取employee_id权限后返回
### 响应体示例
```json
{
    "code": 0,
    "msg": "",
    "data": {
        "user_infos": {
            "on_7dba11ff38a2119f89349876b12af65c": {
                "user_id": "b6e596g8",
                "open_id": "ou_ef74363752064ca799c2bb7b6bdca9f4"
            },
            "on_c132837f686587dd494aa54f5f65b552": {
                "user_id": "384cf662",
                "open_id": "ou_47e071d2656df0df08a49d87e0c7f518"
            },
            "on_d12d180970dcc5e89f26929745166fb3": {
                "user_id": "3cc7eaf6",
                "open_id": "ou_c1ba95fa08f067545a99c8dba5bf588e"
            }
        }
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
