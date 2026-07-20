---
document_id: '6967331173081366533'
directory_id: '6956134701804355590'
title: '批量设置单元格样式 '
full_path: /ukTMukTMukTM/uAzMzUjLwMzM14CMzMTN
breadcrumb:
- Server API
- Docs
- Sheets
- Cell
- Batch Set Cell Style
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:29Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uAzMzUjLwMzM14CMzMTN
---

# 批量设置单元格样式


该接口用于根据 spreadsheetToken 、range和样式信息 批量更新单元格样式；单次写入不超过5000行，100列。
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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/styles_batch_update</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>PUT</md-td>
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
|spreadsheetToken|string|是|spreadsheet 的 token，获取方式见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)| URL PATH|
### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|data||是|请求数据|
|&emsp;∟ranges|array<string>|是|查询范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见 [在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)|
|&emsp;∟style||是|需要更新的样式|
|&emsp;&emsp;∟font| |否|字体相关样式|
|&emsp;&emsp;&emsp;∟bold|bool|否|是否加粗|
|&emsp;&emsp;&emsp;∟italic|bool|否|是否斜体|
|&emsp;&emsp;&emsp;∟fontSize|string|否|字体大小 字号大小为9~36 行距固定为1.5，如:10pt/1.5| |&emsp;&emsp;∟clean|bool|否|清除 font 格式,默认 false|
|&emsp;&emsp;∟textDecoration|int|否|文本装饰 ，0 默认，1 下划线，2 删除线 ，3 下划线和删除线|  
|&emsp;&emsp;∟formatter|string|否|数字格式，详见附录 [sheet支持数字格式类型](/document/ukTMukTMukTM/uMjM2UjLzIjN14yMyYTN) |
|&emsp;&emsp;∟hAlign|int|否|水平对齐，0 左对齐，1 中对齐，2 右对齐|
|&emsp;&emsp;∟vAlign|int|否|垂直对齐， 0 上对齐，1 中对齐， 2 下对齐|
|&emsp;&emsp;∟foreColor|string|否|字体颜色|
|&emsp;&emsp;∟backColor|string|否|背景颜色|
|&emsp;&emsp;∟borderType|string|否|边框类型，可选 "FULL_BORDER"，"OUTER_BORDER"，"INNER_BORDER"，"NO_BORDER"，"LEFT_BORDER"，"RIGHT_BORDER"，"TOP_BORDER"，"BOTTOM_BORDER"|
|&emsp;&emsp;∟borderColor|string|否|边框颜色|
|&emsp;&emsp;∟clean|bool|否|是否清除所有格式,默认 false| 
### 请求体示例
```json
{
    "data":[
        {
            "ranges":[
                "string",
                "string"
            ],
            "style":{
                "font":{
                    "bold":true,
                    "italic":true,
                    "fontSize":"10pt/1.5",
                    "clean":false
                },
                "textDecoration":0,
                "formatter":"",
                "hAlign":0,
                "vAlign":0,
                "foreColor":"#000000",
                "backColor":"#21d11f",
                "borderType":"FULL_BORDER",
                "borderColor":"#ff0000",
                "clean":false
            }
        },
        {
            "ranges":[
                "string"
            ],
            "style":{
              ...
            }
        }
    ]
}
```
  
###  cURL 请求示例
```
curl --location --request PUT 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/styles_batch_update' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
    "data":[
        {
            "ranges":[
                "Q7PlXT!C7:E12",
                "Q7PlXT!I20:K27"
            ],
            "style":{
                "font":{
                    "bold":true,
                    "italic":true,
                    "fontSize":"10pt/1.5",
                    "clean":false
                },
                "textDecoration":0,
                "formatter":"",
                "hAlign":0,
                "vAlign":0,
                "foreColor":"#000000",
                "backColor":"#21d11f",
                "borderType":"FULL_BORDER",
                "borderColor":"#ff0000",
                "clean":false
            }
        },
        {
            "ranges":[
                "BzY8T5!A1:C2"
            ],
            "style":{
                "font":{
                    "bold":true,
                    "italic":true,
                    "fontSize":"10pt/1.5",
                    "clean":false
                },
                "textDecoration":0,
                "formatter":"",
                "hAlign":0,
                "vAlign":0,
                "foreColor":"#000000",
                "backColor":"#21d11f",
                "borderType":"FULL_BORDER",
                "borderColor":"#ff0000",
                "clean":false
            }
        }
    ]
}
'
```
  
## 响应
### 响应体
  |参数|类型|说明|
|--|-----|--|
|spreadsheetToken|string | spreadsheet 的 token|
|totalUpdatedRows|int|设置样式的总行数|
|totalUpdatedColumns|int|设置样式的总列数|
|totalUpdatedCells|int|设置样式的单元格总数|
|revision|int|sheet 的版本号|
|responses||各个范围的设置单元格样式的范围、行列数等|
|&emsp;∟spreadsheetToken|string |spreadsheet 的 token|
|&emsp;∟updatedRange|string |设置样式的范围|
|&emsp;∟updatedRows|int|设置样式的行数|
|&emsp;∟updatedColumns|int|设置样式的列数|
|&emsp;∟updatedCells|int|设置样式的单元格数|
### 响应体示例
```json
    {
    "code": 0,
    "msg": "Success",
    "data":{
        "spreadsheetToken": "string",
        "totalUpdatedCells": 0,
        "totalUpdatedColumns": 0,
        "totalUpdatedRows": 0,
        "revision": 0,
       "responses": [
          {
            "spreadsheetToken": "string",
            "updatedRange": "string",
            "updatedRows": 0,
            "updatedColumns": 0,
            "updatedCells": 0
          }
        ]
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
