---
document_id: '6967331173082218501'
directory_id: '6956134701804355590'
title: 写入图片
full_path: /ukTMukTMukTM/uUDNxYjL1QTM24SN0EjN
breadcrumb:
- Server API
- Docs
- Sheets
- Cell
- Write Images
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:38Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDNxYjL1QTM24SN0EjN
---

# 写入图片

该接口用于根据 spreadsheetToken 和 range 向单个格子写入图片。

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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_image</md-td>
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
|参数|类型|必须|说明|
|--|-----|--|----|
|spreadsheetToken|string|是|spreadsheet的token，获取方式见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)| URL Path|
### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|range|string|是|查询范围  range=<sheetId>!<开始格子>:<结束格子> 如：xxxx!A1:D5，详见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)。此处限定为一个格子，如: xxxx!A1:A1|
|image|array<byte>|是|需要写入的图片二进制流，支持  "PNG", "JPEG", "JPG", "GIF", "BMP", "JFIF", "EXIF", "TIFF", "BPG", "WEBP", "HEIC" 等图片格式|
|name|string|是|写入的图片名字|
### 请求体示例
```json
{ 
    "range": "string", 
    "image": [123,32,42,23],
    "name": "xxx.png"
}
```
### cURL 请求示例
```
curl --location --request POST 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/values_image' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{ 
    "range": "Q7PlXT!H7:H7",
    "image": [137,80,78,71,13,10,26,10,0,0,0,13,73,72,68,82,0,0,0,2,0,0,0,1,1,0,0,0,0,220,89,66,39,0,0,0,2,116,82,78,83,0,0,118,147,205,56,0,0,0,10,73,68,65,84,8,215,99,104,0,0,0,130,0,129,221,67,106,244,0,0,0,0,73,69,78,68,174,66,96,130],
    "name": "test.png"
}'
```
## 响应
### 响应体
  |参数|类型|说明|
|--|-----|--|
|spreadsheetToken|string |spreadsheet 的 token |
### 响应体示例
 
```json
{ 
    "code": 0,
    "data": { 
        "spreadsheetToken": "***" 
    }, 
    "msg": "Success" 
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
