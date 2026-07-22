---
document_id: '7651477917848817079'
directory_id: '7122028361539026950'
title: 撤回审批实例（用户级）
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/recall
breadcrumb:
- Server API
- Approval
- Approval instances
- Revoke approval instances (user-scoped)
document_type: ReferenceDocumentType
updated_at: 2026-06-15T04:55:04Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/recall
---

# 撤回审批实例

在符合撤销规则的情况下，你可以调用本接口将**当前用户身份提交的**的审批实例撤回。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=approval&version=v4&resource=instance&method=recall)

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

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0fa2d2e821074146781c1750e54fc7f6_FECsrbxOXW.png?height=278&maxWidth=550&width=1383)

## 注意事项

- 如果撤回的是审批中的实例，则撤回后审批流程结束。
- 如果撤回的是已通过的实例，则审批实例会变更为 **审批中** 的状态。
- 撤销规则：企业管理员在审批后台的某一审批定义的 **更多设置** 中，勾选了 **允许撤销审批中的申请** 或者 **允许撤销 x 天内通过的审批**

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/approval/v4/instances/recall |
| HTTP Method | POST |
| 接口频率限制 | [100 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="approval:instance:write" desc="操作审批实例" support_app_types="custom,isv" tags="">操作审批实例</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




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
	<md-text type="field-name" >instance_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	审批实例 Code，可通过查询任务列表接口获取到

**示例值**："81D31358-93AF-92D6-7425-01A5D67C4E71"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71"
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
| 400 | 1390001 | param is invalid | 根据接口文档的参数说明，检查请求时传入的参数是否正确 |
| 400 | 1390002 | approval code not found | 找不到审批定义 Code，检查传入的审批定义 Code 是否正确 |
| 400 | 1390003 | instance code not found | 找不到审批实例 Code，检查传入的审批实例 Code 是否正确 |
| 400 | 1390018 | not support handwritten signature | API 方式不支持手写签名功能。你需要登录Lark客户端，在客户端内处理审批。 |
| 400 | 1395001 | There have been some errors. Please try again lat | 服务出现错误，你可根据接口返回的具体消息排查问题，偶发错误可稍后重试 |
| 400 | 1395210 | Instance has been reverted | 单据已经被撤回，无法再次操作，如需提交审批请重新发起新审批 |





