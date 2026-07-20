---
document_id: '6967331173081989125'
directory_id: '7072290825036922886'
title: 插入行列
full_path: /ukTMukTMukTM/uQjMzUjL0IzM14CNyMTN
breadcrumb:
- Server API
- Docs
- Sheets
- Row Column
- Insert Rows or Columns
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:52Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uQjMzUjL0IzM14CNyMTN
---

# 插入行列

该接口用于根据 spreadsheetToken 和维度信息 插入空行/列。  
如 startIndex=3， endIndex=7，则从第 4 行开始开始插入行列，一直到第 7 行，共插入 4 行；单次操作不超过5000行或列。

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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/insert_dimension_range  </md-td>
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
      <md-th style="width: 18%;">名称</md-th>  
      <md-th style="width: 15%;">类型</md-th>  
       <md-th style="width: 15%;">必填</md-th>  
      <md-th>描述</md-th> 
    </md-tr> 
  </md-thead>  
  <md-tbody> 
    <md-tr> 
      <md-td>Authorization</md-td>  
      <md-td>string</md-td>  
      <md-td> 是 </md-td> 
      	<md-td>
<md-tag mode="inline" type="token-user">user_access_token</md-tag> 或 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
 
**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"
          
 [了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)
	</md-td>
</md-tr>
     <md-tr> 
      <md-td>Content-Type</md-td>  
      <md-td>string</md-td>  
      <md-td> 是 </md-td> 
     <md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
   
  </md-tbody> 
</md-table>
:::


### 路径参数
:::html
<md-table>
  <md-thead>
  <md-tr>
      <md-th>参数</md-th>
      <md-th>类型</md-th>
      <md-th>描述</md-th>
  </md-tr>  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>spreadsheetToken</md-td>
       <md-td>string</md-td>
       <md-td>spreadsheet 的 token，获取方式见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)</md-td>
      </md-tr>
  </md-tbody>

</md-table>
::: 


### 请求体  
|参数|类型|必须|说明|
|--|-----|--|----|
|dimension||是|需要插入行列的维度信息| 
|&emsp;∟sheetId|string|是|sheet 的 Id| 
|&emsp;∟majorDimension|string|否|默认 ROWS ，可选 ROWS、COLUMNS| 
|&emsp;∟startIndex|int|是|开始的位置|
|&emsp;∟endIndex|int|是|结束的位置|
|inheritStyle|string|否|BEFORE 或 AFTER，不填为不继承 style| 

### 请求体示例  
```json
{
    "dimension":{
        "sheetId":"string",
        "majorDimension":"ROWS",
        "startIndex":0,
        "endIndex":0
    },
    "inheritStyle":"BEFORE"
}
```
### cURL 请求示例
```
curl --location --request POST 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/insert_dimension_range' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
    "dimension":{
        "sheetId":"Q7PlXT",
        "majorDimension":"ROWS",
        "startIndex":3,
        "endIndex":5
    },
    "inheritStyle":"AFTER"
}'
```
## 响应  

### 响应体示例    

```json
{
    "code": 0,
    "msg": "Success",
    "data": {}
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


