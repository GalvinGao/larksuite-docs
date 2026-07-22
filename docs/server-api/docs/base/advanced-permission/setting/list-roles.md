---
document_id: '7130175416736366598'
directory_id: '7120048623526133765'
title: 列出自定义角色
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/list
breadcrumb:
- Server API
- Docs
- Base
- Advanced Permission
- Setting
- List roles
document_type: ReferenceDocumentType
updated_at: 2023-09-15T03:53:25Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/list
---

# 列出自定义角色

列出自定义角色{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app.role&method=list)

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

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/bitable/v1/apps/:app_token/roles |
| HTTP Method | GET |
| 接口频率限制 | [20 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm><br><md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv" tags="">查看、评论和导出多维表格</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




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
	<md-text type="field-name" >page_size</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	分页大小

**示例值**：10

**数据校验规则**：

- 最大值：`30`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >page_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果

**示例值**：roljRpwIUt
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
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
	<md-text type="field-name" >items</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.role\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	自定义角色列表
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


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >page_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >has_more</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否还有更多项
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >total</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	总数
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
        "items": [
            {
                "role_id": "rola66uIZT",
                "role_name": "role1",
                "table_roles": [
                    {
                        "table_name": "table1",
                        "table_id": "tblFIgBzKEq75HSE",
                        "table_perm": 2,
                        "allow_add_record": false,
                        "allow_delete_record": true,
                        "rec_rule": {
                            "conjunction": "or",
                            "conditions": [
                                {
                                    "field_name": "单选",
                                    "field_type": 3,
                                    "operator": "is",
                                    "value": [
                                        "optbdVHf4q"
                                    ]
                                },
                                {
                                    "field_name": "人员", // 人员 包含 访问者本人
                                    "field_type": 11,
                                    "operator": "contains",
                                    "value": null
                                },
                                {
                                    "field_name": "", // 创建人 包含 访问者本人
                                    "field_type": 1003,
                                    "operator": "contains",
                                    "value": null
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
                                    "field_type": 11,
                                    "operator": "contains",
                                    "value": null
                                },
                                {
                                    "field_name": "多选",
                                    "field_type": 4,
                                    "operator": "is",
                                    "value": [
                                        "opttgKOTSt",
                                        "optWcdXR0W"
                                    ]
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
                        "block_type": "dashboard",
                        "block_perm": 0
                    },
                    {
                        "block_id": "blkAjxjWKvbBi7EA",
                        "block_type": "dashboard",
                        "block_perm": 1
                    }
                ]
            }
        ],
        "page_token": "rola66uIZT",
        "has_more": false,
        "total": 1
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 200 | 1254000 | WrongRequestJson | 请求体错误 |
| 200 | 1254001 | WrongRequestBody | 请求体错误 |
| 200 | 1254002 | Fail | 内部错误，有疑问可咨询客服 |
| 200 | 1254003 | WrongBaseToken | app_token 错误 |
| 200 | 1254010 | ReqConvError | 请求错误 |
| 400 | 1254032 | InvalidRoleName | 自定义角色名无效 |
| 400 | 1254033 | RoleNameDuplicated | 自定义角色名重复 |
| 400 | 1254036 | Base is copying, please try again later. | 多维表格副本复制中，稍后重试 |
| 200 | 1254040 | BaseTokenNotFound | app_token 不存在 |
| 404 | 1254047 | RoleIdNotFound | role_id 不存在 |
| 400 | 1254110 | RoleExceedLimit | 自定义角色数量超限，限制30条 |
| 200 | 1254290 | TooManyRequest | 请求过快，稍后重试 |
| 200 | 1254291 | Write conflict | 同一个数据表(table) 不支持并发调用写接口，请检查是否存在并发调用写接口。写接口包括：新增、修改、删除记录；新增、修改、删除字段；修改表单；修改视图等。 |
| 400 | 1254301 | OperationTypeError | 多维表格未开启高级权限或不支持开启高级权限 |
| 403 | 1254302 | Permission denied. | 无访问权限, 常由表格开启了高级权限造成, 请在高级权限设置中添加一个包含应用的群, 给予这个群读写权限 |
| 200 | 1255001 | InternalError | 内部错误，有疑问可咨询客服 |
| 200 | 1255002 | RpcError | 内部错误，有疑问可咨询客服 |
| 200 | 1255003 | MarshalError | 序列化错误，有疑问可咨询客服 |
| 200 | 1255004 | UmMarshalError | 反序列化错误 |
| 504 | 1255040 | 请求超时 | 进行重试 |





