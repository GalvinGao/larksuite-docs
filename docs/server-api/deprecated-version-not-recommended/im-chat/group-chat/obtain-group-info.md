---
document_id: '6967261389551386629'
directory_id: '6907567266536652801'
title: 获取群信息
full_path: /ukTMukTMukTM/uMTO5QjLzkTO04yM5kDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Group Chat
- Obtain Group Info
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:17Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMTO5QjLzkTO04yM5kDN
---

# 获取群信息
获取群名称、群主 ID、成员列表 ID 等群基本信息。  

:::html
<md-alert type="tip">
需要启用机器人能力；机器人必须在群里
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
      <md-td>https://open.larksuite.com/open-apis/chat/v4/info?chat_id=oc_eb9e82d5657777ebf1bb5b9024f549ef</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
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

### 查询参数
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
chat_id| string| 必填 | 群 ID||oc_eb9e82d5657777ebf1bb5b9024f549ef|

## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
code |int| 返回码，非 0 表示失败
|msg|string|返回码描述|
data | - | - 
&emsp;∟avatar |string| 群头像
&emsp;∟description |string| 群描述
&emsp;∟i18n_names |map| 群国际化名称（设置了国际化名称才会有这个字段）
&emsp;∟chat_id|string| 群 ID
&emsp;∟members|list| 成员列表
&emsp;&emsp;∟open_id|string| 某成员的open_id
&emsp;&emsp;∟user_id|string| 某成员的user_id
&emsp;∟name |string| 群名称，类型为group时有效
&emsp;∟type |string| 群类型，group表示群聊，p2p表示单聊
&emsp;∟owner_user_id |string| 群主的 user_id（机器人是群主的时候没有这个字段）
&emsp;∟owner_open_id |string| 群主的 open_id （机器人是群主的时候没有这个字段）
&emsp;∟only_owner_edit|bool|是否仅群主可编辑群信息，群信息包括头像、名称、描述、公告
&emsp;∟only_owner_add|bool|是否仅群主可以添加人
&emsp;∟share_allowed|bool|是否允许分享群
&emsp;∟add_member_verify|bool|是否开启入群验证
&emsp;∟only_owner_at_all|bool|是否仅群主@all
&emsp;∟send_message_permission|string|允许谁发送消息<br>all: 所有人<br>  owner： 仅群主<br>selected_member: 指定成员
&emsp;∟join_message_visibility|string|成员入群通知<br>all：所有人 <br>owner：仅群主 <br>not_anyone：不通知任何人"
&emsp;∟leave_message_visibility|string|成员退群通知<br>all：所有人 <br>owner：仅群主 <br>not_anyone：不通知任何人
&emsp;∟group_email_enabled|bool|是否开启群邮件
&emsp;∟send_group_email_permission|	string|	发送群邮件的权限<br>owner：仅群主  <br>group_member：群组内成员<br>tenant_member：团队成员 <br>all：所有人

### 响应体示例
```json
{
    "code": 0,
    "data": {
        "chat_id": "oc_fb4fd349d4xxxxxxxxxxxxxxxxxx",
        "type": "group"
        "name": "group name",
        "i18n_names": {
            "en_us": "en_us name",
            "ja_jp": "ja_jp name",
            "zh_cn": "zh_cn name"
        },
        "description": "group description",
        "avatar": "https://p3-lark-file.byteimg.com/img/lark.avatar/default-avatar_xxxxxxxxxxxxxx.jpg",
        "members": [
            {
                "open_id": "ou_xxxxxxx",
                "user_id": "8e17d887"
            }
        ],
        "only_owner_add": false,
        "only_owner_at_all": false,
        "only_owner_edit": false,
        "share_allowed": true,
        "add_member_verify": false,
        "send_message_permission": "all",
        "join_message_visibility": "all",
        "leave_message_visibility": "owner",
        "group_email_enabled": false,
        "send_group_email_permission": "tenant_member",
        "owner_open_id": "ou_6edde2deccabb76c12b30f0345f19aa1",
        "owner_user_id": "8e17d887"
    },
    "msg": "ok"
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
