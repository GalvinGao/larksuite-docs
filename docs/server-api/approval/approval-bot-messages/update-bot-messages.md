---
document_id: '6967331173081907205'
directory_id: '7122028361538813958'
title: 更新审批 Bot 消息
full_path: /ukTMukTMukTM/uAjNyYjLwYjM24CM2IjN
breadcrumb:
- Server API
- Approval
- Approval Bot messages
- Update Bot messages
document_type: GuideDocumentType
updated_at: 2023-01-31T12:17:11Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uAjNyYjLwYjM24CM2IjN
---

# 更新审批 Bot 消息

此接口可以根据审批bot消息id及相应状态，更新相应的审批bot消息，只可用于更新待审批模板的bot消息。例如，给用户推送了审批待办消息，当用户处理该消息后，可以将之前推送的Bot消息更新为已审批。
## 请求
:::html
<md-alert type="tip"> **注意：** 只支持更新 30 天以内的审批 bot 消息。 </md-alert> 
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
      <md-td>https://www.larksuite.com/approval/openapi/v1/message/update</md-td>
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
</md-th>
      <md-td>
<md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm>
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
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
 
**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"
          
 [了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)
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

|参数|类型|必须|说明|
|-|-|-|-|
|message_id|String|是| 卡片 id，发送卡片时会拿到|
|status|string|是|状态类型，用于更新第一个action文字内容，枚举：<br> APPROVED:&emsp;已同意<br>REJECTED:&emsp;已拒绝<br>CANCELLED:&emsp;已撤回<br>FORWARDED:&emsp;已转交<br>ROLLBACK:&emsp;已回退<br>ADD:&emsp;已加签<br>DELETED:&emsp;已删除<br>PROCESSED:&emsp;已处理<br>CUSTOM:&emsp;自定义按钮状态|
|status_name|String|否| status=CUSTOM时可以自定义审批同意/拒绝后title状态 |
|detail_action_name|String|否| status=CUSTOM时可以自定义审批同意/拒绝后“查看详情按钮名称” |
|i18n_resources|String|否| i18n国际化文案 |

### 请求体示例

```json
{
    "message_id":"xxxx",
    "status":"CUSTOM",
    "status_name":"@i18n@status_name", // status=CUSTOM时可以自定义
    "detail_action_name":"@i18n@detail_action_name", // status=CUSTOM时可以自定义
    "i18n_resources":[
        {
          "locale": "zh_cn",
          "texts" : {
              "@i18n@status_name": "已废弃",
              "@i18n@detail_action_name": "已废弃按钮" 
            },
          "is_default": true
        }
    ]
}
```

## 响应

### 响应体

|参数|类型|必须|说明|
|-|-|-|-|
|code|int|是|错误码，非 0 表示失败|
|msg|string|是|返回码的描述|
|data|map|是|返回业务信息|
|&emsp;∟message_id|string|是|消息 id ，用于卡片更新、撤回|

### 响应体示例

```json
{
    "code":0,
    "msg":"success",
    "data":{
        "message_id": "xxxx"
    }
}
```
