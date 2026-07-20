---
document_id: '6967331158355574790'
directory_id: '6907567269107531778'
title: 事件订阅
full_path: /ukTMukTMukTM/uUTNzUjL1UzM14SN1MTN/events
breadcrumb:
- Server API
- Docs
- Subscribe to Events
document_type: GuideDocumentType
updated_at: 2022-03-11T12:24:21Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUTNzUjL1UzM14SN1MTN/events
---

# 事件订阅
:::html
<md-alert type="tip">
了解事件订阅的使用场景和配置流程，请点击查看 [事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)
</md-alert>
:::

该接口用于根据文件token和文件类型订阅 Doc或 Sheet 的事件。
-  注意：目前只支持订阅自己文件的事件
-  依赖权限：==查看、评论、编辑和管理电子表格== 或 ==查看、评论、编辑和管理云文档所有文件== 或 ==查看、评论、编辑和管理文档==
-  特殊说明：应用需要调用该接口订阅之后，还需要到应用开发者后台添加具体的，如[文件已读](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/event/file-read)，[文件标题变更](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/event/file-title-update)等

**请求方式** ：POST <br> 
**请求地址** ：https://open.larksuite.com/open-apis/drive/v1/files/:**file_token**/subscribe <br>
**请求Header**:<br>
key|value
--|--
Authorization|Bearer user_access_token
Content-Type|application/x-www-form-urlencoded

**请求参数说明** :  <br>
|参数|类型|必须|说明|来源|
|--|-----|--|----|----|
|Authorization|string|是|user_access_token 通过接口 [获取登录用户身份](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/authen/access_token) 或者 [code2session](/document/uYjL24iN/ukjM04SOyQjL5IDN) 获得；<br>tenant_access_token 通过接口[获取应用身份访问凭证](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/g)；<br>注意内容不要漏了 "Bearer "|请求 Header|
|file_token|string|是|文件 token| URL PATH|
|file_type|string|是|文件类型  "doc"  or  "sheet"|Query|

**返回 Body** ：  
```json
{
	"code": 0,
	"msg": "Success",
	"data": {}
}
```

