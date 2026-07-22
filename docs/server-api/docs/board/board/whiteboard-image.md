---
document_id: '7493459220906786822'
directory_id: '7491887374051393542'
title: 获取画板缩略图片
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/download_as_image
breadcrumb:
- Server API
- Docs
- Board
- Board
- whiteboard image
document_type: ReferenceDocumentType
updated_at: 2026-05-25T08:16:12Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/download_as_image
---

# 下载画板为图片

获取画板的缩略图片，响应数据为图片的二进制图片流。根据 Content-Type 值区图片格式：image/png、image/jpeg、image/gif、image/svg+xml。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=board&version=v1&resource=whiteboard&method=download_as_image)

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
| HTTP URL | https://open.larksuite.com/open-apis/board/v1/whiteboards/:whiteboard_id/download_as_image |
| HTTP Method | GET |
| 接口频率限制 | [50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="board:whiteboard:node:read" desc="查看画板节点" support_app_types="custom,isv" tags="">查看画板节点</md-perm> |

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
	<md-text type="field-name" >whiteboard_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	画板唯一标识。可通过文档接口 [获取文档所有块](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list) 获取，`block_type` 为 43 的 block 即为画板，对应的 <code>block.token</code> 就是画板的<code>whiteboard_id</code>。


**示例值**："Ru8nwrWFOhEmaFbEU2VbPRsHcxb"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





## 响应



HTTP状态码为 200 时，表示成功

返回文件二进制流



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 2890001 | invalid format | 参数格式不正确。请检查传入的参数格式 |
| 400 | 2890002 | invalid arg | 参数无效。请检查传入的参数是否有效 |
| 400 | 2890003 | record missing | 找不到记录。`whiteboard_id` 不存在或图片不存在。请检查传入的`whiteboard_id`是否正确以及是否存在（可通过[获取文档所有块](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list)接口获取正确的`whiteboard_id`） |
| 401 | 2890004 | auth failed | 认证失败。请检查 Authorization 参数 |
| 403 | 2890005 | forbidden | 请求身份没有当前画板的阅读权限。请参考[云文档常见问题 3](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#78a03ee2)开通权限 |
| 429 | 2890006 | too many request | 请求超过接口频率限流值。请稍后再试 |
| 500 | 2891001 | server internal error | 服务运行错误。请重试或联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |





