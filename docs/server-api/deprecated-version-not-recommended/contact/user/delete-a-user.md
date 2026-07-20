---
document_id: '6965400907875074054'
directory_id: '6907567266541404162'
title: 删除用户
full_path: /ukTMukTMukTM/uUzNz4SN3MjL1czM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- User
- Delete a User
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:23Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUzNz4SN3MjL1czM
---

# 删除用户
:::html

<md-alert type="error">

为了更好地提升该接口的安全性，我们对其进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/delete)

</md-alert>

:::

该接口用于从通讯录中删除用户。


调用该接口需要申请`更新通讯录`以及`以应用身份访问通讯录`权限。应用需要有待删除用户、待删除用户的所有部门的通讯录权限才能删除该用户。应用商店应用无权限调用接口。<br>

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
      <md-td>https://open.larksuite.com/open-apis/contact/v1/user/delete</md-td>
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
|employee_id、open_id|string|是|被删除用户，支持通过 employee_id 或者 open_id 删除用户。请求至少包含被删除用户的 employee_id 或者 open_id 之一，同时传递两个参数时按 employee_id 处理|
|department_chat_acceptor|object|否|部门群接收者<br>被删除用户为部门群群主时，转让群主给指定接收者，不指定接收者则默认转让给群内第一个入群的人<br>接收者通过 employee_id 或者 open_id 标识，两者都设置时按 employee_id 处理|
|external_chat_acceptor|object|否|外部群接收者<br>被删除用户为外部群群主时，转让群主给指定接收者，不指定接收者则默认转让给群内与被删除用户在同一组织的第一个入群的人，如果组织内只有该用户在群里，则解散外部群<br>接收者通过 employee_id 或者 open_id 标识，两者都设置时按 employee_id 处理|
|docs_acceptor|object|否|文档接收者<br>用户被删除时，其拥有的文档转让给接收者，不指定接收者则默认转让给直接领导，如果无直接领导则直接删除文档资源<br>接收者通过 employee_id 或者 open_id 标识，两者都传时按 employee_id 处理|
|calendar_acceptor|object|否|日程接收者<br>用户被删除时，其拥有的日程转让给接收者，不指定接收者则默认转让给直接领导，如果无直接领导则直接删除日程资源<br>接收者通过 employee_id 或者 open_id 标识，两者都传时按 employee_id 处理|
|application_acceptor|object|否|应用接收者<br>用户被删除时，其创建的应用转让给接收者，不指定接收者则默认转让给直接领导，如果无直接领导则不会转移应用，会造成应用不可用<br>接收者通过 employee_id 或者 open_id 标识，两者都传时按 employee_id 处理|


::: note
在调用此接口进行删除用户操作时，请关注docs_acceptor和calendar_acceptor的参数设置（详见参数说明），以免造成文档或日程的误删。
:::
### 请求体示例
```json
{
    "employee_id":"2fab123c",
    "open_id":"ou_63532398008342a799dec0",
    "department_chat_acceptor": {
        "open_id": "ou_63532398008666a799d660",
        "employee_id": "2fab663c"
    },
    "external_chat_acceptor": {
        "open_id": "ou_63532398008666a799d660",
        "employee_id": "2fab663c"
    },
    "docs_acceptor": {
        "open_id": "ou_63532398008666a799d660",
        "employee_id": "2fab663c"
    },
    "calendar_acceptor": {
        "open_id": "ou_63532398008666a799d660",
        "employee_id": "2fab663c"
    },
    "application_acceptor": {
        "open_id": "ou_63532398008666a799d660",
        "employee_id": "2fab663c"
    }
}
```


## 响应
### 响应体
**返回参数说明** : 
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|msg|对返回码的文本描述|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success"
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
  
