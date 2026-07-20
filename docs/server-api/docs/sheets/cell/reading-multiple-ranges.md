---
document_id: '6967331158355623942'
directory_id: '6956134701804355590'
title: 读取多个范围
full_path: /ukTMukTMukTM/ukTMzUjL5EzM14SOxMTN
breadcrumb:
- Server API
- Docs
- Sheets
- Cell
- Reading Multiple Ranges
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:17Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukTMzUjL5EzM14SOxMTN
---

# 读取多个范围


该接口用于根据 spreadsheetToken 和 ranges 读取表格多个范围的值，返回数据限制为10M。

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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_get </md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
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
<md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm>
<md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm>
<md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">查看、评论和导出电子表格</md-perm>
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
:::note
目前对于含有公式的单元格，仅能获取除跨表引用、数组公式以外的公式的值。
:::
:::html

<md-alert type="warn">
user_id_type目前默认值为lark_id，2022年1月26日后将会调整默认值为open_id，且不再支持lark_id，请尽快适配！
</md-alert>

:::
| 参数                 | 类型   | 必须 | 说明                                                         |
| -------------------- | ------ | ---- | ------------------------------------------------------------ |
| ranges               | array<string> | 是   | 多个查询范围 如 url?ranges=range1,range2 ，⁣其中 range 包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见 [在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview) |
| valueRenderOption    | string | 否   | valueRenderOption=ToString 可返回纯文本的值(数值类型除外)；valueRenderOption=FormattedValue 计算并格式化单元格；valueRenderOption=Formula单元格中含有公式时返回公式本身；valueRenderOption=UnformattedValue计算但不对单元格进行格式化|
| dateTimeRenderOption  | string | 否   | dateTimeRenderOption=FormattedString 计算并将时间日期按照其格式进行格式化，但不会对数字进行格式化，返回格式化后的字符串。详见[电子表格常见问题](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/guide/sheets-faq) |
| user_id_type | string | 否 | 返回的用户id类型，可选open_id,union_id |

###  cURL 请求示例
```
curl --location --request GET 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/values_batch_get?ranges=Q7PlXT!A2:B6,0b6377!B1:C8&valueRenderOption=ToString&dateTimeRenderOption=FormattedString' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
```
## 响应  

### 响应体
|参数|类型|说明|
|--|-----|--|
|revision|int |sheet 的版本号|
|spreadsheetToken|string | spreadsheet 的 token，详见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)|
|totalCells|int|读取的单元格总数|
|valueRanges|array<Object>|值与范围|
|&emsp;∟majorDimension|string|插入维度|
|&emsp;∟range|string|返回数据的范围，为空时表示查询范围没有数据|
|&emsp;∟revision|int|sheet 的版本号|
|&emsp;∟values|array<array<Object>>|查询得到的值|
  
### 响应体示例    
```json
{
  "code": 0,
  "data": {
    "revision": 0,
    "spreadsheetToken": "***",
    "totalCells": 0,
    "valueRanges": [
      {
        "majorDimension": "ROWS",
        "range": "range1",
        "revision": 0,
        "values": [
          [
            "***"
          ]
        ]
      },
      {
        "majorDimension": "ROWS",
        "range": "range2",
        "revision": 0,
        "values": [
          [
            "***"
          ]
        ]
      }
    ]
  },
  "msg": "Success"
}
```  
  
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
