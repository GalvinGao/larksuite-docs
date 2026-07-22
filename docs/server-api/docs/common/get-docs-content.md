---
document_id: '7454090400652869644'
directory_id: '7450312168455766028'
title: 获取云文档内容
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-v1/content/get
breadcrumb:
- Server API
- Docs
- Common
- Get docs content
document_type: ReferenceDocumentType
updated_at: 2025-08-15T03:03:20Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-v1/content/get
---

# 获取云文档内容

可获取云文档内容，当前只支持获取新版文档 Markdown 格式的内容。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=docs&version=v1&resource=content&method=get)

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
| HTTP URL | https://open.larksuite.com/open-apis/docs/v1/content |
| HTTP Method | GET |
| 接口频率限制 | [5 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="docs:document.content:read" desc="查看云文档内容" support_app_types="custom,isv" tags="">查看云文档内容</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




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
	<md-text type="field-name" >doc_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	云文档的唯一标识。点击[这里](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-overview)了解如何获取文档的 `doc_token`

**示例值**：B4EPdAYx8oi8HRxgPQQbM15UcBf

**数据校验规则**：

- 长度范围：`22` ～ `27` 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >doc_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	云文档类型

**示例值**：docx

**可选值有**：
<md-enum>
<md-enum-item key="docx" >新版文档</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >content_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	内容类型

**示例值**：markdown

**可选值有**：
<md-enum>
<md-enum-item key="markdown" >Markdown 格式</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >lang</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	云文档中存在 @用户 元素时，指定该用户名称的语言。默认 `zh`，即中文


**示例值**：zh

**可选值有**：
<md-enum>
<md-enum-item key="zh" >中文</md-enum-item>
<md-enum-item key="en" >英文</md-enum-item>
<md-enum-item key="ja" >日文</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::

### 请求示例
```bash 
curl --location --request GET 'https://open.larksuite.com/open-apis/docs/v1/content?doc_token=E5UEdFiudogel8x0l8WbR0abcef&doc_type=docx&content_type=markdown&lang=en' \
--header 'Authorization: Bearer u-xxx'
# 调用前请替换 'Authorization: Bearer u-xxx' 中的 'u-xxx' 为真实的访问令牌
```



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
	内容
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
        "content": "# Markdown Export Example Document\n\n# Text\n\n**Bold**\n\n*Italic*\n\n~~Strikethrough~~\n\nHyperlink: [Lark Open Platform](https%3A%2F%2Fopen.larksuite.com)\n\nInline code: `inline code`\n\nUnderline\n\nBackground color + font color\n\nPlain text with markdown syntax needs to be escaped, such as \\*\\*bold syntax\\*\\*, \\*italic syntax\\*, ~~strikethrough syntax~~, \\[hyperlink syntax\\]\\(https://open\\.larksuite\\.com\\), \\`inline code syntax\\`, etc.\n\nMentionDoc: [Markdown Export Example Document](https://example.larksuite.com/docx/FlYadoUfloTbYcxoJcccEoabcef)\n\nMentionUser: @Zhang San\n\nDate reminder: ⏰2024-04-27 15:30\n\nButton: Follow document updates open hyperlink - google\n\nInline file: \\[Markdown Export Example Document.md\\]\n\n# Bullet\n\n- **Bold**\n\n    - *Italic*\n\n    - ~~Strikethrough~~\n\n- Hyperlink: [Lark Open Platform](https%3A%2F%2Fopen.larksuite.com)\n\n    - Inline code: `inline code`\n\n- Underline\n\n    - Background color + font color\n\n# Ordered\n\n1. **Bold**\n\n    1. *Italic*\n\n    2. ~~Strikethrough~~\n\n2. Hyperlink: [Lark Open Platform](https%3A%2F%2Fopen.larksuite.com)\n\n    1. Inline code: `inline code`\n\n        1. Underline\n\n        2. Background color + font color\n\n# Code\n\n```JavaScript\nfunction greeting() {\n    console.log(\"Hello, World!\");\n}\n```\n\n# Quote\n\n> **Bold**\n> \n> *Italic*\n> \n> ~~Strikethrough~~\n> \n> Hyperlink: [Lark Open Platform](https%3A%2F%2Fopen.larksuite.com)\n> \n> Inline code: `inline code`\n> \n> Underline\n> \n> Background color + font color\n> \n\n# Task\n\n* [ ] ~~This is an incomplete task list [Markdown Export Example Document](https://example.larksuite.com/docx/FlYadoUfloTbYcxoJcccEoabcef)⏰2024-04-27 15:30~~\n\n* [x] ~~This is a completed task list~~\n\n# Divider (Horizontal Rule)\n\n---\n\n# Table\n\n<table><tbody>\n<tr>\n<td>\n\n**Location**\n\n</td>\n<td>\n\n**Features**\n\n</td>\n<td>\n\n**Cuisine**\n\n</td>\n<td>\n\n**Price**\n\n</td>\n</tr>\n<tr>\n<td>\n\nShenzhen Old Street Snacks\n\n</td>\n<td>\n\nTraditional Cuisine\n\n</td>\n<td>\n\n- Roast Meat\n\n    - Roast Duck\n\n    - Roast Goose\n\n    - Roast Pork\n\n- Dim Sum\n\n    - Rice Noodle Roll\n\n    - Glutinous Rice Chicken\n\n- Seafood\n\n    - Shark Fin\n\n    - Seafood Congee\n\n</td>\n<td>\n\nMedium\n\n</td>\n</tr>\n<tr>\n<td>\n\nDameisha Seafood Street\n\n</td>\n<td>\n\nSeafood Market\n\n</td>\n<td>\n\n- Fresh Seafood\n\n    - Lobster\n\n    - Crab\n\n    - Scallops\n\n</td>\n<td>\n\nMedium-High\n\n</td>\n</tr>\n<tr>\n<td>\n\nNanshan District Seafood Street\n\n</td>\n<td>\n\nSeafood Market\n\n</td>\n<td>\n\n- Crab Roe Bun\n\n    - Crab Roe Soup Dumpling\n\n    - Crab Roe Steamed Bun\n\n- Stir-Fried Snail Rice Noodle\n\n    - Stir-Fried Snail Rice Noodle\n\n    - Stir-Fried Rice Noodle\n\n</td>\n<td>\n\nMedium\n\n</td>\n</tr>\n<tr>\n<td>\n\nLianhua Mountain Restaurant\n\n</td>\n<td>\n\nMountain Cuisine\n\n</td>\n<td>\n\n- Wild Game\n\n    - Wild Boar Meat\n\n    - Goat Meat\n\n    - Venison\n\n- Farmer's Dish\n\n    - Farmer's Stir-Fry\n\n    - Farmer's Claypot Rice\n\n</td>\n<td>\n\nMedium-High\n\n</td>\n</tr>\n<tr>\n<td>\n\nShenzhen Huaqiangbei Food Street\n\n</td>\n<td>\n\nFood Street\n\n</td>\n<td>\n\n- Hot Pot\n\n    - Spicy Hot Pot\n\n    - Clear Soup Hot Pot\n\n- Skewered Snacks\n\n    - Spicy Skewered Snacks\n\n    - Sour and Spicy Skewered Snacks\n\n- Barbecue\n\n    - Grilled Fish\n\n    - Grilled Meat\n\n</td>\n<td>\n\nLow to Medium\n\n</td>\n</tr>\n</tbody></table>\n\n# Grid\n\n<table><tbody><tr>\n<td>\n\n**Image One**\n\n</td>\n<td>\n\n**Image Two**\n\n</td>\n<td>\n\n**Image Three**\n\n</td>\n</tr></tbody></table>\n\n# Callout\n\n<div class=\"callout\">\n\nPlain text with markdown syntax needs to be escaped, such as \\*\\*bold syntax\\*\\*, \\*italic syntax\\*, ~~strikethrough syntax~~, \\[hyperlink syntax\\]\\(https://open\\.larksuite\\.com\\), \\`inline code syntax\\`, etc.\n\n</div>\n\n# SyncedSource\n\n**Bold**\n\n*Italic*\n\n~~Strikethrough~~\n\nHyperlink: [Lark Open Platform](https%3A%2F%2Fopen.larksuite.com)\n\nInline code: `inline code`\n\nUnderline\n\n# SyncedReference\n\n# File\n\n\\[Markdown Export Example Document.md\\]\n\n# Bookmark\n\n[https://open.larksuite.com/]()\n\n# Poll\n\n# Agenda\n\nThis is the content of the agenda\n\n# Sheet\n\n# Bitable\n\n# Chart\n\n# Group Name Card\n\n# Whiteboard\n\n# Widget"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 2889901 | hybrid resource expired | 确认混部资源是否已经过期 |
| 403 | 2889902 | no permission | 确认操作者是否有该文档的阅读权限。请参考以下方式为调用身份开通文档权限：<br>- 如果你使用的是 `tenant_access_token`，意味着当前应用没有云文档权限。你需通过云文档网页页面右上方 **「...」** -> **「...更多」** ->**「添加文档应用」** 入口为应用添加权限。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/22c027f63c540592d3ca8f41d48bb107_CSas7OYJBR.png?height=1994&maxWidth=550&width=3278)<br>**注意**：在 **添加文档应用** 前，你需确保目标应用至少开通了一个云文档的 [API 权限](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/scope-list)。否则你将无法在文档应用窗口搜索到目标应用。<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9f3353931fafeea16a39f0eb887db175_0tjzC9P3zU.png?maxWidth=550)<br>- 如果你使用的是 `user_access_token`，意味着当前用户没有云文档权限。你需通过云文档网页页面右上方 **分享** 入口为当前用户添加权限。<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3e052d3bac56f9441296ae22e2969d63_a2DEYrJup8.png?height=278&maxWidth=550&width=1383)<br>了解具体操作步骤或其它添加权限方式，参考[云文档常见问题 3](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#16c6475a)。 |
| 400 | 2889904 | invalid param | 确认传入的参数是否合法 |
| 500 | 2889905 | internal error | 服务内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 403 | 2889906 | docs deleted | 确认文档是否已被删除 |
| 400 | 2889914 | doc token not found | 确认文档 token 是否正确。点击[这里](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-overview)了解如何获取文档的 token |
| 500 | 2889925 | content size exceed limit | 文档内容超过 10 MB 限制 |
| 500 | 2889980 | operation denied on copying page | 确认文档是否正在创建副本中 |





