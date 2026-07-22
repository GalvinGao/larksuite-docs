---
document_id: '7651479274432023990'
directory_id: '7307234311433109510'
title: 查询邮件发送状态
full_path: /uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/send_status
breadcrumb:
- Server API
- Email
- User Message
- Send status of user mailbox message
document_type: ReferenceDocumentType
updated_at: 2026-06-15T04:58:34Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/send_status
---

# 查询邮件发送状态

查询指定邮件的发送状态，返回邮件业务标识 ID 及各收件人的投递状态、最后更新时间。调用前需先通过发送邮件接口获取返回的邮件业务标识 ID（message_id），再使用该 ID 查询发送状态。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=mail&version=v1&resource=user_mailbox.message&method=send_status)

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
| HTTP URL | https://open.larksuite.com/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id/send_status |
| HTTP Method | GET |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="mail:user_mailbox.message:readonly" desc="查询用户邮件" support_app_types="custom" tags="">查询用户邮件</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




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
	用户邮箱地址，作为用户邮箱身份标识。可直接填写用户邮箱地址；使用 user_access_token 调用时，也可填写 me 表示当前授权用户的主邮箱。

**示例值**："aba@aac.com"
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
	邮件业务标识 ID。可通过发送邮件接口的返回值获取。

**示例值**："197c5d72e22e1d78"
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
	<md-text type="field-name" >message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件业务标识 ID，与请求路径中的 message_id 对应。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >details</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >send_status_detail\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	收件人投递状态列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >recipient</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >mail_address</md-text>
	</md-dt-td>
	<md-dt-td>
	收件人信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >mail_address</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件地址
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	投递状态

**可选值有**：
<md-enum>
<md-enum-item key="1" >正在投递</md-enum-item>
<md-enum-item key="2" >投递失败重试</md-enum-item>
<md-enum-item key="3" >投递失败退信</md-enum-item>
<md-enum-item key="4" >投递成功</md-enum-item>
<md-enum-item key="5" >待审批</md-enum-item>
<md-enum-item key="6" >审批拒绝</md-enum-item>
<md-enum-item key="0" >未知</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >last_updated_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	最后更新时间（Unix 时间戳，秒）
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
        "message_id": "197c5d72e22e1d78",
        "details": [
            {
                "recipient": {
                    "mail_address": "mike@outlook.com",
                    "name": "Mike"
                },
                "status": 1,
                "last_updated_time": 1700000000
            }
        ]
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1234008 | request parameter error | 请根据错误信息「request parameter error」核查请求参数后重试。 |
| 403 | 1234017 | permission deny | 权限不足。请确认 access_token 已开通该接口所需的 scope。 |
| 404 | 1234013 | user mailbox not found or user mailbox not active | 指定的邮箱资源不存在。请核对路径参数是否指向真实存在的资源。 |
| 500 | 1236019 | internal server error | 服务端异常。可稍后重试；若问题持续存在，请携带 request id 通过官方技术支持 https://applink.larksuite.com/TLJpeNdW 联系接口负责人。 |
| 404 | 1236010 | send status not found | 发信状态不存在。请核对路径参数是否指向真实存在的资源。 |





