---
document_id: '6965400907878105093'
directory_id: '6907567266537242625'
title: 更新部门信息
full_path: /ukTMukTMukTM/uczNz4yN3MjL3czM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Department
- Update Department Information
document_type: GuideDocumentType
updated_at: 2022-03-11T11:45:02Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uczNz4yN3MjL3czM
---

# 更新部门信息
:::html

<md-alert type="error">

为了更好地提升该接口的安全性，我们对其进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/update)

</md-alert>

:::
该接口用于更新通讯录中部门的信息。<br>

- 调用该接口需要申请`更新通讯录`以及`以应用身份访问通讯录`权限。调用该接口需要具有该部门以及更新操作涉及的部门的通讯录权限。应用商店应用无权限调用此接口。


## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v1/department/update |
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
|name|string|否|部门名称|
|parent_id|string|否|父部门 ID，当前应用需要具有新父部门的通讯录权限|
|id|string|是|部门 ID|
|leader_employee_id、leader_open_id|string|否|部门领导 ID，支持通过 leader_employee_id 或者 leader_open_id 更新部门领导，请求同时传递两个参数时按 leader_employee_id 处理|
|create_group_chat|bool|否|是否创建部门群。当部门已存在部门群时，忽略该参数|
### 请求体示例
```json
{
    "name":"市场部",
    "parent_id":"od-455efa262dc736b3e45a8b17fe945293",
    "id":"od-32410d0320c93a34c1a2bb1a20340825",
    "leader_employee_id":"2fab234c",
    "leader_open_id":"ou_4a2eb24a52b27c0b7fc6fd04162c0246",
    "create_group_chat":true
}
```
## 响应

### 响应体 
|参数|说明|
|-|-|
|code|返回码，非 0 表示失败|
|msg|对返回码的文本描述|
|data|返回内容|
|&emsp;∟department_info    |更新后的部门信息|
|&emsp;&emsp;∟chat_id|部门群 ID，如果存在部门群，返回该字段|
|&emsp;&emsp;∟id|部门 ID|
|&emsp;&emsp;∟parent_id|父部门 ID|
|&emsp;&emsp;∟status|部门状态|
|&emsp;&emsp;∟leader_employee_id|部门领导 employee_id|
|&emsp;&emsp;∟leader_open_id|部门领导 open_id|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "department_info":{
            "chat_id": "oc_a3c1a99d5736e3a269ef7302eb4763ab",
            "id":"od-3c03f7714260801568e82064f4423887",
            "parent_id":"od-c042a4980ba8e1466050e3e8da2378fe",
            "name":"市场部",
            "status":1,
            "leader_employee_id":"2fe234cb",
            "leader_open_id":"ou_e03053f0541cecc3269d7a9dc34a0b21"
        }
    }
}

```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)







