---
document_id: '7643718224995339700'
directory_id: '7491887374051377158'
title: 解析画板语法
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard-node/create_plantuml
breadcrumb:
- Server API
- Docs
- Board
- nodes
- whiteboard syntactic parse
document_type: ReferenceDocumentType
updated_at: 2026-05-25T08:16:17Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard-node/create_plantuml
---

# 解析画板语法

用户可以将PlantUml/Mermaid图表导入画板进行协同编辑{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=board&version=v1&resource=whiteboard.node&method=create_plantuml)

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

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/board/v1/whiteboards/:whiteboard_id/nodes/plantuml |
| HTTP Method | POST |
| 接口频率限制 | [5 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="board:whiteboard:node:create" desc="创建画板节点" support_app_types="custom,isv" tags="">创建画板节点</md-perm> |

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
	<md-text type="field-name" >whiteboard_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	画板唯一标识，可通过云文档下的文档接口 [获取文档所有块](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list) 获取，`block_type` 为 43 的 block 即为画板，对应的 <code>block.token</code> 就是画板的<code>whiteboard_id</code>

**示例值**："VF5Bwo7Z5icC0bk8EWbb57Vbckh"

**数据校验规则**：

- 长度范围：`22` ～ `27` 字符
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
	<md-text type="field-name" >plant_uml_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	plant uml 代码

**示例值**："@startuml\nAlice -> Bob: Authentication Request\nBob --> Alice: Authentication Response\n@enduml"

**数据校验规则**：

- 长度范围：`1` ～ `1000000` 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >style_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	画板样式（默认为2 经典样式）

**示例值**：1

**可选值有**：
<md-enum>
<md-enum-item key="1" >画板样式（解析之后为多个画板节点，粘贴到画板中，不可对语法进行二次编辑）</md-enum-item>
<md-enum-item key="2" >经典样式（解析之后为一张图片，粘贴到画板中，可对语法进行二次编辑）（只有PlantUml语法支持经典样式）</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >syntax_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	语法类型（必传）

**示例值**：1

**可选值有**：
<md-enum>
<md-enum-item key="0" >未知</md-enum-item>
<md-enum-item key="1" >Plantuml解析</md-enum-item>
<md-enum-item key="2" >Mermaid解析</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >diagram_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	PlantUml语法类型（传0会自动识别语法类型，plantUML语法补充超集GML不可自动识别）
当syntax_type为2（Mermaid解析）时，diagram_type传 0， 默认为 0

**示例值**：0

**可选值有**：
<md-enum>
<md-enum-item key="0" >未知</md-enum-item>
<md-enum-item key="1" >思维导图</md-enum-item>
<md-enum-item key="2" >时序图</md-enum-item>
<md-enum-item key="3" >活动图</md-enum-item>
<md-enum-item key="4" >类图</md-enum-item>
<md-enum-item key="5" >ER</md-enum-item>
<md-enum-item key="6" >流程图</md-enum-item>
<md-enum-item key="7" >用例图</md-enum-item>
<md-enum-item key="8" >组件图</md-enum-item>
<md-enum-item key="101" >ai流式生成流程图</md-enum-item>
<md-enum-item key="102" >ai流式生成时序图</md-enum-item>
<md-enum-item key="201" >plantUML语法补充超集GML</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "plant_uml_code": "@startuml\nAlice -> Bob: Authentication Request\nBob --> Alice: Authentication Response\n@enduml",
    "style_type": 1,
    "syntax_type": 1,
    "diagram_type": 0
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
	<md-text type="field-name" >node_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	创建生成的plant uml节点id
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
        "node_id": "t1:1"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 2890001 | invalid format | 参数格式不正确。请检查传入的参数格式，如 json 字符串是否正确 |
| 400 | 2890002 | invalid arg | 参数无效。请检查传入的参数是否有效，如 whiteboard_id 是否正确 |
| 400 | 2890003 | record missing | 找不到记录。`whiteboard_id` 不存在，确认 whiteboard_id 正确性，可通过云文档下的文档接口[获取文档所有块](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list)获取正确的`whiteboard_id` |
| 401 | 2890004 | auth failed | 认证失败。请检查 Authorization 参数 |
| 403 | 2890005 | forbidden | 请求身份没有当前画板的阅读权限。请参考[云文档常见问题 3](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#78a03ee2)开通权限 |
| 429 | 2890006 | too many request | 请求超过接口频率限流值。请稍后再试 |
| 500 | 2891001 | server internal error | 服务运行错误。请重试或联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |





