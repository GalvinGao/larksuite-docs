---
document_id: '7028557407366365189'
directory_id: '6927521875715244034'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview
breadcrumb:
- Server API
- Contacts
- Department
- Overview
document_type: GuideDocumentType
updated_at: 2022-03-09T02:02:27Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview
---

#  部门 Department 资源概述
##  资源定义
Lark某个企业里的组织架构树上的一个节点。

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
	<md-text type="field-name" >department_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>

	<md-td>
	标识租户内一个唯一的部门，支持自定义或默认生成。详细说明参见 [部门ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)

**示例值**："h121921"

**数据校验规则**：

- 最大长度：`64` 字符

- 正则校验：`^0|[^od][A-Za-z0-9]*`
	</md-td>
</md-tr>
    
    <md-tr>
	<md-td>
	<md-text type="field-name" >open_department_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>

	<md-td>
	在某个应用中标识一个部门，同一个 `department_id` 在不同应用中的 `open_department_id` 不相同。详细说明参见 [部门ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)

**示例值**："od-4e6ac4d14bcd5071a37a39de902c7141"

	</md-td>
</md-tr>
<md-tr>
	<md-td>
	<md-text type="field-name" >name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>

	<md-td>
	部门名称

**示例值**："DemoName"

**数据校验规则**：

- 最小长度：`1` 字符

**字段权限要求（满足任一）**：
 <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门基础信息</md-perm>
 <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >i18n_name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >department_i18n_name</md-text>
	</md-td>

	<md-td>
	国际化的部门名称

**字段权限要求（满足任一）**：
 <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门基础信息</md-perm>
 <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >zh_cn</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>

	<md-td>
	部门的中文名

**示例值**："Demo名称"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ja_jp</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>

	<md-td>
	部门的日文名

**示例值**："デモ名"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >en_us</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>

	<md-td>
	部门的英文名

**示例值**："Demo Name"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >parent_department_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>

	<md-td>
	父部门的ID。创建根部门，该参数值为 “0”

**示例值**："od-4e6ac4d14bcd5071a37a39de902c7141"
	</md-td>
</md-tr>





<md-tr>
	<md-td>
	<md-text type="field-name" >leader_user_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>

	<md-td>
	部门主管用户ID

**示例值**："ou_7dab8a3d3cdcc9da365777c7ad535d62"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >order</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>

	<md-td>
	部门的排序，即部门在其同级部门的展示顺序

**示例值**："100"

**字段权限要求（满足任一）**：
 <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门组织架构信息</md-perm>
 <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >unit_ids</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-td>

	<md-td>
	部门单位自定义ID列表，当前只支持一个

**示例值**：custom_unit_id

**字段权限要求（满足任一）**：
 <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门组织架构信息</md-perm>
 <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份读取通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >create_group_chat</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>

	<md-td>
	是否创建部门群，默认不创建

**示例值**：false
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::
###  数据示例
```json
{
    "department_id": "h121921",
    "open_department_id": "od-4e6ac4d14bcd5071a37a39de902c7141",
    "name": "DemoName",
    "i18n_name": {
        "zh_cn": "Demo名称",
        "ja_jp": "デモ名",
        "en_us": "Demo Name"
    },
    "parent_department_id": "od-4e6ac4d14bcd5071a37a39de902c7141",    
    "leader_user_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
    "order": "100",
    "unit_ids": [
        "custom_unit_id"
    ],
    "create_group_chat": false
}
```
##  部门 ID 说明
### 什么是 department_id

`department_id` 用来标识租户内一个唯一的部门，支持在创建部门时自定义。若不自定义则由系统默认生成唯一的 `department_id`。

已经创建的部门，不允许修改 `department_id`。

:::note
你可以将你企业内部系统已有的部门唯一标识写入到Lark的部门 ID 中，由此实现Lark部门 ID 和内部系统部门 ID 的一致性，节省跨系统调用的映射成本。
:::

###  什么是 open_department_id
`open_department_id` 用来在具体某个应用中标识一个部门，同一个`department_id` 在不同应用中的 `open_department_id` 不相同。


### 如何自定义 department_id

在创建部门时，通过 department_id 参数写入自定义部门ID，数据校验规则为：

-   最大长度：`64` 字符
-   正则校验：`^0|[^od][A-Za-z0-9]*`



### 如何获取 department_id

#### 获取建议
-   如果你的部门 ID 是由你自主定义的，建议本地维护一份部门 ID List，以方便自己查阅使用。
-   如果你没有维护部门 ID 数据，可以寻求租户管理员帮助，由管理员在租户管理后台查询部门ID。


#### 租户管理员查询部门 ID 方法

拥有“成员和部门”管理权限的租户管理员，可查询其管理范围内的部门ID。

- 方法1：单个查询部门 ID

操作路径：企业管理后台中 [组织架构/成员与部门](http://www.larksuite.com/admin/contacts/departmentanduser)，点击左侧部门树中的某个部门右侧“...”，进入编辑抽屉


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7dc657b0da3d119318bd4958a18114c3_0j1Rfa21uQ.png?lazyload=true&width=1640&height=904)

- 方法2：批量导出部门 ID

操作路径：企业管理后台中 [组织架构/成员与部门/管理部门/批量导入](http://www.larksuite.com/admin/contacts/departmentanduser/import-department)



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/33159c1355395d55b62c0e2431bc0e8c_9eZYvog1z1.png?lazyload=true&width=1631&height=1079)
