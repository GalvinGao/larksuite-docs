---
document_id: '7298981006978449414'
directory_id: '7296056091455537158'
title: 删除附件
full_path: /uAjLw4CM/ukTMukTMukTM/task-v2/attachment/delete
breadcrumb:
- Server API
- Tasks
- attachment
- Delete Attachment
document_type: ReferenceDocumentType
updated_at: 2023-11-09T09:21:53Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/task-v2/attachment/delete
---

# 删除附件

提供一个附件GUID，删除该附件。删除后该附件不可再恢复。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=task&version=v2&resource=attachment&method=delete)

:::html
<md-alert type="tip">
删除附件需要调用身份拥有被删除附件所属资源的的编辑权限，或者调用身份就是附件的上传人。
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
| HTTP URL | https://open.larksuite.com/open-apis/task/v2/attachments/:attachment_guid |
| HTTP Method | DELETE |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="task:attachment:write" desc="查看、创建、删除任务附件" support_app_types="custom,isv" tags="">查看、创建、删除任务附件</md-perm> |

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
	<md-text type="field-name" >attachment_guid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	要删除附件的GUID。可以通过创建[上传附件](/document/uAjLw4CM/ukTMukTMukTM/task-v2/attachment/upload)接口创建, 或者通过[列取附件](/document/uAjLw4CM/ukTMukTMukTM/task-v2/attachment/list)接口查询得到。

**示例值**："b59aa7a3-e98c-4830-8273-cbb29f89b837"

**数据校验规则**：

- 最大长度：`100` 字符
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
| 400 | 1470400 | 请求参数有误。 | 查看返回中的`msg`确定具体原因。 |
| 404 | 1470404 | 资源不存在或已删除。 | 确认附件是否还存在。 |
| 500 | 1470500 | 服务器错误。 | 使用同样请求重试调用接口。如果持续返回错误，可以联系技术支持排查问题。 |
| 403 | 1470403 | 缺少删除附件的权限。 | 确认调用身份拥有删除附件的权限。 |





