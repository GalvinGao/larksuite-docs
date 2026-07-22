---
document_id: '6967331173082284037'
directory_id: '6907567266540732418'
title: 获取文件夹元信息
full_path: /ukTMukTMukTM/uAjNzUjLwYzM14CM2MTN
breadcrumb:
- Server API
- Docs
- Space
- Folder
- Get Folder Meta
document_type: GuideDocumentType
updated_at: 2023-08-03T02:02:19Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uAjNzUjLwYzM14CM2MTN
---

# 获取文件夹元信息


该接口用于根据 folderToken 获取该文件夹的元信息。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/explorer/v2/folder/:folderToken/meta |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive.metadata:readonly" desc="查看云空间中文件元数据" support_app_types="custom,isv" tags="">查看云空间中文件元数据</md-perm> |

  
### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793aabcef"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >folderToken</md-text> | <md-text type="field-type" >string</md-text> | 文件夹 token，获取方式见[如何获取云文档资源相关 token](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6) |



## 响应
  
### 响应体
|参数|说明|
|--|--|
|id|文件夹的 id|
|name|文件夹的标题|
|token|文件夹的 token|
|createUid|文件夹的创建者 id|
|editUid|文件夹的最后编辑者 id|
|parentId|文件夹的上级目录 id|
|ownUid|文件夹为个人文件夹时，为文件夹的所有者 id；文件夹为共享文件夹时，为文件夹树id|


### 响应体示例
```json
{
	"code": 0,
	"msg": "Success",
	"data": {
      "id": "7110173013420512356",
      "name": "name",
      "token": "nodbcbHUdOsS613xVzTzFEabcef",
      "createUid": "7103496998321312356",
      "editUid": "7103496998321312356",
      "parentId": "0",
      "ownUid": "7110173013420512356"
    }
}
```
  
### 错误码

| 错误码 | 说明 | 排查建议 |
| --- | --- | --- |
| 91201 | FAILED | 处理失败，稍后重试。 |
| 91202 | PARAMERR | 参数错误，检查参数是否正确，如：`type`、`fileToken`、`dstFolderToken`。 |
| 91203 | NOTEXIST | 请检查请求参数是否正确，如：`type`跟`fileToken`是否匹配。 |
| 91204 | FORBIDDEN | 检查当前账户对文档、文件夹的权限。参考[接入流程授权](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-overview#6d744fe3) |
| 91205 | DELETED | 来源文件已被删除，检查是否还存在。 |
| 91206 | OUT_OF_LIMIT | 超过限制。 |
| 91207 | DUPLICATE | 重复记录。 |
| 91208 | REVIEW | 内容审查不通过。 |

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
