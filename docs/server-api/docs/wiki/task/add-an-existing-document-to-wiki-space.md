---
document_id: '7132687771718123525'
directory_id: '7072190414392213510'
title: 添加已有云文档至知识库
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/move_docs_to_wiki
breadcrumb:
- Server API
- Docs
- Wiki
- task
- Add an existing document to Wiki space
document_type: ReferenceDocumentType
updated_at: 2022-08-17T03:51:24Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/move_docs_to_wiki
---

# 添加已有云文档至知识库

该接口允许添加已有云文档至知识库，并挂载在指定父页面下{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=wiki&version=v2&resource=space.node&method=move_docs_to_wiki)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">
### 移动操作 ###
移动后，文档将从“我的空间”或“共享空间”转移至“知识库”，并将从以下功能入口消失：
- 云空间主页：最近访问、快速访问
- 我的空间
- 共享空间
- 收藏

### 权限变更 ###
移动后，文档会向所有可查看“页面树”的用户显示，默认继承父页面的权限设置。
</md-alert
</md-alert>
:::

:::html
<md-alert type="tip">
仅支持文档所有者发起请求

此接口为异步接口。若移动已完成（或节点已在Wiki中），则直接返回结果（Wiki token）。若尚未完成，则返回task id。请使用[获取任务结果](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/task/get)接口进行查询。

知识库权限要求：
- 文档可管理权限
- 原文件夹编辑权限
- 目标父节点容器编辑权限
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/wiki/v2/spaces/:space_id/nodes/move_docs_to_wiki |
| HTTP Method | POST |
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
	知识库id

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
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >parent_wiki_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	节点的父亲token

**示例值**："wikbcOHIFxB0PJS2UTd2kF2SP6c"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >obj_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	文档类型

**示例值**："doc"

**可选值有**：
<md-enum>
<md-enum-item key="doc" >doc（文档）</md-enum-item>
<md-enum-item key="sheet" >sheet（表格）</md-enum-item>
<md-enum-item key="bitable" >bitable（多维表格）</md-enum-item>
<md-enum-item key="mindnote" >mindnote（思维导图）</md-enum-item>
<md-enum-item key="docx" >docx</md-enum-item>
<md-enum-item key="file" >file (文件)</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >obj_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	文档token

**示例值**："docbc6e1qBqt1O5mCBVA1QUKVEg"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >apply</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	没有权限时，是否申请迁入文档

**示例值**：true
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 请求体示例
:::html
<md-code-json>
{
    "parent_wiki_token": "wikbcOHIFxB0PJS2UTd2kF2SP6c",
    "obj_type": "doc",
    "obj_token": "docbc6e1qBqt1O5mCBVA1QUKVEg"
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


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >wiki_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	移动后的知识库token
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >task_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	任务id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >applied</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否提交了文档迁入申请
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
// 操作已完成
{
    "code": 0,
    "data": {
        "wiki_token": "wikbcLZuhp4r9QuJumHzV2fzF7T"
    },
    "msg": "success"
}
// 或者操作尚未完成
{
    "code": 0,
    "data": {
        "task_id": "7037044037068177428-075c9481e6a0007c1df689dfbe5b55a08b6b06f7"
    },
    "msg": "success"
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





