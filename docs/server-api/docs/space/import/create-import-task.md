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
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/drive/v1/import_tasks</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    <md-tr>
      <md-th>接口频率限制</md-th>
      <md-td>[100 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom,isv"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
      </md-th>
      <md-td>
            <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 35%;">名称</md-th>
      <md-th style="width: 13%;">类型</md-th>
       <md-th style="width: 15%;" filters="是,否" >必填</md-th>
      <md-th  style="width: 37%;">描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
或
<md-tag mode="inline" type="token-user">user_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"

[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)

</md-td>
</md-tr>
<md-tr>
<md-td>Content-Type</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
</md-tbody>
</md-table>
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
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">HTTP状态码</md-th>
            <md-th style="width: 15%;">错误码</md-th>
            <md-th style="width: 30%;">描述</md-th>
            <md-th style="width: 30%;">排查建议</md-th>
        </md-tr>
    </md-thead>
  <md-tbody>

<md-tr>
  <md-td>500</md-td>
  <md-td>1069901</md-td>
  <md-td>internal error</md-td>
  <md-td>服务内部错误，详询 [Oncall](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-overview#51f94b41)</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1069902</md-td>
  <md-td>no permission</md-td>
  <md-td>无阅读或导出权限</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1069903</md-td>
  <md-td>internal error</md-td>
  <md-td>服务内部错误，详询客服</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1069904</md-td>
  <md-td>invalid param</md-td>
  <md-td>无效参数，请检查文件后缀名与导入文档类型是否正确</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1069905</md-td>
  <md-td>internal error</md-td>
  <md-td>服务内部错误，详询客服</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1069906</md-td>
  <md-td>docs deleted</md-td>
  <md-td>文档已被删除</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1069907</md-td>
  <md-td>file token not found</md-td>
  <md-td>不存在的 file token</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1069908</md-td>
  <md-td>mount point not found or no permission</md-td>
  <md-td>挂载点不存在或者无权限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1069909</md-td>
  <md-td>import file size over limit</md-td>
  <md-td>文件超过20M</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1069910</md-td>
  <md-td>import file extension not match</md-td>
  <md-td>上传文件和导入任务文件后缀不一致</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1069911</md-td>
  <md-td>import file type not match</md-td>
  <md-td>上传文件和导入任务指定的文件类型不一致</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1069912</md-td>
  <md-td>folder not exist</md-td>
  <md-td>目录不存在</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1069913</md-td>
  <md-td>import file token expired</md-td>
  <md-td>上传文件过期，有效期5分钟</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1069914</md-td>
  <md-td>invalid file token</md-td>
  <md-td>导出文档 token 不合法</md-td>
</md-tr>


<md-tr>
  <md-td>429</md-td>
  <md-td>1069923</md-td>
  <md-td>too many requests</md-td>
  <md-td>请求发生限频，请降低请求频率并稍后重试。</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




