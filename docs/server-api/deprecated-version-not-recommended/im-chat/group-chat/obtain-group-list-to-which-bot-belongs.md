---
document_id: '6967261316286644229'
directory_id: '6907567266536652801'
title: 获取机器人所在群列表
full_path: /ukTMukTMukTM/uITO5QjLykTO04iM5kDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Group Chat
- Obtain Group List to Which Bot Belongs
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:23Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uITO5QjLykTO04iM5kDN
---

# 获取群列表
获取机器人所在的群列表。

:::html
<md-alert type="tip">
需要启用机器人能力
</md-alert>
:::

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
      <md-td>https://open.larksuite.com/open-apis/chat/v4/list</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
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
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
 
**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"
          
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
page_size | int | 选填 | 分页大小，最大支持 200；默认为 100 |100|100|
page_token | string | 选填 | 分页标记，第一次请求不填，表示从头开始遍历；分页查询还有更多群时会同时返回新的 page_token, 下次遍历可采用该 page_token 获取更多群||1530027865231834600|


## 响应

### 响应体
|参数|类型|说明|
|-|-|-|
code |int| 返回码，非 0 表示失败
msg  |string| 返回码描述
data | - | -
&emsp;∟has_more|bool|还有群未读取完
&emsp;∟page_token|string|见请求参数说明
&emsp;∟groups|-|-
&emsp;&emsp;∟avatar |string| 群头像
&emsp;&emsp;∟description |string| 群描述
&emsp;&emsp;∟chat_id|string| 群 ID
&emsp;&emsp;∟name |string| 群名称
&emsp;&emsp;∟owner_open_id |string| 群主的 open_id
&emsp;&emsp;∟owner_user_id |string| 群主的 user_id（机器人是群主的时候没有这个字段）

### 响应体示例
```json
{
    "code": 0,
    "data": {
        "groups": [
            {
                "avatar": "http://p3.pstatp.com/origin/78c0000676df676a7f6e",
                "chat_id": "oc_9e9619b938c9571c1c3165681cdaead5",
                "description": "description1",
                "name": "test1",
                "owner_open_id": "ou_194911f90c43ec42d1ba0e93f22b8fb1",
                "owner_user_id": "ca51d83b"
            },
            {
                "avatar": "http://p4.pstatp.com/origin/dae10015cfb346541010",
                "chat_id": "oc_5ce6d572455d361153b7cb51da133945",
                "description": "description2",
                "name": "test2",
                "owner_open_id": "ou_194911f90c43ec42d1ba0e93f22b8fb1",
                "owner_user_id": "ca51d83b"
            }
        ],
        "has_more": false,
        "page_token": "0"
    },
    "msg": "ok"
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
