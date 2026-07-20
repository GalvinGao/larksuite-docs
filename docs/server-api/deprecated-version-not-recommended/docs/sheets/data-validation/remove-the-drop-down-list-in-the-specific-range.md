---
document_id: '7538736582376030214'
directory_id: '7538357428568670214'
title: 删除下拉列表设置
full_path: /uAjLw4CM/ukTMukTMukTM/historic-version/docs/sheets/datavalidation/delete-datavalidation
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- sheets
- Data Validation
- Remove the Drop-down List in the Specific Range
document_type: GuideDocumentType
updated_at: 2025-08-19T04:56:13Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/historic-version/docs/sheets/datavalidation/delete-datavalidation
---

# ~~删除下拉列表设置~~

**注意：该接口于 2025 年 9 月 20 号起不可用，请使用最新接口[删除下拉列表设置](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/delete-datavalidation)**。

该接口根据 spreadsheetToken 、range 移除选定数据范围单元格的下拉列表设置，但保留选项文本。单个删除范围不超过5000单元格。单次请求range最大数量100个。

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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>DELETE</md-td>
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
|spreadsheetToken|string|是|spreadsheet 的 token，获取方式见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)| 



### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|dataValidationRanges|array||范围数组，每个range 最大单元格数量5000，每个range独立执行，一个range的失败不影响其他range的执行。返回结果会返回每个range的执行结果
|&emsp;∟range|string|是|查询范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)| 
|&emsp;∟dataValidationIds|array<int>|是|指定需要删除的dataValidationIds| 

### 请求体示例

```json
{
    "dataValidationRanges":[
        {
            "range": "4d30c6!A10:C10",
            "dataValidationIds": [1,2]
        }

    ]
}
```
### cURL 请求示例
  ```
  curl --location --request DELETE 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/dataValidation' \
--header 'Authorization: Bearer t-ce3540c5f02ac074535f1f14d64fa90fa49621c0' \
--header 'Content-Type: application/json' \
--data-raw '{
    "dataValidationRanges":[
        {
            "range": "BzY8T5!A1:A1",
            "dataValidationIds": [2]
        }
    ]
}'
  ```
 ## 响应

### 响应体
  
|参数|类型|必须|说明|
|--|-----|--|----|
|code|int|是|状态码，0代表成功|
|msg|string|否|状态信息|
|data||||
|&emsp;∟rangeResults|array|||
|&emsp;&emsp;∟range|string|是|执行的range,与请求入参中的range 对应|
|&emsp;&emsp;∟msg|string|否|结果信息|
|&emsp;&emsp;∟success|bool|是|执行结果|
|&emsp;&emsp;∟updatedCells|int|是|影响的单元格数量|

  
### 响应体示例  

```json
{
    "code": 0,
    "data": {
        "rangeResults": [
            {
                "msg": "",
                "range": "4d30c6!A10:C10",
                "success": true,
                "updatedCells": 3
            }
        ]
    },
    "msg": "Success"
}
```  
  
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
