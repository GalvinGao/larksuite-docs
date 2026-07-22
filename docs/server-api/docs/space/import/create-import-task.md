---
document_id: '7031483915610324997'
directory_id: '7031445675032199173'
title: 创建导入任务
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/create
breadcrumb:
- Server API
- Docs
- Space
- import
- Create Import Task
document_type: ReferenceDocumentType
updated_at: 2023-05-24T02:40:04Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/create
---

# 创建导入任务

创建导入任务。支持导入为新版文档、电子表格、多维表格以及旧版文档。该接口为异步接口，需要通过[查询导入结果](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/get)接口获取导入结果，调用方式可参考[导入使用指南](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/import-user-guide)。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=import_task&method=create)

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

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/v1/import_tasks |
| HTTP Method | POST |
| 接口频率限制 | [100 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




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
	<md-text type="field-name" >file_extension</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	导入文件格式后缀

**示例值**："xlsx"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >file_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	导入文件Drive FileToken

**示例值**："boxcnxe5OxxxxxxxSNdsJviENsk"
	</md-dt-td>
</md-dt-tr>


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
	导入目标云文档格式

**示例值**："sheet"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >file_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	导入目标云文档文件名 ，若为空使用Drive文件名

**示例值**："test"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >point</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >import_task_mount_point</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	挂载点
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >mount_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	挂载类型

**示例值**：1

**可选值有**：
<md-enum>
<md-enum-item key="1" >挂载到云空间</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >mount_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	挂载位置,对于mount_type=1, 云空间目录token，空表示根目录

**示例值**："fldxxxxxxxx"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "file_extension": "xlsx",
    "file_token": "boxcnxe5OxxxxxxxSNdsJviENsk",
    "type": "sheet",
    "file_name": "test",
    "point": {
        "mount_type": 1,
        "mount_key": "fldxxxxxxxx"
    }
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
	<md-text type="field-name" >ticket</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	导入任务ID
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
        "ticket": "6990281xxxxxxxxxxx843"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 1069901 | internal error | 服务内部错误，详询 [Oncall](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-overview#51f94b41) |
| 403 | 1069902 | no permission | 无阅读或导出权限 |
| 500 | 1069903 | internal error | 服务内部错误，详询客服 |
| 400 | 1069904 | invalid param | 无效参数，请检查文件后缀名与导入文档类型是否正确 |
| 500 | 1069905 | internal error | 服务内部错误，详询客服 |
| 403 | 1069906 | docs deleted | 文档已被删除 |
| 404 | 1069907 | file token not found | 不存在的 file token |
| 403 | 1069908 | mount point not found or no permission | 挂载点不存在或者无权限 |
| 400 | 1069909 | import file size over limit | 文件超过20M |
| 400 | 1069910 | import file extension not match | 上传文件和导入任务文件后缀不一致 |
| 400 | 1069911 | import file type not match | 上传文件和导入任务指定的文件类型不一致 |
| 400 | 1069912 | folder not exist | 目录不存在 |
| 400 | 1069913 | import file token expired | 上传文件过期，有效期5分钟 |
| 400 | 1069914 | invalid file token | 导出文档 token 不合法 |
| 429 | 1069923 | too many requests | 请求发生限频，请降低请求频率并稍后重试。 |





