---
document_id: '6965400907875221510'
directory_id: '6907567266541404162'
title: 获取角色成员列表
full_path: /ukTMukTMukTM/uczMwUjL3MDM14yNzATN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- User
- Obtain a List of Role Members
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:32Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uczMwUjL3MDM14yNzATN
---

# 获取角色成员列表

该接口用于获取角色下的用户列表。



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
      <md-td>https://open.larksuite.com/open-apis/contact/v2/role/members?role_id=or_846ea69995a259a27cc690182f27de87&page_size=2&page_token=763bd1e74d05e958</md-td>
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

## 查询参数
|参数|类型|必须|说明|
|-|-|-|-|
|role_id|string|是|角色 ID|
|page_token|string|否|分页标记，第一次请求不填，表示从头开始遍历；分页查询还有更多成员时会同时返回新的 page_token, 下次遍历可采用该 page_token 获取更多成员|
|page_size|int|否|分页大小，最大支持 200；默认为 20|

## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码的描述|
|data|-|返回业务信息|
|&emsp;∟has_more|bool|是否还有更多成员。当 has_more 为 true 时，会同时返回新的 page_token|
|&emsp;∟page_token|string|分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token|
|&emsp;∟user_list|list|用户列表|
|&emsp;&emsp;∟name|string|用户名|
|&emsp;&emsp;∟open_id|string|用户 open_id|
|&emsp;&emsp;∟user_id|string|用户企业内唯一标识，企业自建应用返回，应用商店应用不返回|
|&emsp;&emsp;∟scope|-|用户在角色内的管理范围|
|&emsp;&emsp;&emsp;∟is_all_department|bool|管理范围是否为租户全部部门，仅在 true 时返回该参数|
|&emsp;&emsp;&emsp;∟department_ids|list|部门自定义 ID，没有 is_all_department 参数时返回具体管理的部门|
|&emsp;&emsp;&emsp;∟open_department_ids|list|部门 open_id，没有 is_all_department 参数时返回具体管理的部门|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "has_more": true,
        "page_token": "763bd1e74d05e95e",
        "user_list": [
            {
                "name": "Jack",
                "open_id": "ou_84aad35d084aa403a838cf73ee144ec1",
                "user_id": "gbdfb31g"，
                "scope": { 
                	"department_ids": ["TT-1234","TT-1235"],
                	"open_department_ids": ["od-c02cc3b685a711db3a0f14fc4cdb76dc","od-fa83d3690de01787618b85bb27a013bc"],
    		}
            },
            {
                "name": "Hency",
                "open_id": "ou_6b77dbe8459863069c1a62f9cd810217",
                "user_id": "977ea122",
                "scope": {
                  "is_all_department": true
            	}
            }
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

