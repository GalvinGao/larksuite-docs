---
document_id: '7491144743687258118'
directory_id: '7273782524454715397'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/passport-v1/session/usum
breadcrumb:
- Server API
- Authenticate and Authorize
- Login state management
- Resource introduction
document_type: GuideDocumentType
updated_at: 2025-04-09T03:07:36Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/passport-v1/session/usum
---

#  资源介绍
##  资源定义
用于标识当前Lark用户的身份、设备及登录状态。

##  字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >Authorization</md-text> | <md-text type="field-type" >string</md-text> | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| <md-text type="field-name" >Content-Type</md-text> | <md-text type="field-type" >string</md-text> | **固定值**："application/json; charset=utf-8" |
| <md-text type="field-name" >user_id_type</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID 类型<br>**示例值**："open_id"<br>**可选值有**：<br>- `open_id`：用户的 open id<br>- `union_id`：用户的 union id<br>- `user_id`：用户的 user id<br>**默认值**：`open_id`<br>**当值为 `user_id`，字段权限要求**：<br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |
| <md-text type="field-name" >user_ids</md-text> | <md-text type="field-type" >string</md-text> | 用户ID​<br>​**示例值**：["47f621ff"]<br>**数据校验规则**：<br>最大长度：`100` |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败​ |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述​ |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \-` |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mask_sessions</md-text> | <md-text type="field-type" >mask_session\[\]</md-text> | 用户登录信息​ |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >create_time</md-text> | <md-text type="field-type" >string</md-text> | 创建时间​ |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >terminal_type</md-text> | <md-text type="field-type" >int</md-text> | 客户端类型<br>**可选值有**：<br>- `0`：未知<br>- `1`：个人电脑<br>- `2`：浏览器<br>- `3`：安卓手机<br>- `4`：Apple手机<br>- `5`：服务端 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户ID​ |

##  数据示例​
```json
{​
    "code": 0,​
    "data": {​
        "mask_sessions": [​
            {​
                "create_time": "1644980493",​
                "terminal_type": 2,​
                "user_id": "47f183f1f1"​
            },​
            {​
                "create_time": "1644983127",​
                "terminal_type": 2,​
                "user_id": "47f183ff1"​
            },​
            {​
                "create_time": "1644983127",​
                "terminal_type": 2,​
                "user_id": "47f183ff2"​
            }​
        ]​
    },​
    "msg": ""​
}
```
  
  
  
  
  
  
    
    
    
    
    
    
