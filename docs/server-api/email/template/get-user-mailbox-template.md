---
document_id: '7651479274432138678'
directory_id: '7647748462545964469'
title: 获取邮件模板
full_path: /uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-template/get
breadcrumb:
- Server API
- Email
- Template
- Get user mailbox template
document_type: ReferenceDocumentType
updated_at: 2026-06-15T04:57:39Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-template/get
---

# 获取个人邮件模板详情

获取指定邮件模板的完整详情，包括模板名称、主题、正文（HTML 或纯文本）、收件人/抄送/密送地址、附件信息等所有字段。常用于编辑模板前回填表单，或在发送邮件场景下读取模板内容做二次填充。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=mail&version=v1&resource=user_mailbox.template&method=get)

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
      <md-td>https://open.larksuite.com/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates/:template_id</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>
    <md-tr>
      <md-th>接口频率限制</md-th>
      <md-td>[5 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)</md-td>
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
            
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
            
      </md-th>
      <md-td>
            <md-perm name="mail:user_mailbox.message:modify" desc="修改邮件" support_app_types="custom" tags="">修改邮件</md-perm>
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
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
或
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

**示例值**："user@example.com"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >template_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件模板 ID。可通过列出个人邮件模板接口或创建个人邮件模板接口的返回值获取。

**示例值**："7281187859195772947"
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
	<md-text type="field-name" >template</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >template</md-text>
	</md-dt-td>
	<md-dt-td>
	模板详情
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >template_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	模板 id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	模板名称，不超过 100 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >subject</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	邮件主题，不超过 1000 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >template_content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	模板正文（HTML 或纯文本）。单模板正文大小上限 3 MB（3 \* 1024 \* 1024 字节），超过将返回错误码 1230006 template content size limit exceeded。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >is_plain_text_mode</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否为纯文本模式。`true` 表示模板正文按纯文本渲染，`false` 表示按 HTML 渲染。默认 `false`（HTML 模式）。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >tos</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >mail_address\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	默认收件人地址列表
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
	<md-text type="field-name" >ccs</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >mail_address\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	默认抄送地址列表
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
	<md-text type="field-name" >bccs</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >mail_address\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	默认密送地址列表
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
	<md-text type="field-name" >attachments</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >template_attachment\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	模板附件与内嵌图片列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
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


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	附件 id（Drive file_key，用于引用 Drive medias 上传接口返回的 file_key）
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
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


<md-dt-tr level="3">
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


<md-dt-tr level="3">
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


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >create_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	模板创建时间（毫秒级时间戳字符串，避免 JS 弱类型侧 i64 精度丢失）
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
        "template": {
            "template_id": "7281187859195772947",
            "name": "销售跟进模板",
            "subject": "关于本周订单跟进",
            "template_content": "<p>Hi ${name},</p>",
            "is_plain_text_mode": false,
            "tos": [
                {
                    "mail_address": "mike@outlook.com",
                    "name": "Mike"
                }
            ],
            "ccs": [
                {
                    "mail_address": "mike@outlook.com",
                    "name": "Mike"
                }
            ],
            "bccs": [
                {
                    "mail_address": "mike@outlook.com",
                    "name": "Mike"
                }
            ],
            "attachments": [
                {
                    "filename": "plan.xlsx",
                    "id": "boxcnrHpsg1QDqXPrJXWPwbqsKh",
                    "attachment_type": 1,
                    "is_inline": false,
                    "cid": "image1@example.com"
                }
            ],
            "create_time": "1716279320000"
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
  <md-td>1230001</md-td>
  <md-td>param invalid</md-td>
  <md-td>请根据错误信息「param invalid」核查请求参数后重试。</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1230003</md-td>
  <md-td>internal server error</md-td>
  <md-td>服务端异常（1230003 internal server error）。可稍后重试；若问题持续存在，请携带 request id 通过官方技术支持 https://applink.larksuite.com/TLJpeNdW 联系接口负责人。</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1230002</md-td>
  <md-td>permission deny</md-td>
  <md-td>权限不足（1230002 permission deny）。请确认 access_token 已开通该接口所需的 scope。</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1230007</md-td>
  <md-td>template not found</md-td>
  <md-td>资源不存在（1230007 template not found）。请核对路径参数是否指向真实存在的资源。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1230009</md-td>
  <md-td>invalid template param</md-td>
  <md-td>请根据错误信息「invalid template param」核查请求参数后重试。</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




