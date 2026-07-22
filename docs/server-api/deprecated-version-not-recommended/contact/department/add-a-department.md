---
document_id: '6965400907878268933'
directory_id: '6907567266537242625'
title: 新增部门
full_path: /ukTMukTMukTM/uYzNz4iN3MjL2czM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Department
- Add a Department
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:56Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYzNz4iN3MjL2czM
---

# 新增部门
:::html

<md-alert type="error">

为了更好地提升该接口的安全性，我们对其进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/create)

</md-alert>

:::

该接口用于向通讯录中增加新的部门。<br>

- 调用该接口需要申请`更新通讯录`以及`以应用身份访问通讯录`权限。应用需要拥有待新增部门的父部门的通讯录授权。应用商店应用无权限调用接口



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v1/department/add |
| HTTP Method | POST |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 更新通讯录 </md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份访问通讯录（历史版本）</md-perm> |
| 字段权限要求<br><md-tooltip type="info">接口返回的部分字段受权限控制，开启字段权限才可获取对应字段数据；如无需获取这些字段，则无需开启。</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">根据要获取的字段开启相应权限</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户 user ID</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体
|参数|类型|必须|说明|
|-|-|-|-|
|name|string|是|部门名称|
|parent_id|string|否|父部门自定义 ID，选填，parent_id、parent_open_department_id至少指定其中之一；**同时传两个参数，优先使用parent_open_department_id**|
|parent_open_department_id|string|否|父部门 openID，选填，parent_id、parent_open_department_id至少指定其中之一；**同时传两个参数，优先使用parent_open_department_id**|
|id|string|否|自定义部门 ID。<br>该字段企业内必须唯一，不区分大小写，长度为 1 ~ 64 个字符。只能由数字、字母和“_-@.”四种字符组成，且第一个字符必须是数字或字母。<br>**创建部门时指定的 ID 后续不允许修改，不指定该字段由系统自动生成 ID，系统生成的 ID 仅允许修改一次。**
|leader_employee_id、leader_open_id|string|否|部门领导 ID，支持通过 leader_employee_id 或者 leader_open_id 设置部门领导，**请求同时传递两个参数时按 leader_employee_id 处理**|
|create_group_chat|bool|否|是否同时创建部门群，默认不创建部门群|
### 请求体示例
```json
{
    "name":"市场部",
    "parent_id":"D_marketing",
    "parent_open_department_id": "od-455efa262dc736b3e45a8b17fe945293",
    "id":"tt_123456",
    "leader_employee_id":"2fab234c",
    "leader_open_id":"ou_4a2eb24a52b27c0b7fc6fd04162c0246",
    "create_group_chat":true
}
```



## 响应 
### 响应体

|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|对返回码的文本描述|
|data|-|返回内容|
|&emsp;∟department_info    |-|部门信息|
|&emsp;&emsp;∟chat_id|string|部门群 ID，如果存在部门群，返回该字段|
|&emsp;&emsp;∟id|string|部门自定义 ID|
|&emsp;&emsp;∟parent_id|string|父部门自定义 ID|
|&emsp;&emsp;∟status|int|部门状态|
|&emsp;&emsp;∟leader_employee_id|string|部门领导 employee_id，申请了"获取用户 user_id"权限的应用返回该字段|
|&emsp;&emsp;∟leader_open_id|string|部门领导 open_id|
|&emsp;&emsp;∟open_department_id|string|部门 open_id|
|&emsp;&emsp;∟parent_open_department_id|string|父部门 open_id|


### 响应体示例  
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "department_info":{
            "chat_id": "oc_a3c1a99d5736e3a269ef7302eb4763ab",
            "id":"tt_123456",
            "parent_id":"D_marketing",
            "name":"市场部",
            "status":1,
            "leader_employee_id":"2fe234cb",
            "leader_open_id":"ou_e03053f0541cecc3269d7a9dc34a0b21",
            "open_department_id":"od-3c03f7714260801568e82064f4423887",
            "parent_open_department_id":"od-455efa262dc736b3e45a8b17fe945293"
        }
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)



