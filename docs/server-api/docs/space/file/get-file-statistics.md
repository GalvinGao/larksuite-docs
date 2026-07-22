---
document_id: '7031483915610423301'
directory_id: '7031445675029561349'
title: 获取文件统计信息
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-statistics/get
breadcrumb:
- Server API
- Docs
- Space
- File
- Get File Statistics
document_type: ReferenceDocumentType
updated_at: 2022-03-13T13:40:21Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-statistics/get
---

# 获取文件统计信息

此接口用于获取文件统计信息，包括文档阅读人数、次数和点赞数。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=file.statistics&method=get)

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

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/v1/files/:file_token/statistics |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm><br><md-perm name="drive:drive.metadata:readonly" desc="查看云空间中文件元数据" support_app_types="custom,isv" tags="">查看云空间中文件元数据</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >file_token</md-text> | <md-text type="field-type" >string</md-text> | 文件 token<br>**示例值**："doccnRs*******" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >file_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 文档类型<br>**示例值**："doc"<br>**可选值有**：<br>- `doc`：文档<br>- `sheet`：表格<br>- `mindnote`：思维笔记<br>- `bitable`：多维表格<br>- `wiki`：知识库<br>- `file`：文件 |






## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >file_token</md-text> | <md-text type="field-type" >string</md-text> | 文件 token |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >file_type</md-text> | <md-text type="field-type" >string</md-text> | 文件类型 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >statistics</md-text> | <md-text type="field-type" >file_statistics</md-text> | 文件统计信息 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >uv</md-text> | <md-text type="field-type" >int</md-text> | 文件历史访问人数，同一用户（user_id）多次访问按一次计算。 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pv</md-text> | <md-text type="field-type" >int</md-text> | 文件历史访问次数，同一用户（user_id）多次访问按多次计算。（注：同一用户相邻两次访问间隔在半小时内视为一次访问） |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >like_count</md-text> | <md-text type="field-type" >int</md-text> | 文件历史点赞总数，若对应的文档类型不支持点赞，返回 -1 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >timestamp</md-text> | <md-text type="field-type" >int</md-text> | 时间戳（秒） |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "file_token": "doccnRs*******",
        "file_type": "doc",
        "statistics": {
            "uv": 10,
            "pv": 15,
            "like_count": 2,
            "timestamp": 1627367349
        }
    }
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 1069601 | fail | 重试，若稳定失败请联系相关业务方 oncall 人员 |
| 400 | 1069602 | param error | 检查参数有效性 |
| 403 | 1069603 | forbidden | 检查用户是否有文件对应的阅读权限 |
| 400 | 1069604 | document not found | 检查文件是否存在 |





