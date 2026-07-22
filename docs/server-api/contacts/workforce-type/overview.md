---
document_id: '7055272807039713286'
directory_id: '6986153129993945093'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/overview
breadcrumb:
- Server API
- Contacts
- Workforce type
- Overview
document_type: GuideDocumentType
updated_at: 2022-03-09T02:02:32Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/overview
---

#  人员类型 Workforce Type 资源概述
##  资源定义
一种特殊的用户属性字段，字段取值格式为单选选项，用于标记用户的身份类型。

默认选项为：正式、实习、外包、劳务、顾问。
<br>企业可基于自身管理诉求增加自定义选项，每个用户只能引用一个选项。


##  字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >enum_id</md-text> | <md-text type="field-type" >string</md-text> | 枚举值id，**由系统自动生成**<br>**示例值**："exGeIjow7zIqWMy+ONkFxA==" |
| <md-text type="field-name" >enum_value</md-text> | <md-text type="field-type" >string</md-text> | 枚举值<br>**示例值**："2"<br>**数据校验规则**：<br>- 长度范围：`1` ～ `100` 字符 |
| <md-text type="field-name" >content</md-text> | <md-text type="field-type" >string</md-text> | 枚举内容<br>**示例值**："专家"<br>**数据校验规则**：<br>- 长度范围：`1` ～ `100` 字符 |
| <md-text type="field-name" >enum_type</md-text> | <md-text type="field-type" >int</md-text> | 类型<br>**示例值**：2<br>**可选值有**：<br>- `1`：内置类型<br>- `2`：自定义 |
| <md-text type="field-name" >enum_status</md-text> | <md-text type="field-type" >int</md-text> | 使用状态<br>**示例值**：1<br>**可选值有**：<br>- `1`：激活<br>- `2`：未激活 |
| <md-text type="field-name" >i18n_content</md-text> | <md-text type="field-type" >i18n_content\[\]</md-text> | i18n定义 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >locale</md-text> | <md-text type="field-type" >string</md-text> | 语言<br>**示例值**："zh_cn" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text> | <md-text type="field-type" >string</md-text> | i18n内容<br>**示例值**："专家" |


###  数据示例
```json
{
    "enum_id": "exGeIjow7zIqWMy+ONkFxA==",
    "enum_value": "2",
    "content": "专家",
    "enum_type": 2,
    "enum_status": 1,
    "i18n_content": [
    	{
            "locale": "zh_cn",
            "value": "专家"
        }
    ]
}
```
