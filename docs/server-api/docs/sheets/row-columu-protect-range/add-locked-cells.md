---
document_id: '6967331173081071621'
directory_id: '6956134701804306438'
title: 增加保护范围
full_path: /ukTMukTMukTM/ugDNzUjL4QzM14CO0MTN
breadcrumb:
- Server API
- Docs
- Sheets
- Row Columu - Protect Range
- Add Locked Cells
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:55Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugDNzUjL4QzM14CO0MTN
---

# 增加保护范围


该接口用于根据 spreadsheetToken 和维度信息增加多个保护范围；单次操作不超过5000行或列。


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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_dimension</md-td>
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

### 查询参数
:::html

<md-alert type="warn">
user_id_type目前默认值为lark_id，2022年1月26日后将会调整默认值为open_id，且不再支持lark_id，请尽快适配！
</md-alert>

:::
:::html
<md-table>
  <md-thead>
  <md-tr>
      <md-th>参数</md-th>
      <md-th>类型</md-th>
      <md-th>必须</md-th>
	  <md-th>说明</md-th>
  </md-tr>
      </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>user_id_type</md-td>
       <md-td>string</md-td>
       <md-td>否</md-td>
      <md-td>请求的用户id类型，可选open_id,union_id</md-td>
      </md-tr>
</md-tbody>
</md-table>
::: 

### 路径参数
|参数|类型|必须|说明|
|--|-----|--|----|
|spreadsheetToken|string|是|spreadsheet 的 token，获取方式见 [在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)| 

### 请求体
:::html

<md-alert type="warn">
editors字段已废弃，2022年1月26日后将不再支持editors字段，请使用users作为替代。users为需要增加保护范围编辑权限的用户id，id类型由user_id_type字段决定。
</md-alert>

:::
|参数|类型|必须|说明|
|--|-----|--|----|
|addProtectedDimension||是|需要增加保护范围的维度信息，可多个范围| 
|&emsp;∟dimension||是|需要保护行列的维度信息| 
|&emsp;&emsp;∟sheetId|string|是|sheetId| 
|&emsp;&emsp;∟majorDimension|string|否|默认 ROWS ，可选 ROWS、COLUMNS| 
|&emsp;&emsp;∟startIndex|int|是|开始的位置|
|&emsp;&emsp;∟endIndex|int|是|结束的位置| 
|&emsp;∟editors|array<int64>|否|允许编辑保护范围的用户的 userID| 
|&emsp;∟users|array<string>|否|允许编辑保护范围的用户的id，id类型取决于user_id_type| 
|&emsp;∟lockInfo|string|否|保护范围的信息|

### 请求体示例

```json
{
    "addProtectedDimension":[
        {
            "dimension":{
                "sheetId":"string",
                "majorDimension":"COLUMNS",
                "startIndex":10,
                "endIndex":13
            },
            "users":[
                "ou_326f4b0552770f2de069deb256de5b30"
            ],
            "lockInfo":"你能编辑"
        }
    ]
}
```
  
###  cURL 请求示例
```
curl --location --request POST 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/protected_dimension?user_id_type=open_id' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
    "addProtectedDimension":[
        {
            "dimension":{
                "sheetId":"Q7PlXT",
                "majorDimension":"COLUMNS",
                "startIndex":10,
                "endIndex":13
            },
            "users":[
              "ou_326f4b0552770f2de069deb256de5b30"
            ],
            "lockInfo":"你能编辑"
        }
    ]
}'
```
  
 ## 响应
### 响应体
:::html

<md-alert type="warn">
editors字段已废弃，2022年1月26日后将不再支持editors字段，请使用users作为替代。users为需要增加保护范围编辑权限的用户id，id类型由user_id_type字段决定。
</md-alert>

:::
|参数|类型|说明|
|--|-----|--|
|addProtectedDimension|array<interface>|需要增加保护范围的维度信息，可多个范围| 
|&emsp;∟dimension||需要保护行列的维度信息| 
|&emsp;&emsp;∟sheetId|string|sheetId| 
|&emsp;&emsp;∟majorDimension|string|默认 ROWS ，可选 ROWS、COLUMNS| 
|&emsp;&emsp;∟startIndex|int|开始的位置|
|&emsp;&emsp;∟endIndex|int|结束的位置| 
|&emsp;∟editors|array<int64>|允许编辑保护范围的用户的 userID| 
|&emsp;∟users|array<string>|允许编辑保护范围的用户的id，id类型取决于user_id_type|
|&emsp;∟lockInfo|string|保护范围的信息|
|&emsp;∟protectId|string|保护区域的唯一 uid ，可用做后续解除保护|
  
### 响应体示例  

```json
{
    "code": 0,
    "data": {
        "addProtectedDimension": [
            {
                "dimension": {
                    "endIndex": 0,
                    "majorDimension": "COLUMNS",
                    "sheetId": "***",
                    "startIndex": 0
                },
                "users": [
                    "ou_326f4b0552770f2de069deb256de5b30"
                ],
                "lockInfo": "***",
                "protectId": "***"
            }
        ]
    },
    "msg": "Success"
}

```  
  
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
