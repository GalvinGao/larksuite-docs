---
document_id: '6967331158355886086'
directory_id: '6935677787475738626'
title: 数据校验使用指南（含下拉列表）
full_path: /ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/datavalidation-guide
breadcrumb:
- Server API
- Docs
- Sheets
- Sheet - Data Validation
- Data Validation User Guide (Drop-down List Included)
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:44Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/datavalidation-guide
---

# 应用场景
使用数据验证来限制数据类型或用户输入单元格的值。 目前支持下拉列表的数据验证

## 1 下拉列表
下拉列表功能是数据验证/有效性功能的一种，通过设置下拉列表，可以限制单元格中输入的内容必须为指定的值，从而起到保证数据准确性和规范性的作用。

下拉列表也满足了信息整理中快速打标签的需求，开启选项颜色，可以使得下拉选项更具区分度和美观度；还可选择单选或多选类型，适应多样场景。如以下"性别", "能力属性"列分别使用了下拉列表的单选和多选。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7d6e6ddb3e5e187b32e3c64b2d62ccd4_x8I164hQB9.png)

### 1.1 下拉列表支持接口
（1）[设置下拉列表](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/set-dropdown)：实现对指定范围的单元格设置下拉列表校验规则。

（2）[删除指定范围单元格下拉列表](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/delete-datavalidation)：实现移除指定范围内的下拉列表校验规则。

（3）[更新下拉列表校验规则](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/update-datavalidation) :实现对指定下拉列表规则的属性更新。

（4）[查询指定范围内的下拉列表规则](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/query-datavalidation)：查询指定范围内的下拉列表规则。


