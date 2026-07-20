---
document_id: '7055272807039746054'
directory_id: '6982479736958484486'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/custom_attr/overview
breadcrumb:
- Server API
- Contacts
- Custom user fields
- Overview
document_type: GuideDocumentType
updated_at: 2022-03-09T02:02:32Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/custom_attr/overview
---

#  自定义用户字段 Custom user fields 资源概述

##  资源定义
由企业自定义的用户字段，用于不同企业基于自身管理诉求灵活表征用户信息，自定义字段由[企业管理员](https://www.larksuite.com/hc/zh-CN/articles/360048488029)在[企业管理后台 - 组织架构 - 成员字段管理](https://www.larksuite.com/admin/contacts/employee-field-new/custom) 里进行创建、更新操作，可以通过接口可获取企业的所有自定义字段。
  

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e6800efa88ea61103ac231e092ce52f9_z0kACWCTj9.png?lazyload=true&width=1640&height=776)
  


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/69c13775666697ce31b179fb2d0bb659_7tb2K7PuJE.png?lazyload=true&width=1640&height=933)


##  字段说明
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: 40%;">名称</md-th>
      <md-th style="width: 20%;">类型</md-th>
      <md-th style="width: 30%;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >items</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >custom_attr\[\]</md-text>
	</md-td>
	<md-td>
	自定义字段定义
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	自定义字段id
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	自定义字段类型，可选值有:
- `TEXT`：纯文本，用于纯文本描述人员，如备注
- `HREF`：静态 URL，用于人员 Profile 跳转链接
- `ENUMERATION`：枚举，用于结构化描述人员，如民族
- `GENERIC_USER`：用户，用于描述人和人关系，如 HRBP
- `PICTURE_ENUM`：枚举图片，以结构化的图片描述人员，如在人员 Profile 展示荣誉徽章
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >options</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >custom_attr_options</md-text>
	</md-td>
	<md-td>
	选项定义，当type为`ENUMERATION`或者`PICTURE_ENUM`时此项有值，列举所有可选项
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >default_option_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	默认选项id
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >option_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	选项类型

**可选值有**：
- `TEXT`：文本选项
- `PICTURE`：图片选项
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >options</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >custom_attr_option\[\]</md-text>
	</md-td>
	<md-td>
	选项列表
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	枚举类型选项id
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	枚举选项值，当option_type为`TEXT`为文本值，当option_type为`PICTURE`时为图片链接
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	名称，仅option_type为PICTURE时有效
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >i18n_content\[\]</md-text>
	</md-td>
	<md-td>
	自定义字段的字段名称
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >locale</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	语言版本
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	字段名
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >page_token</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >has_more</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	是否还有更多项
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::

###  数据示例
```json
 "items": [
     {
		"id": "C-6965457429001748507",
		"type": "TEXT",
		"options": {
			"default_option_id": "qasdefgr",
			"option_type": "TEXT",
			"options": [
				{
					"id": "qasdefgr",
					"value": "Option",
					"name": "Name"
				}
			]
		},
		"i18n_name": [
			{
				"locale": "zh_cn",
				"value": "专家"
			}
		]
    }
]
```
