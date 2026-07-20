---
document_id: '6967331158355738630'
directory_id: '7312653929568059398'
title: 删除Doc
full_path: /ukTMukTMukTM/uATM2UjLwEjN14CMxYTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- Drive
- File
- Delete a Doc
document_type: GuideDocumentType
updated_at: 2023-12-25T07:13:07Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uATM2UjLwEjN14CMxYTN
---

# 删除 Doc


该接口用于根据 docToken 删除对应的 Docs 文档。

:::html

<md-alert type="warn">

文档只能被文档所有者删除，文档被删除后将会放到回收站里
</md-alert>

:::
:::note
该接口不支持并发调用，且调用频率上限为5QPS
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
      <md-td>https://open.larksuite.com/open-apis/drive/explorer/v2/file/docs/:docToken</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>DELETE</md-td>
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
<md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm>
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
或
<md-tag mode="inline" type="token-user">user_access_token</md-tag>
          
**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"
          
 [了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)
	</md-td>
   </md-tr>
  </md-tbody> 
</md-table>
:::

::: note
::: html
使用 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag> 前，请确保该应用是文档的所有者，否则会报无权限错误。
</md-td>
:::
<br>

### 路径参数
:::html
<md-table>
  <md-thead>
      <tr>
      <md-th style="width: 30%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
      <md-th >描述</md-th>
      </tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >docToken</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	doc 的 token，获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::


## 响应
### 响应体
|参数|说明|
|--|--|
|id|doc 的 id「字符串类型」|
|result|删除结果|


### 响应体示例
```json
{
    "code":0,
    "msg":"success",
    "data":
    {
        "id":"id string",
        "result":true
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
