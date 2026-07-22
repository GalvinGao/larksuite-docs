---
document_id: '7277487675921006598'
directory_id: '7277448822627106822'
title: 将自定义字段移出资源
full_path: /uAjLw4CM/ukTMukTMukTM/task-v2/custom_field/remove
breadcrumb:
- Server API
- Tasks
- Custom Field
- Remove Custom Field From Resource
document_type: ReferenceDocumentType
updated_at: 2023-11-09T09:22:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field/remove
---

# 将自定义字段移出资源

将自定义字段从资源中移出。移除后，该资源将无法再使用该字段。目前资源的类型支持"tasklist"。

如果要移除自定义字段本来就不存在于资源，本接口将正常返回。

注意自定义字段是通过清单来实现授权的，如果将自定义字段从所有关联的清单中移除，就意味着任何调用身份都无法再访问改自定义字段。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=task&version=v2&resource=custom_field&method=remove)

:::html
<md-alert type="tip">
需要资源的可编辑权限。
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
| HTTP URL | https://open.larksuite.com/open-apis/task/v2/custom_fields/:custom_field_guid/remove |
| HTTP Method | POST |
| 接口频率限制 | [100 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="task:custom_field:write" desc="查看、创建、更新自定义字段" support_app_types="custom,isv" tags="">查看、创建、更新自定义字段</md-perm> |

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
	<md-text type="field-name" >custom_field_guid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	自定义字段GUID。自定义字段GUID。可以通过[创建自定义字段](/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field/create)接口创建, 或者通过[列取自定义字段](/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field/list)接口查询得到。

**示例值**："0110a4bd-f24b-4a93-8c1a-1732b94f9593"
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
	<md-text type="field-name" >resource_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	要从某个资源移除自定义字段的资源类型，目前只支持清单。

**示例值**："tasklist"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >resource_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	要从某个资源移除自定义字段的资源id，`resource_type`为"tasklist"时，需填写清单的GUID。

**示例值**："0110a4bd-f24b-4a93-8c1a-1732b94f9593"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "resource_type": "tasklist",
    "resource_id": "0110a4bd-f24b-4a93-8c1a-1732b94f9593"
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
| 400 | 1470400 | 请求参数错误，如向resource_type传入了不支持的类型。 | 错误原因见返回的msg提示的信息。 |
| 403 | 1470403 | 缺少将自定义字段从资源移出的权限。 | 确认调用身份拥有资源的编辑权限。 |
| 404 | 1470404 | 要被移除的自定义字或者要移出的资源不存在或已删除。 | 确认要被移除的自定义字或者要移出的资源是否存在或已删除。 |
| 500 | 1470500 | 服务器错误。 | 尝试重试调用。如持续失败，请联系支持人员进行反馈。 |





