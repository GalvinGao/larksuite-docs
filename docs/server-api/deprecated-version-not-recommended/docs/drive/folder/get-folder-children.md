---
document_id: '6967331158355820550'
directory_id: '7312653929568092166'
title: 获取文件夹下的文档清单
full_path: /ukTMukTMukTM/uEjNzUjLxYzM14SM2MTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- Drive
- Folder
- Get Folder Children
document_type: GuideDocumentType
updated_at: 2023-12-25T07:13:00Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uEjNzUjLxYzM14SM2MTN
---

# 获取文件夹下文档清单

该接口用于根据 folderToken 获取该文件夹的文档清单，如 doc、sheet、file、bitable、docx、folder。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/explorer/v2/folder/:folderToken/children |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

### 路径参数
|参数|类型|必须|说明|
|--|-----|--|----|
|folderToken|string|是|文件夹的 token，获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)| 

### 查询参数
|参数|类型|必须|说明|
|--|-----|--|----|
|types|array<string>|否|需要查询的文件类型，默认返回所有 children；types 可多选，可选类型有 doc、sheet、file、bitable、docx、folder 。如 url?types=folder&types=sheet|

### 响应体
|参数|说明|
|--|--|
|parentToken|文件夹的 token|
|children|文件夹的下的文件|
|&emsp;∟token|文件的 token|
|&emsp;∟name|文件的标题|
|&emsp;∟type|文件的类型|
  
### 响应体示例

```json
  {
	"code": 0,
	"msg": "Success",
	"data": {
		"parentToken": "token",
		"children": {
			"nodbc9eC8*****UFOq05rLhpjzc": {
				"token": "fldbcRho4*****3mJkOAuPUZR9d", // 文件夹 token
				"name": "test_folder_name",
				"type": "folder",
          },
                  "nodbcOjPU*****aVVjRDSw4mpeb": {
                          "token": "boxbcj55r*****YAS3C7Z4GWKNg", // 文件 token
                          "name": "test_file_name",
                          "type": "file",
                      }
		}
    }
}
```
>  当请求对象为权限升级后的个人空间时，返回的数据除个人空间中的文件/文件夹外，还会返回共享空间中自己创建的文件夹
  
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
