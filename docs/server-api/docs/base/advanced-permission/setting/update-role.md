---
document_id: '7130175416736399366'
directory_id: '7120048623526133765'
title: 更新自定义角色
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/update
breadcrumb:
- Server API
- Docs
- Base
- Advanced Permission
- Setting
- Update role
document_type: ReferenceDocumentType
updated_at: 2023-11-23T06:38:32Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/update
---

# 更新自定义角色

更新自定义角色{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app.role&method=update)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">
更新自定义角色是增量更新，仅对传值的字段进行更新，不传值则不更新
</md-alert>
:::

:::html
<md-alert type="tip">

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
      <md-td>https://open.larksuite.com/open-apis/bitable/v1/apps/:app_token/roles/:role_id</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>PUT</md-td>
    </md-tr>
    <md-tr>
      <md-th>接口频率限制</md-th>
      <md-td>[10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom,isv"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
      </md-th>
      <md-td>
            <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm>
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
或
<md-tag mode="inline" type="token-user">user_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"

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
	<md-text type="field-name" >app_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格的唯一标识符 [app_token 参数说明](/document/uAjLw4CM/ukTMukTMukTM/bitable/notification#8121eebe)

**示例值**："appbcbWCzen6D8dezhoCH2RpMAh"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >role_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	自定义角色的id

**示例值**："roljRpwIUt"
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
	<md-text type="field-name" >role_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	自定义角色的名字

**示例值**："自定义角色1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >table_roles</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role.table_role\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	数据表角色

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >table_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	数据表名

**示例值**："数据表1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >table_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	数据表ID

**示例值**："tblKz5D60T4JlfcT"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >table_perm</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	数据表权限，`协作者可编辑自己的记录`和`可编辑指定字段`是`可编辑记录`的特殊情况，可通过指定`rec_rule`或`field_perm`参数实现相同的效果

**示例值**：0

**可选值有**：
<md-enum>
<md-enum-item key="0" >无权限</md-enum-item>
<md-enum-item key="1" >可阅读</md-enum-item>
<md-enum-item key="2" >可编辑记录</md-enum-item>
<md-enum-item key="4" >可编辑字段和记录</md-enum-item>
</md-enum>

**默认值**：`0`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >rec_rule</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role.table_role.rec_rule</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	记录筛选条件，在table_perm为1或2时有意义，用于指定可编辑或可阅读某些记录
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >conditions</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role.table_role.rec_rule.condition\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	记录筛选条件

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >field_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	字段名，记录筛选条件是`创建人包含访问者本人`时，此参数值为""

**示例值**："单选"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >operator</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	运算符

**示例值**："is"

**可选值有**：
<md-enum>
<md-enum-item key="is" >等于</md-enum-item>
<md-enum-item key="isNot" >不等于</md-enum-item>
<md-enum-item key="contains" >包含</md-enum-item>
<md-enum-item key="doesNotContain" >不包含</md-enum-item>
<md-enum-item key="isEmpty" >为空</md-enum-item>
<md-enum-item key="isNotEmpty" >不为空</md-enum-item>
</md-enum>

**默认值**：`is`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	单选或多选字段的选项id

**示例值**：["optbdVHf4q"]
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >conjunction</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	多个筛选条件的关系

**示例值**："and"

**可选值有**：
<md-enum>
<md-enum-item key="and" >与</md-enum-item>
<md-enum-item key="or" >或</md-enum-item>
</md-enum>

**默认值**：`and`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >other_perm</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	其他记录权限，仅在table_perm为2时有意义

**示例值**：0

**可选值有**：
<md-enum>
<md-enum-item key="0" >禁止查看</md-enum-item>
<md-enum-item key="1" >仅可阅读</md-enum-item>
</md-enum>

**默认值**：`0`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >field_perm</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role.table_role.field_perm</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	字段权限，仅在table_perm为2时有意义，设置字段可编辑或可阅读。类型为 map，key 是字段名，value 是字段权限

**value 枚举值有：**
- `1`：可阅读
- `2`：可编辑

**示例值**：{"姓名": 1, "年龄": 2}
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >allow_add_record</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	新增记录权限，仅在table_perm为2时有意义，用于设置记录是否可以新增。

**示例值**：true

**默认值**：`true`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >allow_delete_record</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	删除记录权限，仅在table_perm为2时有意义，用于设置记录是否可以删除

**示例值**：true

**默认值**：`true`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >block_roles</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role.block_role\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	block权限

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >block_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	Block 的 ID，例如列出仪表盘接口中的仪表盘 block  id

**示例值**："blknkqrP3RqUkcAW"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >block_perm</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	Block权限

**示例值**：0

**可选值有**：
<md-enum>
<md-enum-item key="0" >无权限</md-enum-item>
<md-enum-item key="1" >可阅读</md-enum-item>
</md-enum>

**默认值**：`0`
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "role_name": "自定义角色1",
    "table_roles": [
        {
            "table_name": "数据表1",
            "table_id": "tblKz5D60T4JlfcT",
            "table_perm": 0,
            "rec_rule": {
                "conditions": [
                    {
                        "field_name": "单选",
                        "operator": "is",
                        "value": [
                            "optbdVHf4q"
                        ]
                    }
                ],
                "conjunction": "and",
                "other_perm": 0
            },
            "field_perm": {},
            "allow_add_record": true,
            "allow_delete_record": true
        }
    ],
    "block_roles": [
        {
            "block_id": "blknkqrP3RqUkcAW",
            "block_perm": 0
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


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >role</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role</md-text>
	</md-dt-td>
	<md-dt-td>
	自定义角色
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >role_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	自定义角色的名字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >role_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	自定义角色的id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >table_roles</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role.table_role\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	数据表角色
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >table_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	数据表名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >table_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	数据表ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >table_perm</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	数据表权限，`协作者可编辑自己的记录`和`可编辑指定字段`是`可编辑记录`的特殊情况，可通过指定`rec_rule`或`field_perm`参数实现相同的效果

**可选值有**：
<md-enum>
<md-enum-item key="0" >无权限</md-enum-item>
<md-enum-item key="1" >可阅读</md-enum-item>
<md-enum-item key="2" >可编辑记录</md-enum-item>
<md-enum-item key="4" >可编辑字段和记录</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >rec_rule</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role.table_role.rec_rule</md-text>
	</md-dt-td>
	<md-dt-td>
	记录筛选条件，在table_perm为1或2时有意义，用于指定可编辑或可阅读某些记录
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >conditions</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role.table_role.rec_rule.condition\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	记录筛选条件
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >field_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	字段名，记录筛选条件是`创建人包含访问者本人`时，此参数值为""
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >operator</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	运算符

**可选值有**：
<md-enum>
<md-enum-item key="is" >等于</md-enum-item>
<md-enum-item key="isNot" >不等于</md-enum-item>
<md-enum-item key="contains" >包含</md-enum-item>
<md-enum-item key="doesNotContain" >不包含</md-enum-item>
<md-enum-item key="isEmpty" >为空</md-enum-item>
<md-enum-item key="isNotEmpty" >不为空</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	单选或多选字段的选项id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >field_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	字段类型
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >conjunction</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多个筛选条件的关系

**可选值有**：
<md-enum>
<md-enum-item key="and" >与</md-enum-item>
<md-enum-item key="or" >或</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >other_perm</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	其他记录权限，仅在table_perm为2时有意义

**可选值有**：
<md-enum>
<md-enum-item key="0" >禁止查看</md-enum-item>
<md-enum-item key="1" >仅可阅读</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >field_perm</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role.table_role.field_perm</md-text>
	</md-dt-td>
	<md-dt-td>
	字段权限，仅在table_perm为2时有意义，设置字段可编辑或可阅读。类型为 map，key 是字段名，value 是字段权限

**value 枚举值有：**
- `1`：可阅读
- `2`：可编辑
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >allow_add_record</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	新增记录权限，仅在table_perm为2时有意义，用于设置记录是否可以新增。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >allow_delete_record</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	删除记录权限，仅在table_perm为2时有意义，用于设置记录是否可以删除
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >block_roles</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role.block_role\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	block权限
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >block_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Block 的 ID，例如列出仪表盘接口中的仪表盘 block  id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >block_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Block类型

**可选值有**：
<md-enum>
<md-enum-item key="dashboard" >仪表盘</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >block_perm</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	Block权限

**可选值有**：
<md-enum>
<md-enum-item key="0" >无权限</md-enum-item>
<md-enum-item key="1" >可阅读</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "data": {
        "role": {
            "role_name": "role1",
            "table_roles": [
                {
                    "table_name": "table1",
                    "table_id": "tblFIgBzKEq75HSE",
                    "table_perm": 2,
                    "addrecords_perm": false,
                    "deleterecords_perm": true,
                    "rec_rule": {
                        "conjunction": "or",
                        "conditions": [
                            {
                                "field_name": "单选",
                                "operator": "is",
                                "field_type": 3,
                                "value": [
                                    "optbdVHf4q"
                                ]
                            },
                            {
                                "value": null,
                                "field_name": "人员",
                                "operator": "contains",
                                "field_type": 11
                            },
                            {
                                "operator": "contains",
                                "field_type": 1003,
                                "value": null,
                                "field_name": ""
                            }
                        ],
                        "other_perm": 0
                    },
                    "field_perm": {
                        "单选": 1,
                        "年龄": 2
                    }
                },
                {
                    "table_name": "table2",
                    "table_id": "tblMPI6OC1aWvTvs",
                    "table_perm": 1,
                    "rec_rule": {
                        "conditions": [
                            {
                                "field_name": "人员",
                                "operator": "contains",
                                "field_type": 11,
                                "value": null
                            },
                            {
                                "operator": "is",
                                "field_type": 4,
                                "value": [
                                    "opttgKOTSt",
                                    "optWcdXR0W"
                                ],
                                "field_name": "多选"
                            }
                        ],
                        "other_perm": 0,
                        "conjunction": "and"
                    }
                },
                {
                    "table_name": "table3",
                    "table_id": "tblmkLF7Tg6IWbRb",
                    "table_perm": 0
                },
                {
                    "table_name": "table4",
                    "table_id": "tbl5VQHDTms19Qe7",
                    "table_perm": 4
                }
            ],
            "block_roles": [
                {
                    "block_id": "blknkqrP3RqUkcAW",
                    "block_perm": 0
                },
                {
                    "block_id": "blkAjxjWKvbBi7EA", 
                    "block_perm": 1
                }
            ]
        }
    },
    "code": 0,
    "msg": "success"
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
  <md-td>200</md-td>
  <md-td>1254000</md-td>
  <md-td>WrongRequestJson</md-td>
  <md-td>请求体错误</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1254001</md-td>
  <md-td>WrongRequestBody</md-td>
  <md-td>请求体错误</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1254002</md-td>
  <md-td>Fail</md-td>
  <md-td>内部错误，有疑问可咨询客服</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1254003</md-td>
  <md-td>WrongBaseToken</md-td>
  <md-td>app_token 错误</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1254010</md-td>
  <md-td>ReqConvError</md-td>
  <md-td>请求错误</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1254032</md-td>
  <md-td>InvalidRoleName</md-td>
  <md-td>自定义角色名无效</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1254033</md-td>
  <md-td>RoleNameDuplicated</md-td>
  <md-td>自定义角色名重复</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1254036</md-td>
  <md-td>Base is copying, please try again later.</md-td>
  <md-td>多维表格副本复制中，稍后重试</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1254040</md-td>
  <md-td>BaseTokenNotFound</md-td>
  <md-td>app_token 不存在</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1254047</md-td>
  <md-td>RoleIdNotFound</md-td>
  <md-td>role_id 不存在</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1254110</md-td>
  <md-td>RoleExceedLimit</md-td>
  <md-td>自定义角色数量超限，限制30条</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1254290</md-td>
  <md-td>TooManyRequest</md-td>
  <md-td>请求过快，稍后重试</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1254291</md-td>
  <md-td>Write conflict</md-td>
  <md-td>同一个数据表(table) 不支持并发调用写接口，请检查是否存在并发调用写接口。写接口包括：新增、修改、删除记录；新增、修改、删除字段；修改表单；修改视图等。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1254301</md-td>
  <md-td>OperationTypeError</md-td>
  <md-td>多维表格未开启高级权限或不支持开启高级权限</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1254302</md-td>
  <md-td>Permission denied.</md-td>
  <md-td>无访问权限, 常由表格开启了高级权限造成, 请在高级权限设置中添加一个包含应用的群, 给予这个群读写权限</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1254304</md-td>
  <md-td>Only Available For Business and Enterprise Editions</md-td>
  <md-td>仅企业版和旗舰版Lark支持行列权限</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1255001</md-td>
  <md-td>InternalError</md-td>
  <md-td>内部错误，有疑问可咨询客服</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1255002</md-td>
  <md-td>RpcError</md-td>
  <md-td>内部错误，有疑问可咨询客服</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1255003</md-td>
  <md-td>MarshalError</md-td>
  <md-td>序列化错误，有疑问可咨询客服</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1255004</md-td>
  <md-td>UmMarshalError</md-td>
  <md-td>反序列化错误</md-td>
</md-tr>


<md-tr>
  <md-td>504</md-td>
  <md-td>1255040</md-td>
  <md-td>请求超时</md-td>
  <md-td>进行重试</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




