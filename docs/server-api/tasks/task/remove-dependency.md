---
document_id: '7277487675920842758'
directory_id: '7253088311760617478'
title: 移除依赖
full_path: /uAjLw4CM/ukTMukTMukTM/task-v2/task/remove_dependencies
breadcrumb:
- Server API
- Tasks
- Task
- Remove Dependency
document_type: ReferenceDocumentType
updated_at: 2023-11-09T09:20:06Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/remove_dependencies
---

# 移除依赖

从一个任务移除一个或者多个依赖。移除时只需要输入要移除的`task_guid`即可。

注意，如果要移除的依赖非当前任务的依赖，会被自动忽略。接口会返回成功。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=task&version=v2&resource=task&method=remove_dependencies)

:::html
<md-alert type="tip">
移除任务依赖时，需要当前任务的编辑权限。
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
| HTTP URL | https://open.larksuite.com/open-apis/task/v2/tasks/:task_guid/remove_dependencies |
| HTTP Method | POST |
| 接口频率限制 | [100 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="task:task:write" desc="查看、创建、更新、删除任务" support_app_types="custom,isv" tags="">查看、创建、更新、删除任务</md-perm> |

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
	<md-text type="field-name" >task_guid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	要移除依赖的任务GUID

**示例值**："93b7bd05-35e6-4371-b3c9-6b7cbd7100c0"
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
	<md-text type="field-name" >dependencies</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >task_dependency\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	要移除的依赖

**数据校验规则**：

- 长度范围：`1` ～ `50`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >task_guid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	依赖任务的GUID

**示例值**："93b7bd05-35e6-4371-b3c9-6b7cbd7100c0"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "dependencies": [
        {
            "task_guid": "93b7bd05-35e6-4371-b3c9-6b7cbd7100c0"
        }
    ]
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
	<md-text type="field-name" >dependencies</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >task_dependency\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	移除之后的任务GUID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	依赖类型

**可选值有**：
<md-enum>
<md-enum-item key="prev" >前置依赖</md-enum-item>
<md-enum-item key="next" >后置依赖</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >task_guid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	依赖任务的GUID
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
        "dependencies": [
            {
                "type": "next",
                "task_guid": "93b7bd05-35e6-4371-b3c9-6b7cbd7100c0"
            }
        ]
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1470400 | 请求参数错误，如输入的`task_guid`非法。 | 具体错误原因参考返回的错误信息。 |
| 403 | 1470403 | 缺少任务的可编辑权限。 | 移除依赖需要当前任务的编辑权限。 |
| 404 | 1470404 | 要移除依赖的任务不存在或已删除。 | 确认当前任务是否存在或被删除。 |
| 500 | 1470500 | 服务器错误。 | 服务器内部错误。若多次重试调用后持续出错可以联系支持。 |





