---
document_id: '7072724425749102597'
directory_id: '7072711453267181573'
title: 审批任务加签
full_path: /ukTMukTMukTM/ukTM5UjL5ETO14SOxkTN/approval-task-addsign
breadcrumb:
- Server API
- Approval
- Approval tasks
- Add approvers
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:48Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukTM5UjL5ETO14SOxkTN/approval-task-addsign
---

# 审批任务加签

对于单个审批任务进行加签操作。

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
      <md-td>https://open.larksuite.com/open-apis/approval/v4/instances/add_sign</md-td>
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
|approval_code|String|是|审批定义 Code|
|instance_code|String|是|审批实例 Code|
|user_id|String|是|操作用户|
|task_id|String|是|任务 ID<br>审批实例详情task_list中id，详情请参考[](/document/ukTMukTMukTM/uEDNyUjLxQjM14SM0ITN)|
|comment|String|否|意见|
|add_sign_user_ids|List<string>|是|被加签人id|
|add_sign_type|String|是|1/2/3分别代表前加签/后加签/并加签|
|approval_method|String|否|仅在前加签、后加签时需要填写，1/2 分别代表或签/会签|

### 请求体示例

```json
{
    "approval_code": "3B68E280-CF10-4198-B4CD-2E3BB97981D8",
    "instance_code": "289330DE-FBF1-4A47-91F9-9EFCCF11BCAE",
    "user_id": "b16g66e3",
    "task_id": "6955096766400167956",
    "comment": "addSignComment",
    "add_sign_user_ids": ["d19b913b","3313g62b"],
    "add_sign_type": 1,
    "approval_method": 1
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
