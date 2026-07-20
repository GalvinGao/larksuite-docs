---
document_id: '6965400907878285317'
directory_id: '6907567266537242625'
title: 删除部门
full_path: /ukTMukTMukTM/ugzNz4CO3MjL4czM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Department
- Delete a Department
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:59Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugzNz4CO3MjL4czM
---

# 删除部门
:::html

<md-alert type="error">

为了更好地提升该接口的安全性，我们对其进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/delete)

</md-alert>

:::
该接口用于从通讯录中删除部门。 <br>

- 调用该接口需要申请`更新通讯录`以及`以应用身份访问通讯录`权限。应用需要同时拥有待删除部门及其父部门的通讯录授权。应用商店应用无权限调用该接口。


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
      <md-td>https://open.larksuite.com/open-apis/contact/v1/department/delete</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    
    
    <md-tr>
      <md-th>
权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
<div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
</md-th>
      <md-td>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 更新通讯录 </md-perm>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份访问通讯录（历史版本）</md-perm>
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
|id|string|是|待删除部门 ID|
### 请求体示例 
```json
{
    "id": "od-455efa262dc736b3e45a8b17fe945293"
}
```


## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|对返回码的文本描述|
### 响应体示例

```json
{
    "code": 0,
    "msg": "success"
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)






