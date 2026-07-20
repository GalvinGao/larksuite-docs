---
document_id: '6967331173081481221'
directory_id: '6908984614440042498'
title: 获取旧版文档元信息
full_path: /ukTMukTMukTM/uczN3UjL3czN14yN3cTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- Docs
- Docs
- Obtain Document Meta
document_type: GuideDocumentType
updated_at: 2023-09-22T08:05:09Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uczN3UjL3czN14yN3cTN
---

# 获取旧版文档元信息
:::note 
此接口只支持查询旧版文档元信息，如果需要查询新版文档元信息，请使用[获取元数据](/document/ukTMukTMukTM/uMjN3UjLzYzN14yM2cTN)接口。
:::
该接口用于根据 docToken 获取元数据。

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
      <md-td>https://open.larksuite.com/open-apis/doc/v2/meta/:docToken</md-td>
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
<md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm>
<md-perm name="drive:drive.metadata:readonly" desc="查看云空间中文件元数据" support_app_types="custom,isv" tags="">查看云空间中文件元数据</md-perm>
<md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm>
<md-perm name="docs:doc:readonly" desc="查看、评论和导出文档" support_app_types="custom,isv" tags="">查看、评论和导出文档</md-perm>
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
          
 [了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)
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
|参数|类型|必须|说明|
|--|-----|--|----|----|
|docToken|string|是|doc 的 token，获取方式见[如何获取云文档资源相关 token](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)|

### Curl 请求 Demo
```
curl -H 'Authorization: Bearer u-s12okJw4R1VCZLWhk9Zyzg' 'https://open.larksuite.com/open-apis/doc/v2/meta/doccnilYPZU5b34ow4ca7aNoU6a' 
```

## 响应
### 响应体
|参数|类型|说明|
|--|--|--|
|create_date|string|创建日期|
|create_time|integer|创建时间戳|
|creator|string|创建者open_id|
|create_user_name|string|创建者用户名|
|delete_flag|integer|删除标志，0表示正常访问未删除，1表示在回收站，2表示已经彻底删除|
|edit_time|integer|最后编辑时间戳|
|edit_user_name|string|最后编辑者用户名|
|is_external|bool|是否外部文档|
|is_pined|bool|是否在接口调用者目录里快速访问|
|is_stared|bool|是否在接口调用者目录里收藏|
|obj_type|string|文档类型，固定是doc|
|owner|string|当前所有者open_id|
|owner_user_name|string|当前所有者用户名|
|server_time|integer|处理请求时的服务器时间戳|
|tenant_id|string|文档所在租户id|
|title|string|文档名称|
|type|integer|文档类型，固定是2|
|url|string|文档url|

### 响应体示例
```json
{
    "code": 0,
    "msg": "Success",
    "data": {
        "create_date": "integer string",
        "create_time": integer,
        "creator": "string",
        "create_user_name": "string",
        "delete_flag": integer,
        "edit_time": integer,
        "edit_user_name": "string",
        "is_external": bool,
        "is_pined": bool,
        "is_stared": bool,
        "obj_type": "doc",
        "owner": "string",
        "owner_user_name": "string",
        "server_time": integer,
        "tenant_id": "interger string",
        "title": "string",
        "type": 2,
        "url": "string"
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
        <md-td>91401</md-td>
        <md-td>PARAMERR</md-td>
        <md-td>参数出现错误，检查参数有效性</md-td>
    </md-tr>
    <md-tr>
        <md-td>91402</md-td>
        <md-td>NOTEXIST</md-td>
        <md-td>未找到，检查token是否有效</md-td>
    </md-tr>
    <md-tr>
        <md-td>91403</md-td>
        <md-td>FORBIDDEN</md-td>
        <md-td>没有权限，检查是否有文档读权限</md-td>
    </md-tr>
    <md-tr>
        <md-td>91404</md-td>
        <md-td>LOGIN_REQUIRED</md-td>
        <md-td>需要登录</md-td>
    </md-tr>
    <md-tr>
        <md-td>95001</md-td>
        <md-td>internal error</md-td>
        <md-td>内部错误，请稍后重试</md-td>
    </md-tr>
    <md-tr>
        <md-td>95003</md-td>
        <md-td>internal error</md-td>
        <md-td>内部错误，请稍后重试</md-td>
    </md-tr>
    <md-tr>
        <md-td>95005</md-td>
        <md-td>internal error</md-td>
        <md-td>内部错误，请稍后重试</md-td>
    </md-tr>
    <md-tr>
        <md-td>95006</md-td>
        <md-td>Failed</md-td>
        <md-td>文档未找到，检查token是否有效</md-td>
    </md-tr>
    <md-tr>
        <md-td>95007</md-td>
        <md-td>Failed</md-td>
        <md-td>文档已删除，已删除文件无法获取文档meta信息</md-td>
    </md-tr>
    <md-tr>
        <md-td>95008</md-td>
        <md-td>FORBIDDEN</md-td>
        <md-td>检查用户对文档、文件夹的权限</md-td>
    </md-tr>
    <md-tr>
        <md-td>95009</md-td>
        <md-td>Failed</md-td>
        <md-td>没有权限，检查是否有文档读权限。[添加文档权限](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create)</md-td>
    </md-tr>
    <md-tr>
        <md-td>95010</md-td>
        <md-td>internal error</md-td>
        <md-td>内部错误，请稍后重试</md-td>
    </md-tr>
    <md-tr>
        <md-td>95011</md-td>
        <md-td>internal error</md-td>
        <md-td>内部错误，请稍后重试</md-td>
    </md-tr>
    <md-tr>
        <md-td>95017</md-td>
        <md-td>具体错误信息</md-td>
        <md-td>读取文档内容失败，检查revison是否正确</md-td>
    </md-tr>
    <md-tr>
        <md-td>95018</md-td>
        <md-td>具体错误信息</md-td>
        <md-td>解析文档内容失败，详见具体错误信息</md-td>
    </md-tr>
    <md-tr>
        <md-td>95023</md-td>
        <md-td>revision too old</md-td>
        <md-td>版本号太老，请使用最新版本号</md-td>
    </md-tr>
    <md-tr>
        <md-td>95024</md-td>
        <md-td>Failed</md-td>
        <md-td>参数无效，检查参数有效性</md-td>
    </md-tr>
    <md-tr>
        <md-td>95053</md-td>
        <md-td>this API does not support the Upgraded Docs(docx)</md-td>
        <md-td>此 API 不支持新版文档（docx）</md-td>
    </md-tr>
  </md-tbody> 
</md-table>
:::
具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
