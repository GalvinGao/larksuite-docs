---
document_id: '7530972827513700358'
directory_id: '7527931135172411398'
title: 列出工作流
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-block_workflow/list
breadcrumb:
- Server API
- Docs
- Base
- Workflow
- List Workflows
document_type: ReferenceDocumentType
updated_at: 2026-02-12T02:49:34Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-block_workflow/list
---

# 列出工作流

此接口用于返回多维表格中所有工作流，多维表格管理员可通过此接口来管理表中的工作流{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app.block_workflow&method=list)

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
| HTTP URL | https://open.larksuite.com/open-apis/bitable/v1/apps/:app_token/block_workflows |
| HTTP Method | GET |
| 接口频率限制 | [20 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="base:workflow:read" desc="查看自动化流程/工作流" support_app_types="custom,isv" tags="">查看自动化流程/工作流</md-perm><br><md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm> |

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
	<md-text type="field-name" >app_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格 App 的唯一标识。不同形态的多维表格，其 app_token 的获取方式不同：
- 如果多维表格的 URL 以 ==**feishu.cn/base**== 开头，该多维表格的 app_token 是下图高亮部分：
    ![app_token.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6916f8cfac4045ba6585b90e3afdfb0a_GxbfkJHZBa.png?height=766&lazyload=true&width=3004)

- 如果多维表格的 URL 以 ==**feishu.cn/wiki**== 开头，你需调用知识库相关[获取知识空间节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)接口获取多维表格的 app_token。当 obj_type 的值为 bitable 时，obj_token 字段的值才是多维表格的 app_token。

了解更多，参考[多维表格 app_token 获取方式](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/bitable-overview#-752212c)。

**示例值**："U9sGw5wyoiOIqdk1C4mcbYmMnbt"

**数据校验规则**：

- 长度范围：`1` ～ `200` 字符
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
	<md-text type="field-name" >workflows</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >block_workflow\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	工作流列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >workflow_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	工作流唯一键
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >title</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	工作流标题
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	工作流状态

**可选值有**：
<md-enum>
<md-enum-item key="Enable" >启用</md-enum-item>
<md-enum-item key="Disable" >禁用</md-enum-item>
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
        "workflows": [
            {
                "workflow_id": "12412312421312",
                "title": "工作流",
                "status": "Enable"
            }
        ]
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1254000 | WrongRequestJson | 请求体格式错误。请检查请求体格式是否符合 JSON 规范 |
| 400 | 1254001 | WrongRequestBody | 请求体参数有误。请检查请求体中是否已传入所有必填参数 |
| 400 | 1254002 | Fail | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 400 | 1254003 | WrongBaseToken | app_token 错误，请检查是否从链接中截取了正确的 token 值 |
| 400 | 1254004 | WrongTableId | table_id 错误，请检查是否从链接中截取了正确的 table id 值 |
| 400 | 1254005 | WrongViewId | view id 错误，请检查是否从链接中截取了正确的 view id 值 |
| 400 | 1254006 | WrongRecordId | record id 错误，请检查是否从页面上获取正确的 record id |
| 400 | 1254007 | EmptyValue | 空值，请检查输入参数值 |
| 400 | 1254008 | EmptyView | 空视图，请检查查询的视图是否合法 |
| 400 | 1254009 | WrongFieldId | field id 错误，请检查 field id 是否合法 |
| 400 | 1254010 | ReqConvError | 请检查请求参数的类型或格式是否与接口要求一致，请检查请求参数是否有误 |
| 400 | 1254015 | Field types do not match. | 字段类型不匹配，请检查字段类型是否与多维表格中的字段类型一致 |
| 403 | 1254027 | UploadAttachNotAllowed | 附件未挂载, 禁止写入，请先上传并挂载附件 |
| 400 | 1254030 | InvalidPageToken | page token 错误，请从返回体中取出合法的 next page token 来作为请求的 page token |
| 400 | 1254036 | Bitable is copying, please try again later. | 操作的多维表格是一个副本，正在复制中，请稍后重试 |
| 400 | 1254037 | Invalid client token, make sure that it complies with the specification. | 幂等键为空，请检查幂等键是否为空 |
| 400 | 1254040 | BaseTokenNotFound | 多维表格 Token 不存在，请检查传入的 app token 是否合法 |
| 400 | 1254041 | TableIdNotFound | table_id 不存在，请检查传入的 table id 是否合法 |
| 400 | 1254042 | ViewIdNotFound | view id 不存在，请检查传入的 view id 是否合法 |
| 400 | 1254043 | RecordIdNotFound | record id 不存在，请检查传入的 record id 是否合法 |
| 400 | 1254044 | FieldIdNotFound | field id 不存在，请检查传入的 field id 是否存在 |
| 400 | 1254045 | FieldNameNotFound | 字段名称不存在，请检查传入的字段名称是否合法 |
| 400 | 1254060 | TextFieldConvFail | 多行文本单元格格式错误，请检查多行文本字段的格式或值是否符合要求 |





