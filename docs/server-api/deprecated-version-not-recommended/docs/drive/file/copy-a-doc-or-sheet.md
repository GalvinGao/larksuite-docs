---
document_id: '6967331158355345414'
directory_id: '7312653929568059398'
title: 复制文档
full_path: /ukTMukTMukTM/uYTNzUjL2UzM14iN1MTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- Drive
- File
- Copy a Doc or Sheet
document_type: GuideDocumentType
updated_at: 2023-12-25T07:13:03Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYTNzUjL2UzM14iN1MTN
---

# 复制文档


该接口用于根据文件 token 复制 Doc 或 Sheet  到目标文件夹中。
若没有特定的文件夹用于承载创建的文档，可以先调用「获取文件夹元信息」文档中的「获取 root folder (我的空间) meta」接口，获得我的空间的 token，然后再使用此接口。复制的文档将会在「我的空间」的「归我所有」列表里。


:::note
该接口不支持并发创建，且调用频率上限为 5QPS 且 10000次/天
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
      <md-td>https://open.larksuite.com/open-apis/drive/explorer/v2/file/copy/files/:fileToken</md-td>
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
        	<md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm>
            <md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm>
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
	<md-text type="field-name" >fileToken</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	需要复制的源文件或文档的 token, 获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::

### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|type|string|是|需要创建文档的类型   "doc"、"sheet"、"bitable"、"docx" |||
|dstFolderToken|string|是|目标文件夹的 token, 获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction) |||
|dstName|string|是|复制的副本文件的新名称 |||
|commentNeeded|bool|否|是否复制评论 |||


### 请求体示例
```json
{
  "type":"objType",
  "dstFolderToken":"string",
  "dstName":"string",
  "commentNeeded":true
}
```

## 响应
### 响应体
|参数|说明|
|--|--|
|folderToken|目标文件夹的 token|
|revision|新创建文档的版本号|
|token|新创建文档的 token|
|type|新建文档的类型，"doc"、"sheet"、"bitable"、"docx" |
|url|新创建文档的 url|


### 响应体示例
```json
{
    "code":0,
    "msg":"Success",
    "data":{
        "folderToken":"fldcne0HujIvzDmRF4Pbg0xxxxx",
        "revision":0,
        "token":"shtcnvJ358XqcZq87CCZHdxxxxx",
        "type":"sheet",
        "url":"https://bytedance.larksuite.com/space/sheet/shtcnvJ358XqcZq87CCZHdxxxxx"
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
