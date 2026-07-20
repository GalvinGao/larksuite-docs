---
document_id: '6967331158355083270'
directory_id: '7312699691689836550'
title: 判断协作者是否有某权限
full_path: /ukTMukTMukTM/uYzN3UjL2czN14iN3cTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- Drive
- permission
- Querying if a Collaborator Has a Specific Permission
document_type: GuideDocumentType
updated_at: 2023-12-25T07:12:53Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYzN3UjL2czN14iN3cTN
---

# 判断当前用户对某文档是否有某权限

该接口用于根据 filetoken 判断当前登录用户是否具有某权限。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive_permission&version=v1&resource=member&method=permitted)

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
      <md-td>https://open.larksuite.com/open-apis/drive/permission/member/permitted</md-td>
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
<div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
</md-th>
   <md-td>
<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm>
<md-perm name="drive:file" desc="上传、下载文件到云空间" support_app_types="custom,isv" tags="">上传、下载文件到云空间</md-perm>
<md-perm name="drive:file:readonly" desc="查看和下载云空间中的文件" support_app_types="custom,isv" tags="">查看和下载云空间中的文件</md-perm>
<md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm>
<md-perm name="docs:doc:readonly" desc="查看、评论和导出文档" support_app_types="custom,isv" tags="">查看、评论和导出文档</md-perm>
<md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理表格</md-perm>
<md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出表格" support_app_types="custom,isv" tags="">查看、评论和导出表格</md-perm>
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
<md-tag mode="inline" type="token-user">user_access_token</md-tag> 或 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
 
**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"
          
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

::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|token|string|是|文件的 token，获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction) | 
|type|string|是|文档类型  "doc"  or  "sheet" or "file"| 
|perm|string|是|权限，"view" or "edit" or "share"| 

### 请求体示例
```json
{
	"token": "doccnBKgoMyY5OMbUG6FioTXuBe",
	"type": "doc",
 	"perm": "string"
}
```
## 响应
### 响应体
|参数|说明|
|--|--|
|is_permitted|是否具有指定权限|

### 响应体示例
```json
{
    "code": 0,
    "msg": "Success",
    "data": {
        "is_permitted": true
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
