---
document_id: '7055272807039582214'
directory_id: '7050040770682814470'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/overview
breadcrumb:
- Server API
- Contacts
- User group
- User group member
- Overview
document_type: GuideDocumentType
updated_at: 2022-03-09T02:02:31Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/overview
---

#  用户组成员概述
##  资源定义
用户组成员可以是组织架构中的用户或者部门（暂时只支持用户），通过用户组成员可以定义用户组中包括哪些用户/部门。


## 用户组成员字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >member_type</md-text> | <md-text type="field-type" >string</md-text> | 用户组成员类型<br>**member_type支持的类型**：<br>- user：用户<br>- department：部门，即将开放支持<br>**示例值**："user" |
| <md-text type="field-name" >member_id</md-text> | <md-text type="field-type" >string</md-text> | 成员的 ID<br>**member_id描述**：<br>- member_type=user 时，支持用户 ID，用户 ID 类型可选为：open_id、union_id、user_id（[查看ID类型描述](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/field-overview)）<br>**示例值**："ou_7dab8a3d3cdcc9da365777c7ad535d62" |


## 数据示例


```json 
{
   "member_id": "u287xj12",
   "member_type": "user"
}
 
``` 

