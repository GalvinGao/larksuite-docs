---
document_id: '7543849381893737905'
directory_id: '7530974965350875142'
title: 更新员工信息
full_path: /uAjLw4CM/ukTMukTMukTM/directory-v1/employee/patch
breadcrumb:
- Server API
- Organization
- Employee management
- Update employee information
document_type: ReferenceDocumentType
updated_at: 2025-08-29T03:47:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/patch
---

# 更新员工信息

本接口用于更新在职/离职员工的信息、冻结/恢复员工。未传递的参数不会进行更新。
员工指Lark企业内身份为「Employee」的成员，等同于通讯录OpenAPI中的「User」。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=directory&version=v1&resource=employee&method=patch)

:::html
<md-alert type="tip">
- 员工状态的修改遵循生命周期流转的规则，具体规则详见 [Directory-员工管理-资源介绍](/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/resources-introduction) 。
- 本接口支持tenant_access_token和user_access_token，接口获取方式参考[获取访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)
。
  - 使用tenant_access_token时，只能在当前应用的通讯录授权范围内更新员工信息。可在开发者后台 > 权限管理 > 通讯录权限 中查看。
    - 当变更员工的部门信息时，应用需要有变更前后的部门权限，才能变更成功。
  - 使用user_access_token 时，默认为管理员用户，将校验管理员管理范围。当用户有多个管理员身份均可更新员工信息时，管理员管理范围取最大集。管理员权限可查看帮助中心文档：[管理员创建管理员角色及分配权限](https://www.larksuite.com/hc/zh-CN/articles/360043595213)
- 变更「未加入」、「未激活」状态的员工的联系手机号、工作邮箱，会修改员工的登录凭证，并将员工重置为「未加入」状态，并发送邀请短信/邮件。其他状态的员工修改联系方式不影响登录凭证。
- 修改员工ID（employee_id）需要悉知以下影响：
  - 员工ID（employee_id）是员工在企业内的唯一ID，可能会被应用引用来实现各种内部逻辑，唯一ID修改之后可能会导致引用失败，导致所有引用且保存了‘被修改 ID 员工’的业务全部受影响。
- 更新离职状态的员工信息时，以下字段不可更新：
  - email、mobile、department_ids、leader_id、is_frozen、work_city_id
</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="error">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/directory/v1/employees/:employee_id |
| HTTP Method | PATCH |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="directory:employee.update:write" desc="更新员工" support_app_types="custom" tags="">更新员工</md-perm><br><md-perm name="directory:employee:write" desc="创建、更新、离职、恢复员工" support_app_types="custom" tags="">创建、更新、离职、恢复员工</md-perm> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="directory:employee.base.external_id:read" desc="查看员工自定义 ID" support_app_types="custom,isv" tags="">查看员工自定义 ID</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 路径参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >employee_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	员工ID，与employee_id_type类型保持一致。

**示例值**："eehsdna"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 查询参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;" >描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >employee_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户 ID 类型

**示例值**：open_id

**可选值有**：
<md-enum>
<md-enum-item key="open_id" >标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)</md-enum-item>
<md-enum-item key="union_id" >标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)</md-enum-item>
<md-enum-item key="employee_id" >企业内在职员工的唯一标识。支持自定义，未自定义时系统自动生成。ID支持修改。
获取employee_id的方式：
  - 企业管理员在 管理后台 > 组织架构 > 成员与部门 页面，点击 成员详情，查询员工ID
  - 通过 [批量获取员工列表](/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/filter) 的接口，通过手机号或邮箱查询员工ID。</md-enum-item>
</md-enum>

**默认值**：`open_id`

**当值为 `employee_id`，字段权限要求**：
<md-perm name="directory:employee.base.external_id:read" desc="查看员工自定义 ID" support_app_types="custom,isv" tags="">查看员工自定义 ID</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >department_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门ID类型

**示例值**：open_department_id

**可选值有**：
<md-enum>
<md-enum-item key="department_id" >department_id</md-enum-item>
<md-enum-item key="open_department_id" >open_department_id</md-enum-item>
</md-enum>

**默认值**：`open_department_id`
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 请求体

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >employee</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >update_employee</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	更新员工对象
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >upsert_name</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	姓名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_text</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	员工的姓名。

	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >default_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	默认值

长度范围：1- 64 字符


**示例值**："工位"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >i18n_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >map&lt;string, string&gt;</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	国际化值，key为zh_cn, ja_jp, en_us, value为对应的值

**示例值**：{"zh_cn":"工位1"}
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >another_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	别名，最多可输入 64 字


**示例值**："Jack"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >mobile</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	员工的手机号，最多可输入 255 字。注意：
1. 在企业内的在职员工中不可重复
2. 未认证企业仅支持添加中国大陆手机号，通过Lark认证的企业允许添加海外手机号
3. 国际电话区号前缀中必须包含加号 +

**示例值**："13011111111" 或 "+8613011111111"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >custom_employee_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	企业内在职员工的唯一标识。支持自定义，未自定义时系统自动生成。ID支持修改。注意：
1. 在职员工的ID不可重复。
2. ID不能包含空格。

**示例值**："eesadeq"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >avatar_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	员工的头像key。获取图片的key请使用 [上传图片 - 服务端 API - 开发文档 - Lark开放平台](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)，上传时图片类型需要选择 用于设置头像

**示例值**："dadwqeqwdsa"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >email</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	员工在工作中的邮箱。注意：
1. 在企业内的在职员工中不可重复。
2. 非中国大陆手机号成员必须同时添加邮箱。

**示例值**："zhangsan@163.com"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >enterprise_email</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	员工的企业邮箱。请先确保已在管理后台启用Lark邮箱服务。企业邮箱的域名需要企业在管理后台申请并开启。如果企业没有开启对应域名的企业邮箱，设置用户的企业邮箱会操作失败。

**示例值**："zhangsan@163.com"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >gender</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	性别

**示例值**：1

**可选值有**：
<md-enum>
<md-enum-item key="0" >未知</md-enum-item>
<md-enum-item key="1" >男</md-enum-item>
<md-enum-item key="2" >女</md-enum-item>
<md-enum-item key="3" >其他</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >employee_order_in_departments</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >upsert_user_department_sort_info\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	员工在所属部门内的排序信息

**数据校验规则**：

- 长度范围：`0` ～ `10`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >department_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门id，与department_id_type类型保持一致。

**示例值**："eediasdjw"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >order_weight_in_deparment</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	员工在部门内的排序权重

**数据校验规则：**



**示例值**："100"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >order_weight_among_deparments</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	该部门在用户所属的多个部门间的排序权重

**示例值**："20"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >is_main_department</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	是否为用户的主部门（用户只能有一个主部门，且排序权重应最大，不填则默认使用排序第一的部门作为主部门)

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >background_image_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	背景图的key。获取图片的key请使用 [上传图片 - 服务端 API - 开发文档 - Lark开放平台](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)，上传时图片类型需要选择 用于发送消息

**示例值**："qweasdqawqeq"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >description</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	员工的个性签名

**示例值**："新员工入职"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >leader_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	员工的直属上级ID。注意：
1. 不可成环，即A的上级是B，B的上级是A。
2. 上级需要是一个在职的员工。

**示例值**："eeshfosd"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >dotted_line_leader_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	员工的虚线上级ID，与employee_id_type类型保持一致。注意：
1. 不可成环，即A的上级是B，B的上级是A。
2. 上级需要是一个在职的员工。

**示例值**：["eefhdgsd"]

**数据校验规则**：

- 长度范围：`0` ～ `10`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >work_country_or_region</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	工作地国家/地区码。获取国家/地区的编码请使用 [分页批量查询国家/地区](/document/uAjLw4CM/ukTMukTMukTM/mdm-v3/country_region/list)。

**示例值**："MDM2312312"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >work_place_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	工作地点ID

**示例值**："1111sdda"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >work_station</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_text</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	工位
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >default_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	默认值

**示例值**："工位"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >i18n_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >map&lt;string, string&gt;</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	国际化值，key为zh_cn, ja_jp, en_us, value为对应的值

**示例值**：{"zh_cn":"工位"}
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >job_number</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	工号。企业内在职员工的工号不可重复。

**示例值**："28549233"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >extension_number</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	分机号，最多可输入 99 字。企业内所有员工的分机号不可重复。

**示例值**："2854923"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >join_date</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	入职日期

固定格式为：'YYYY-MM-DD' , 固定长度为：10

**示例值**："2022-10-10"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >employment_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	员工类型

**示例值**：1

**可选值有**：
<md-enum>
<md-enum-item key="0" >未知</md-enum-item>
<md-enum-item key="1" >全职</md-enum-item>
<md-enum-item key="2" >实习</md-enum-item>
<md-enum-item key="3" >外包</md-enum-item>
<md-enum-item key="4" >劳务</md-enum-item>
<md-enum-item key="5" >顾问</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >job_title_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	职务ID

**示例值**："ewdjdssd"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >job_level_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	职级ID

**示例值**："asdfghjk"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >job_family_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	序列ID

**示例值**："qwertyui"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >resign_date</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	离职日期

固定格式为：'YYYY-MM-DD' , 固定长度为：10

**示例值**："2022-10-10"

**数据校验规则**：

- 长度范围：`0` ～ `20` 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >resign_reason</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	离职原因

**示例值**："1"

**可选值有**：
<md-enum>
<md-enum-item key="0" >置空</md-enum-item>
<md-enum-item key="1" >薪酬不符合预期</md-enum-item>
<md-enum-item key="2" >工作时间过长</md-enum-item>
<md-enum-item key="3" >不满意工作内容</md-enum-item>
<md-enum-item key="4" >不认可上级或管理层</md-enum-item>
<md-enum-item key="5" >职业发展机会有限</md-enum-item>
<md-enum-item key="6" >对公司文化缺乏认同</md-enum-item>
<md-enum-item key="7" >组织架构调整（主动离职）</md-enum-item>
<md-enum-item key="8" >合同到期</md-enum-item>
<md-enum-item key="9" >跳槽</md-enum-item>
<md-enum-item key="10" >转行</md-enum-item>
<md-enum-item key="11" >家庭原因</md-enum-item>
<md-enum-item key="12" >健康状况不佳</md-enum-item>
<md-enum-item key="13" >工作地点原因</md-enum-item>
<md-enum-item key="14" >其他(主动离职)</md-enum-item>
<md-enum-item key="15" >意外</md-enum-item>
<md-enum-item key="16" >身故</md-enum-item>
<md-enum-item key="17" >解雇</md-enum-item>
<md-enum-item key="18" >试用期不通过</md-enum-item>
<md-enum-item key="19" >工作表现不佳</md-enum-item>
<md-enum-item key="20" >工作产出低</md-enum-item>
<md-enum-item key="21" >组织架构调整（被动离职）</md-enum-item>
<md-enum-item key="22" >违纪</md-enum-item>
<md-enum-item key="23" >违法</md-enum-item>
<md-enum-item key="24" >其他（被动离职）</md-enum-item>
<md-enum-item key="25" >其他（其他）</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >resign_remark</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	离职备注信息

**示例值**："个人原因"

**数据校验规则**：

- 长度范围：`0` ～ `255` 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >resign_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	离职类型

**示例值**："1"

**可选值有**：
<md-enum>
<md-enum-item key="0" >置空</md-enum-item>
<md-enum-item key="1" >主动</md-enum-item>
<md-enum-item key="2" >被动</md-enum-item>
<md-enum-item key="3" >其他</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >is_frozen</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	是否冻结员工账号。
true为冻结，false为恢复账号。

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >custom_field_values</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >custom_field_value\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	自定义字段

**数据校验规则**：

- 长度范围：`0` ～ `100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >field_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	自定义字段类型

**示例值**："1"

**可选值有**：
<md-enum>
<md-enum-item key="1" >多行文本</md-enum-item>
<md-enum-item key="2" >网页链接</md-enum-item>
<md-enum-item key="3" >枚举选项</md-enum-item>
<md-enum-item key="4" >人员</md-enum-item>
<md-enum-item key="9" >电话</md-enum-item>
<md-enum-item key="10" >多选枚举类型(目前仅支持文本类型)</md-enum-item>
<md-enum-item key="11" >人员列表</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >text_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_text</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	文本字段值
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >default_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	默认值

**示例值**："姓名字段"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >i18n_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >map&lt;string, string&gt;</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	国际化值，key为zh_cn, ja_jp, en_us, value为对应的值

**示例值**：{"zh_cn":"姓名字段"}
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >url_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >url_value</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	网页链接字段值
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >link_text</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_text</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	网页标题
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >default_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	默认值

**示例值**："网页标题"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >i18n_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >map&lt;string, string&gt;</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	国际化值，key为zh_cn, ja_jp, en_us, value为对应的值

**示例值**：{"zh_cn":"网页标题"}
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	移动端网页链接

**示例值**："https://m.bytedance.com/afnasjfna"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >pcurl</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	桌面端网页链接

**示例值**："http://www.fs.cn"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >enum_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >enum_value</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	枚举
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >enum_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	选项结果ID

**示例值**：["1"]

**数据校验规则**：

- 长度范围：`0` ～ `100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >enum_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	选项类型

**示例值**："1"

**可选值有**：
<md-enum>
<md-enum-item key="1" >文本</md-enum-item>
<md-enum-item key="2" >图片</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >user_values</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >user_value\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	人员字段值

**数据校验规则**：

- 长度范围：`0` ～ `100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	人员ID，与employee_id_type类型保持一致。

**示例值**：["1"]

**数据校验规则**：

- 长度范围：`0` ～ `100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >phone_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >phone_value</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	电话字段值
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >phone_number</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	电话号

**示例值**："18812345678"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >extension_number</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	分机号

**示例值**："234234234"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >field_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	自定义字段key

**示例值**："C-1000001"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "employee": {
        "name": {
            "name": {
                "default_value": "张三",
                "i18n_value": {
                    "zh_cn": "张三",
                    "ja_jp": "佐藤はるか",
                    "en_us": "Alex Zhang"
                }
            },
            "another_name": "Jack"
        },
        "mobile": "+8613011111111",
        "custom_employee_id": "u273y71",
        "avatar_key": "8abc397a-9950-44ea-9302-e1d8fe00858g",
        "email": "zhangsan@test.com",
        "enterprise_email": "zhangsan@test.com",
        "gender": 1,
        "employee_order_in_departments": [
            {
                "department_id": "0",
                "order_weight_in_deparment": "100",
                "order_weight_among_deparments": "20",
                "is_main_department": false
            }
        ],
        "leader_id": "eeasdqwwe",
        "dotted_line_leader_ids": [
            "hdsuqw"
        ],
        "work_country_or_region": "MDM34234234",
        "work_place_id": "eqwedas",
        "work_station": {
            "default_value": "张三",
            "i18n_value": {
                "zh_cn": "工位",
                "ja_jp": "工位",
                "en_us": "Work Station"
            }
        },
        "job_number": "2845435",
        "extension_number": "2845435",
        "join_date": "2022-10-10",
        "employment_type": "1",
        "staff_status": "",
        "job_title_id": "wqedsaqw",
        "resign_reason": "",
        "resign_type": "",
        "custom_field_values": [
            {
                "field_type": "1",
                "text_value": {
                    "default_value": "姓名字段",
                    "i18n_value": {
                        "zh_cn": "姓名字段",
                        "ja_jp": "姓名字段",
                        "en_us": "Name Filed"
                    }
                },
                "url_value": {
                    "link_text": {
                        "default_value": "网页地址",
                        "i18n_value": {
                            "zh_cn": "网页地址",
                            "ja_jp": "This is a URL",
                            "en_us": "This is a URL"
                        }
                    },
                    "url": "https://www.larksuite.com/",
                    "pcurl": "https://www.larksuite.com/"
                },
                "enum_value": {
                    "enum_ids": [
                        "75ec5g97"
                    ],
                    "enum_type": "1"
                },
                "user_values": [
                    {
                        "ids": [
                            "27al2hef"
                        ]
                ],
                "field_key": "C-1000001"
            }
        ]
    }
}
</md-code-json>
:::



## 响应





### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	错误码，非 0 表示失败
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >msg</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	错误描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >data</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >\-</md-text>
	</md-dt-td>
	<md-dt-td>
	\-
	</md-dt-td>
</md-dt-tr>


  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "msg": "success",
    "data": {}
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 2221164 | User name exceeds limit | 姓名长度超过限制，最多可输入 64 字 |
| 400 | 2221165 | User en_name exceeds limit | 英文名长度超过限制，最多可输入 64 字 |
| 400 | 2221166 | User another_name exceeds limit | 别名长度超过限制，最多可输入 64 字 |
| 400 | 2221103 | Mobile already exists | 手机号已存在，请修改手机号 |
| 400 | 2221106 | Invalid mobile | 无效手机号，请修改手机号 |
| 400 | 2221113 | Mobile or email not set | 手机号或邮箱必填 |
| 400 | 2221114 | User must have a mobile in China | 中国必须有手机号，请添加中国手机号 |
| 400 | 2221156 | Unable to edit verified mobile | 不允许编辑已认证手机号，请不要编辑已认证手机号或通过其他方式修改 |
| 400 | 2221104 | Email already exists | 邮箱已存在，请求修改邮箱 |
| 400 | 2221107 | Invalid email | 无效的邮箱，请求修改企业邮箱 |
| 400 | 2221118 | Enterprise email already exists | 企业邮箱已存在，请求修改企业邮箱 |
| 400 | 2221278 | Invalid enterprise email | 无效的邮箱，请求修改企业邮箱 |
| 400 | 2221126 | Enterprise email domain unavailable | 企业邮箱域名不可用，请求修改企业邮箱 |
| 400 | 2221146 | Enterprise email alias exceeds limit | 企业邮箱别名超过长度限制，最多可输入 255 字 |
| 400 | 2221147 | Enterprise email address in recycle bin | 企业邮箱地址在回收站中，请修改企业邮箱 |
| 400 | 2221176 | Add Lark allow list tenant. Email must be included with non+86mobile | Lark租户添加非+86的手机号时必须包含邮件信息，请包含邮件信息 |
| 400 | 2221255 | Main department must be the first | 员工在所属部门内的排序信息中主部门必须在第一个，请修改员工所属部门内的排序信息 |
| 400 | 2221125 | The number of members within the department exceeds the limit. Please contact an administrator for help | 部门内成员人数超过限制，人数不能超过一万，请联系管理员寻求帮助。 |
| 400 | 2221129 | User department is empty | 员工所属部门为空，请修改员工所属部门内的排序信息 |
| 400 | 2221141 | Unable to join multiple departments. Please upgrade relevant 'Organizational Structure Visible'. | 无法加入多个部门，请修改员工所属部门内的排序信息 |
| 400 | 2221181 | Department does not exist | 部门不存在，请修改员工所属部门内的排序信息 |
| 400 | 2221239 | Leader loop error | 直属上级成环，请修改直属上级 |
| 400 | 2221238 | DottedLineLeaderID loop error | 虚线上级成环，请修改虚线上级 |
| 400 | 2221221 | DottedLineLeaderID exceeds length limit | 虚线上级长度超过限制，请修改虚线上级 |
| 400 | 2221222 | Invalid dottedLineLeaderID | 无效的虚线上级ID，请修改虚线上级 |
| 400 | 2221242 | Invalid custom field | 无效的自定义字段，请修改自定义字段 |
| 400 | 2221216 | Invalid work country or region | 无效的工作国家或区域，请修改工作国家或区域 |
| 400 | 2221217 | WorkplaceID not found | 工作城市不存在，请修改工作城市 |
| 400 | 2221240 | JobNumber not unique | 工号已存在，请修改工号 |
| 400 | 2221191 | Invalid extension number | 分机号无效，请修改分机号 |
| 400 | 2221192 | Repeated extension number within the tenant | 分机号已存在，请修改分机号 |
| 400 | 2221193 | Extension number exceeds limit | 分机号长度超过限制，最多可输入 99 字 |
| 400 | 2221210 | Invalid join date | 无效的入职时间，请修改入职时间 |
| 400 | 2221144 | EmployeeType not found | 人员类型不存在，请修改人员类型 |
| 400 | 2221145 | EmployeeType inactive | 人员类型未激活，请修改人员类型 |
| 400 | 2221223 | Invalid job title ID | 职务不存在，请修改职务 |
| 400 | 2221263 | Tenant has not activated multi geo | 租户未开启Multi-Geo，请先开通再试。Multi-Geo指的是多地理位置数据驻留。 |
| 400 | 2221264 | User geo does not exist | 指定的Geo不存在，请检查Geo参数是否正确。 |
| 400 | 2221265 | The application does not have permission to write to the geo | 无权限指定员工Geo，需申请，点击api调试台-权限配置，会显示需要的权限，点击“操作”-“...”-“开通”，即可。<br><md-perm name="directory:employee.base.geo:write" desc="写入员工数据所在地" support_app_types="custom" tags="">写入员工数据所在地</md-perm> |
| 400 | 2221266 | The application does not have permission to write to the SubscriptionID | 无权限指定席位，需申请，点击api调试台-权限配置，会显示需要的权限，点击“操作”-“...”-“开通”，即可。        <md-perm name="directory:employee.base.subscription_ids:write" desc="写入员工席位信息" support_app_types="custom" tags="">写入员工席位信息</md-perm> |
| 400 | 2224001 | No permission to operate | 无操作权限，请检查当前应用的权限或企业版本是否是商业专业版本及以上。 |
| 400 | 2224002 | No permission to operate record | 无操作该记录权限，请检查当前应用的数据管理范围的权限或当前应用所操作的成员是否可更新。 |
| 400 | 2224003 | No permission to operate dependent object | 无操作依赖对象权限，请检查要更新到的部门是否有权限。 |
| 400 | 2221252 | Hybrid license tenant prohibits passing empty licenses to create users | 混合许可证租户禁止传递空许可证来创建用户，请传递有效的许可证 |
| 400 | 2221253 | Designated licenseID is insufficient | 剩余席位不足，请修改席位信息 |
| 400 | 2221254 | Designated licenseID is invalid | 应用没有权限写入员工数据驻留地，需申请写入员工数据驻留地权限(directory:employee.base.geo:write) |
| 400 | 2221111 | Exceeds certified seat limit | 混合许可证租户禁止传递空许可证来创建用户，请传递有效的许可证 |
| 400 | 2221112 | Exceeds billing plan seat limit | 剩余席位不足，请修改席位信息 |
| 400 | 2221115 | ExternalID is not unique | 自定义ID已存在，请修改自定义ID |
| 400 | 2221116 | Invalid ExternalID | 无效的自定义ID，请修改自定义ID |
| 400 | 2221175 | Lark only supports +86mobile | Lark租户仅支持+86手机号，请修改手机号 |
| 400 | 2221182 | Unable to freeze tenant founder | 无法冻结企业管理员，请选择非企业管理员用户进行冻结操作 |
| 400 | 2221109 | Name contains sensitive info | 姓名包含敏感信息，请修改姓名 |
| 400 | 2221292 | User department is disabled | 用户部门已停用，请将用户转移至未停用的部门或激活该部门 |
| 400 | 2221213 | Resign date invalid or earlier than join date or empty | 请检查离职日期格式可能非法、离职日期早于加入日期、离职日期为空 |
| 400 | 2221214 | Resign reason invalid or not match resign type | 离职原因非法，或者离职原因和离职类型不匹配，请修改离职原因或离职类型使其匹配 |
| 400 | 2221293 | Only allow update preResigned\resigned employee's resign info field | 只允许更新待离职、离职人员的离职信息字段（离职备注、离职类型、离职原因、离职日期），请更新待离职或离职人员的对应字段 |
| 400 | 2221231 | Resign type invalid or not match resign reason | 离职类型非法，或者离职原因和离职类型不匹配，请修改离职类型或离职原因使其匹配 |





