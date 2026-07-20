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
      <md-td>https://open.larksuite.com/open-apis/security_and_compliance/v1/multi_geo_entity/tenant</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
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
            <md-perm name="security_and_compliance:multi_geo_entity.tenant:readonly" desc="查看数据驻留租户信息" support_app_types="custom" tags="">查看数据驻留租户信息</md-perm>
            <md-perm name="security_and_compliance:user_migration:multi-geo" desc="查询、更新员工的数据驻留地" support_app_types="custom" tags="">查询、更新员工的数据驻留地</md-perm>
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
  <md-td>1781001</md-td>
  <md-td>请求参数无效</md-td>
  <md-td>修正请求参数</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1781002</md-td>
  <md-td>操作人没有数据驻留服务操作权限</md-td>
  <md-td>在管理员后台为操作人开通数据驻留服务操作权限，参考: https://www.larksuite.com/hc/zh-CN/articles/360043595213</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1781003</md-td>
  <md-td>租户未开通数据驻留服务</md-td>
  <md-td>需联系服务台技术支持开通「数据驻留服务」</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1782001</md-td>
  <md-td>服务端内部报错</md-td>
  <md-td>寻求客服帮助</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




