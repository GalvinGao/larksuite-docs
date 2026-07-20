---
document_id: '6967331158356082694'
directory_id: '7122028361538994182'
title: 上传文件
full_path: /ukTMukTMukTM/uUDOyUjL1gjM14SN4ITN
breadcrumb:
- Server API
- Approval
- Native approval document
- Upload files
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:51Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDOyUjL1gjM14SN4ITN
---

# 上传文件

当审批表单中有图片或附件控件时，开发者需在创建审批实例前通过审批上传文件接口将文件上传到审批系统，且附件上传大小限制为50M，图片上传大小为10M。

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
      <md-td>https://www.larksuite.com/approval/openapi/v2/file/upload</md-td>
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
      <md-td> 否 </md-td> 
     <md-td>不支持用户指定 content-type，填写该参数可能会报错

</md-td>
</md-tr>
   
  </md-tbody> 
</md-table>
:::

### 请求体

| 名称         | 类型           | 必须        | 说明        |
| --------- | --------------- | -------   | --------- |
|name | string | 是 |  文件名（需包含文件扩展名，如“文件.doc” |
|type |string | 是 | 文件类型（image 或 attachment）  |
|content | file | 是 |文件 |

::: note
**注意**：每次只能上传一个文件，如有多个文件，请分多次上传。文件类型（image 或 attachment）取决于审批定义表单控件中的具体类型，请按定义使用。
:::

### 请求体示例

```json
{
	"name":"123.doc",
	"type":"attachment",
	"content":123.doc
}
````

## 响应

### 响应体

| 参数         |类型         |必须  | 说明        |
| --------- | ----------|----- | --------- |
|code |int |是 |错误码，非0表示失败 |
|msg | string |是| 返回码的描述|
|data | map |是| 返回业务信息 |
|&emsp;∟code|string|是| 文件标识码（用于创建审批实例）|
|&emsp;∟url|string|是| 文件 url|

::: note
**注意**：返回的 url 有效期为12小时，发起审批后, 每次获取详情都会获得新的 url。
:::
### 响应体示例

```json
{
    "code":0,
    "msg":"success",
    "data": {
        "code": "D93653C3-2609-4EE0-8041-61DC1D84F0B5",
        "url": "https://p3-approval-sign.byteimg.com/lark-approval-attachment/image/20210819/a8c1a1f1-47ae-4147-9deb-a8bf2cd833b1.jpg~tplv-ottatrvjsm-image.image?x-expires=1634941752&x-signature=oaZ6Tfv50ryUesNwKTUTnBlJivY%3D#.jpg"
    }
}
```
