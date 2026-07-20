---
document_id: '6967331173082316805'
directory_id: '6931268519897120795'
title: 获取条件格式
full_path: /ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-get
breadcrumb:
- Server API
- Docs
- Sheets
- Sheet - Conditional Formatting
- Obtain Conditional Formatting Rules
document_type: GuideDocumentType
updated_at: 2022-03-11T12:22:27Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-get
---

# 获取条件格式

该接口用于根据sheetId查询详细的条件格式信息，最多支持同时查询10个sheetId。

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
      <md-td>https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats    </md-td>
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
| 参数             | 类型   | 必须 | 说明&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;                           | 
| ---------------- | ------ | ---- | ------------------------------------------------------------ | 
| sheet_ids       | array<string> | 是   | 工作表ID，可以通过[获取表格元数据](/document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN)接口获取，多个ID用逗号分隔，如xxxID1,xxxID2 | 
###  cURL 请求示例
```
curl --location --request GET 'https://open.larksuite.com/open-apis/sheets/v2/spreadsheets/shtcngNygNfuqhxTBf588jwgWbJ/condition_formats?sheet_ids=Q7PlXT' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
```
## 响应  
### 响应体
| 参数                            |类型| 说明                                       |
| ----------------------------- |-----| ---------------------------------------- |
|sheet_condition_formats|array<interface>|表格的条件格式信息|
| &emsp;∟sheet_id                      |string| sheet的id                                 |
| &emsp;∟condition_format              || 一个条件格式的详细信息                              |
| &emsp;&emsp;∟cf_id                  |string| 条件格式的id                                  |
| &emsp;&emsp;∟ranges                 |array<string>| 条件格式应用的范围，支持：sheetId（整表）；sheetId!1:2（整行）；sheetId!A:B（整列）；sheetId!A1:B2（普通范围）；sheetId!A1:C（应用至最后一行）。应用范围不能超过表格的行总数和列总数 |
| &emsp;&emsp;∟rule_type              |string| 条件格式规则类型，目前只有7种：***containsBlanks（为空）、notContainsBlanks（不为空）、duplicateValues（重复值）、uniqueValues（唯一值）、cellIs（限定值范围）、containsText（包含内容）、timePeriod（日期）***                                  |
| &emsp;&emsp;∟attrs                  || rule_type对应的具体属性信息，详见 [条件格式指南](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-guide)                        |
| &emsp;&emsp;∟style                  || 条件格式样式，只支持以下样式                           |
| &emsp;&emsp; &emsp;∟font            || 字体样式                                     |
| &emsp;&emsp;  &emsp;&emsp; ∟bold     |bool| 加粗                                       |
| &emsp;&emsp; &emsp; &emsp;∟italic   |bool| 斜体                                       |
| &emsp;&emsp; &emsp;∟text_decoration |int| 文本装饰 ，0 默认，1 下划线，2 删除线 ，3 下划线和删除线        |
| &emsp;&emsp; &emsp;∟fore_color      |string| 字体颜色                                     |
| &emsp;&emsp; &emsp;∟back_color      |string| 背景颜色                                     |
### 响应体示例

```json
{
    "code": 0,
    "msg": "Success",
    "data": {
        "sheet_condition_formats": [
            {
                "condition_format": {
                    "cf_id": "r9sYuhgAl6",
                    "ranges": [
                        "uEnW3A!C4:C4"
                    ],
                    "rule_type": "timePeriod",
                    "attrs": [
                        {
                            "operator": "is",
                            "time_period": "today"
                        }
                    ],
                    "style": {
                        "back_color": "#d9f5d6",
                        "font": {
                            "bold": true,
                            "italic": false
                        },
                        "fore_color": "#faf1d1",
                        "text_decoration": 3
                    }
                },
                "sheet_id": "uEnW3A"
            }
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
