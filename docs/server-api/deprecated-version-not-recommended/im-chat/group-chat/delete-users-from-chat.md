---
document_id: '6967261389551304709'
directory_id: '6907567266536652801'
title: 移除用户出群
full_path: /ukTMukTMukTM/uADMwUjLwADM14CMwATN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Group Chat
- Delete Users From Chat
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:21Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uADMwUjLwADM14CMwATN
---

# 移除用户出群
机器人移除用户出群。

:::html
<md-alert type="tip">
需要启用机器人能力；“机器人是群主” 或 “机器人在群内，是群的创建者  且  具备 ==更新应用创建的群信息== 权限”。
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
      <md-td>https://open.larksuite.com/open-apis/chat/v4/chatter/delete/</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    
    
    <md-tr>
    </md-tr>
    <md-tr>
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

### 请求体
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
|chat_id|string|必填| 群 ID||oc_e03a63d98c0f3b36c329d3546405c490|
|user_ids|list|选填| 需要移除出群的用户的 user_id 列表，最多可以传200个(open_ids 和 user_ids 参数不能同时为空)|nil|["33417745","cb93bdca"]|
|open_ids|list|选填| 需要移除出群的用户的 open_id 列表，最多可以传200个(open_ids 和 user_ids 参数不能同时为空)|nil|["ou_4065981088f8ef67a504ba8bd6b24d85","ou_111111111111111111111111111111111"]|

### 请求体示例
```json
{
    "chat_id": "oc_e03a63d98c0f3b36c329d3546405c490",
    "user_ids": [
        "33417745",
        "cb93bdca"
    ],
    "open_ids": [
        "ou_4065981088f8ef67a504ba8bd6b24d85",
        "ou_111111111111111111111111111111111"
    ]
}
```

## 响应

### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码描述|
|data|-|-
&emsp;∟invalid_open_ids|list| 无效的 open_id 	列表|
&emsp;∟invalid_user_ids|list| 无效的 user_id 列表|

### 响应体示例
```json
{
    "code": 0,
    "data": {
        "invalid_open_ids": [
            "ou_111111111111111111111111111111111"
        ],
        "invalid_user_ids": [
            "33417745"
        ]
    },
    "msg": "ok"
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
