---
document_id: '6965400907875303430'
directory_id: '6907567266537242625'
title: 获取子部门列表
full_path: /ukTMukTMukTM/ugzN3QjL4czN04CO3cDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Department
- Obtain a Sub-department List
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:50Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugzN3QjL4czN04CO3cDN
---

# 获取子部门列表

该接口用于获取当前部门子部门列表。
:::html
<md-alert type="warn">
调用该接口需要具有当前部门的授权范围。企业根部门 ID 为 0，当获取根部门子部门列表时，通讯录授权范围必须为全员权限。
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
      <md-td>https://open.larksuite.com/open-apis/contact/v1/department/simple/list?open_department_id=od-2efe30807a10608754862a63b108828f&page_size=10&fetch_child=true<br> https://open.larksuite.com/open-apis/contact/v1/department/simple/list?department_id=TT-1234&page_size=10&fetch_child=true</md-td>
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
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 获取部门组织架构信息 </md-perm>
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
### 查询参数 
|参数|类型|必须|说明|
|-|-|-|-|
|department_id、open_department_id|string|是|部门自定义 ID 或者 openID|
|page_token|string|否|分页标记，第一次请求不填，表示从头开始遍历；分页查询还有更多子部门时会同时返回新的 page_token, 下次遍历可采用该 page_token 获取更多子部门|
|page_size|int|是|分页大小，最大支持 100|
|fetch_child|bool|否|是否递归返回子部门列表，默认不递归|  

## 响应
### 响应体 

|参数|说明|
|-|-|
|code|返回码，非 0 表示失败|
|msg|返回码的描述|
|data|返回业务数据|
|&emsp;∟has_more|分页查询时返回，代表是否还有更多子部门|
|&emsp;∟page_token|分页标记，当 has_more 为 true 时返回该参数，使用该参数调用接口可以获取当前部门的更多子部门信息， has_more 为 false 时不返回|
|&emsp;∟department_infos|子部门列表|
|&emsp;&emsp;∟id|部门自定义 ID|
|&emsp;&emsp;∟name|部门名称|
|&emsp;&emsp;∟parent_id|父部门自定义 ID|
|&emsp;&emsp;∟open_department_id|部门 openID|
|&emsp;&emsp;∟parent_open_department_id|父部门 openID|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "has_more":true,
        "page_token":"763bd1e74d05e95e",
        "department_infos": [
            {
                "id": "TT-2058",
                "name": "Finance",
                "parent_id": "od-2efe30807a10608754862a63b1088266",
                "open_department_id": "od-5cefe25147a103456cf21a63b1132ad",
            	"parent_open_department_id": "od-2efe30807a10608754862a63b108828f"
            },
            {
                "id": "TT-2059",
                "name": "Human Resources",
                "parent_id": "0",
                "open_department_id": "od-342df12587ec2a644003f380dd123cbf",
           		"parent_open_department_id": "od-2efe30807a10608754862a63b108828f"
            }
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


