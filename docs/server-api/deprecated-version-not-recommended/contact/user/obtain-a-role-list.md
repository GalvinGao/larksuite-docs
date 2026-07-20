---
document_id: '6965400907875368966'
directory_id: '6907567266541404162'
title: 获取角色列表
full_path: /ukTMukTMukTM/uYzMwUjL2MDM14iNzATN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- User
- Obtain a Role List
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:29Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYzMwUjL2MDM14iNzATN
---

# 获取角色列表
:::html

:::

该接口用于获取企业的用户角色列表。<br>




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
      <md-td>https://open.larksuite.com/open-apis/contact/v2/role/list</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>
    
    
    <md-tr>
      <md-th>
权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
<div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
</md-th>
      <md-td>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 获取角色 </md-perm>
         <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 读取通讯录 </md-perm>
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

## 响应
### 响应体

|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码的描述|
|data|-|返回业务数据|
|&emsp;∟role_list|list|角色列表|
|&emsp;&emsp;∟id|string|角色 ID|
|&emsp;&emsp;∟name|string|角色名称|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "role_list": [
            {
                "id": "or_846ea69995a259a27cc690182f27de87",
                "name": "IT"
            },
            {
                "id": "or_b337b102aff4167bd29189b1d94b77f6",
                "name": "HR"
            }
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

