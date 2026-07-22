---
document_id: '6965400907875172358'
directory_id: '6907567266537242625'
title: 获取部门详情
full_path: /ukTMukTMukTM/uAzNz4CM3MjLwczM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Department
- Obtain Department Details
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:44Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uAzNz4CM3MjLwczM
---

# 获取部门详情
:::html

<md-alert type="error">

为了更好地提升该接口的安全性，我们对其进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/get)

</md-alert>

:::
该接口用于获取部门详情信息。

- 调用该接口需要具有`以应用身份访问通讯录` 。应用需要拥有待查询部门的通讯录授权。



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v1/department/info/get?open_department_id=od-c042a4980ba8e1466050e3e8da2378fe <br> or https://open.larksuite.com/open-apis/contact/v1/department/info/get?department_id=TT-1234 |
| HTTP Method | GET |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份访问通讯录 </md-perm> |
| 字段权限要求<br><md-tooltip type="info">接口返回的部分字段受权限控制，开启字段权限才可获取对应字段数据；如无需获取这些字段，则无需开启。</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">根据要获取的字段开启相应权限</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户 user ID</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门基础信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门组织架构信息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |

### 查询参数  
|参数|类型|必须|说明|
|-|-|-|-|
|department_id、open_department_id|string|是|请求部门的自定义 ID 或 openID, 二者传其一即可，同时传两个参数优先使用 openID|

## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|错误码，非 0 表示失败|
|msg|string|返回码的描述|
|data|-|返回业务信息|
|&emsp;∟department_info|-|部门信息|
|&emsp;&emsp;∟id|string|部门自定义 ID|
|&emsp;&emsp;∟i18n_name|-|国际化的部门名称|
|&emsp;&emsp;&emsp;∟en_us|-|部门英文名|
|&emsp;&emsp;&emsp;∟ja_jp|-|部门日文名|
|&emsp;&emsp;&emsp;∟zh_cn|-|部门中文名|
|&emsp;&emsp;∟open_department_id|string|部门 openID|
|&emsp;&emsp;∟name|string|部门名称|
|&emsp;&emsp;∟chat_id|string|部门群ID|
|&emsp;&emsp;∟member_count|int|部门成员数量|
|&emsp;&emsp;∟parent_id|string|父部门自定义 ID|
|&emsp;&emsp;∟parent_open_department_id|string|父部门 openID|
|&emsp;&emsp;∟status|int|部门状态，0 无效，1 有效|
|&emsp;&emsp;∟leader_employee_id|string|部门负责人 employee_id，申请了 "获取用户 user_id"权限的应用返回该字段| 
|&emsp;&emsp;∟leader_open_id|string|部门负责人 open_id| 
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "department_info": {
            "id":"TT-1234",
            "i18n_name":{
                "en_us":"",
                "ja_jp":"",
                "zh_cn":""
            }
            "open_department_id": "od-c042a4980ba8e1466050e3e8da2378fe",
            "leader_employee_id":"612a67ef",
            "leader_open_id":"ou_05065996251935ada9c2b0ecc50be91e",
            "chat_id": "oc_405333f8fc89c3262865b014ccbbb274",
            "member_count": 79,
            "name": "市场部",
            "parent_id": "0",
            "parent_open_department_id": "0"
            "status": 1
        }
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)



