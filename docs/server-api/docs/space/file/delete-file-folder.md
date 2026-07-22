---
document_id: '7031483915610472453'
directory_id: '7031445675029561349'
title: 删除文件/文件夹
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete
breadcrumb:
- Server API
- Docs
- Space
- File
- Delete File/Folder
document_type: ReferenceDocumentType
updated_at: 2023-08-03T02:02:23Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete
---

# 删除文件

删除用户在云空间内的文件或者文件夹。文件或者文件夹被删除后，会进入用户回收站里。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=file&method=delete)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">
要删除文件需要确保应用具有下述两种权限之一：
1. 该应用是文件所有者并且具有该文件所在父文件夹的编辑权限。
2. 该应用并非文件所有者，但是是该文件所在父文件夹的所有者或者拥有该父文件夹的所有权限（full access）。
</md-alert>
:::

:::html
<md-alert type="tip">
该接口不支持并发调用，且调用频率上限为5QPS。删除文件夹会异步执行并返回一个task_id，可以使用[task_check](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/task_check)接口查询任务执行状态。
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/v1/files/:file_token |
| HTTP Method | DELETE |
| 接口频率限制 | [特殊频控](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> |

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
	<md-text type="field-name" >file_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	需要删除的文件token

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
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	被删除文件的类型

**示例值**：file

**可选值有**：
<md-enum>
<md-enum-item key="file" >文件类型</md-enum-item>
<md-enum-item key="docx" >新版文档类型</md-enum-item>
<md-enum-item key="bitable" >多维表格类型</md-enum-item>
<md-enum-item key="folder" >文件夹类型</md-enum-item>
<md-enum-item key="doc" >文档类型</md-enum-item>
<md-enum-item key="sheet" >电子表格类型</md-enum-item>
<md-enum-item key="mindnote" >思维笔记类型</md-enum-item>
<md-enum-item key="shortcut" >快捷方式类型</md-enum-item>
</md-enum>
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
	<md-text type="field-name" >task_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	异步任务id，删除文件夹时返回
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
        "task_id": "12345"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1061001 | internal error. | 服务内部错误，包括超时，错误码没处理。 |
| 400 | 1061002 | params error. | 请检查请求参数是否正确。 |
| 404 | 1061003 | not found. | 请确认对应资源是否存在。 |
| 403 | 1061004 | forbidden. | 请确认当前身份是否有对应上传节点的的权限，如用户是否有上传到指定doc的编辑权限。 |
| 401 | 1061005 | auth failed. | 请使用正确身份访问该接口。 |
| 404 | 1061007 | file has been delete. | 请确认对应节点未被删除。 |
| 403 | 1062501 | operate node no permission. | 请确认拥有操作节点的权限。 |
| 403 | 1062502 | operate sub node no permission. | 请确认拥有操作节点的子节点权限。 |
| 404 | 1065200 | source node not found. | 操作的源节点找不到， 指查询不到数据库记录,  也可能是主从延迟。 |


错误码具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


