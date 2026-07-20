---
document_id: '7651479274432171446'
directory_id: '7647748462545931701'
title: 发送草稿
full_path: /uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-draft/send
breadcrumb:
- Server API
- Email
- Draft
- Send draft
document_type: ReferenceDocumentType
updated_at: 2026-06-15T04:58:05Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-draft/send
---

# 发送草稿

发送指定草稿，并生成对应的已发送邮件和邮件会话。适用于在创建或更新草稿后触发实际发送，发送成功后返回已发送邮件 ID 和所属会话 ID。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=mail&version=v1&resource=user_mailbox.draft&method=send)

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
      <md-td>https://open.larksuite.com/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts/:draft_id/send</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    <md-tr>
      <md-th>接口频率限制</md-th>
      <md-td>[20 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)</md-td>
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
            <md-perm name="mail:user_mailbox.message:send" desc="发送用户邮件" support_app_types="custom" tags="">发送用户邮件</md-perm>
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
<md-tag mode="inline" type="token-user">user_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"

[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)

</md-td>
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
	<md-text type="field-name" >user_mailbox_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户邮箱地址。当使用用户身份访问时，可以输入"me"代表当前调用接口用户

**示例值**："aba@aac.com"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >draft_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	草稿ID，可通过创建草稿、更新草稿或列出草稿列表接口获得

**示例值**："268dce11-85f7-427d-8756-6be3abc850fd"
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
	发送后生成的已发送邮件ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >thread_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件所属会话ID
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
        "message_id": "197c5d72e22e1d79",
        "thread_id": "197c5d72e22e1d78"
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
  <md-td>1234008</md-td>
  <md-td>request parameter error</md-td>
  <md-td>参数错误，请检查请求参数的类型、格式或值是否与接口要求一致，具体可参考接口文档中的参数说明</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1236003</md-td>
  <md-td>the number of recipients exceeds the limit</md-td>
  <md-td>收件人数量超过限制，请减少收件人后重试</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1236004</md-td>
  <md-td>the number of attachments exceeds the limit</md-td>
  <md-td>附件数量超过限制，请减少附件数量后重试</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1236014</md-td>
  <md-td>content risk</md-td>
  <md-td>草稿存在风险内容，请检查草稿中的内容是否包含违规信息（如敏感词、违法内容等），修改后重新提交</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1236017</md-td>
  <md-td>sender check fail</md-td>
  <md-td>邮件发件人检查失败，请检查发件人信息和状态</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1236018</md-td>
  <md-td>receiver check fail</md-td>
  <md-td>邮件收件人检查失败，请检查收件人信息</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1234017</md-td>
  <md-td>permission deny</md-td>
  <md-td>无权限访问，请确认应用是否具备访问该资源的权限。如使用用户身份访问，请确认具备此用户的访问权限；如使用租户身份访问，请确认已申请对应的数据范围权限。</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1234050</md-td>
  <md-td>draft not found</md-td>
  <md-td>指定草稿不存在，请核对草稿ID，或重新创建草稿</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1234013</md-td>
  <md-td>user mailbox not found or user mailbox not active</md-td>
  <md-td>用户邮箱地址不存在，请检查输入的用户邮箱地址是否正确，或确认用户的邮箱处于正常状态</md-td>
</md-tr>


<md-tr>
  <md-td>409</md-td>
  <md-td>1236005</md-td>
  <md-td>send mail repeatedly</md-td>
  <md-td>邮件已发送成功，请勿重复发送邮件</md-td>
</md-tr>


<md-tr>
  <md-td>429</md-td>
  <md-td>1236006</md-td>
  <md-td>Concurrent write conflict. Please retry later</md-td>
  <md-td>存在并发写冲突。同用户请勿并发请求，请稍后重试。</md-td>
</md-tr>


<md-tr>
  <md-td>429</md-td>
  <md-td>1236007</md-td>
  <md-td>the daily number of emails sent by the user exceeds the limit</md-td>
  <md-td>触达用户每日发信数量上限，请联系管理员提升限额后重试</md-td>
</md-tr>


<md-tr>
  <md-td>429</md-td>
  <md-td>1236008</md-td>
  <md-td>the number of external recipients the user sends messages to each day exceeds the limit</md-td>
  <md-td>用户每天发送邮件的外部收件人数量超过限制，请联系管理员提升限额后重试</md-td>
</md-tr>


<md-tr>
  <md-td>429</md-td>
  <md-td>1236009</md-td>
  <md-td>the number of external recipients the tenant sends messages to each day exceeds the limit</md-td>
  <md-td>企业每天发送邮件的外部收件人数量超过限制，请联系管理员提升限额后重试</md-td>
</md-tr>


<md-tr>
  <md-td>429</md-td>
  <md-td>1236010</md-td>
  <md-td>mail quota limit</md-td>
  <md-td>用户的发信请求被系统限流，请重试</md-td>
</md-tr>


<md-tr>
  <md-td>429</md-td>
  <md-td>1236013</md-td>
  <md-td>tenant storage limit</md-td>
  <md-td>租户存储空间已满，无法发送更多邮件，联系管理员提升空间后重试</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1236019</md-td>
  <md-td>internal server error</md-td>
  <md-td>内部服务错误，请重试</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




