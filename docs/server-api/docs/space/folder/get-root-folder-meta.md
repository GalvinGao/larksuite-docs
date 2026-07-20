---
document_id: '7031483915610488837'
directory_id: '6907567266540732418'
title: 获取我的空间（root folder）元信息
full_path: /ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta
breadcrumb:
- Server API
- Docs
- Space
- Folder
- Get Root Folder Meta
document_type: GuideDocumentType
updated_at: 2023-08-03T02:02:15Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta
---

# 获取我的空间（root folder）元信息

该接口用于获取 "我的空间" 的元信息

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
      <md-td>https://open.larksuite.com/open-apis/drive/explorer/v2/root_folder/meta</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
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
<md-perm name="drive:drive.metadata:readonly" desc="查看云空间中文件元数据" support_app_types="custom,isv" tags="">查看云空间中文件元数据</md-perm>
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

**示例值**："Bearer u-7f1bcd13fc57d46bac21793aabcef"
          
 [了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)
	</md-td>
</md-tr>
     
   
  </md-tbody> 
</md-table>
:::
::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

## 响应
  
### 响应体
|参数|说明|
|--|--|
|token|文件夹的 token|
|id|文件夹的 id|
|user_id|文件夹的所有者 id


### 响应体示例
```json
{
  "code": 0,
  "msg": "Success",
  "data": {
    "token": "nodbcbHUdOsS613xVzTzFEabcef",
    "id": "7110173013420512356",
    "user_id": "7103496998321312356"
	}
}
```
  
### 错误码

:::html
<md-table> 
  <md-thead> 
    <md-tr> 
      <md-th style="width: 15%;">错误码</md-th>  
      <md-th style="width: 25%;">说明</md-th>  
      <md-th style="width: 60%;">排查建议</md-th>  
    </md-tr> 
  </md-thead>  
  <md-tbody> 
     <md-tr> 
      <md-td>91201</md-td>  
      <md-td>FAILED</md-td>  
      <md-td>处理失败，稍后重试。</md-td> 
	</md-tr>
     <md-tr> 
      <md-td>91202</md-td>  
      <md-td>PARAMERR</md-td>  
      <md-td>参数错误，检查参数是否正确，如：`type`、`fileToken`、`dstFolderToken`。</md-td> 
	</md-tr>
     <md-tr> 
      <md-td>91203</md-td>  
      <md-td>NOTEXIST</md-td>  
      <md-td>请检查请求参数是否正确，如：`type`跟`fileToken`是否匹配。</md-td> 
	</md-tr>
     <md-tr> 
      <md-td>91204</md-td>  
      <md-td>FORBIDDEN</md-td>  
      <md-td>检查当前账户对文档、文件夹的权限。参考[接入流程授权](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-overview#6d744fe3)</md-td> 
	</md-tr>
     <md-tr> 
      <md-td>91205</md-td>  
      <md-td>DELETED</md-td>  
      <md-td>来源文件已被删除，检查是否还存在。</md-td> 
	</md-tr>
     <md-tr> 
      <md-td>91206</md-td>  
      <md-td>OUT_OF_LIMIT</md-td>  
      <md-td>超过限制。</md-td> 
	</md-tr>
     <md-tr> 
      <md-td>91207</md-td>  
      <md-td>DUPLICATE</md-td>  
      <md-td>重复记录。</md-td> 
	</md-tr>
     <md-tr> 
      <md-td>91208</md-td>  
      <md-td>REVIEW</md-td>  
      <md-td>内容审查不通过。</md-td> 
	</md-tr>
  </md-tbody> 
</md-table>
:::
具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
