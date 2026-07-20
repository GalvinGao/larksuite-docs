---
document_id: '6967331173081841669'
directory_id: '7031445675029561349'
title: 新建在线文档
full_path: /ukTMukTMukTM/uQTNzUjL0UzM14CN1MTN
breadcrumb:
- Server API
- Docs
- Space
- File
- Create online document
document_type: GuideDocumentType
updated_at: 2023-11-03T07:58:25Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uQTNzUjL0UzM14CN1MTN
---

# 新建在线文档

在用户云空间指定文件夹中创建文档、电子表格或者多维表格。

如果目标文件夹是我的空间，则新建的文档会在「我的空间」的「归我所有」列表里。

:::html
<md-alert type="error">
云空间中文件夹单层节点上限是1500个，超过限制新建文档接口会返回失败，如果有这类需求，可以考虑将文档新建在不同文件夹中。
  
为了更好地提升该接口的安全性，我们对其进行了升级，如果需要创建文档可以尝试
  [新版本>>](/document/ukTMukTMukTM/ugDM2YjL4AjN24COwYjN)，如果需要创建电子表格可以尝试[新版本>>](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create)

</md-alert>
:::

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
      <md-td>https://open.larksuite.com/open-apis/drive/explorer/v2/file/:folderToken</md-td>
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
	<md-text type="field-name" >folderToken</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文件夹 token，用于在此文件夹下新建文档，获取方式见[如何获取云文档资源相关 token](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::

### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|title|string|是|创建文档的标题。注：type 为 "doc" 时不可用（非必填，请求会被过滤），有创建带标题doc文档需求可用 [创建文档](/document/ukTMukTMukTM/ugDM2YjL4AjN24COwYjN) 接口|||
|type|string|是|需要创建文档的类型  "doc" 、 "sheet"  or  "bitable"|||

### 请求体示例
```json
{
   "title":"测试表格",
   "type":"sheet"
}
```

## 响应
### 响应体
|参数|说明|
|--|--|
|url|新创建文档的 url|
|token|新创建文档的 token|
|revision|新创建文档的版本号|


### 响应体示例
```json
{
   "code":0,
   "msg":"Success",
   "data":{
      "url":"https://example.larksuite.com/sheets/shtcnOko1Ad0HU48HH8KHabcef",
      "token":"shtcnOko1Ad0HU48HH8KHabcef",
      "revision":0
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
