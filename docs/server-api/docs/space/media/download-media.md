---
document_id: '7031483915610292229'
directory_id: '7026178505877667845'
title: 下载素材
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/download
breadcrumb:
- Server API
- Docs
- Space
- Media
- Download Media
document_type: ReferenceDocumentType
updated_at: 2024-05-16T11:03:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/download
---

# 下载素材

使用该接口可以下载素材。素材表示在各种创作容器里的文件，如Doc文档内的图片，文件均属于素材。支持range下载。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=media&method=download)

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
该接口不支持太高的并发，且调用频率上限为5QPS
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/v1/medias/:file_token/download |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm><br><md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv" tags="">查看、评论和导出多维表格</md-perm><br><md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm><br><md-perm name="docs:doc:readonly" desc="查看、评论和导出文档" support_app_types="custom,isv" tags="">查看、评论和导出文档</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="moments:moments" desc="查询、创建、修改和删除公司圈内容、板块" support_app_types="custom,isv" tags="">查询、创建、修改和删除公司圈内容、板块</md-perm><br><md-perm name="moments:moments:readonly" desc="查询公司圈内容、板块" support_app_types="custom,isv" tags="">查询公司圈内容、板块</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">查看、评论和导出电子表格</md-perm><br><md-perm name="vc:material" desc="更新视频会议特效素材" support_app_types="custom,isv" tags="">更新视频会议特效素材</md-perm><br><md-perm name="vc:material:readonly" desc="获取视频会议特效素材" support_app_types="custom,isv" tags="">获取视频会议特效素材</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::
### 部分下载

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Range | string | 否 | 指定文件下载部分，示例:bytes=0-1024 |


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
	<md-text type="field-name" >file_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	文件的 token，获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)

**示例值**："boxcnrHpsg1QDqXAAAyachabcef"
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
	<md-text type="field-name" >extra</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	拥有高级权限的多维表格在下载素材时，需要添加额外的扩展信息作为 URL 查询参数鉴权。详情参考[上传点类型及对应Extra说明](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/introduction)。未填正确填写该参数的接口将返回 403 的 HTTP 状态码。

**示例值**："无"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





## 响应

### 响应头

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >content-type</md-text> | <md-text type="field-type" >string</md-text> | 文件的MIME |
| <md-text type="field-name" >content-disposition</md-text> | <md-text type="field-type" >string</md-text> | 文件名 |


HTTP状态码为 200 时，表示成功

返回文件二进制流




