---
document_id: '7651479274432007606'
directory_id: '7145371477523644421'
title: 列出可访问的邮箱
full_path: /uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox/accessible_mailboxes
breadcrumb:
- Server API
- Email
- User Mailbox
- List Accessible Mailboxes
document_type: ReferenceDocumentType
updated_at: 2026-06-15T04:58:25Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox/accessible_mailboxes
---

# 列出可访问的邮箱

列出可访问的邮箱，包括拥有读信和发信权限的主账号、公共邮箱{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=mail&version=v1&resource=user_mailbox&method=accessible_mailboxes)

:::html
<md-alert type="tip">

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
| HTTP URL | https://open.larksuite.com/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/accessible_mailboxes |
| HTTP Method | GET |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="mail:user_mailbox" desc="查询、创建、修改和删除用户的企业邮箱" support_app_types="custom" tags="">查询、创建、修改和删除用户的企业邮箱</md-perm><br><md-perm name="mail:user_mailbox:readonly" desc="查询用户的企业邮箱" support_app_types="custom" tags="">查询用户的企业邮箱</md-perm> |

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
	<md-text type="field-name" >user_mailbox_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户邮箱地址，作为用户邮箱身份标识。使用 user_access_token 调用时，可使用占位符 `me` 表示当前授权用户的主邮箱。 注意：不支持使用公共邮箱访问此接口。

**示例值**："user@example.com"
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
	<md-text type="field-name" >accessible_mailboxes</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >email_info\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	用户可访问的所有邮箱信息，包含主邮箱和公共邮箱
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >email_address</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮箱地址
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >email_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮箱地址类型

**可选值有**：
<md-enum>
<md-enum-item key="MAIL_GROUP" >邮件组</md-enum-item>
<md-enum-item key="PUBLIC_MAILBOX" >公共邮箱</md-enum-item>
<md-enum-item key="USER_PRIMARY" >用户主地址</md-enum-item>
<md-enum-item key="USER_ALIAS" >用户别名</md-enum-item>
<md-enum-item key="PUBLIC_MAILBOX_ALIAS" >公共邮箱别名</md-enum-item>
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
    "code": 0,
    "msg": "success",
    "data": {
        "accessible_mailboxes": [
            {
                "email_address": "abc@abc.com",
                "email_type": "USER_PRIMARY"
            }
        ]
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1230001 | param is invalid, check user_mailbox_id | 参数错误，请检查user_mailbox_id参数是否正确 |
| 403 | 1230004 | permission denied: mailbox not found, unsupported mailbox type, or no access to this mailbox | 权限不足：邮箱不存在、邮箱类型不支持，或无权访问该邮箱，请检查user_mailbox_id参数和权限申请状态 |
| 500 | 1230003 | internal server error, please retry later | 服务器内部错误，请稍后重试 |
| 400 | 1230005 | tenant access token not support me | 使用 tenant_access_token 调用时，不支持使用 `me` 作为 user_mailbox_id。请传入完整的具体用户邮箱地址，例如 user@example.com。 |





