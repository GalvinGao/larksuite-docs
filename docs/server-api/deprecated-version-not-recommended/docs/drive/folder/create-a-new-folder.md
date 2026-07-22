---
document_id: '6967331158356164614'
directory_id: '7312653929568092166'
title: 新建文件夹
full_path: /ukTMukTMukTM/ukTNzUjL5UzM14SO1MTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- Drive
- Folder
- Create a New Folder
document_type: GuideDocumentType
updated_at: 2023-12-25T07:12:56Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukTNzUjL5UzM14SO1MTN
---

# 新建文件夹

该接口用于根据 folderToken 在该 folder 下创建文件夹。

:::html
<md-alert type="warn">
该接口不支持并发创建，且调用频率上限为 5QPS 以及 10000次/天
</md-alert>
:::

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/explorer/v2/folder/:folderToken |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>or<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

### 路径参数
|参数|类型|必须|说明|
|--|-----|--|----|
|folderToken|string|是|文件夹的 token，获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)| 

### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|title|string|是|文件夹标题| 

### 请求体示例
```json
{
	"title": "string"
}
```
### 响应

### 响应体
|参数|说明|
|--|--|
|url|新创建文件夹的 url|
|revision|新创建文件夹的版本号|
|token|新创建文件夹的 token|

### 响应体示例

```json
{
	"code": 0,
	"msg": "Success",
	"data": {
		"url": "string",
    	"revision": 0,
    	"token": "string"
    }
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

