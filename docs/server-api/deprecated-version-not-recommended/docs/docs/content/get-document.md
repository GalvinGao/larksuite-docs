---
document_id: '6967331173081268229'
directory_id: '7154288300269879302'
title: 获取旧版文档富文本内容
full_path: /ukTMukTMukTM/uUDM2YjL1AjN24SNwYjN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- Docs
- Content
- Get Document
document_type: GuideDocumentType
updated_at: 2023-09-22T08:05:21Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDM2YjL1AjN24SNwYjN
---

# 获取旧版文档富文本内容
:::note 
在使用此接口前，请仔细阅读[文档概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-doc-overview)和[准备接入文档 API](/document/ukTMukTMukTM/ugzNzUjL4czM14CO3MTN/guide/getting-start)了解文档调用的规则和约束，确保你的文档数据不会丢失或出错。  
文档数据结构定义可参考：[文档数据结构概述](/document/ukTMukTMukTM/uAzM5YjLwMTO24CMzkjN) 
:::
:::note 
此接口只支持获取旧版文档富文本内容，如果需要获取新版文档的富文本内容，请调用新版文档相关接口：
- [获取文档所有块](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list)
- [获取指定块](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/get)
- [获取指定块下所有子块](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/get)
:::
该接口用于获取结构化的文档内容。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/doc/v2/:docToken/content |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm><br><md-perm name="docs:doc:readonly" desc="查看、评论和导出文档" support_app_types="custom,isv" tags="">查看、评论和导出文档</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag> 或 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::
<br>
### 路径参数

| 参数          | 类型   | 必须 | 说明 | 
| ------------- | ------ | ---- | --------- | 
| docToken      | string | 是   | 获取方式详见[如何获取云文档资源相关 token](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6) | 

### Curl 请求 Demo
```
curl -H 'Authorization: Bearer u-CMtIuEpUAu7NdYOI4fLNjg' 'https://open.larksuite.com/open-apis/doc/v2/doccng20xjcYZ2C4FNEoHFGQZng/content'
```

## 响应
### 响应体
| 参数     | 说明                              |
| -------- | --------------------------------- |
| content  | 详情参考[文档数据结构](/document/ukTMukTMukTM/ukDM2YjL5AjN24SOwYjN) |
| revision | 文档当前版本号                    |
### 响应体示例

```json
{
    "code": 0,
    "msg": "Success",
    "data": {
        "content": "{}",
        "revision": 1000  
    }
}
```

### 错误码

| 错误码 | 说明 | 排查建议 |
| --- | --- | --- |
| 91401 | PARAMERR | 参数出现错误，检查参数有效性 |
| 91402 | NOTEXIST | 未找到，检查token是否有效 |
| 91403 | FORBIDDEN | 没有权限，检查是否有文档读权限 |
| 91404 | LOGIN_REQUIRED | 需要登录 |
| 95001 | internal error | 内部错误，请稍后重试 |
| 95003 | internal error | 内部错误，请稍后重试 |
| 95005 | internal error | 内部错误，请稍后重试 |
| 95006 | Failed | 文档未找到，检查token是否有效 |
| 95007 | Failed | 文档已删除，已删除文件无法获取文档meta信息 |
| 95008 | FORBIDDEN | 检查用户对文档、文件夹的权限 |
| 95009 | Failed | 没有权限，检查是否有文档读权限。[添加文档权限](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create) |
| 95010 | internal error | 内部错误，请稍后重试 |
| 95011 | internal error | 内部错误，请稍后重试 |
| 95017 | 具体错误信息 | 读取文档内容失败，检查revison是否正确 |
| 95018 | 具体错误信息 | 解析文档内容失败，详见具体错误信息 |
| 95023 | revision too old | 版本号太老，请使用最新版本号 |
| 95024 | Failed | 参数无效，检查参数有效性 |
| 95053 | this API does not support the Upgraded Docs(docx) | 此 API 不支持新版文档（docx） |

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
