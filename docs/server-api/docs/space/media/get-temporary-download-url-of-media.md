---
document_id: '7028022131297173509'
directory_id: '7026178505877667845'
title: 获取素材临时下载链接
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/batch_get_tmp_download_url
breadcrumb:
- Server API
- Docs
- Space
- Media
- Get Temporary Download URL of Media
document_type: ReferenceDocumentType
updated_at: 2022-03-13T13:40:22Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/batch_get_tmp_download_url
---

# 获取素材临时下载链接

通过file_token获取素材临时下载链接，链接时效性是24小时，过期失效。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=media&method=batch_get_tmp_download_url)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="tip">
该接口不支持太高的并发，且调用频率上限为5QPS
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/v1/medias/batch_get_tmp_download_url |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="vc:material" desc="更新视频会议特效素材" support_app_types="custom,isv" tags="">更新视频会议特效素材</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm><br><md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv" tags="">查看、评论和导出多维表格</md-perm><br><md-perm name="docs:doc:readonly" desc="查看、评论和导出文档" support_app_types="custom,isv" tags="">查看、评论和导出文档</md-perm><br><md-perm name="sheets:spreadsheet:readonly" desc="查看、评论和导出电子表格" support_app_types="custom,isv" tags="">查看、评论和导出电子表格</md-perm><br><md-perm name="moments:moments" desc="查询、创建、修改和删除公司圈内容、板块" support_app_types="custom,isv" tags="">查询、创建、修改和删除公司圈内容、板块</md-perm><br><md-perm name="moments:moments:readonly" desc="查询公司圈内容、板块" support_app_types="custom,isv" tags="">查询公司圈内容、板块</md-perm><br><md-perm name="vc:material:readonly" desc="获取视频会议特效素材" support_app_types="custom,isv" tags="">获取视频会议特效素材</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >file_tokens</md-text> | <md-text type="field-type" >string\[\]</md-text> | 是 | 文件标识符列表<br>**示例值**：boxcnabcdefg |
| <md-text type="field-name" >extra</md-text> | <md-text type="field-type" >string</md-text> | 否 | 拓展信息(可选)<br>**示例值**："[请参考-上传点类型及对应Extra说明](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/introduction)" |






## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >tmp_download_urls</md-text> | <md-text type="field-type" >tmp_download_url\[\]</md-text> | 临时下载列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >file_token</md-text> | <md-text type="field-type" >string</md-text> | 文件标识符 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >tmp_download_url</md-text> | <md-text type="field-type" >string</md-text> | 文件临时下载链接 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "tmp_download_urls": [
            {
                "file_token": "boxcnabcdefg",
                "tmp_download_url": "https://internal-api-drive-stream.larksuite.com/space/api/box/stream/download/authcode/?code=ZDA3MzNiNmUwMjE2MGUzZmQ1OGZlOWYzMWQ4YmI0ZjdfMDYzOWNlZjgyMmI1MmY5NTUxZmM0MjJlYWIyMGVjOWZfSUQ6Njk3NjgzMTY0Mjc5OTI5MjQyMl8xNjI0NDMxMDY3OjE2MjQ1MTc0NjdfVjM"
            }
        ]
    }
}
```




