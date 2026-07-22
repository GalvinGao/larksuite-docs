---
document_id: '7278988143918514182'
directory_id: '6907567266537291777'
title: 更新应用可用范围
full_path: /uAjLw4CM/ukTMukTMukTM/application-v6/application-visibility/patch
breadcrumb:
- Server API
- App Information
- Admin
- Update app availability
document_type: ReferenceDocumentType
updated_at: 2023-09-18T06:45:05Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-visibility/patch
---

# 更新应用可用范围

该接口用于更新当前企业内自建应用或已安装的商店应用的可见范围，包括可用人员与禁用人员。更新后对线上立即生效。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=application&version=v6&resource=application.visibility&method=patch)

:::html
<md-alert type="tip">
当通过接口新增用户或部门时，提前判断对应用户或部门是否已在禁用名单中，如果已在禁用名单中，则即便将用户或部门添加到可用名单，该用户或部门也无法看到该应用，即禁用名单优先级高于可用名单。
</md-alert>
:::

:::html
<md-alert type="warn">
同一个成员(user_id)在30s内不能重复添加到禁用名单，否则会导致调用失败
</md-alert>
:::

:::html
<md-alert type="error">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/application/v6/applications/:app_id/visibility |
| HTTP Method | PATCH |
| 接口频率限制 | [20 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="admin:app.visibility" desc="管理应用可见范围" support_app_types="custom" tags="">管理应用可见范围</md-perm> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
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
	<md-text type="field-name" >app_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	应用id

**示例值**："cli_9b445f5258795107"
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
	<md-text type="field-name" >department_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门id 类型

**示例值**：open_department_id

**可选值有**：
<md-enum>
<md-enum-item key="open_department_id" >以open_department_id标识部门</md-enum-item>
<md-enum-item key="department_id" >以department_id标识部门</md-enum-item>
</md-enum>

**默认值**：`open_department_id`
	</md-dt-td>
</md-dt-tr>


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

**示例值**：open_id

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
	<md-text type="field-name" >add_visible_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app_visibility_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	添加可用人员列表，如果参数is_visible_to_all不设置且当前已经是全员可见，或者参数is_visible_to_all设置为true，则该参数不生效
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >user_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	成员id列表 id类型根据user_id_type参数指定

**示例值**：["ou_84aad35d084aa403a838cf73ee18467"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >department_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门id列表 id类型根据department_id_type参数指定

**示例值**：["od-4e6ac4d14bcd5071a37a39de902c7141"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >group_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户组id列表

**示例值**：["g193821"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >del_visible_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app_visibility_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	删除可用人员列表，如果参数is_visible_to_all不设置且当前已经是全员可见，或者参数is_visible_to_all设置为true，则该参数不生效
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >user_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	成员id列表 id类型根据user_id_type参数指定

**示例值**：["ou_84aad35d084aa403a838cf73ee18467"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >department_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门id列表 id类型根据department_id_type参数指定

**示例值**：["od-4e6ac4d14bcd5071a37a39de902c7141"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >group_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户组id

**示例值**：["g193821"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >add_invisible_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app_visibility_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	添加禁用人员列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >user_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	成员id列表 id类型根据user_id_type参数指定

相同的成员不能在30s内重复添加到禁用列表，否则会导致调用失败

**示例值**：["ou_84aad35d084aa403a838cf73ee18467"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >department_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门id列表 id类型根据department_id_type参数指定

**示例值**：["od-4e6ac4d14bcd5071a37a39de902c7141"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >group_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户组id列表

**示例值**：["g193821"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >del_invisible_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app_visibility_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	删除禁用人员列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >user_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	成员id列表 id类型根据user_id_type参数指定

**示例值**：["ou_84aad35d084aa403a838cf73ee18467"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >department_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	部门id列表 id类型根据department_id_type参数指定

**示例值**：["od-4e6ac4d14bcd5071a37a39de902c7141"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >group_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户组id列表

**示例值**：["g193821"]

**数据校验规则**：

- 最大长度：`100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >is_visible_to_all</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	是否全员可见

false：否

true：是

不设置：继续保持当前状态不改变


如果参数不设置且当前已经是全员可见，或者设置为true，则add_visible_list/del_visible_list不生效

**示例值**：false
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "add_visible_list": {
        "user_ids": [
            "ou_84aad35d084aa403a838cf73ee18467"
        ],
        "department_ids": [
            "od-4e6ac4d14bcd5071a37a39de902c7141"
        ],
        "group_ids": [
            "g193821"
        ]
    },
    "del_visible_list": {
        "user_ids": [
            "ou_84aad35d084aa403a838cf73ee18467"
        ],
        "department_ids": [
            "od-4e6ac4d14bcd5071a37a39de902c7141"
        ],
        "group_ids": [
            "g193821"
        ]
    },
    "add_invisible_list": {
        "user_ids": [
            "ou_84aad35d084aa403a838cf73ee18467"
        ],
        "department_ids": [
            "od-4e6ac4d14bcd5071a37a39de902c7141"
        ],
        "group_ids": [
            "g193821"
        ]
    },
    "del_invisible_list": {
        "user_ids": [
            "ou_84aad35d084aa403a838cf73ee18467"
        ],
        "department_ids": [
            "od-4e6ac4d14bcd5071a37a39de902c7141"
        ],
        "group_ids": [
            "g193821"
        ]
    },
    "is_visible_to_all": false
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
| 400 | 210001 | invalid request | 检查请求参数是否正确 |
| 200 | 210002 | invalid app_id or app not exists | 检查app_id是否存在或者企业是否安装了该应用 |
| 200 | 210003 | please check if param is empty or if there is conflicts between add and del list | 检查参数add_visible_list/del_visible 或 add_invisible_list/del_invisible_list 中的元素是否有冲突 |
| 200 | 210004 | server internal error | 服务内部错误 |
| 200 | 210005 | invalid group_ids | 检查group_id 是否有效 |
| 200 | 210006 | can not modify visibility of special app | 检查要修改的应用是否为企业管理后台无法修改可用性的应用 |





