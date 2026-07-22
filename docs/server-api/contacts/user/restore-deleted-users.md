---
document_id: '7410005333439889413'
directory_id: '6920532209305042945'
title: 恢复已删除用户
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/resurrect
breadcrumb:
- Server API
- Contacts
- User
- Restore deleted users
document_type: ReferenceDocumentType
updated_at: 2024-12-03T08:18:01Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/resurrect
---

# 恢复已删除用户

该接口用于恢复已删除用户（已离职的成员）。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=contact&version=v3&resource=user&method=resurrect)

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

## 使用限制

- 该接口仅适用于Lark专业版、旗舰版。版本信息参见[版本对比](https://www.larksuite.com/zh_cn/plans)。
- 该接口仅适用于企业自建应用，商店应用无权限调用此接口。应用类型介绍参见[应用类型简介](/document/home/app-types-introduction/overview)。

## 注意事项
- 调用该接口仅支持恢复离职 30 天内的成员。恢复后，部分用户数据仍不可恢复，请谨慎调用。
   - 可恢复的数据：单聊记录、外部联系人、群聊、企业邮箱地址和邮件；未转移的文档、妙记、问卷。
   - 不可恢复的数据：已转移的资源、成员所属部门、管理员角色等数据。
- 待恢复成员的用户 ID 不能被企业内其他成员使用。如有重复，请先离职重复 ID 的成员，否则接口会报错。
- 待恢复成员的手机号和邮箱不能被企业内其他成员使用。如有重复，请先修改重复成员的手机号或邮箱信息，否则接口会报错。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v3/users/:user_id/resurrect |
| HTTP Method | POST |
| 接口频率限制 | [10 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="contact:contact" desc="更新通讯录" support_app_types="custom" tags="">更新通讯录</md-perm> |
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
	<md-text type="field-name" >user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户 ID。ID 类型需要与查询参数中的 user_id_type类型保持一致。用户 ID 获取方式可参见[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)。

**示例值**："ou_7dab8a3d3cdcc9da365777c7ad535d62"
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

**示例值**：user_id

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
	此次调用中使用的部门 ID 类型。关于部门 ID 的详细介绍，可参见[部门 ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)。

**示例值**：department_id

**可选值有**：
<md-enum>
<md-enum-item key="department_id" >支持用户自定义配置的部门 ID。自定义配置时可复用已删除的 department_id，因此在未删除的部门范围内 department_id 具有唯一性。</md-enum-item>
<md-enum-item key="open_department_id" >由系统自动生成的部门 ID，ID 前缀固定为 od-，在租户内全局唯一。</md-enum-item>
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
	<md-text type="field-name" >departments</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >user_department_info\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户排序信息。用户可能存在多个部门中，且有不同的排序，该参数用于设置用户部门排序。

**说明**：如果请求时不传入 departments 参数，则用户将恢复至根部门。

**数据校验规则**：

- 最大长度：`50`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >department_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	排序信息对应的部门 ID。表示用户所在的、且需要排序的部门。部门 ID 类型与查询参数 `department_id_type` 保持一致。

了解不同类型的部门 ID 以及获取部门 ID 的方式，可参见 [部门 ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)。

**示例值**："od-4e6ac4d14bcd5071a37a39de902c7141"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >user_order</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户在其直属部门内的排序。数值越大，排序越靠前。

**示例值**：0
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >department_order</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户所属的多个部门之间的排序。数值越大，排序越靠前。

**示例值**：0
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >subscription_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	如果用户正常状态时分配了席位，则可以通过该参数指定恢复后分配的席位 ID。

**注意**：
- 当在[混合license模式](/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant-product_assign_info/query)下，该字段为必填。
- 该字段需开通 **分配用户席位** 权限。

**示例值**：["23213213213123123"]
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "departments": [
        {
            "department_id": "od-4e6ac4d14bcd5071a37a39de902c7141",
            "user_order": 0,
            "department_order": 0
        }
    ],
    "subscription_ids": [
        "23213213213123123"
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
| 400 | 44028 | Exceed recoverable time | 超过可恢复时间。当前传入的用户对应的离职时间已超过 30 天，不可恢复。 |
| 400 | 44029 | No access to resurrect | 无法恢复。需申请接口的使用权限，请联系[技术支持](https://applink.larksuite.com/client/helpdesk/open)。 |
| 400 | 44030 | Mobile duplicated | 手机号重复。你可以选择修改用户重复的手机号，或者将手机号重复的用户转为离职状态，然后重试。 |
| 400 | 44031 | Email duplicated | 邮箱重复。你可以选择修改用户重复的邮箱，或者将邮箱重复的用户转为离职状态，然后重试。 |
| 400 | 44032 | UserID duplicated | 用户的 user_id 重复。待恢复用户的 user_id 不能被租户内其他用户占用。如有重复，请先离职占用 user_id 的在职用户，再尝试恢复操作。<br>**注意**： user_id 重复的情况下，不可使用 user_id 请求本接口。请使用待恢复用户的 open_id 或 union_id 请求本接口。获取用户 ID 可参见[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)。 |
| 400 | 44033 | User not resigned | 用户在职无需恢复。如果你使用的 user_id 请求本接口，则需要检查请求参数传入的 user_id 是否已被在职用户使用。如果已被使用，请先离职占用 user_id 的在职用户，再尝试恢复操作。<br>**注意**： user_id 重复的情况下，不可使用 user_id 请求本接口。请使用待恢复用户的 open_id 或 union_id 请求本接口。获取用户 ID 可参见[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)。 |
| 400 | 44034 | User is in delete progress, retry later | 用户离职流程仍在进行中，请等待至多 48 小时后重试。 |
| 400 | 41008 | exceed bill seat limit error | 租户成员数目超过系统限制，用户无法恢复。你需要清理租户内的无效成员或者扩容Lark成员数量限制。详情咨询[技术支持](https://applink.larksuite.com/TLJpeNdW)。 |
| 400 | 41007 | exceed uncertain tenant seat limit error | 未认证租户的成员数目超过系统限制，用户无法恢复。你需要清理租户内的无效成员或者扩容Lark成员数量限制。详情咨询[技术支持](https://applink.larksuite.com/TLJpeNdW)。 |
| 400 | 44023 | exceed feature contact seat limit | 超出通讯录用户资源上限。如果需要更多用户额度，请升级Lark版本。了解更多参见[版本对比](https://www.larksuite.com/zh_cn/plans)。 |
| 400 | 44041 | anonymize user info is not allowed to update | 已匿名的用户信息不允许更新。 |
| 400 | 44046 | user license subscription id must not empty in multi-license tenant | 多 License 的租户内，席位 ID 不可为空。你需要在请求时设置 subscription_ids 参数。你可以调用[获取企业席位信息接口](/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant-product_assign_info/query)接口，获取正确可用的席位 ID。 |
| 400 | 44047 | license subscription id exceed limit | 该席位 ID 已超过使用上限。请修改为可用的席位 ID。你可以调用[获取企业席位信息接口](/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant-product_assign_info/query)接口，获取正确可用的席位 ID。 |
| 400 | 44048 | user license subscription id invalid | 席位 ID 无效。你可以调用[获取企业席位信息接口](/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant-product_assign_info/query)接口，获取正确可用的席位 ID。 |
| 403 | 44050 | not set subscription ids auth | 席位 ID 未开通权限。你需要在 [开发者后台](https://open.larksuite.com/app) > 应用详情页 > 权限管理 > API 权限中，开通 **分配用户席位** 权限。 |


更多错误码信息，参见[通用错误码](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)。


