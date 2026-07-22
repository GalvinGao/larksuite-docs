---
document_id: '7393239659645698054'
directory_id: '7050040770682814470'
title: 批量移除用户组成员
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/batch_remove
breadcrumb:
- Server API
- Contacts
- User group
- User group member
- Batch remove user group members
document_type: ReferenceDocumentType
updated_at: 2024-07-19T07:05:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/batch_remove
---

# 批量移除用户组成员

调用该接口从指定普通用户组内移除一个或多个成员。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=contact&version=v3&resource=group.member&method=batch_remove)

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

## 注意事项

- 目前仅支持移除用户类型的成员，暂不支持移除部门类型的成员。

- 如果应用的通讯录权限范围是 **全部员工**，则可以将任何用户移除任何用户组。如果应用的通讯录权限范围不是 **全部员工**，则所要移除的用户以及对应的用户组，均需要在应用的通讯录权限范围内。了解通讯录权限范围，可参见[权限范围资源介绍](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v3/group/:group_id/member/batch_remove |
| HTTP Method | POST |
| 接口频率限制 | [100 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="contact:group" desc="更新用户组信息" support_app_types="custom" tags="">更新用户组信息</md-perm> |

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
	<md-text type="field-name" >group_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户组 ID。

用户组 ID 可在创建用户组时从返回值中获取，你也可以调用[查询用户组列表](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/simplelist)接口，获取用户组的 ID。

**示例值**："test_group"
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
	<md-text type="field-name" >members</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >memberlist\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	待移除成员信息。

**数据校验规则**：

- 长度范围：`1` ～ `100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >member_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	移除的用户 ID，ID 类型与 member_id_type 的取值保持一致。

你可以调用[查询用户组成员列表](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/simplelist)接口，获取用户组内的成员 ID，并将需要移除的成员 ID 传入当前参数。注意仅支持移除用户类型的成员，且需要使用相同的用户 ID 类型，否则会报错。

**示例值**："u287xj12"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >member_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	用户组成员的类型，目前仅支持选择 user，表示用户类型。

**示例值**："user"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >member_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	当 `member_type` 取值为 `user`时，该参数必填，需通过该参数设置用户 ID 类型，包括：

- open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。
- union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。
- user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用中都保持一致。User ID 主要用于在不同的应用间打通用户数据。

**示例值**："user_id"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "members": [
        {
            "member_id": "u287xj12",
            "member_type": "user",
            "member_id_type": "user_id"
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
| 400 | 40001 | param error | 参数错误。请检查请求时传入的参数是否有误。 |
| 500 | 40003 | internal error | 内部错误，请获取请求的 X-Request-Id，并向[技术支持](https://applink.larksuite.com/TLJpeNdW)进行反馈。 |
| 400 | 40022 | invalid member_id_type | 无效的成员 ID 类型。你需要按照接口文档内 member_id_type 的参数描述设置正确的值。 |
| 400 | 41073 | invalid member_id | 成员 ID 无效。你需要参考接口文档的 member_id 参数描述，传入正确的成员 ID。 |
| 400 | 41074 | invalid member_type | 无效的成员类型。你需要按照接口文档内 member_type 的参数描述设置正确的值。 |
| 400 | 42002 | invalid group_id | 用户组 ID 无效。你可以调用[查询用户组列表](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/simplelist)接口，获取准确的用户组 ID。 |
| 403 | 42009 | no group authority | 缺少用户组权限。应用的通讯录权限范围需包含当前操作的用户组。了解应用的通讯录权限范围，可参见[权限范围资源介绍](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)。 |
| 403 | 41050 | no user authority error | 缺少用户权限。应用的通讯录权限范围需要包含当前操作的用户。了解应用的通讯录权限范围，可参见[权限范围资源介绍](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)。 |


更多错误码信息，参见[通用错误码](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)。


