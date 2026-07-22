---
document_id: '7202139792681910278'
directory_id: '7202139961926336517'
title: 获取地理位置列表
full_path: /uAjLw4CM/ukTMukTMukTM/security_and_compliance-v1/multi_geo_entity-tenant/get
breadcrumb:
- Server API
- security_and_compliance
- Data Residency
- Get a list of geographic locations
document_type: ReferenceDocumentType
updated_at: 2025-07-17T06:48:55Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v1/multi_geo_entity-tenant/get
---

# 获取地理位置列表

获取企业可使用的地理位置列表。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=security_and_compliance&version=v1&resource=multi_geo_entity.tenant&method=get)

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



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/security_and_compliance/v1/multi_geo_entity/tenant |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="security_and_compliance:multi_geo_entity.tenant:readonly" desc="查看数据驻留租户信息" support_app_types="custom" tags="">查看数据驻留租户信息</md-perm><br><md-perm name="security_and_compliance:user_migration:multi-geo" desc="查询、更新员工的数据驻留地" support_app_types="custom" tags="">查询、更新员工的数据驻留地</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |






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
	<md-text type="field-name" >tenant</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >tenant</md-text>
	</md-dt-td>
	<md-dt-td>
	数据驻留租户信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >available_geo_locations</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	可选地理位置列表
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
        "tenant": {
            "available_geo_locations": [
                "us"
            ]
        }
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1781001 | 请求参数无效 | 修正请求参数 |
| 403 | 1781002 | 操作人没有数据驻留服务操作权限 | 在管理员后台为操作人开通数据驻留服务操作权限，参考: https://www.larksuite.com/hc/zh-CN/articles/360043595213 |
| 400 | 1781003 | 租户未开通数据驻留服务 | 需联系服务台技术支持开通「数据驻留服务」 |
| 500 | 1782001 | 服务端内部报错 | 寻求客服帮助 |





