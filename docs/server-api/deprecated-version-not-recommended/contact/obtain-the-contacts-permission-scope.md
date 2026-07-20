---
document_id: '6965400907875401734'
directory_id: '6916079000750178306'
title: 获取通讯录授权范围
full_path: /ukTMukTMukTM/ugjNz4CO2MjL4YzM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Obtain the Contacts Permission Scope
document_type: GuideDocumentType
updated_at: 2022-03-11T11:45:17Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugjNz4CO2MjL4YzM
---

# 获取通讯录授权范围
该接口用于获取应用被授权可访问的通讯录范围，包括可访问的部门列表及用户列表。<br>
授权范围为全员时，返回的部门列表为该企业所有的一级部门；否则返回的部门为管理员在设置授权范围时勾选的部门（不包含勾选部门的子部门）。<br>

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
      <md-td>https://open.larksuite.com/open-apis/contact/v1/scope/get</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>
    
    
    <md-tr>
      <md-th>
 权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
</md-th>
      <md-td>
        <md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
        <md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            字段权限要求
            <md-tooltip type="info">接口返回的部分字段受权限控制，开启字段权限才可获取对应字段数据；如无需获取这些字段，则无需开启。</md-tooltip>
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">根据要获取的字段开启相应权限</div>
      </md-th>
      <md-td>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户 user ID</md-perm>
       
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
|code|int|错误码，非 0 表示失败|
|msg|string|返回码的描述|
|data|-|返回业务数据|
|&emsp;∟authed_departments|list|已授权部门自定义 ID 列表，授权范围为全员可见时返回的是当前企业的所有一级部门列表|
|&emsp;∟authed_open_departments|list|已授权部门 openID 列表，授权范围为全员可见时返回的是当前企业的所有一级部门列表|
|&emsp;∟authed_employee_ids|list|已授权用户 employee_id 列表，应用申请了 `获取用户user_id` 权限时返回；当授权范围为全员可见时返回的是当前企业所有顶级部门用户列表|
|&emsp;∟authed_open_ids|list|已授权用户 open_id 列表；当授权范围为全员可见时返回的是当前企业所有顶级部门用户列表|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "authed_departments": [
            "TT-0001",
            "TT-0002"
        ],
        "authed_open_departments": [
        	 "od-8756c536552a91988b1b64559356c5a4",
            "od-a140b4eeb892b90a0ab3e616fc2054d6"
        ],
        "authed_employee_ids": [
            "c248363e",
            "98a42b48"
        ],
        "authed_open_ids": [
            "ou_5f3e2df282bf5aaeeaa2ea71ab9f1229",
            "ou_89ad6bf1fbf5a39796b556ae06a10d4d"
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

