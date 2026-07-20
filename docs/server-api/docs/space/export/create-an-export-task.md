---
document_id: '7242548239429861381'
directory_id: '7234348439831265285'
title: 创建导出任务
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/create
breadcrumb:
- Server API
- Docs
- Space
- export
- Create an export task
document_type: ReferenceDocumentType
updated_at: 2023-06-12T10:47:11Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/create
---

# 创建导出任务

创建导出任务，将云文档导出为指定格式的本地文件，目前支持新版文档、电子表格、多维表格和旧版文档。该接口为异步接口，任务创建完成即刻返回，并不会阻塞等待到任务执行成功，因此需要结合[查询导出任务结果](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/get)接口获取导出结果。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=export_task&method=create)

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

::: note
请求用户需要对该文档有导出权限。
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
      <md-td>https://open.larksuite.com/open-apis/drive/v1/export_tasks</md-td>
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
      <md-app-support types="custom"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
      </md-th>
      <md-td>
            <md-perm name="drive:export:readonly" desc="导出云文档" support_app_types="custom" tags="">导出云文档</md-perm>
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
	导出文件扩展名

**示例值**："csv"

**可选值有**：
<md-enum>
<md-enum-item key="docx" >Microsoft Word (DOCX) 格式</md-enum-item>
<md-enum-item key="pdf" >pdf 格式</md-enum-item>
<md-enum-item key="xlsx" >Microsoft Excel (XLSX) 格式</md-enum-item>
<md-enum-item key="csv" >csv 格式</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	导出文档 token [获取文档 token](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)

**示例值**："shtcnxe5OxxxxxxxSNdsJviENsk"
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
	导出文档类型 [文档类型说明](/ssl::ttdoc/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#560bf735)

**示例值**："sheet"

**可选值有**：
<md-enum>
<md-enum-item key="doc" >旧版Lark云文档类型，支持导出为 docx、pdf 格式</md-enum-item>
<md-enum-item key="sheet" >Lark电子表格类型，支持导出为 xlsx、csv 格式</md-enum-item>
<md-enum-item key="bitable" >Lark多维表格类型，支持导出为 xlsx、csv 格式</md-enum-item>
<md-enum-item key="docx" >新版Lark云文档类型，支持导出为 docx、pdf 格式</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >sub_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	导出子表ID，仅当将电子表格/多维表格导出为 csv 时使用

**示例值**："tblKz5D60T4JlfcT"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "file_extension": "csv",
    "token": "shtcnxe5OxxxxxxxSNdsJviENsk",
    "type": "sheet",
    "sub_id": "tblKz5D60T4JlfcT"
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
	导出任务ID
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
        "ticket": "6933093124755423251"
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
  <md-td>400</md-td>
  <md-td>1069904</md-td>
  <md-td>invalid param</md-td>
  <md-td>无效参数，导出 csv 是否传入 sub_id</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1069906</md-td>
  <md-td>docs deleted</md-td>
  <md-td>文档已被删除</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1069914</md-td>
  <md-td>invalid file token</md-td>
  <md-td>导出文档 token 不合法</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1069918</md-td>
  <md-td>file extension and 
 type mismatch</md-td>
  <md-td>导出文件类型与云文档类型不匹配</md-td>
</md-tr>


<md-tr>
  <md-td>429</md-td>
  <md-td>1069923</md-td>
  <md-td>too many requests</md-td>
  <md-td>请求发生限频，请降低请求频率并稍后重试</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::

其他错误码可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


