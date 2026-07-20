---
document_id: '7236573236108492806'
directory_id: '7234348439831265285'
title: 查询导出任务结果
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/get
breadcrumb:
- Server API
- Docs
- Space
- export
- get export task result
document_type: ReferenceDocumentType
updated_at: 2023-05-24T02:40:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/get
---

# 查询导出任务结果

根据[创建导出任务](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/create)返回的`ticket`轮询导出任务的结果，通过本接口获取到导出产物的文件`token`之后，可调用[下载导出文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/download)接口将导出产物下载到本地。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=export_task&method=get)

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
获取导出结果的用户需要与创建导出任务的用户相一致。
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
      <md-td>https://open.larksuite.com/open-apis/drive/v1/export_tasks/:ticket</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
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
</md-tbody>
</md-table>
:::



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
	<md-text type="field-name" >ticket</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	导出任务ID，[创建导出任务](/ssl::ttdoc//uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/create) 响应中的 ticket 字段

**示例值**："6933093124755423251"
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
	<md-text type="field-name" >token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	导出文档的 token

[如何获取文档 token](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)

**示例值**："doccnZVxxxxxxxxxxxxGiyBgYqe"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





## 响应

### 请求示例
```curl
curl --location --request GET 'https://open.larksuite.com/open-apis/drive/v1/export_tasks/7143131813848809492?token=docbcZVGtv1papC6jAVGiyBgYqe' \
--header 'Authorization: Bearer t-g1029efgIY34MWDJL4CEYQOVN5TZF2OMPJXTDVOP'
```



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
	<md-text type="field-name" >result</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >export_task</md-text>
	</md-dt-td>
	<md-dt-td>
	导出任务结果
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >file_extension</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	导出文件扩展名

**可选值有**：
<md-enum>
<md-enum-item key="docx" >Microsoft Word (DOCX) 格式</md-enum-item>
<md-enum-item key="pdf" >pdf 格式</md-enum-item>
<md-enum-item key="xlsx" >Microsoft Excel (XLSX) 格式</md-enum-item>
<md-enum-item key="csv" >csv 格式</md-enum-item>
</md-enum>
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
	导出文档类型 [文档类型说明](/ssl::ttdoc/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#560bf735)

**可选值有**：
<md-enum>
<md-enum-item key="doc" >旧版Lark云文档类型，支持导出为 docx、pdf 格式</md-enum-item>
<md-enum-item key="sheet" >Lark电子表格类型，支持导出为 xlsx、csv 格式</md-enum-item>
<md-enum-item key="bitable" >Lark多维表格类型，支持导出为 xlsx、csv 格式</md-enum-item>
<md-enum-item key="docx" >新版Lark云文档类型，支持导出为 docx、pdf 格式</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >file_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	导出文件名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >file_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	导出文件 drive token
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >file_size</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	导出文件大小，单位字节
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >job_error_msg</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	任务失败原因
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >job_status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	任务状态

**可选值有**：
<md-enum>
<md-enum-item key="0" >成功</md-enum-item>
<md-enum-item key="1" >初始化</md-enum-item>
<md-enum-item key="2" >处理中</md-enum-item>
<md-enum-item key="3" >内部错误</md-enum-item>
<md-enum-item key="107" >导出文档过大</md-enum-item>
<md-enum-item key="108" >处理超时</md-enum-item>
<md-enum-item key="109" >导出内容块无权限</md-enum-item>
<md-enum-item key="110" >无权限</md-enum-item>
<md-enum-item key="111" >导出文档已删除</md-enum-item>
<md-enum-item key="122" >创建副本中禁止导出</md-enum-item>
<md-enum-item key="123" >导出文档不存在</md-enum-item>
<md-enum-item key="6000" >导出文档图片过多</md-enum-item>
</md-enum>
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
        "result": {
            "file_extension": "pdf",
            "type": "doc",
            "file_name": "docName",
            "file_token": "boxcnxe5OxxxxxxxSNdsJviENsk",
            "file_size": 34356,
            "job_error_msg": "success",
            "job_status": 0
        }
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
  <md-td>410</md-td>
  <md-td>1069906</md-td>
  <md-td>docs deleted</md-td>
  <md-td>文档已被删除</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::

其他错误码可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


