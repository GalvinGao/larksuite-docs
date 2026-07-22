---
document_id: '7055272807039729670'
directory_id: '7050040770682830854'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/overview
breadcrumb:
- Server API
- Contacts
- User group
- Overview
document_type: GuideDocumentType
updated_at: 2022-03-09T02:02:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/overview
---

#  用户组资源概述
##  资源定义
Lark组织架构里的一个基础实体，用户组可关联用户/部门，同时可被各类业务权限管控引用，从而实现“高效便捷”的人群管控。

举例：通过 OpenAPI 将“外包人员”维护到”外包用户组“中，在应用可用范围或其他业务管控规则中引用”外包用户组“，可节省在各业务权限里手动配置外包人员的成本。

<br>如下管控场景已支持按用户组管控：
1. 管理员对用户组设置应用可用性
2. 管理员对用户组设置工作台推荐应用
3. 管理员基于用户组设置组织架构可见范围
4. 管理员基于用户组设置个人名片页字段可见性
5. 管理员基于用户组设置组织内会话权限
6. 管理员对用户组设置文档禁用权限
7. 管理员对用户组设置禁止对外沟通
8. 管理员基于用户组设置文档权限默认值

## 用户组字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >group_id</md-text> | <md-text type="field-type" >string</md-text> | 租户内用户组的唯一标识，可在创建用户组时自定义，不自定义则由系统自动生成，已创建用户组不允许修改 group_id 。<br>**自定义`group_id`数据校验规则**：<br>- 最大长度：`64` 字符<br>- 校验规则：数字、大小写字母的组合，不能包含空格<br>**示例值**："g122817" |
| <md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 用户组的名字，企业内唯一<br>- 最大长度：`100` 字<br>- 校验规则：1-100 个字，企业内唯一<br>**示例值**："IT 外包组" |
| <md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 用户组描述信息<br>- 最大长度：`500` 字<br>- 校验规则：0-500<br>**示例值**："IT 外包用户组，需要进行细粒度权限管控" |
| <md-text type="field-name" >type</md-text> | <md-text type="field-type" >int</md-text> | 用户组的类型<br>- option字段<br>可选值有：<br>1：普通用户组<br>- 默认值：1<br>- 说明：类型字段的定义，预留未来扩展 |



## 用户组 ID 说明
用户组 ID `group_id` 用来标识租户内一个唯一的用户组，支持在创建用户组时自定义，若不自定义则由系统默认生成。已经创建的用户组不允许修改 `group_id`。

如果你的企业内部系统已有类似“用户组”的实体并且希望同步到Lark实现一些业务权限管控，你可以在调用创建用户组接口时将本企业内部系统已有的用户组唯一标识写入到Lark的 `group_id` 中，由此实现Lark用户组 ID 和内部系统“用户组” ID 的一致性，节省跨系统调用的映射成本。

## 如何获取用户组 ID
你有两种方法可以获取用户组 ID：
1. 通过 **查询用户组信息接口** 获取用户组 ID
2. 通过企业管理员在管理后台查询用户组 ID，具体查询路径为：**管理后台/组织架构/用户组管理**，点击 **用户组详情** 即可查看。


## 数据示例


```json 
  {
        "id": "g193821",
        "name": "IT 外包组",
        "description": "IT 外包组，需要对该组人群进行细颗粒度权限管控。",
        "member_user_count": 2,
        "member_department_count": 0
  }
``` 

