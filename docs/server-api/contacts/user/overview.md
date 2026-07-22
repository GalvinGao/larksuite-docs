---
document_id: '7028557407366414341'
directory_id: '6920532209305042945'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/field-overview
breadcrumb:
- Server API
- Contacts
- User
- Overview
document_type: GuideDocumentType
updated_at: 2023-05-15T02:37:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/field-overview
---

#  用户 User 资源概述
##  资源定义
Lark某个企业里的一个用户。

##  字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 租户内用户的唯一标识，该值可以自定义也可以由系统随机生成。详细定义参见[什么是 User ID](/document/home/user-identity-introduction/user-id)；获取方式参见[如何获得 User ID、Open ID 和 Union ID](/document/home/user-identity-introduction/how-to-get)<br>**自定义`user_id`数据校验规则**：<br>- 最大长度：`64` 字符<br>- 正则校验：不能包含空格字<br>**示例值**："u273y71"<br>**字段权限要求**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户 user ID</md-perm> |
| <md-text type="field-name" >open_id</md-text> | <md-text type="field-type" >string</md-text> | 租户内用户的唯一标识，详细定义参见[什么是 Open ID](/document/home/user-identity-introduction/open-id)；获取方式参见[如何获得 User ID、Open ID 和 Union ID](/document/home/user-identity-introduction/how-to-get)<br>**示例值**："ou_7dab8a3d3cdcc9da365777c7ad535d62" |
| <md-text type="field-name" >union_id</md-text> | <md-text type="field-type" >string</md-text> | 租户内用户的唯一标识，详细定义参见[什么是 Union ID](/document/home/user-identity-introduction/introduction)；获取方式参见[如何获得 User ID、Open ID 和 Union ID](/document/home/user-identity-introduction/how-to-get)<br>**示例值**："on_cad4860e7af114fb4ff6c5d496d1dd76" |
| <md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 用户名<br>**示例值**："张三"<br>**数据校验规则**：<br>- 最小长度：`1` 字符<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户基本信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| <md-text type="field-name" >en_name</md-text> | <md-text type="field-type" >string</md-text> | 英文名<br>**示例值**："San Zhang"<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户基本信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| <md-text type="field-name" >email</md-text> | <md-text type="field-type" >string</md-text> | 邮箱<br>**示例值**："zhangsan@gmail.com"<br>**字段权限要求**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户邮箱信息</md-perm> |
| <md-text type="field-name" >mobile</md-text> | <md-text type="field-type" >string</md-text> | 手机号<br>**示例值**："13011111111"<br>**字段权限要求**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户手机号</md-perm> |
| <md-text type="field-name" >mobile_visible</md-text> | <md-text type="field-type" >boolean</md-text> | 手机号码可见性，true 为可见，false 为不可见，目前默认为 true。不可见时，组织员工将无法查看该员工的手机号码<br>**示例值**：false |
| <md-text type="field-name" >gender</md-text> | <md-text type="field-type" >int</md-text> | 性别<br>**示例值**：1<br>**可选值有**：<br>- `0`：保密<br>- `1`：男<br>- `2`：女<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户性别</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| <md-text type="field-name" >avatar_key</md-text> | <md-text type="field-type" >string</md-text> | 头像的文件Key，可通过“消息与群组/消息/图片信息”中的“上传图片”接口上传并获取头像文件 Key<br>**示例值**："2500c7a9-5fff-4d9a-a2de-3d59614ae28g" |
| <md-text type="field-name" >department_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 用户所属部门的ID列表<br>**示例值**：od-4e6ac4d14bcd5071a37a39de902c7141 |
| <md-text type="field-name" >leader_user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的直接主管的用户ID<br>**示例值**："ou_7dab8a3d3cdcc9da365777c7ad535d62" |
| <md-text type="field-name" >city</md-text> | <md-text type="field-type" >string</md-text> | 城市<br>**示例值**："杭州"<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户雇佣信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| <md-text type="field-name" >country</md-text> | <md-text type="field-type" >string</md-text> | 国家或地区缩写Code，[具体对应列表](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/country-code-description)<br>**示例值**："CN"<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户雇佣信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| <md-text type="field-name" >work_station</md-text> | <md-text type="field-type" >string</md-text> | 工位<br>**示例值**："北楼-H34"<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户雇佣信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| <md-text type="field-name" >join_time</md-text> | <md-text type="field-type" >int</md-text> | 入职时间<br>**示例值**：2147483647<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户雇佣信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| <md-text type="field-name" >employee_no</md-text> | <md-text type="field-type" >string</md-text> | 工号<br>**示例值**："1"<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户雇佣信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >int</md-text> | 员工类型，可选值有：<br>- 1：正式员工<br>- 2：实习生<br>- 3：外包<br>- 4：劳务<br>- 5：顾问<br>同时可读取到自定义员工类型的 int 值，可通过下方接口获取到该租户的自定义员工类型的名称<br>[获取人员类型](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/list)<br>**示例值**：1 |
| <md-text type="field-name" >orders</md-text> | <md-text type="field-type" >user_order\[\]</md-text> | 用户排序信息 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >department_id</md-text> | <md-text type="field-type" >string</md-text> | 排序信息对应的部门ID<br>**示例值**："od-4e6ac4d14bcd5071a37a39de902c7141" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_order</md-text> | <md-text type="field-type" >int</md-text> | 用户在其直属部门内的排序，数值越大，排序越靠前<br>**示例值**：100 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >department_order</md-text> | <md-text type="field-type" >int</md-text> | 用户所属的多个部门间的排序，数值越大，排序越靠前<br>**示例值**：100 |
| <md-text type="field-name" >custom_attrs</md-text> | <md-text type="field-type" >user_custom_attr\[\]</md-text> | 自定义字段<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户雇佣信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >type</md-text> | <md-text type="field-type" >string</md-text> | 自定义字段类型<br>- `TEXT`<br>- `HREF`<br>- `ENUMERATION`<br>- `PICTURE_ENUM`<br>- `GENERIC_USER`<br>**示例值**："TEXT" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 自定义字段ID<br>**示例值**："DemoId" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text> | <md-text type="field-type" >user_custom_attr_value</md-text> | 自定义字段取值, [相关问题](/document/ugTN1YjL4UTN24CO1UjN/uQzN1YjL0cTN24CN3UjN) |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >text</md-text> | <md-text type="field-type" >string</md-text> | 字段类型为 TEXT 时该参数定义字段值，字段类型为 HREF 时该参数定义网页标题<br>**示例值**："DemoText" |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >url</md-text> | <md-text type="field-type" >string</md-text> | 字段类型为 HREF 时，该参数定义默认 URL<br>**示例值**："http://www.larksuite.com" |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_url</md-text> | <md-text type="field-type" >string</md-text> | 字段类型为 HREF 时，该参数定义PC端 URL<br>**示例值**："http://www.larksuite.com" |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >option_id</md-text> | <md-text type="field-type" >string</md-text> | 字段类型为 ENUMERATION 或 PICTURE_ENUM 时，该参数定义选项值<br>**示例值**："edcvfrtg" |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >generic_user</md-text> | <md-text type="field-type" >custom_attr_generic_user</md-text> | 字段类型为 GENERIC_USER 时，该参数定义引用人员 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 用户的user_id<br>**示例值**："9b2fabg5" |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >type</md-text> | <md-text type="field-type" >int</md-text> | 用户类型    1：用户<br>**示例值**：1 |
| <md-text type="field-name" >enterprise_email</md-text> | <md-text type="field-type" >string</md-text> | 企业邮箱，请先确保已在管理后台启用Lark邮箱服务<br>**示例值**："demo@mail.com"<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户雇佣信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| <md-text type="field-name" >job_title</md-text> | <md-text type="field-type" >string</md-text> | 职务<br>**示例值**："xxxxx"<br>**字段权限要求（满足任一）**：<br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户雇佣信息</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm> |
| <md-text type="field-name" >need_send_notification</md-text> | <md-text type="field-type" >boolean</md-text> | 是否发送提示消息<br>**示例值**：false |


###  数据示例
```json
{
    "user_id": "u273y71",
    "open_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
    "union_id": "ou_be2c1742f2bb189469bdd33f0b1516ea",
    "name": "张三",
    "en_name": "San Zhang",
    "email": "zhangsan@gmail.com",
    "mobile": "13011111111",
    "mobile_visible": false,
    "gender": 1,
    "avatar_key": "2500c7a9-5fff-4d9a-a2de-3d59614ae28g",
    "department_ids": [
        "od-4e6ac4d14bcd5071a37a39de902c7141"
    ],
    "leader_user_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
    "city": "杭州",
    "country": "CN",
    "work_station": "北楼-H34",
    "join_time": 2147483647,
    "employee_no": "1",
    "employee_type": 1,
    "orders": [
        {
            "department_id": "od-4e6ac4d14bcd5071a37a39de902c7141",
            "user_order": 100,
            "department_order": 100
        }
    ],
    "custom_attrs": [
        {
            "type": "TEXT",
            "id": "DemoId",
            "value": {
                "text": "DemoText",
                "url": "http://www.larksuite.com",
                "pc_url": "http://www.larksuite.com",
                "option_id": "edcvfrtg",
                "generic_user": {
                    "id": "9b2fabg5",
                    "type": 1
                }
            }
        }
    ],
    "enterprise_email": "demo@mail.com",
    "job_title": "xxxxx",
    "need_send_notification": false,
    "notification_option": {
        "channels": [
            "sms"
        ],
        "language": "zh-CN"
    }
}
```

## 用户ID说明
了解`user_id`,`open_id`,`union_id`的区别和用途，参见教程 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)
