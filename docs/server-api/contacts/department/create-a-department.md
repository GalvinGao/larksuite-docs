---
document_id: '6965400907878334469'
directory_id: '6927521875715244034'
title: 创建部门
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/create
breadcrumb:
- Server API
- Contacts
- Department
- Create a department
document_type: ReferenceDocumentType
updated_at: 2024-04-23T07:27:13Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/create
---

# 创建部门

该接口用于向通讯录中创建部门。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=contact&version=v3&resource=department&method=create)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="tip">
只可在应用的通讯录权限范围内的部门下创建部门。若需要在根部门下创建子部门，则应用通讯录权限范围需要设置为“全部成员”。应用商店应用无权限调用此接口。
</md-alert>
:::



## 请求
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/contact/v3/departments</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
      </md-th>
      <md-td>
            <md-perm name="contact:contact" desc="更新通讯录" support_app_types="custom" tags="">更新通讯录</md-perm>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            字段权限要求
      </md-th>
      <md-td>
        <md-alert type="tip" icon="none">
        该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请
        </md-alert>
        <md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
        <md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
        <md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
        <md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
        <md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
        <md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 35%;">名称</md-th>
      <md-th style="width: 13%;">类型</md-th>
       <md-th style="width: 15%;" filters="是,否" >必填</md-th>
      <md-th  style="width: 37%;">描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"

[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)

</md-td>
</md-tr>
<md-tr>
<md-td>Content-Type</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
</md-tbody>
</md-table>
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
	<md-text type="field-name" >user_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户 ID 类型

**示例值**："open_id"

**可选值有**：
<md-enum>
<md-enum-item key="open_id" >标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)</md-enum-item>
<md-enum-item key="union_id" >标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)</md-enum-item>
<md-enum-item key="user_id" >标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)</md-enum-item>
</md-enum>

**默认值**：`open_id`

**当值为 `user_id`，字段权限要求**：
<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
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
	此次调用中使用的部门ID的类型

不同 ID 的说明参见[部门ID说明](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)

**示例值**："open_department_id"

**可选值有**：
<md-enum>
<md-enum-item key="department_id" >用来标识租户内一个唯一的部门</md-enum-item>
<md-enum-item key="open_department_id" >用来在具体某个应用中标识一个部门，同一个部门 在不同应用中的 open_department_id 不相同。</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >client_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用于幂等判断是否为同一请求，避免重复创建。字符串类型，自行生成。

**示例值**："473469C7-AA6F-4DC5-B3DB-A3DC0DE3C83E"
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
      <md-dt-th style="width: 40%;">名称</md-dt-th>
      <md-dt-th style="width: 20%;">类型</md-dt-th>
      <md-dt-th style="width: 10%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 30%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	部门名称

**示例值**："DemoName"

**数据校验规则**：

- 最小长度：`1` 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >i18n_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >department_i18n_name</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	国际化的部门名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >zh_cn</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门的中文名

**示例值**："Demo名称"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >ja_jp</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门的日文名

**示例值**："デモ名"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >en_us</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门的英文名

**示例值**："Demo Name"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >parent_department_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	父部门的ID

* 在根部门下创建新部门，该参数值为 “0”

**示例值**："D067"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
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
	本部门的自定义部门ID

注意：除需要满足正则规则外，同时不能以`od-`开头

**示例值**："D096"

**数据校验规则**：

- 最大长度：`64` 字符

- 正则校验：`^[a-zA-Z0-9][a-zA-Z0-9_\-@.]{0,63}$`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >leader_user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门主管用户ID

**示例值**："ou_7dab8a3d3cdcc9da365777c7ad535d62"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >order</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门的排序，即部门在其同级部门的展示顺序

**示例值**："100"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >unit_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门单位自定义ID列表，当前只支持一个

**示例值**：custom_unit_id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >create_group_chat</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	是否创建部门群，默认不创建。创建部门群时，默认群名为部门名，默认群主为部门主负责人

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >leaders</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >departmentLeader\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门负责人
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >leaderType</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	负责人类型

**示例值**：1

**可选值有**：
<md-enum>
<md-enum-item key="1" >主负责人</md-enum-item>
<md-enum-item key="2" >副负责人</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >leaderID</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	负责人ID

**示例值**："ou_7dab8a3d3cdcc9da365777c7ad535d62"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >group_chat_employee_types</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门群雇员类型限制。[]空列表时，表示为无任何雇员类型。类型字段可包含以下值，支持多个类型值；若有多个，用英文','分隔：
1、正式员工
2、实习生
3、外包
4、劳务
5、顾问
6、其他自定义类型字段，可通过下方接口获取到该租户的自定义员工类型的名称，参见[获取人员类型](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/list)。

**示例值**：[1,2,3]
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 请求体示例
:::html
<md-code-json>
{
    "name": "DemoName",
    "i18n_name": {
        "zh_cn": "Demo名称",
        "ja_jp": "デモ名",
        "en_us": "Demo Name"
    },
    "parent_department_id": "D067",
    "department_id": "D096",
    "leader_user_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
    "order": "100",
    "unit_ids": [
        "custom_unit_id"
    ],
    "create_group_chat": false,
     "leaders": [
                {
                    "leaderID": "ou_357368f98775f04bea02afc6b1d33478",
                    "leaderType": 1
                }
            ]
}
</md-code-json>
:::



## 响应



### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 40%;">名称</md-dt-th>
      <md-dt-th style="width: 20%;">类型</md-dt-th>
      <md-dt-th style="width: 30%;">描述</md-dt-th>
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


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >department</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >department</md-text>
	</md-dt-td>
	<md-dt-td>
	部门信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	部门名称

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >i18n_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >department_i18n_name</md-text>
	</md-dt-td>
	<md-dt-td>
	国际化的部门名称

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >zh_cn</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	部门的中文名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >ja_jp</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	部门的日文名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >en_us</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	部门的英文名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >parent_department_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	父部门的ID

* 在根部门下创建新部门，该参数值为 “0”

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
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
	本部门的自定义部门ID

注意：除需要满足正则规则外，同时不能以`od-`开头

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >open_department_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	部门的open_id，类型与通过请求的查询参数传入的department_id_type相同
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >leader_user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	部门主管用户ID

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >chat_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	部门群ID

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >order</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	部门的排序，即部门在其同级部门的展示顺序

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >unit_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	部门单位自定义ID列表，当前只支持一个

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >member_count</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	部门下用户的个数

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >department_status</md-text>
	</md-dt-td>
	<md-dt-td>
	部门状态

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >is_deleted</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否被删除
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >leaders</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >departmentLeader\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	部门负责人
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >leaderType</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	负责人类型

**可选值有**：
<md-enum>
<md-enum-item key="1" >主负责人</md-enum-item>
<md-enum-item key="2" >副负责人</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >leaderID</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	负责人ID

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >group_chat_employee_types</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	部门群雇员类型限制。[]空列表时，表示为无任何雇员类型。类型字段可包含以下值，支持多个类型值；若有多个，用英文','分隔：
1、正式员工
2、实习生
3、外包
4、劳务
5、顾问
6、其他自定义类型字段，可通过下方接口获取到该租户的自定义员工类型的名称，参见[获取人员类型](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/list)。
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
    "data": {
        "department": {
            "name": "DemoName",
            "i18n_name": {
                "zh_cn": "Demo名称",
                "ja_jp": "デモ名",
                "en_us": "Demo Name"
            },
            "parent_department_id": "D067",
            "department_id": "D096",
            "open_department_id": "od-4e6ac4d14bcd5071a37a39de902c7141",
            "leader_user_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
            "chat_id": "oc_5ad11d72b830411d72b836c20",
            "order": "100",
            "unit_ids": [
                "custom_unit_id"
            ],
            "member_count": 100,
            "status": {
                "is_deleted": false
            },
          "leaders": [
                {
                    "leaderID": "ou_357368f98775f04bea02afc6b1d33478",
                    "leaderType": 1
                }
            ]
        }
    }
}
</md-code-json>
:::



### 错误码
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">HTTP状态码</md-th>
            <md-th style="width: 15%;">错误码</md-th>
            <md-th style="width: 30%;">描述</md-th>
            <md-th style="width: 30%;">排查建议</md-th>
        </md-tr>
    </md-thead>
  <md-tbody>

<md-tr>
  <md-td>400</md-td>
  <md-td>43005</md-td>
  <md-td>duplicate order error</md-td>
  <md-td>部门的order必须唯一，请检查后重试。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>40018</md-td>
  <md-td>duplicate name error</md-td>
  <md-td>（同一父部门下）部门名必须唯一，请检查后重试。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43008</md-td>
  <md-td>custom dept id invalid error</md-td>
  <md-td>部门自定义ID不合法，不得以od-开头，不能为“0”，长度不可超过64。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43013</md-td>
  <md-td>dept too many children error</md-td>
  <md-td>子部门数量过多。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>40003</md-td>
  <md-td>internal error</md-td>
  <md-td>内部错误，请提供 X-Request-Id向客服反馈。[联系客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>40004</md-td>
  <md-td>no dept authority error</md-td>
  <md-td>操作的部门需在通讯录权限范围中，[了解更多](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>40008</md-td>
  <md-td>dept Info is null error</md-td>
  <md-td>部门的信息不能为空</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>40010</md-td>
  <md-td>chat id is invalid error</md-td>
  <md-td>部门群ID格式错误。</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>40014</md-td>
  <md-td>no parent dept authority error</md-td>
  <md-td>没有父部门权限（组织架构可见范围权限）。</md-td>
</md-tr>


<md-tr>
  <md-td>401</md-td>
  <md-td>43000</md-td>
  <md-td>dept has already exist error</md-td>
  <md-td>创建的部门已经存在。</md-td>
</md-tr>


<md-tr>
  <md-td>401</md-td>
  <md-td>40016</md-td>
  <md-td>dept name can not be nul error</md-td>
  <md-td>部门名不能为空。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>42006</md-td>
  <md-td>user has resigned error</md-td>
  <md-td>用户已经离职。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>41012</md-td>
  <md-td>user id invalid error</md-td>
  <md-td>用户ID无效。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>40021</md-td>
  <md-td>no a same request error</md-td>
  <md-td>两次操作不是同一个请求，请检查request值是否有改动。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43004</md-td>
  <md-td>illegal unit error</md-td>
  <md-td>部门unit id无效。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43007</md-td>
  <md-td>duplicated department custom id error</md-td>
  <md-td>部门自定义ID 企业内重复</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>40004</md-td>
  <md-td>no dept authority error</md-td>
  <md-td>操作的部门需在通讯录权限范围中，[了解更多](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>44101</md-td>
  <md-td>miss parent department id error</md-td>
  <md-td>请求缺少 parent department id</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43017</md-td>
  <md-td>relate dept over limit</md-td>
  <md-td>关联部门超过限制</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43018</md-td>
  <md-td>duplicate i18n name</md-td>
  <md-td>重复的i18n名字</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43019</md-td>
  <md-td>exceed dept max level</md-td>
  <md-td>超过部门层级深度</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43021</md-td>
  <md-td>department chat not exist</md-td>
  <md-td>部门群不存在</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43022</md-td>
  <md-td>department name duplicate</md-td>
  <md-td>部门名重复</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43023</md-td>
  <md-td>dept structure no permissione</md-td>
  <md-td>组织架构没有权限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43024</md-td>
  <md-td>dept structure tenant lock fail</md-td>
  <md-td>部门结构变动获取租户锁失败</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43025</md-td>
  <md-td>top department leader unjoined</md-td>
  <md-td>用户未加入不能成为企业负责人</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43026</md-td>
  <md-td>employee type is not valid</md-td>
  <md-td>请使用准确的雇员类型</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




