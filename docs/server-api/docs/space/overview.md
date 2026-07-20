---
document_id: '7031483915610439685'
directory_id: '6921376028544499713'
title: 概述
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction
breadcrumb:
- Server API
- Docs
- Space
- Overview
document_type: GuideDocumentType
updated_at: 2022-10-08T09:37:24Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction
---

# 云空间概述

云空间是便捷管理知识资源的企业云盘。在Lark所有文档都集中存储在云端，支持多格式文件的高速上传、下载及预览，并通过电脑、手机随时随地查看、编辑、共享和协作，打造触手可及的知识智库。

:::html
<md-table>
<md-thead>
<tr>
<md-th style="width: 20%;">资源</md-th>
<md-th style="width: 80%;">资源定义</md-th>
</tr>
</md-thead>
<md-tbody>
  
<md-tr>
<md-td>
**文件夹**
</md-td>
<md-td>
用于管理文件和其它文件夹的容器。
</md-td>
</md-tr>

<md-tr>
<md-td>
**文件**
</md-td>
<md-td>
各种类型的文件的统称，泛指云空间内所有的文件。
</md-td>
</md-tr>

</md-tbody>

</md-table>
:::



## 资源：文件夹 Folder
文件夹是用于管理文件和其它文件夹的容器。每个文件夹都有唯一 token 作为标识。
:::note
由于线上资源存在新老规范，文件夹 token 在部分接口中的命名可能为  folder_token, token, folderToken，在调用时请仔细阅读接口文档，避免因命名问题导致报错。
:::

##  字段说明
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 20%;">类型</md-th>
      <md-th style="width: 50%;">描述</md-th>
      </md-tr>
  </md-thead>
  
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >folder_token</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	一个文件夹的唯一标识。  <br><br>
    由于线上资源存在新老规范，文件夹 token 在部分接口中的命名可能为  folder_token, token, folderToken，在调用时请仔细阅读接口文档，避免因命名问题导致报错。<br><br>
      
**示例值**："fldcnK0sP9zb1TejQsaN0S54cHc"

**字段权限要求（任选其一）**：<br>
<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="drive:drive:readonly	" desc="查看、评论和下载云空间中所有文件" tags="">查看、评论和下载云空间中所有文件</md-perm>
	</md-td>
</md-tr>

<md-tr>
	<md-td>
	<md-text type="field-name" >name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文件夹的名称。

	</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::



### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)
:::html

<md-table>

<md-thead>

<tr>

<md-th style="width: 70%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 10%;">权限要求（满足任一）</md-th>

<md-th style="width: 10%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）**</md-td></md-th>

<md-th style="width: 5%;">商店</md-th>
<md-th style="width: 5%;">自建</md-th>

</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" >[新建文件夹](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/create_folder)

   `POST` /open-apis/drive/v1/files/create_folder
  
  </md-text>

</md-td>

<md-td><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>
<md-tag type="token-user" >user_access_token</md-tag>

</md-td>

<md-td>
**✓**
</md-td>
<md-td>
**✓**
</md-td>


</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[获取空间根目录](/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta)</md-text>
  
`GET` /open-apis/drive/explorer/v2/root_folder/meta
  
  > 获取云空间的根目录
</md-td>


<md-td><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>
<md-tag type="token-user" >user_access_token</md-tag>

</md-td>

<md-td>
**✓**
</md-td>
<md-td>
**✓**
</md-td>


</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[获取文件夹元信息](/document/ukTMukTMukTM/uAjNzUjLwYzM14CM2MTN)</md-text>
  
`GET` /open-apis/drive/explorer/v2/folder/:folderToken/meta
 
  > 获取文件夹的元信息
</md-td>

<md-td>
<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>


</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>
<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>
**✓**
</md-td>
<md-td>
**✓**
</md-td>


</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[获取文件夹下的文档清单](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list)</md-text>
  
`GET` /open-apis/drive/v1/files

</md-td>

<md-td>

<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>
<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>
**✓**
</md-td>
<md-td>
**✓**
</md-td>


</md-tr>



</md-tbody>

</md-table>

:::
  
  
## 资源：文件 Files
文件是各种类型的文件的统称，泛指云空间内所有的文件。每个文件都有唯一 token 作为标识。


##  字段说明
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 20%;">类型</md-th>
      <md-th style="width: 50%;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >file_token</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	一个文件的唯一标识。  <br><br>
    

      
**示例值**："boxcnK7G8kasZRac70Wo50y6NGh"

**字段权限要求（任选其一）**：<br>
<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="drive:drive:readonly	" desc="查看、评论和下载云空间中所有文件" tags="">查看、评论和下载云空间中所有文件</md-perm>
	</md-td>
</md-tr>

<md-tr>
	<md-td>
	<md-text type="field-name" >file_name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文件的名称。

	</md-td>

</md-tr>
    
    <md-tr>
	<md-td>
	<md-text type="field-name" >parent_node</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文件所在文件夹的 token。

	</md-td>

</md-tr>

  </md-tbody>
</md-table>
:::



### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)
:::html

<md-table>

<md-thead>

<tr>

<md-th style="width: 70%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 10%;">权限要求（满足任一）</md-th>

<md-th style="width: 10%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）**</md-td></md-th>

<md-th style="width: 5%;">商店</md-th>
<md-th style="width: 5%;">自建</md-th>

</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" >[上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/create)</md-text>

`POST` /open-apis/drive/v1/files/upload_all
  > 用于上传 20M 以内的文件
 

</md-td>

<md-td>

<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm>

</md-td>

<md-td>

<md-tag ype="token-tenant">tenant_access_token</md-tag>
<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[分片上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/multipart-upload-file-/introduction)</md-text>

  `POST` /open-apis/drive/v1/files/upload_prepare<br>
    `POST` /open-apis/drive/v1/files/upload_part<br>
    `POST` /open-apis/drive/v1/files/upload_finish

  
>上传较大文件（>20M）时，推荐使用分片上传

</md-td>

<md-td>

<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm>

</md-td>

<md-td>

<md-tag ype="token-tenant">tenant_access_token</md-tag>
  <md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[下载文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/download)</md-text>

  `GET` /open-apis/drive/v1/files/:file_token/download
  

</md-td>

<md-td>

<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[复制文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/copy)</md-text>

`POST` /open-apis/drive/v1/files/:file_token/copy
  

</md-td>

<md-td>

<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>
  
  <md-tr>

<md-td>

<md-text type="field-name" >[移动文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/move)</md-text>

`POST` /open-apis/drive/v1/files/:file_token/move
  

</md-td>

<md-td>

<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[删除文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete)</md-text>

  `DELETE` /open-apis/drive/v1/files/:file_token
  


</md-td>

<md-td>

<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

<md-tag type="token-user">user_access_token</md-tag>
</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[获取文件元数据](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/meta/batch_query)</md-text>

`POST` /open-apis/drive/v1/metas/batch_query
>支持批量获取文件的元数据

</md-td>

<md-td>

<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="drive:file" desc="上传、下载文件" tags="">上传、下载文件</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

<md-tag type="token-user">user_access_token</md-tag>
</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>


</md-tbody>

</md-table>

:::
