---
document_id: '7132687771717861381'
directory_id: '7072190414392459270'
title: 创建知识空间
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/create
breadcrumb:
- Server API
- Docs
- Wiki
- space
- Create Wiki space
document_type: ReferenceDocumentType
updated_at: 2022-09-28T09:41:53Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/create
---

# 创建知识空间

此接口用于创建知识空间{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=wiki&version=v2&resource=space&method=create)

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
此接口不支持tenant access token（应用身份访问）
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
      <md-td>https://open.larksuite.com/open-apis/wiki/v2/spaces</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom,isv"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
      </md-th>
      <md-td>
            <md-perm name="wiki:wiki" desc="查看、编辑和管理知识库" support_app_types="custom,isv" tags="">查看、编辑和管理知识库</md-perm>
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

[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)

</md-td>
</md-tr>
<md-tr>
<md-td>Content-Type</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
</md-tbody>
</md-table>
:::



### 请求体

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 40%;">名称</md-dt-th>
      <md-dt-th style="width: 20%;">类型</md-dt-th>
      <md-dt-th style="width: 10%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 30%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	知识空间名称

**示例值**："知识空间"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >description</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	知识空间描述

**示例值**："知识空间描述"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 请求体示例
:::html
<md-code-json>
{
    "name": "知识空间",
    "description": "知识空间描述"
}
</md-code-json>
:::



## 响应



### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 40%;">名称</md-dt-th>
      <md-dt-th style="width: 20%;">类型</md-dt-th>
      <md-dt-th style="width: 30%;">描述</md-dt-th>
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
	<md-text type="field-name" >space</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >space</md-text>
	</md-dt-td>
	<md-dt-td>
	知识空间
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
	知识空间名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >description</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	知识空间描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >space_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	知识空间id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >space_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	表示知识空间类型（团队空间 或 个人空间）

**可选值有**：
<md-enum>
<md-enum-item key="team" >团队空间</md-enum-item>
<md-enum-item key="person" >个人空间</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >visibility</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	表示知识空间可见性（公开空间 或 私有空间）

**可选值有**：
<md-enum>
<md-enum-item key="public" >公开空间</md-enum-item>
<md-enum-item key="private" >私有空间</md-enum-item>
</md-enum>
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
        "space": {
            "name": "知识空间",
            "description": "知识空间描述",
            "space_id": "1565676577122621"
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
  <md-td>131001</md-td>
  <md-td>rpc fail</md-td>
  <md-td>调用下游错误，拿返回值的header头里的x-tt-logid咨询oncall定位。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>131002</md-td>
  <md-td>param err</md-td>
  <md-td>通常为传参有误，例如数据类型不匹配。请查看**具体接口报错信息**，报错不明确时请咨询oncall。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>131003</md-td>
  <md-td>out of limit</md-td>
  <md-td>超出操作限制，例如节点数量限制。请参阅下表。
- 原/目标知识空间总节点数不超过40万。
- 原/目标知识空间目录树不超过50层。
- 目的父节点下单层节点数不超过2000。
- 单次移动节点数（带子节点）不超过2000。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>131004</md-td>
  <md-td>invalid user</md-td>
  <md-td>非法用户。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>131005</md-td>
  <md-td>not found</md-td>
  <md-td>未找到相关数据，例如id不存在。相关报错信息参考：

- member not found：用户不是知识空间成员（管理员），无法删除。
- identity not found: userid不存在，无法添加/删除成员。
- space not found：知识空间不存在
- node not found：节点不存在
- document not found：文档不存在


报错不明确时请咨询oncall。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>131006</md-td>
  <md-td>permission denied</md-td>
  <md-td>权限拒绝，相关报错信息参考：
- wiki space permission denied：需要为知识空间成员（管理员）
- node permission denied：读操作时需要有节点阅读权限。写操作（创建、移动等）需要节点容器编辑权限。
- no source parent node permission：需要原父节点容器编辑权限。
- no destination parent node permission：需要目的父节点容器编辑权限。
- only task creator can query status：为任务创建者（用户或应用/机器人）

如果使用tenant_access_token调用，请确保应用/机器人为知识空间成员。参阅[如何将应用添加为知识库管理员（成员）](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/wiki-qa#b5da330b)。

当接口报错信息不明确时，请咨询oncall。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>131007</md-td>
  <md-td>internal err</md-td>
  <md-td>内部错误，拿返回值的header头里的x-tt-logid定位</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




