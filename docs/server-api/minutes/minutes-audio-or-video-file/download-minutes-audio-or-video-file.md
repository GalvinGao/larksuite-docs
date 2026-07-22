---
document_id: '7372150552220876805'
directory_id: '7344989060279271429'
title: 下载妙记音视频文件
full_path: /uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-media/get
breadcrumb:
- Server API
- Minutes
- Minutes audio or video file
- Download minutes audio or video file
document_type: ReferenceDocumentType
updated_at: 2025-04-07T11:16:30Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-media/get
---

# 下载妙记音视频文件

获取妙记的音视频文件{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=minutes&version=v1&resource=minute.media&method=get)

:::html
<md-alert type="tip">
通过接口获取妙记音视频文件的下载链接（有效期1天），以进行批量下载
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
| HTTP URL | https://open.larksuite.com/open-apis/minutes/v1/minutes/:minute_token/media |
| HTTP Method | GET |
| 接口频率限制 | [5 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="minutes:minute:download" desc="导出妙记文件" support_app_types="custom,isv" tags="">导出妙记文件</md-perm><br><md-perm name="minutes:minutes.media:export" desc="下载妙记的音视频文件" support_app_types="custom" tags="">下载妙记的音视频文件</md-perm> |

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
	<md-text type="field-name" >minute_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	妙记唯一标识

**示例值**："obcnq3b9jl72l83w4f149w9c"

**数据校验规则**：

- 长度范围：`24` ～ `24` 字符
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
	<md-text type="field-name" >download_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	妙记音视频文件下载链接
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
        "download_url": "https://internal-api-drive-stream.larksuite-boe.com/space/api/box/stream/download/authcode/?code=xxx"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 2091001 | param is invalid | 检查参数是否正确 |
| 404 | 2091002 | resource not found | 无法找到对应妙记，检查Token是否正确 |
| 400 | 2091003 | minute not ready , try later | 检查妙记是否转写完成 |
| 400 | 2091004 | resource deleted | 检查妙记是否已被删除 |
| 403 | 2091005 | permission deny | 检查是否有该篇妙记的阅读权限 |
| 500 | 2091006 | service internal error | 服务器出错，请稍后重试 |





