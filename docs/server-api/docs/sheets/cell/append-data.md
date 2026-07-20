---
document_id: '6967331173082120197'
directory_id: '6956134701804355590'
title: 追加数据
full_path: /ukTMukTMukTM/uMjMzUjLzIzM14yMyMTN
breadcrumb:
- Server API
- Docs
- Sheets
- Cell
- Append Data
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:11Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMjMzUjLzIzM14yMyMTN
---

# 追加数据


该接口用于根据 spreadsheetToken 和 range 遇到空行则进行覆盖追加或新增行追加数据。 空行：默认该行第一个格子是空，则认为是空行；单次写入不超过5000行，100列，每个格子不超过5万字符。

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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_append </md-td>
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


### 查询参数  
|参数|类型|必须|说明|
|--|-----|--|----|
|insertDataOption|string|否|遇到空行追加，默认 OVERWRITE，若空行的数量小于追加数据的行数，则会覆盖已有数据；可选 INSERT_ROWS ，会在插入足够数量的行后再进行数据追加| 

### 请求体 
|参数|类型|必须|说明|
|--|-----|--|----|
|valueRange||是|值与范围|
|&emsp;∟range|string|是|⁣查询范围，包含 sheetId 与单元格范围两部分，目前支持三种索引方式，详见 [在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)。range所表示的范围需要大于等于values占用的范围。|
|&emsp;∟values|array<array<interface>>|是|需要写入的值，如要写入公式、超链接、email、@人等，可详看附录[sheet 支持写入数据类型](/document/ukTMukTMukTM/ugjN1UjL4YTN14CO2UTN)|
  
### 请求体示例  
```json
{
  "valueRange": {
    "range": "string",
    "values": [
      [
        "string",
        1,
        "https://www.xxx.com"
      ]
    ]
  }
}
```
  
  ### cURL 请求示例
```
curl --location --request POST 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/values_append' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
  "valueRange": {
    "range": "Q7PlXT!A1:B4",
    "values": [
      [
        "===",
        "https://www.xxx.com/"
      ],
      [
        "Hello",
        "https://www.xxx.com/"
      ],
      [
        "World",
        "https://www.xxx.com/"
      ],
      [
        "===",
        "https://www.xxx.com/"
      ]
    ]
  }
}'
```
  
## 响应  
### 响应体
|参数|类型|说明|
|--|-----|--|
|spreadsheetToken|string|spreadsheet 的 token|
|tableRange|string|写入的范围|
|revision|int|sheet 的版本号| 
|updates||插入数据的范围、行列数等|
|&emsp;∟spreadsheetToken|string|spreadsheet 的 token|
|&emsp;∟updatedRange|string|写入的范围|
|&emsp;∟updatedRows|int|写入的行数|
|&emsp;∟updatedColumns|int|写入的列数|
|&emsp;∟updatedCells|int|写入的单元格总数|
|&emsp;∟revision|int|sheet 的版本号|
 
 ### 响应体示例  
```json
{
  "code": 0,
  "msg": "Success",
  "data": {
    "revision": 0,
    "spreadsheetToken": "***",
    "tableRange": "***",
    "updates": {
      "spreadsheetToken": "***",
      "updatedRange": "***",
      "updatedRows": 0,
      "updatedColumns": 0,
      "updatedCells": 0,
      "revision": 0
    }
  }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
