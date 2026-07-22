---
document_id: '7070695615567200262'
directory_id: '6907567266537291777'
title: 查看待审核的应用列表
full_path: /uAjLw4CM/ukTMukTMukTM/application-v6/application/underauditlist
breadcrumb:
- Server API
- App Information
- Admin
- Get Application Release Request List
document_type: ReferenceDocumentType
updated_at: 2022-03-03T02:30:09Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/underauditlist
---

# 查看待审核的应用列表

查看本企业下所有待审核的自建应用列表{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=application&version=v6&resource=application&method=underauditlist)

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
| HTTP URL | https://open.larksuite.com/open-apis/application/v6/applications/underauditlist |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="admin:app.info:readonly" desc="获取应用信息" support_app_types="custom" tags="">获取应用信息</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >lang</md-text> | <md-text type="field-type" >string</md-text> | 是 | 指定返回的语言<br>**示例值**："zh_cn"<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| <md-text type="field-name" >page_token</md-text> | <md-text type="field-type" >string</md-text> | 否 | 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果<br>**示例值**："new-e3c5a0627cdf0c2e057da7257b90376a" |
| <md-text type="field-name" >page_size</md-text> | <md-text type="field-type" >int</md-text> | 否 | 分页大小<br>**示例值**：10<br>**数据校验规则**：<br>- 最大值：`50` |






## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >items</md-text> | <md-text type="field-type" >application\[\]</md-text> | 待审核应用列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 应用的 app_id |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >creator_id</md-text> | <md-text type="field-type" >string</md-text> | 应用创建者（所有者） |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >status</md-text> | <md-text type="field-type" >int</md-text> | 应用状态<br>**可选值有**：<br>- `0`：停用状态<br>- `1`：启用状态<br>- `2`：未启用状态<br>- `3`：未知状态 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scene_type</md-text> | <md-text type="field-type" >int</md-text> | 应用类型<br>**可选值有**：<br>- `0`：自建应用<br>- `1`：应用商店应用<br>- `2`：个人应用商店应用<br>- `3`：未知应用类型 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >redirect_urls</md-text> | <md-text type="field-type" >string\[\]</md-text> | 安全设置中的重定向 URL |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >online_version_id</md-text> | <md-text type="field-type" >string</md-text> | 发布在线上的应用版本 ID，若没有则为空 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >unaudit_version_id</md-text> | <md-text type="field-type" >string</md-text> | 在审核中的版本 ID，若没有则为空 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_name</md-text> | <md-text type="field-type" >string</md-text> | 应用名称<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >avatar_url</md-text> | <md-text type="field-type" >string</md-text> | 应用图标 url |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用默认描述<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scopes</md-text> | <md-text type="field-type" >app_scope\[\]</md-text> | 应用权限列表 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scope</md-text> | <md-text type="field-type" >string</md-text> | 应用权限 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用权限的国际化描述 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >level</md-text> | <md-text type="field-type" >string</md-text> | 权限等级描述<br>**可选值有**：<br>- `1`：普通权限<br>- `2`：高级权限<br>- `3`：超敏感权限<br>- `0`：未知等级 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >back_home_url</md-text> | <md-text type="field-type" >string</md-text> | 后台主页地址 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text> | <md-text type="field-type" >app_i18n_info\[\]</md-text> | 应用的国际化信息列表<br>**数据校验规则**：<br>- 最小长度：`1` |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text> | <md-text type="field-type" >string</md-text> | 国际化语言的 key<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 应用国际化名称<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用国际化描述（副标题）<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >help_use</md-text> | <md-text type="field-type" >string</md-text> | 帮助国际化文档链接 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >primary_language</md-text> | <md-text type="field-type" >string</md-text> | 应用主语言<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >common_categories</md-text> | <md-text type="field-type" >string\[\]</md-text> | 应用分类的国际化描述<br>**数据校验规则**：<br>- 最大长度：`3` |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >has_more</md-text> | <md-text type="field-type" >boolean</md-text> | 是否还有更多项 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >page_token</md-text> | <md-text type="field-type" >string</md-text> | 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "app_id": "cli_9b445f5258795107",
                "creator_id": "ou_d317f090b7258ad0372aa53963cda70d",
                "status": 1,
                "scene_type": 0,
                "redirect_urls": [
                    "https://www.example.com"
                ],
                "online_version_id": "oav_d317f090b7258ad0372aa53963cda70d",
                "unaudit_version_id": "oav_d317f090b7258ad0372aa53963cda70d",
                "app_name": "应用名称",
                "avatar_url": "https://sf1-ttcdn-tos.pstatp.com/img/avatar/d279000ca4d3f7f6aaff~72x72.jpg",
                "description": "应用描述",
                "scopes": [
                    {
                        "scope": "contact:user.base",
                        "description": "获取应用信息",
                        "level": "low_level"
                    }
                ],
                "back_home_url": "https://www.example.com",
                "i18n": [
                    {
                        "i18n_key": "zh_cn",
                        "name": "应用名称",
                        "description": "应用描述",
                        "help_use": "https://www.example.com"
                    }
                ],
                "primary_language": "zh_cn",
                "common_categories": [
                    "分析工具"
                ]
            }
        ],
        "has_more": true,
        "page_token": "new-xxxxxxxxxxx"
    }
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 210500 | page_token does not exist or has expired | 请检查 page_token 是否合法，page_token 过期时间为 2h，若超过 2h 请重新获取 |
| 400 | 210501 | invalid page_token | page_token 在应用间不互通，请检查该 page_token 是否由调用接口的应用获取到 |
| 400 | 210502 | page_size out of range, should be between 1 and 50 | 请检查 page_size 范围是否在 [1, 50] 范围内 |





