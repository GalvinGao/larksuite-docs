---
document_id: '6967261389551337477'
directory_id: '6907567266536652801'
title: 获取用户所在的群列表
full_path: /ukTMukTMukTM/uQzMwUjL0MDM14CNzATN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Group Chat
- Obtain Group List to Which User Belongs
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:22Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uQzMwUjL0MDM14CNzATN
---

# 获取用户所在的群列表
获取用户所在的群列表。

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
      <md-td>https://open.larksuite.com/open-apis/user/v4/group_list?page_size=2&page_token=6592161138799017988</md-td>
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
page_size | int | 选填 | 分页大小，最大支持 200；默认为 100|100|100|
page_token | string | 选填 | 分页标记，第一次请求不填，表示从头开始遍历；分页查询还有更多群时会同时返回新的 page_token, 下次遍历可采用该 page_token 获取更多群||6592161138799017988|

## 响应

### 响应体
参数 |类型| 说明 
--  | -- | --
code |int| 返回码，非 0 表示失败
msg  |string| 返回码描述
data | - | -
&emsp;∟page_token|string|见请求参数说明
&emsp;∟has_more|bool|还有群未读取完
&emsp;∟groups|-|-
&emsp;&emsp;∟avatar |string| 群头像
&emsp;&emsp;∟description |string| 群描述
&emsp;&emsp;∟chat_id|string| 群 ID
&emsp;&emsp;∟name |string| 群名称
&emsp;&emsp;∟owner_open_id |string| 群主的 open_id
&emsp;&emsp;∟owner_user_id |string| 群主的 user_id（机器人是群主或者应用商店应用没有这个字段）


### 响应体示例
```json
{
    "code": 0,
    "msg": "ok",
    "data": {
        "has_more": true,
        "page_token": "6631829734835617796",
        "groups": [
            {
                "avatar": "http://p2.pstatp.com/origin/78c100066939cf1374f1",
                "chat_id": "oc_41e7bdf4877cfc316136f4ccf6c32613",
                "description": "description 1",
                "name": "group 1",
                "owner_open_id": "ou_f407fcf504d40eac629a91740b4c8ce0",
                "owner_user_id": "deb1gcc7"
            },
            {
                "avatar": "http://p2.pstatp.com/origin/78bb00136a55b07eac95",
                "chat_id": "oc_e7081e51485dce75d6262ceede7a77b8",
                "description": "description 2",
                "name": "group 2",
                "owner_open_id": "ou_50c7e1b638610b781511fadf97118e4a",
                "owner_user_id": "5543fe1d"
            }
        ]
    }
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

