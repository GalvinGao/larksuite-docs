---
document_id: '7651479274432040374'
directory_id: '7307234311433109510'
title: 修改邮件
full_path: /uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/modify
breadcrumb:
- Server API
- Email
- User Message
- Modify Mail Message
document_type: ReferenceDocumentType
updated_at: 2026-06-15T04:58:44Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/modify
---

# 修改邮件

修改邮件标签、所属文件夹、已读未读状态，可为邮件添加旗标、归档、移入垃圾邮件等操作。不支持移动邮件到已删除文件夹，如需，请使用删除邮件接口。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=mail&version=v1&resource=user_mailbox.message&method=modify)

:::html
<md-alert type="tip">
不支持移动邮件到已删除文件夹，如需，请使用删除邮件接口。
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
| HTTP URL | https://open.larksuite.com/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id/modify |
| HTTP Method | PUT |
| 接口频率限制 | [5 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="mail:user_mailbox.message:modify" desc="修改邮件" support_app_types="custom" tags="">修改邮件</md-perm> |

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
	<md-text type="field-name" >user_mailbox_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户邮箱地址，作为用户邮箱身份标识。可通过获取用户邮箱信息接口获取用户主邮箱地址；使用 user_access_token 调用时，也可使用占位符 me 表示当前授权用户的主邮箱。

**示例值**："abc@abc.com"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件ID，可通过列出邮件接口、收信事件通知等方式获得

**示例值**："bskfsxxcvve="

**数据校验规则**：

- 长度范围：`0` ～ `200` 字符
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
	<md-text type="field-name" >add_label_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	待添加的标签 ID 列表。可选值包括 UNREAD、IMPORTANT、OTHER、FLAGGED，以及自定义标签 ID。该参数与 remove_label_ids、add_folder 均为可选且可组合；与 remove_label_ids 同时设置时，会在同一次请求中分别添加和移除对应标签，请勿在两个列表中传入同一标签 ID。

**示例值**：["UNREAD"]

**数据校验规则**：

- 长度范围：`0` ～ `20`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >remove_label_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	待移除的标签 ID 列表。可选值包括 UNREAD、IMPORTANT、OTHER、FLAGGED，以及自定义标签 ID。该参数与 add_label_ids、add_folder 均为可选且可组合；与 add_label_ids 同时设置时，会在同一次请求中分别移除和添加对应标签，请勿在两个列表中传入同一标签 ID。

**示例值**：["UNREAD"]

**数据校验规则**：

- 长度范围：`0` ～ `20`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >add_folder</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	需要移入的文件夹 ID。支持 INBOX、SENT、SPAM、ARCHIVED 以及自定义文件夹 ID。该参数与 add_label_ids、remove_label_ids 均为可选且可组合；设置后会在同一次请求中将邮件移动到指定文件夹，若仅需调整标签可不传该字段。

**示例值**："INBOX"

**数据校验规则**：

- 长度范围：`0` ～ `100` 字符
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "add_label_ids": [
        "UNREAD"
    ],
    "remove_label_ids": [
        "UNREAD"
    ],
    "add_folder": "INBOX"
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
| 400 | 1230001 | param is invalid | 请求参数错误。请检查本接口的路径参数 user_mailbox_id、message_id，以及请求体 add_label_ids、remove_label_ids、add_folder 的取值、格式和枚举范围后重试；标签 ID 或文件夹 ID 可通过对应的标签列表、文件夹列表接口获取。 |
| 500 | 1230002 | Internal error | 服务器内部错误，请稍后重试 |
| 429 | 1236006 | Concurrent write conflict. Please retry later | 无法并发修改邮件，请稍后重试 |
| 400 | 1230007 | message not exist | 指定邮件不存在，请检查邮件参数 |
| 400 | 1236008 | label or folder not exist | 指定标签或文件夹不存在，请检查参数 |
| 403 | 1230009 | permission denied | 无权限访问，请检查权限申请状态 |





