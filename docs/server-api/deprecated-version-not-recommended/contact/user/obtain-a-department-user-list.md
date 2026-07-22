---
document_id: '6965400907877974021'
directory_id: '6907567266541404162'
title: 获取部门用户列表
full_path: /ukTMukTMukTM/uEzNz4SM3MjLxczM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- User
- Obtain a Department User List
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:14Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uEzNz4SM3MjLxczM
---

# 获取部门用户列表
:::html

<md-alert type="error">

为了更好地提升该接口的安全性，我们对其进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/find_by_department)

</md-alert>

:::

该接口用于获取部门用户列表。


- 应用需要有被调用部门的通讯录授权



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v1/department/user/list?open_department_id=od-c042a4980ba8e1466050e3e8da2378fe&page_size=100&fetch_child=true<br>https://open.larksuite.com/open-apis/contact/v1/department/user/list?department_id=TT-1234&page_size=100&fetch_child=true<br> |
| HTTP Method | GET |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 获取部门组织架构信息 </md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 读取通讯录 </md-perm> |
| 字段权限要求<br><md-tooltip type="info">接口返回的部分字段受权限控制，开启字段权限才可获取对应字段数据；如无需获取这些字段，则无需开启。</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">根据要获取的字段开启相应权限</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户基本信息</md-perm></md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户雇佣信息</md-perm></md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 查询参数
|参数|类型|必须|说明|
|-|-|-|-|
|open_department_id、 department_id|string|是|请求部门的ID，可以通过部门的 openID 或者自定义 ID 请求， 同时传 openID 和自定义 ID 忽略自定义 ID|
|page_token|string|否|分页标记，第一次请求可以不填，表示从头开始遍历；分页查询返回结果has_more 为 true 时会同时返回新的 page_token, 下次遍历可使用该返回的 page_token 获取更多部门成员|
|page_size|int|是|分页大小，取值范围 1-100|
|fetch_child|bool|是|是否递归返回子部门用户。如果应用授权范围内，可递归的部门数量超过500个，接口将会返回错误Code 40162。此时请先使用[获取子部门列表](/document/ukTMukTMukTM/ugzN3QjL4czN04CO3cDN)查询一级子部门，再调用本接口查询部门用户列表。|  

## 响应
### 响应体
|参数|说明|
|-|-|
|code|错误码，非 0 表示失败|
|msg|返回码的描述|
|data|返回业务信息|
|&emsp;∟has_more|分页查询时返回，代表是否还有更多用户|
|&emsp;∟page_token|分页标记，当 has_more 为 true 时，会同时返回该参数，使用该参数作为下一次接口调用参数可以获取部门更多用户，has_more 为 false 时不返回该字段|
|&emsp;∟user_list|用户列表|
|&emsp;&emsp;∟employee_id|用户的 employee_id，申请了"获取用户 user_id"权限后返回该字段|
|&emsp;&emsp;∟open_id|用户的 open_id|
|&emsp;&emsp;∟name|用户名|
|&emsp;&emsp;∟employee_no|工号|
|&emsp;&emsp;∟union_id|用户的 union_id|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "has_more": true,
        "page_token": "763bd1e74d05e95e",
        "user_list": [
            {
                "employee_id": "22996742",
                "open_id": "ou_e03053f0541cecc3269d7a9dc34a0b21",
                "name":"张三1",
                "employee_no":"456234",
                "union_id":"on_c132837f686587dd494aa54f5f65b552"
            },
            {
                "employee_id": "81ab2e27",
                "open_id": "ou_6dfd8d7e5e881bed9be19c043940bf60",
                "name":"张三2",
                "employee_no":"346356",
                "union_id":"on_d12d180970dcc5e89f26929745166fb3"
            },
            {
                "employee_id": "d2e16e41",
                "open_id": "ou_76a95e7f1b7841f74576d2fc00838f47",
                "name":"张三3",
                "employee_no":"214678",
                "union_id":"on_c132837f686587dd494aa54f5f65b552"
            }
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)









