---
document_id: '6967331173081874437'
directory_id: '6956134701804306438'
title: 修改保护范围
full_path: /ukTMukTMukTM/uUTM5YjL1ETO24SNxkjN
breadcrumb:
- Server API
- Docs
- Sheets
- Row Columu - Protect Range
- Modify Protection Scopes
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:01Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUTM5YjL1ETO24SNxkjN
---

# 修改保护范围

该接口用于根据保护范围ID修改保护范围，单次最多支持同时修改10个ID。
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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_update</md-td>
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
| 参数              | 类型   | 必须 | 说明&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;                           | 
| ----------------- | ------ | ---- | ------------------------------------------------------------ | ----------- |
| spreadsheetToken  | string | 是   | sheet 的 token，获取方式见[在线表格开发指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)| 
### 请求体
| 参数              | 类型   | 必须 | 说明&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;                           | 
| ----------------- | ------ | ---- | ------------------------------------------------------------ | ----------- |
|requests||是|请求
| &emsp;protectId         | string | 是   | 保护范围ID，可以通过[获取表格元数据](/document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN) 接口获取                                          |
| &emsp;∟dimension         |        | 否   | 行列保护信息                                                 |
| &emsp;&emsp;∟sheetId          | string | 是   | sheetId                                                      |
| &emsp;&emsp;∟startIndex       | int    | 是   | 保护行列起始下标，下标从1开始                                |
| &emsp;&emsp;∟endIndex         | int    | 是   | 保护行列终止下标，下标从1开始                                |
| &emsp;&emsp;∟majorDimension   | string | 是   | 保护范围ID对应的保护范围的维度，COLUMNS为保护列，ROWS为保护行 | 请求 body   |
| &emsp;∟editors           |        | 否   | 可编辑保护范围的用户                                         |
| &emsp;&emsp;∟addEditors       |        | 否   | 需要增加的用户的列表，用户需要有文档的编辑权限               |
| &emsp;&emsp;&emsp;∟memberType | string | 是   | 用户类型，支持userId,openId,unionId                          |
| &emsp;&emsp;&emsp;∟memberId   | string | 是   | 用户类型对应的用户ID                                         |
| &emsp;&emsp;∟delEditors       |        | 否   | 需要删除的用户的列表                                         |
| &emsp;&emsp;&emsp;∟memberType | string | 是   | 用户类型，支持userId,openId,unionId                          |
| &emsp;&emsp;&emsp;∟memberId   | string | 是   | 用户类型对应的用户ID                                         |
| &emsp;∟lockInfo          | string | 否   | 保护说明                                                     |

### 请求体示例
```json
{
    "requests": [
        {
            "protectId": "***",
            "dimension": {
                "majorDimension": "***",
                "sheetId": "***",
                "startIndex": 0,
                "endIndex": 0
            },
            "editors": {
                "addEditors": [
                    {
                        "memberType": "userId",
                        "memberId": "****"
                    }
                ],
                "delEditors": [
                    {
                        "memberType": "userId",
                        "memberId": "****"
                    }
                ]
            },
            "lockInfo": "****"
        }
    ]
}
```
### cURL 请求示例
```
curl --location --request POST 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/protected_range_batch_update' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
--header 'Content-Type: application/json' \
--data-raw '{
    "requests": [
        {
            "protectId": "6947942538267541505",
            "dimension": {
                "majorDimension": "ROWS",
                "sheetId": "Q7PlXT",
                "startIndex": 2,
                "endIndex": 4
            },
            "editors": {
                "addEditors": [
                    {
                        "memberType": "userId",
                        "memberId": "667338922291111404"
                    }
                ],
                "delEditors": [
                    {
                        "memberType": "userId",
                        "memberId": "667338922291122404"
                    }
                ]
            },
            "lockInfo": "1234"
        }
    ]
}'
```
## 响应
### 响应体
| 参数              |类型| 说明&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; |
| ----------------- |-----| ---------------------------------- |
|replies|array<interface>|响应|
| &emsp;∟sheetId           |string| sheet的id                          |
| &emsp;∟dimension         || 成功修改的保护行列信息             |
| &emsp;&emsp;∟sheetId          |string| sheetId                            |
| &emsp;&emsp;∟startIndex       |int| 保护行列起始下标，下标从1开始      |
| &emsp;&emsp;∟endIndex         |int| 保护行列终止下标，下标从1开始      |
| &emsp;&emsp;∟majorDimension   |string| 保护范围的维度                     |
| &emsp;∟editors           || 可编辑保护范围的用户               |
| &emsp;&emsp;∟addEditors       |array<interface>| 成功增加的用户的列表               |
| &emsp;&emsp;&emsp;∟memberType |string| 用户类型                           |
| &emsp;&emsp;&emsp;∟memberId   |string| 用户类型对应的用户ID               |
| &emsp;&emsp;∟delEditors       |array<interface>| 成功删除的用户的列表               |
| &emsp;&emsp;&emsp;∟memberType |string| 用户类型                           |
| &emsp;&emsp;&emsp;∟memberId   |string| 用户类型对应的用户ID               |
| &emsp;∟lockInfo          |string| 成功修改的保护说明                 |
### 响应体示例
```json
{
    "code": 0,
    "msg": "Success",
    "data": {
        "replies": [
            {
                "protectId": "***",
                "dimension": {
                    "sheetId": "***",
                    "startIndex": 0,
                    "endIndex": 0,
                    "majorDimension": "ROWS"
                },
                "editors": {
                    "addEditors": [
                        {
                            "memberType": "userId",
                            "memberId": "*"
                        }
                    ],
                    "delEditors": []
                },
                "lockInfo": "Info11",
                "sheetId": "abb54d"
            }
        ]
    }
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
