---
document_id: '6965400907877990405'
directory_id: '6907567266541404162'
title: 更新用户信息
full_path: /ukTMukTMukTM/uQzNz4CN3MjL0czM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- User
- Update a User’s Information
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:26Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uQzNz4CN3MjL0czM
---

# 更新用户信息
:::html

<md-alert type="error">

为了更好地提升该接口的安全性，我们对其进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/update)

</md-alert>

:::

该接口用于更新通讯录中用户信息。  


- 调用该接口需要申请`更新通讯录`以及`以应用身份访问通讯录`权限。应用需要拥有待更新用户的通讯录授权，如果涉及到用户部门变更，还需要同时拥有所有新部门的通讯录授权。应用商店应用无权限调用此接口。<br>


## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v1/user/update |
| HTTP Method | POST |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 更新通讯录 </md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份访问通讯录（历史版本）</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体
|参数|类型|必须|说明|
|-|-|-|-|
|employee_id、open_id|string|是|被更新的用户，支持通过 employee_id 或者 open_id 更新用户。请求中 employee_id 和 open_id 至少传递其中之一，同时传递两个参数时按 employee_id 处理|
|name|string|否|用户名|
|employee_type|int|否|员工类型。1:正式员工；2:实习生；3:外包；4:劳务；5:顾问|
|email|string|否|用户邮箱地址，同一企业内邮箱必须唯一，不能将用户的 email 和 mobile 同时更新为空|
|mobile|string|否|用户手机号，同一企业内手机号必须唯一，未激活用户不能更新 mobile 为空；已激活用户，不能将用户的 email 和 mobile 同时更新为空|
|department_ids、open_department_ids|list|否|用户新部门自定义 ID 或者 openID，需要更新用户的部门时指定其一即可，同时传两个参数优先使用 open_department_ids，仅支持一个用户在一个部门下，需要有新部门的通讯录权限|
|mobile_visible|bool|否|手机号码可见性，true 为可见，false 为不可见，目前默认为 true。不可见时，组织员工将无法查看该员工的手机号码|
|city|string|否|用户所在城市|
|country|string|否|用户所在国家|
|gender|int|否|性别，1: 男，2: 女|
|join_time|int|否|加入时间戳|
|leader_employee_id、leader_open_id|string|否|直接领导，支持通过 leader_employee_id 或者 leader_open_id 更新用户的直接领导，请求同时传递两个参数时按 leader_employee_id 处理|
|employee_no|string|否|工号|
|is_frozen|bool|否|是否暂停账号|
|custom_attrs|object|否|自定义用户属性。<br>传入的每个自定义用户属性包括平台生成的属性 ID 和用户自定义属性值。<br>企业开放自定义用户属性时，才允许设置用户自定义属性，否则会忽略该字段传入的内容。<br>传入的自定义属性 ID 不存在或者非法会忽略该条属性设置信息|
|work_station|string|否|工位|
### 请求体示例
```json
{
    "employee_id":"2cb23fa3",
    "open_id":"ou_6dfd8d7e5e881bed9be19c043940bf60",
    "name":"张三",
    "employee_type":1,
    "department_ids":["TT-1234"],
    "open_department_ids": ["od-8c6c97ab9a34c1a649001d7ad36b97a7"],
    "email":"zhangsan@gmail.com",
    "mobile":"13812345678",
    "mobile_visible":false,
    "city":"北京市",
    "country":"CN",
    "gender":1,
    "join_time":1562050148,
    "leader_open_id":"ou_6dfd8d7e5e881bed9be19c043940bf60",
    "leader_employee_id":"2cb2dfa3",
    "employee_no":"239473",
    "is_frozen":true,
    "custom_attrs":{
        "C-6702376000044400907":{
            "value":"value1"
        },
        "C-6702376000048595214":{
            "value":"value2"
        }
    },
    "work_station":"Poly, F6-123"
}
```


## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|对返回码的文本描述|
|data|-|返回内容|
### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
