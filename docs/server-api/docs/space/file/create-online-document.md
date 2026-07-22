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

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/explorer/v2/file/:folderToken |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::
<br>

### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >folderToken</md-text> | <md-text type="field-type" >string</md-text> | 文件夹 token，用于在此文件夹下新建文档，获取方式见[如何获取云文档资源相关 token](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6) |


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
