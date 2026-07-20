---
document_id: '6967331173081284613'
directory_id: '7072290825036939270'
title: 更新工作表属性
full_path: /ukTMukTMukTM/ugjMzUjL4IzM14COyMTN
breadcrumb:
- Server API
- Docs
- Sheets
- sheet
- Update Sheet Properties
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:15Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugjMzUjL4IzM14COyMTN
---

# 更新工作表属性
::: note
该接口和 [操作工作表](/document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN) 的请求地址相同，但参数不同，调用前请仔细阅读文档。
:::

该接口用于根据 spreadsheetToken 更新工作表属性。


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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update   </md-td>
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
<md-tag mode="inline" type="token-user">user_access_token</md-tag>或 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
 
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
:::html

<md-alert type="warn">
userIds字段已废弃，2022年1月26日后将不再支持userIds字段，请使用userIDs作为替代。userIDs为需要增加保护范围编辑权限的用户id，id类型由user_id_type字段决定。
</md-alert>

:::
|参数|类型|必须|说明|
|--|-----|--|-----------|
|requests| |是|请求操作，支持增、删、复制工作表，三个操作选一个| 
|&emsp;∟updateSheet | |否|更新工作表| 
|&emsp;&emsp;∟properties| |是| 工作表属性|
|&emsp;&emsp;&emsp;∟sheetId| string|是|read-only ,作为表格唯一识别参数| 
|&emsp;&emsp;&emsp;∟title|string |否|更改工作表标题| 
|&emsp;&emsp;&emsp;∟index|int |否|移动工作表的位置| 
|&emsp;&emsp;&emsp;∟hidden|bool |否|隐藏表格，默认 false| 
|&emsp;&emsp;&emsp;∟frozen<br>&emsp;&emsp;&emsp;&emsp;RowCount|int |否|冻结行数，小于等于工作表的最大行数，0表示取消冻结行|
|&emsp;&emsp;&emsp;∟frozen<br>&emsp;&emsp;&emsp;&emsp;ColCount|int |否|该 sheet 的冻结列数，小于等于工作表的最大列数，0表示取消冻结列|
|&emsp;&emsp;&emsp;∟protect|  |否|锁定表格| 
|&emsp;&emsp;&emsp;&emsp;∟lock| string |是|LOCK 、UNLOCK 上锁/解锁 | 
|&emsp;&emsp;&emsp;&emsp;∟lockInfo|string  |否|锁定信息| 
|&emsp;&emsp;&emsp;&emsp;∟userIds| array<int64> |否|除了本人与所有者外，添加其他的可编辑人员，已废弃 | 
|&emsp;&emsp;&emsp;&emsp;∟userIDs| array<string> |否|除了本人与所有者外，添加其他的可编辑人员,user_id_type不为空时使用该字段 | 
### 请求体示例    
```json
{
  "requests": [
    {
      "updateSheet": {
        "properties": {
          "sheetId": "string",
          "title": "string",
          "index": "int",
          "hidden": "bool",
          "frozenColCount": "int",
          "frozenRowCount": "int",
          "protect": {
            "lock": "LOCK",
            "lockInfo": "111",
            "userIDs": [
              "ou_xxxxxxxxxx"
            ]
          }
        }
      }
    }
  ]
}
```
  ### cURL 请求示例
```
curl --location --request POST 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/sheets_batch_update?user_id_type=open_id' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
  "requests": [
    {
      "updateSheet": {
        "properties": {
          "sheetId": "zajIJ",
          "title": "",
          "index": 0,
          "frozenColCount": 10,
          "frozenRowCount": 20,
          "protect": {
            "lock": "LOCK",
            "lockInfo": "Lock Test"
            "userIDs": [
               "ou_xxxxxxxxxxxxx"
            ]
          }
        }
      }
    }
  ]
}'
```
## 响应  
### 响应体
:::html

<md-alert type="warn">
  userIds字段已废弃，2022年1月26日后将不再支持userIds字段，请使用userIDs作为替代。userIDs为需要增加保护范围编辑权限的用户id，id类型由user_id_type字段决定。
</md-alert>

:::
|参数|类型|说明|
|--|-----|--|
|replies| |返回本次相关操作工作表的结果|
|&emsp;∟updateSheet| |更新工作表的属性|
|&emsp;&emsp;∟properties| |工作表属性|
|&emsp;&emsp;&emsp;∟sheetId|string| 表格的 sheetId|
|&emsp;&emsp;&emsp;∟title|string |更新的工作表标题|
|&emsp;&emsp;&emsp;∟index|int |移动工作表的位置|
|&emsp;&emsp;&emsp;∟hidden|bool |是否隐藏表格|
|&emsp;&emsp;&emsp;∟frozenRowCount|int |冻结行数|
|&emsp;&emsp;&emsp;∟frozenColCount|int |冻结列数|
|&emsp;&emsp;&emsp;∟protect| |保护工作表| 
|&emsp;&emsp;&emsp;&emsp;∟lock|string |LOCK 、UNLOCK 保护/取消保护 | 
|&emsp;&emsp;&emsp;&emsp;∟lockInfo|string |保护信息| 
|&emsp;&emsp;&emsp;&emsp;∟userIds|array<int64> |除了本人与所有者外，添加其他的可编辑人员，已废弃 | 
|&emsp;&emsp;&emsp;&emsp;∟userIDs|array<string> |除了本人与所有者外，添加其他的可编辑人员，user_id_type不为空时返回该字段 |
 ### 响应体示例    
```json
 {
  "code": 0,
  "msg": "Success",
  "data": {
    "replies": [
      {
        "updateSheet": {
          "properties": {
            "sheetId": "string",
            "title": "string",
            "index": 0,
            "hidden": true,
            "frozenColCount": 0,
            "frozenRowCount": 0,
            "protect": {
              "lock": "LOCK",
              "sheetName": "",
              "permId": "",
              "userIDs": [
                "ou_xxxxxxxxx"
              ]
            }
          }
        }
      }
    ]
  }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
  
