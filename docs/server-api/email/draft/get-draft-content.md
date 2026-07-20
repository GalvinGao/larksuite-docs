---
document_id: '7651479274431958454'
directory_id: '7647748462545931701'
title: 获取草稿内容
full_path: /uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-draft/get
breadcrumb:
- Server API
- Email
- Draft
- Get Draft Content
document_type: ReferenceDocumentType
updated_at: 2026-06-15T04:58:10Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-draft/get
---

# 获取草稿内容

根据草稿 ID 获取指定草稿的详细内容，包括草稿所属邮件、主题、正文、收件人、抄送人、密送人、附件和安全信息等。适用于在发送前回显草稿内容或进入编辑页面前加载草稿详情。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=mail&version=v1&resource=user_mailbox.draft&method=get)

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
      <md-td>https://open.larksuite.com/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts/:draft_id</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>
    <md-tr>
      <md-th>接口频率限制</md-th>
      <md-td>[10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)</md-td>
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
            <md-perm name="mail:user_mailbox.message:readonly" desc="查询用户邮件" support_app_types="custom" tags="">查询用户邮件</md-perm>
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
	用户邮箱地址，作为用户邮箱身份标识。可通过获取用户邮箱信息接口获取用户主邮箱地址；使用 user_access_token 调用时，也可使用占位符 me 表示当前授权用户的主邮箱。

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
	草稿 ID。可通过列出草稿列表接口获取。

**示例值**："268dce11-85f7-427d-8756-6be3abc850fd"
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
	<md-text type="field-name" >format</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	需要获取的草稿内容样式，取值：metadata / full（默认）/ raw

**示例值**：full

**可选值有**：
<md-enum>
<md-enum-item key="metadata" >草稿元数据信息，包括邮件摘要、主题、收发件人等信息</md-enum-item>
<md-enum-item key="raw" >获取草稿EML</md-enum-item>
<md-enum-item key="full" >邮件全文，获取包括纯文本、HTML等在内的邮件全文信息</md-enum-item>
</md-enum>
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
	<md-text type="field-name" >draft</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >draft</md-text>
	</md-dt-td>
	<md-dt-td>
	草稿内容
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	草稿ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >message</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >message</md-text>
	</md-dt-td>
	<md-dt-td>
	草稿内容
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >subject</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	主题
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >to</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >mail_address\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	收件人
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
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


<md-dt-tr level="4">
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


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >cc</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >mail_address\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	抄送
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
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


<md-dt-tr level="4">
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


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >bcc</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >mail_address\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	秘送
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
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


<md-dt-tr level="4">
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


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >head_from</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >mail_address</md-text>
	</md-dt-td>
	<md-dt-td>
	发件人
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
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


<md-dt-tr level="4">
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


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >body_html</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	正文(base64url)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >internal_date</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	创建/收/发信时间（毫秒）
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >message_state</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件状态，1（收信）2（发信）3（草稿）
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >smtp_message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	RFC协议id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >attachments</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >attachment\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件附件列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >filename</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	附件文件名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	附件 id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >attachment_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	附件类型

**可选值有**：
<md-enum>
<md-enum-item key="1" >普通附件</md-enum-item>
<md-enum-item key="2" >超大附件</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >is_inline</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否为内联图片，true 表示是内联图片
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >cid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	内容 ID，HTML 中通过 cid: 协议引用该图片
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >body_plain_text</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	正文纯文本(base64url)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >thread_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	会话id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >body_preview</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件正文纯文本内容的前100个字符，基于base64url编码，用于快速预览邮件核心内容，无需解码完整正文
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >label_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	标签ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >folder_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	文件夹ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >in_reply_to</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	In-Reply-To邮件头
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >reply_to</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Reply-To邮件头
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >priority_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件优先级

**可选值有**：
<md-enum>
<md-enum-item key="0" >无优先级</md-enum-item>
<md-enum-item key="1" >高优先级</md-enum-item>
<md-enum-item key="3" >正常优先级</md-enum-item>
<md-enum-item key="5" >低优先级</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >security_level</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >security_level</md-text>
	</md-dt-td>
	<md-dt-td>
	安全信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >is_risk</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否风险邮件
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >risk_banner_level</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	风险邮件等级

**可选值有**：
<md-enum>
<md-enum-item key="WARNING" >警告</md-enum-item>
<md-enum-item key="DANGER" >危险</md-enum-item>
<md-enum-item key="INFO" >提示</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >risk_banner_reason</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	风险邮件原因

**可选值有**：
<md-enum>
<md-enum-item key="NO_REASON" >未知</md-enum-item>
<md-enum-item key="IMPERSONATE_DOMAIN" >相似域名仿冒</md-enum-item>
<md-enum-item key="IMPERSONATE_KP_NAME" >KP姓名仿冒</md-enum-item>
<md-enum-item key="UNAUTH_EXTERNAL" >未认证外部域名</md-enum-item>
<md-enum-item key="MALICIOUS_URL" >恶意链接</md-enum-item>
<md-enum-item key="MALICIOUS_ATTACHMENT" >高危附件</md-enum-item>
<md-enum-item key="PHISHING" >钓鱼邮件</md-enum-item>
<md-enum-item key="IMPERSONATE_PARTNER" >仿冒合作伙伴</md-enum-item>
<md-enum-item key="EXTERNAL_ENCRYPTION_ATTACHMENT" >外部邮件携带加密附件</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >is_header_from_external</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	发件人是否外部邮件
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >via_domain</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	代发或伪造邮件展示SPF或DKIM域名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >spam_banner_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	垃圾邮件原因

**可选值有**：
<md-enum>
<md-enum-item key="USER_REPORT" >用户曾标记邮件是垃圾邮件</md-enum-item>
<md-enum-item key="USER_BLOCK" >用户曾将发件人的邮件标记为垃圾邮件</md-enum-item>
<md-enum-item key="ANTI_SPAM" >系统判为垃圾邮件</md-enum-item>
<md-enum-item key="USER_RULE" >命中收信规则进入垃圾邮件</md-enum-item>
<md-enum-item key="BLOCK_DOMIN" >用户已拦截来自该域名的邮件</md-enum-item>
<md-enum-item key="BLOCK_ADDRESS" >用户已拦截来自该邮件地址的邮件</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >spam_user_rule_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	命中的收信规则ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >spam_banner_info</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	命中用户黑名单的地址或域名信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >references</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	References邮件头
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
        "draft": {
            "id": "268dce11-85f7-427d-8756-6be3abc850fd",
            "message": {
                "subject": "邮件标题",
                "to": [
                    {
                        "mail_address": "mike@outlook.com",
                        "name": "Mike"
                    }
                ],
                "cc": [
                    {
                        "mail_address": "mike@outlook.com",
                        "name": "Mike"
                    }
                ],
                "bcc": [
                    {
                        "mail_address": "mike@outlook.com",
                        "name": "Mike"
                    }
                ],
                "head_from": {
                    "mail_address": "mike@outlook.com",
                    "name": "Mike"
                },
                "body_html": "PHA-SGVsbG8sIHRoaXMgaXMgYSBkcmFmdCBlbWFpbC48L3A-",
                "internal_date": "1682377086000",
                "message_state": 1,
                "smtp_message_id": "ay0azrJDvbs3FJAg@outlook.com",
                "message_id": "tfuh9N4WnzU6jdDw=",
                "attachments": [
                    {
                        "filename": "helloworld.txt",
                        "id": "YQqYbQHoQoDqXjxWKhJbo8Gicjf",
                        "attachment_type": 1,
                        "is_inline": false,
                        "cid": "image1@example.com"
                    }
                ],
                "body_plain_text": "SGVsbG8sIHRoaXMgaXMgYSBkcmFmdCBlbWFpbC4",
                "thread_id": "tfuh9N4WnzU6jdDw=",
                "body_preview": "Hello, this is a draft email.",
                "label_ids": [
                    "FLAGGED"
                ],
                "folder_id": "INBOX",
                "in_reply_to": "06d20.dbf451a3.808a.475a.acc9.1363dfd20f36@larksuite.com",
                "reply_to": "06d20.dbf451a3.808a.475a.acc9.1363dfd20f36@larksuite.com",
                "priority_type": "0",
                "security_level": {
                    "is_risk": false,
                    "risk_banner_level": "WARNING",
                    "risk_banner_reason": "IMPERSONATE_DOMAIN",
                    "is_header_from_external": false,
                    "via_domain": "larksuite.com",
                    "spam_banner_type": "USER_REPORT",
                    "spam_user_rule_id": "7618365627924925388",
                    "spam_banner_info": "larksuite.com"
                },
                "references": "<5678.abcd@test.com>\r\n\t<1234.abcd@message-id>"
            }
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
  <md-td>1234008</md-td>
  <md-td>request parameter error</md-td>
  <md-td>参数错误，请检查请求参数的类型、格式或值是否与接口要求一致，具体可参考接口文档中的参数说明</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1234017</md-td>
  <md-td>permission deny</md-td>
  <md-td>权限不足（1234017 permission deny）。请确认应用已申请并开通该接口所需权限，并使用包含对应 scope 的 access_token 调用。</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1234050</md-td>
  <md-td>draft not found</md-td>
  <md-td>草稿不存在（1234050 draft not found）。请确认传入的 draft_id 是否正确；可通过列出草稿列表接口获取正确的草稿 ID。</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1234013</md-td>
  <md-td>user mailbox not found or user mailbox not active</md-td>
  <md-td>用户邮箱不存在或未激活（1234013 user mailbox not found or user mailbox not active）。请确认传入的 user_mailbox_id 是否为真实存在且已激活的用户邮箱地址。</md-td>
</md-tr>


<md-tr>
  <md-td>429</md-td>
  <md-td>1236006</md-td>
  <md-td>too many request</md-td>
  <md-td>请求被限流，请勿并发调用，请重试</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1236019</md-td>
  <md-td>internal server error</md-td>
  <md-td>服务端异常（1236019 internal server error）。请稍后重试；若问题持续存在，请携带 request id 通过官方技术支持 https://applink.larksuite.com/TLJpeNdW 联系接口负责人。</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




