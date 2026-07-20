---
document_id: '7073817463670636550'
directory_id: '7072290825036972038'
title: 创建筛选视图
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/create
breadcrumb:
- Server API
- Docs
- Sheets
- Filter view
- Create Filter View
document_type: ReferenceDocumentType
updated_at: 2022-09-29T10:54:57Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/create
---

# 创建筛选视图

根据传入的参数创建一个筛选视图。Id 和 名字可选，不填的话会默认生成；range 必填。Id 长度为10，由 0-9、a-z、A-Z 组合生成。名字长度不超过100。单个子表内的筛选视图个数不超过 150。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=sheets&version=v3&resource=spreadsheet.sheet.filter_view&method=create)

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
筛选范围的设置参考：[筛选视图的筛选条件指南](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/filter-view-condition-user-guide)
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
      <md-td>https://open.larksuite.com/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
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
            
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
            
      </md-th>
      <md-td>
            <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
            <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm>
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

[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)

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
	<md-text type="field-name" >spreadsheet_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	表格 token

**示例值**："shtcnmBA*****yGehy8"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >sheet_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	子表 id

**示例值**："0b**12"
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
      <md-dt-th style="width: 40%;">名称</md-dt-th>
      <md-dt-th style="width: 20%;">类型</md-dt-th>
      <md-dt-th style="width: 10%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 30%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >filter_view_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	筛选视图 id

**示例值**："pH9hbVcCXA"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >filter_view_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	筛选视图名字

**示例值**："筛选视图 1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >range</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	筛选视图的筛选范围

**示例值**："0b**12!C1:H14"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 请求体示例
:::html
<md-code-json>
{
    "filter_view_id": "pH9hbVcCXA",
    "filter_view_name": "筛选视图 1",
    "range": "0b**12!C1:H14"
}
</md-code-json>
:::

```
curl --location --request POST 'https://open.larksuite.com/open-apis/sheets/v3/spreadsheets/shtcnspY6YDVPxWjhG09Qxxxxxx/sheets/phwh0X/filter_views' \
--header 'Authorization: Bearer t-2602ac1d050a2a308ab8a98639d25cbaaaf26c9f' \
--header 'Content-Type: application/json' \
--data-raw '{
    
    "range": "phwh0X!C1:H14"
}'
```

## 响应



### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 40%;">名称</md-dt-th>
      <md-dt-th style="width: 20%;">类型</md-dt-th>
      <md-dt-th style="width: 30%;">描述</md-dt-th>
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
	<md-text type="field-name" >filter_view</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >filter_view</md-text>
	</md-dt-td>
	<md-dt-td>
	创建的筛选视图的 id 、name、range
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >filter_view_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	筛选视图 id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >filter_view_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	筛选视图名字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >range</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	筛选视图的筛选范围
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
        "filter_view": {
            "filter_view_id": "pH9hbVcCXA",
            "filter_view_name": "筛选视图 1",
            "range": "0b**12!C1:H14"
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
  <md-td>1315201</md-td>
  <md-td>Server Error</md-td>
  <md-td>服务内部错误，详询客服</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1315203</md-td>
  <md-td>Server Error</md-td>
  <md-td>服务内部错误，详询客服</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310217</md-td>
  <md-td>Too Many Request</md-td>
  <md-td>检查请求是否发送过于频繁</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310235</md-td>
  <md-td>Retry Later</md-td>
  <md-td>稍后重试</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310214</md-td>
  <md-td>SpreadSheet Not Found</md-td>
  <md-td>检查表格 token</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310215</md-td>
  <md-td>Sheet Id Not Found</md-td>
  <md-td>检查 sheet_id</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310223</md-td>
  <md-td>Col Id Not Found</md-td>
  <md-td>列 id 没有找到，可能是错误填写、或者超过表格的列范围</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310213</md-td>
  <md-td>Permission Fail</md-td>
  <md-td>没有文档相应权限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310204</md-td>
  <md-td>Wrong Request Body</md-td>
  <md-td>检查请求体参数</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310229</md-td>
  <md-td>Wrong URL Param</md-td>
  <md-td>检查 url 参数</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310202</md-td>
  <md-td>Wrong Range</md-td>
  <md-td>区域范围错误</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310226</md-td>
  <md-td>Excess Limit</md-td>
  <md-td>超出限制，参考响应体中的错误提示</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310211</md-td>
  <md-td>Wrong Sheet Id</md-td>
  <md-td>检查 sheet_id</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310232</md-td>
  <md-td>Wrong Style</md-td>
  <md-td>样式错误，包括颜色、字体等</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310218</md-td>
  <md-td>Locked Cell</md-td>
  <md-td>检查操作的是否为保护范围</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310237</md-td>
  <md-td>Wrong Filter View Id</md-td>
  <md-td>错误的筛选视图 id</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310238</md-td>
  <md-td>Exist Filter View Id</md-td>
  <md-td>筛选视图 id 已存在</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310239</md-td>
  <md-td>Wrong Filter View Name</md-td>
  <md-td>筛选视图名字错误</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310240</md-td>
  <md-td>Exist Filter View Name</md-td>
  <md-td>筛选视图名字已存在</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310241</md-td>
  <md-td>Filter View Excess</md-td>
  <md-td>筛选视图个数超过上限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310242</md-td>
  <md-td>In Mix state</md-td>
  <md-td>Retey Later</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1310249</md-td>
  <md-td>Spreadsheet Deleted</md-td>
  <md-td>恢复表格后重试</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




