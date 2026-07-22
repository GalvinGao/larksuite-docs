---
document_id: '7104193248416677894'
directory_id: '7072949713020567557'
title: 获取文档纯文本内容
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/raw_content
breadcrumb:
- Server API
- Docs
- Document
- API reference
- Document
- Query Document Raw Content
document_type: ReferenceDocumentType
updated_at: 2023-09-22T08:04:31Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/raw_content
---

# 获取文档纯文本内容

获取文档的纯文本内容。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=docx&version=v1&resource=document&method=raw_content)

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
**应用频率限制**：单个应用调用频率上限为每秒 5 次，超过该频率限制，接口将返回 HTTP 状态码 <font color="blue">400</font> 及错误码 <font color="blue">99991400</font>。当请求被限频，应用需要处理限频状态码，并使用指数退避算法或其它一些频控策略降低对 API 的调用速率。
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/docx/v1/documents/:document_id/raw_content |
| HTTP Method | GET |
| 接口频率限制 | [特殊频控](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="docx:document" desc="创建及编辑新版文档" support_app_types="custom,isv" tags="">创建及编辑新版文档</md-perm><br><md-perm name="docx:document:readonly" desc="查看新版文档" support_app_types="custom,isv" tags="">查看新版文档</md-perm> |

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
	<md-text type="field-name" >document_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	文档的唯一标识。对应新版文档 Token，[点击了解如何获取云文档 Token](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)。

**示例值**："doxbcmEtbFrbbq10nPNu8gO1F3b"

**数据校验规则**：

- 长度范围：`27` ～ `27` 字符
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 查询参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;" >描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >lang</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	语言（用于 MentionUser 语言的选取）

**示例值**：0

**可选值有**：
<md-enum>
<md-enum-item key="0" >中文</md-enum-item>
<md-enum-item key="1" >英文</md-enum-item>
<md-enum-item key="2" >日文</md-enum-item>
</md-enum>

**默认值**：`0`
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
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	文档纯文本
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
        "content": "云文档\n多人实时协同，插入一切元素。不仅是在线文档，更是强大的创作和互动工具\n云文档：专为协作而生\n"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1770001 | invalid param | 确认传入的参数是否合法 |
| 404 | 1770002 | not found | 确认文档是否已被删除 |
| 400 | 1770003 | resource deleted | 确认资源是否已被删除 |
| 400 | 1770004 | too many blocks in document | 确认文档 Block 数量是否超上限 |
| 400 | 1770005 | too deep level in document | 确认文档 Block 层级是否超上限 |
| 400 | 1770006 | schema mismatch | 确认文档结构是否合法 |
| 400 | 1770007 | too many children in block | 确认指定 Block 的 Children 数量是否超上限 |
| 400 | 1770008 | too big file size | 确认上传的文件尺寸是否超上限 |
| 400 | 1770010 | too many table column | 确认表格列数是否超上限 |
| 400 | 1770011 | too many table cell | 确认表格单元格数量是否超上限 |
| 400 | 1770012 | too many grid column | 确认 Grid 列数量是否超上限 |
| 400 | 1770013 | relation mismatch | 确认图片、文件等资源的关联关系是否正确，插入图片、文件需先上传素材 |
| 400 | 1770014 | parent children relation mismatch | 确认 Block 父子关系是否正确 |
| 400 | 1770015 | single edit with multi document | 确认 Block 所属文档与指定的 Document 是否相同 |
| 400 | 1770019 | repeated blockID in document | 确认 Document 中的 BlockID 是否有重复 |
| 400 | 1770020 | operation denied on copying document | 确认 Document 是否正在创建副本中 |
| 400 | 1770021 | too old document | 确认指定的 Document 版本过旧 |
| 400 | 1770022 | invalid page token | 确认查询参数中的 page_token 是否合法 |
| 400 | 1770024 | invalid operation | 确认操作是否合法 |
| 400 | 1770025 | operation and block not match | 确认指定 Block 应用对应操作是否合法 |
| 400 | 1770026 | row operation over range | 确认行操作下标是否越界 |
| 400 | 1770027 | column operation over range | 确认列操作下标是否越界 |
| 400 | 1770028 | block not support create children | 确认指定 Block 添加 Children 是否合法 |
| 400 | 1770029 | block not support to create | 确认指定 Block 是否支持创建 |
| 400 | 1770030 | invalid parent children relation | 确认指定操作其父子关系是否合法 |
| 400 | 1770031 | block not support to delete children | 确认指定 Block 是否支持删除 Children |
| 400 | 1770033 | raw content size exceed limited | 纯文本内容大小超过限制 |
| 400 | 1770034 | operation count exceed limited | 当前请求中涉及单元格个数过多，请拆分成多次请求 |
| 403 | 1770032 | forbidden | 确认操作者是否有该文档的权限。[点击了解什么是文档权限](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#2431c595) |
| 500 | 1771001 | server internal error | 服务器内部错误 |
| 500 | 1771006 | mount folder failed | 挂载文档到云空间目录失败 |
| 500 | 1771002 | gateway server internal error | 网关服务内部错误 |
| 500 | 1771003 | gateway marshal error | 网关服务解析错误 |
| 500 | 1771004 | gateway unmarshal error | 网关服务反解析错误 |
| 503 | 1771005 | system under maintenance | 系统服务正在维护中 |





