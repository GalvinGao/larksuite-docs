---
document_id: '6967331158355296262'
directory_id: '6956134701804306438'
title: 获取保护范围
full_path: /ukTMukTMukTM/uQTM5YjL0ETO24CNxkjN
breadcrumb:
- Server API
- Docs
- Sheets
- Row Columu - Protect Range
- Retrieve Protection Scopes
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:58Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uQTM5YjL0ETO24CNxkjN
---

# 获取保护范围

该接口用于根据保护范围ID查询详细的保护行列信息，最多支持同时查询5个ID。
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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_get    </md-td>
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
| 参数             | 类型   | 必须 | 说明&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;                           | 
| ---------------- | ------ | ---- | ------------------------------------------------------------ | 
| protectIds       | string | 是   | 保护范围ID，可以通过[获取表格元数据](/document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN)接口获取，多个ID用逗号分隔，如xxxID1,xxxID2 | 
| memberType       | string | 否   | 返回的用户类型，可选userId,openId,unionId,默认使用userId     | 

### cURL 请求示例
```
curl --location --request GET 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/protected_range_batch_get?protectIds=6946456074476339204,6947648349520592923,6947942538267541505&memberType=userId' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
```

## 响应  
### 响应体
| 参数                        |类型| 说明                                          |
| --------------------------- |-----| --------------------------------------------- |
| protectedRange              |array<interface>| 保护范围                                      |
| &emsp;∟protectId            |string| 保护范围ID                                    |
| &emsp;∟dimension            ||  保护范围，如果为空，则为保护子表              |
| &emsp;&emsp;∟sheetId        |string| sheet 的 id                                   |
| &emsp;&emsp;∟startIndex     |int| 保护行列起始下标，下标从1开始                 |
| &emsp;&emsp;∟endIndex       |int| 保护行列终止下标，下标从1开始                 |
| &emsp;&emsp;∟majorDimension |string| 保护范围的维度，COLUMNS为保护列，ROWS为保护行 |
| &emsp;∟sheetId              |string| sheet的id                                     |
| &emsp;∟lockInfo             |string| 保护说明                                      |
| &emsp;∟editors              || 用户信息                                      |
| &emsp;&emsp;∟users	|array<interface>|用户信息列表|
| &emsp;&emsp;&emsp;∟memberType     |string| 用户类型                                      |
| &emsp;&emsp;&emsp;∟memberId       |string| 用户ID                                        |

### 响应体示例

```json
{
    "code": 0,
    "msg": "Success",
    "data": {
        "protectedRanges": [
            {
                "protectId": "*****",
                "dimension": {
                    "sheetId": "***",
                    "startIndex": 0,
                    "endIndex": 0,
                    "majorDimension": "COLUMNS"
                },
                "editors": {
                    "users": [
                        {
                            "memberType": "userId",
                            "memberId": "***"
                        }
                    ]
                },
                "sheetId": "***"
            }
        ]
    }
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
