---
document_id: '6967261389551321093'
directory_id: '6907567266536652801'
title: 获取群成员列表
full_path: /ukTMukTMukTM/uUzMwUjL1MDM14SNzATN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Group Chat
- Obtain Member List
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:21Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUzMwUjL1MDM14SNzATN
---

# 获取群成员列表
如果用户在群中，则返回该群的成员列表。

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
      <md-td>https://open.larksuite.com/open-apis/chat/v4/members?chat_id=oc_92c3f700c2ae31369cefee459fb93870&page_token=0&page_size=3</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>
    
    
    <md-tr>
      <md-th>
 权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
</md-th>
      <md-td>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 读取群信息 </md-perm>
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
<md-tag mode="inline" type="token-user">user_access_token</md-tag>
 
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
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
chat_id| string| 必填 | 群 ID ||oc_92c3f700c2ae31369cefee459fb93870|
page_size | int | 选填 | 分页大小，最大支持 200；默认为 100；page_size只是大概的数目，实际获取到的群成员数目可能大于或者小于page_zise|100|100|
page_token | string | 选填 | 分页标记，第一次请求不填，表示从头开始遍历；分页查询还有更多群成员时会同时返回新的 page_token, 下次遍历可采用该 page_token 获取更多群成员 ||1559288627|


## 响应

### 响应体
参数 |类型| 说明 
--  | -- | --
code |int| 返回码，非 0 表示失败
msg  |string| 返回码描述
data | - | -
&emsp;∟chat_id|string| 群 ID
&emsp;∟has_more|bool| 还有群成员未读取完
&emsp;∟page_token|string| 见请求参数说明
&emsp;∟members|list| 成员列表 
&emsp;&emsp;∟name|string| 成员的 name
&emsp;&emsp;∟open_id|string| 成员的 open_id
&emsp;&emsp;∟user_id|string| 成员的 user_id，只返回给企业自建应用

### 响应体示例  
```json
{
    "code": 0,
    "data": {
        "chat_id": "oc_92c3f700c2ae31369cefee459fb93870",
        "has_more": true,
        "members": [
            {
                "open_id": "ou_56799ac95e82434b49e1cf00c3a3a251",
                "user_id": "1g6gbf73",
                "name": "张三"
            },
            {
                "open_id": "ou_9c7a2ce4f61e78dfe00ffa8b11524e2a",
                "user_id": "296f8dfb",
                "name": "李四"
            },
            {
                "open_id": "ou_fdeaf10d447a41d0a8d561454da197c9",
                "user_id": "afc6b4fb",
                "name": "王五"
            },
            {
                "open_id": "ou_7f8d47e1a788345c0d167abfbf3835b4",
                "user_id": "84eeea8d",
                "name": "赵六"
            }
        ],
        "page_token": "1559288627"
    },
    "msg": "ok"
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
