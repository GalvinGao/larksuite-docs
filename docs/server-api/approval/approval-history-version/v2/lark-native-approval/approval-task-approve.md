---
document_id: '6967331173081890821'
directory_id: '7122028361538797574'
title: 审批任务同意
full_path: /ukTMukTMukTM/uMDNyUjLzQjM14yM0ITN
breadcrumb:
- Server API
- Approval
- Approval（history version）
- v2
- Lark native approval
- Approval Task Approve
document_type: GuideDocumentType
updated_at: 2022-07-20T09:39:15Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMDNyUjLzQjM14yM0ITN
---

# 审批任务同意
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/approve)
</md-alert>
:::
对于单个审批任务进行同意操作。同意后审批流程会流转到下一个审批人。

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
      <md-td>https://www.larksuite.com/approval/openapi/v2/instance/approve</md-td>
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

|参数|类型|必须|说明|
|-|-|-|-|
|approval_code|String|是|审批定义 Code|
|instance_code|String|是|审批实例 Code|
|open_id|String|否|用户open_id，如果没有user_id，必须要有open_id|
|user_id|String|是|操作用户|
|task_id|String|是|任务 ID<br>审批实例详情task_list中id，详情请参考[](/document/ukTMukTMukTM/uEDNyUjLxQjM14SM0ITN)|
|comment|String|否|意见|

### 请求体示例

```
{
    "approval_code": "7C468A54-8745-2245-9675-08B7C63E7A85",
    "instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71",
    "user_id": "f7cb567e",
    "task_id": "12345",
    "comment": "OK",
    "open_id": "ou_123456"
}
```

## 响应

### 响应体

|参数|类型|必须|说明|
|-|-|-|-|
|code|int|是|错误码，非0表示失败|
|msg|String|是|返回码的描述|

### 响应体示例

```json
{
    "code": 0,
    "msg": "success"
}
```
