---
document_id: '6965400907875368966'
directory_id: '6907567266541404162'
title: 获取角色列表
full_path: /ukTMukTMukTM/uYzMwUjL2MDM14iNzATN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- User
- Obtain a Role List
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:29Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYzMwUjL2MDM14iNzATN
---

# 获取角色列表
:::html

:::

该接口用于获取企业的用户角色列表。<br>




## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v2/role/list |
| HTTP Method | GET |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 获取角色 </md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 读取通讯录 </md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


## 响应
### 响应体

|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码的描述|
|data|-|返回业务数据|
|&emsp;∟role_list|list|角色列表|
|&emsp;&emsp;∟id|string|角色 ID|
|&emsp;&emsp;∟name|string|角色名称|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "role_list": [
            {
                "id": "or_846ea69995a259a27cc690182f27de87",
                "name": "IT"
            },
            {
                "id": "or_b337b102aff4167bd29189b1d94b77f6",
                "name": "HR"
            }
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

