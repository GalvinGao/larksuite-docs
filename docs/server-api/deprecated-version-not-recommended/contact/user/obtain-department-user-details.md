---
document_id: '6965400907875057670'
directory_id: '6907567266541404162'
title: 获取部门用户详情
full_path: /ukTMukTMukTM/uYzN3QjL2czN04iN3cDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- User
- Obtain Department User Details
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:17Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYzN3QjL2czN04iN3cDN
---

# 获取部门用户详情

该接口用于获取部门用户详情信息。<br>


:::html
<md-alert type="warn">
调用该接口需要根据需要返回字段申请对应的[用户数据权限](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)。应用需要有被调用部门的通讯录授权。
</md-alert>
:::


## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v1/department/user/detail/list?open_department_id=od-846fa088e3dcac5d0ce0b4f6b4cde899&page_size=10&fetch_child=true<br>https://open.larksuite.com/open-apis/contact/v1/department/user/detail/list?department_id=TT-1234&page_size=10&fetch_child=true |
| HTTP Method | GET |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 获取部门组织架构信息 </md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 读取通讯录 </md-perm> |
| 字段权限要求<br><md-tooltip type="info">接口返回的部分字段受权限控制，开启字段权限才可获取对应字段数据；如无需获取这些字段，则无需开启。</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">根据要获取的字段开启相应权限</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户 user ID</md-perm></md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户基本信息</md-perm></md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户邮箱信息</md-perm></md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户手机号</md-perm></md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户性别</md-perm></md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户雇佣信息</md-perm></md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户组织架构信息</md-perm></md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 查询参数
|参数|类型|必须|说明|
|-|-|-|-|
|open_department_id、department_id|string|是|部门 ID，可通过部门的 openID 或者自定义 ID 进行请求，同时传部门 openID 和自定义 ID 时忽略自定义 ID|
|page_token|string|否|分页标记，第一次请求不填，表示从头开始遍历；分页查询还有更多成员时会同时返回新的 page_token, 下次遍历可采用该 page_token 获取更多成员|
|page_size|int|是|分页大小，取值范围 1-100|
|fetch_child|bool|否|是否递归返回子部门用户，默认不递归。如果应用授权范围内，可递归的部门数量超过500个，接口将会返回错误Code 40162。此时请先使用[获取子部门列表](/document/ukTMukTMukTM/ugzN3QjL4czN04CO3cDN)查询一级子部门，再调用本接口查询部门用户列表。|  

## 响应
### 响应体

|参数|说明|
|-|-|
|code|返回码，非 0 表示失败|
|msg|返回码的描述|
|data|返回业务数据|
|&emsp;∟has_more|分页查询时返回，代表是否还有更多用户|
|&emsp;∟page_token|分页标记，当 has_more 为 true 时返回该参数，下一次接口调用使用该参数可以获取到当前部门更多用户， has_more 为 false 时不返回|
|&emsp;∟user_infos|用户详情列表|
|&emsp;&emsp;∟name|用户名|
|&emsp;&emsp;∟name_py|用户名拼音|
|&emsp;&emsp;∟en_name|英文名|
|&emsp;&emsp;∟employee_id|用户的 employee_id，申请了"获取用户 user_id"权限后返回|
|&emsp;&emsp;∟employee_no|工号|
|&emsp;&emsp;∟open_id|用户的 open_id|
|&emsp;&emsp;∟union_id|用户的 union_id|
|&emsp;&emsp;∟status|用户状态，bit0(最低位): 1冻结，0未冻结；bit1:1离职，0在职；bit2:1未激活，0已激活|
|&emsp;&emsp;∟employee_type|员工类型。1:正式员工；2:实习生；3:外包；4:劳务；5:顾问|
|&emsp;&emsp;∟avatar_72|用户头像，72*72px|
|&emsp;&emsp;∟avatar_240|用户头像，240*240px|
|&emsp;&emsp;∟avatar_640|用户头像，640*640px|
|&emsp;&emsp;∟avatar_url|用户头像，原始大小|
|&emsp;&emsp;∟gender|性别，未设置不返回该字段。1:男；2:女|
|&emsp;&emsp;∟email|用户邮箱地址，已申请"获取用户邮箱"权限的应用返回该字段|
|&emsp;&emsp;∟mobile|用户手机号，已申请"获取用户手机号"权限的企业自建应用返回该字段|
|&emsp;&emsp;∟description|用户个人签名|
|&emsp;&emsp;∟country|用户所在国家|
|&emsp;&emsp;∟city|用户所在城市|
|&emsp;&emsp;∟work_station|工位|
|&emsp;&emsp;∟is_tenant_manager|是否是企业超级管理员|
|&emsp;&emsp;∟join_time|入职时间，未设置不返回该字段|
|&emsp;&emsp;∟update_time|更新时间|
|&emsp;&emsp;∟leader_employee_id|用户直接领导的 employee_id，申请了"获取用户 user_id"权限返回该字段|
|&emsp;&emsp;∟leader_open_id|用户直接领导的 open_id|
|&emsp;&emsp;∟leader_union_id|用户直接领导的 union_id|
|&emsp;&emsp;∟departments|用户所在部门的自定义 ID，用户可能同时存在于多个部门|
|&emsp;&emsp;∟open_departments|用户所在部门的 openID，用户可能同时存在于多个部门|
|&emsp;&emsp;∟custom_attrs|用户的自定义属性信息。<br>该字段返回的每一个属性包括自定义属性 ID 和自定义属性值。 <br>企业开放了自定义用户属性且为该用户设置了自定义属性的值，才会返回该字段|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "has_more": false,
        "page_token": "763bd1e74d05e95e",
        "user_infos": [
            {
                "name":"zhang san",
                "name_py":"zhang san",
                "en_name":"John",
                "employee_id":"a0615a67",
                "employee_no":"235634",
                "open_id":"ou_e03053f0541cecc3269d7a9dc34a0b21",
                "union_id":"on_7dba11ff38a2119f89349876b12af65c", 
                "status":2,
                "employee_type": 1,
                "avatar_72": "https://sf3-ttcdn-tos.pstatp.com/img/avatar/62db96e8-c5b6-4077-bb9d-2697d65a29eb~72x72.png",
                "avatar_240": "https://sf3-ttcdn-tos.pstatp.com/img/avatar/62db96e8-c5b6-4077-bb9d-2697d65a29eb~240x240.png",
                "avatar_640": "https://sf3-ttcdn-tos.pstatp.com/img/avatar/62db96e8-c5b6-4077-bb9d-2697d65a29eb~640x640.png",
                "avatar_url": "https://sf3-ttcdn-tos.pstatp.com/img/avatar/62db96e8-c5b6-4077-bb9d-2697d65a29eb~noop.png",
                "gender":1,
                "email":"zhangsan@gmail.com",
                "mobile":"+8615343215730",
                "description": "",
                "country": "CN",
                "city":"Beijing",
                "work_station":"Poly, F6-123",  
                "is_tenant_manager":false,
                "join_time":1562342314,
                "update_time":1569140032,
                "leader_employee_id":"a0615a67",
                "leader_open_id":"ou_e03053f0541cecc3269d7a9dc34a0b21",
                "leader_union_id":"on_c132837f686587dd494aa54f5f65b552",
                "departments":[
                    "od-8c6c97ab9a34c1a649001d7ad36b97a7"
                ],
                "open_departments":[
                    "TT-1234"
                ],
                "custom_attrs": {
                    "C-6702376000044400907": {
                        "value": "value1"
                    },
                    "C-6702376000048595214": {
                        "value": "value2"
                    }
                }  
            },
            {
                "name":"li si",
                "name_py":"li si",
                "en_name":"Jack",
                ...
            }
        ]
    }
}

```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
