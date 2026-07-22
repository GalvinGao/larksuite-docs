---
document_id: '7132687771718057989'
directory_id: '7072190414392328198'
title: 更新知识空间设置
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-setting/update
breadcrumb:
- Server API
- Docs
- Wiki
- Setting
- Update Wiki space settings
document_type: ReferenceDocumentType
updated_at: 2022-08-17T03:51:24Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-setting/update
---

# 更新知识空间设置

根据space_id更新知识空间公共设置{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=wiki&version=v2&resource=space.setting&method=update)

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
知识库权限要求：
- 为知识空间管理员
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/wiki/v2/spaces/:space_id/setting |
| HTTP Method | PUT |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="wiki:wiki" desc="查看、编辑和管理知识库" support_app_types="custom,isv" tags="">查看、编辑和管理知识库</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
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
	<md-text type="field-name" >space_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	知识空间id

**示例值**："1565676577122621"
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
      <md-dt-th style="width: 40%;">名称</md-dt-th>
      <md-dt-th style="width: 20%;">类型</md-dt-th>
      <md-dt-th style="width: 10%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 30%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >create_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	谁可以创建空间的一级页面： "admin_and_member" = 管理员和成员 "admin"  - 仅管理员

**示例值**："admin/admin_and_member"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >security_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	可阅读用户可否创建副本/打印/导出/复制： "allow" - 允许 "not_allow" - 不允许

**示例值**："allow/not_allow"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >comment_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	可阅读用户可否评论： "allow" - 允许 "not_allow" - 不允许

**示例值**："allow/not_allow"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 请求体示例
:::html
<md-code-json>
{
    "create_setting": "admin/admin_and_member",
    "security_setting": "allow/not_allow",
    "comment_setting": "allow/not_allow"
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
	<md-text type="field-name" >setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >setting</md-text>
	</md-dt-td>
	<md-dt-td>
	知识空间设置
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >create_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	谁可以创建空间的一级页面： "admin_and_member" = 管理员和成员 "admin"  - 仅管理员
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >security_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	可阅读用户可否创建副本/打印/导出/复制： "allow" - 允许 "not_allow" - 不允许
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >comment_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	可阅读用户可否评论： "allow" - 允许 "not_allow" - 不允许
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
        "setting": {
            "create_setting": "admin/admin_and_member",
            "security_setting": "allow/not_allow",
            "comment_setting": "allow/not_allow"
        }
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 131001 | rpc fail | 调用下游错误，拿返回值的header头里的x-tt-logid咨询oncall定位。 |
| 400 | 131002 | param err | 通常为传参有误，例如数据类型不匹配。请查看**具体接口报错信息**，报错不明确时请咨询oncall。 |
| 400 | 131003 | out of limit | 超出操作限制，例如节点数量限制。请参阅下表。<br>- 原/目标知识空间总节点数不超过40万。<br>- 原/目标知识空间目录树不超过50层。<br>- 目的父节点下单层节点数不超过2000。<br>- 单次移动节点数（带子节点）不超过2000。 |
| 400 | 131004 | invalid user | 非法用户。 |
| 400 | 131005 | not found | 未找到相关数据，例如id不存在。相关报错信息参考：<br>- member not found：用户不是知识空间成员（管理员），无法删除。<br>- identity not found: userid不存在，无法添加/删除成员。<br>- space not found：知识空间不存在<br>- node not found：节点不存在<br>- document not found：文档不存在<br>报错不明确时请咨询oncall。 |
| 400 | 131006 | permission denied | 权限拒绝，相关报错信息参考：<br>- wiki space permission denied：需要为知识空间成员（管理员）<br>- node permission denied：读操作时需要有节点阅读权限。写操作（创建、移动等）需要节点容器编辑权限。<br>- no source parent node permission：需要原父节点容器编辑权限。<br>- no destination parent node permission：需要目的父节点容器编辑权限。<br>- only task creator can query status：为任务创建者（用户或应用/机器人）<br>如果使用tenant_access_token调用，请确保应用/机器人为知识空间成员。参阅[如何将应用添加为知识库管理员（成员）](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/wiki-qa#b5da330b)。<br>当接口报错信息不明确时，请咨询oncall。 |
| 400 | 131007 | internal err | 内部错误，拿返回值的header头里的x-tt-logid定位 |
| 400 | 131008 | already exist | 数据已存在，请勿重复操作。 |





