---
document_id: '7070695615567265798'
directory_id: '6907567266537291777'
title: 更新应用分组信息
full_path: /uAjLw4CM/ukTMukTMukTM/application-v6/application/patch
breadcrumb:
- Server API
- App Information
- Admin
- Update application information
document_type: ReferenceDocumentType
updated_at: 2022-03-03T02:30:16Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/patch
---

# 更新应用分组信息

更新应用的分组信息（分组会影响应用在工作台中的分类情况，请谨慎更新）{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=application&version=v6&resource=application&method=patch)

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
| HTTP URL | https://open.larksuite.com/open-apis/application/v6/applications/:app_id |
| HTTP Method | PATCH |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="application:application" desc="更新应用信息" support_app_types="custom" tags="">⁣更新应用信息</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 应用的 id<br>**示例值**："cli_9b445f5258795107" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >lang</md-text> | <md-text type="field-type" >string</md-text> | 是 | 指定返回的语言<br>**示例值**："zh_cn"<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |




### 请求体
:::html
引用类型：<md-text type="field-type" >application</md-text>
:::

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >common_categories</md-text> | <md-text type="field-type" >string\[\]</md-text> | 否 | 应用分类的国际化描述<br>**数据校验规则**：<br>- 长度范围：`1` ～ `3` |




### 请求体示例

```json
{
    "common_categories": [
        "分析工具"
    ]
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success"
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 210503 | invalid app_id | 请检查请求路径中的 app_id 是否合法 |
| 400 | 210504 | no such app in tenant | 请检查被查询应用与当前调用接口应用是否在同一企业内 |
| 400 | 210505 | target app not a custom app | 请检查被查询应用是否是自建应用 |
| 400 | 210506 | no such app | 请检查请求路径中的 app_id 是否存在 |
| 400 | 211000 | size of common categories out of range, should be between 1 and 3 | 请检查传入的 categories  列表长度是否在 [1, 3] 范围内 |
| 400 | 211001 | common_categories[%d](%s) not exist (index starts from 0) | 请按照提示中的下标，核对传入的应用分类值是否正确，应用分类语言取值需与传入的 lang 参数对应 |





