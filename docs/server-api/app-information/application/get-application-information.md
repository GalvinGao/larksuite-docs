---
document_id: '7070695615567216646'
directory_id: '7069596583364526085'
title: 获取应用信息
full_path: /uAjLw4CM/ukTMukTMukTM/application-v6/application/get
breadcrumb:
- Server API
- App Information
- Application
- Get Application Information
document_type: ReferenceDocumentType
updated_at: 2022-03-03T02:30:11Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/get
---

# 获取应用信息

根据app_id获取应用的基础信息{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=application&version=v6&resource=application&method=get)

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
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="application:application:self_manage" desc="管理应用自身资源" support_app_types="custom,isv" tags="">管理应用自身资源</md-perm><br><md-perm name="admin:app.info:readonly" desc="获取应用信息" support_app_types="custom" tags="">获取应用信息</md-perm> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |




### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 应用的 app_id，需要查询其他应用信息时，必须申请[获取应用信息](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)权限，仅查询本应用信息时，可填入 "me" 或者应用自身 app_id<br>**示例值**："cli_9b445f5258795107" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >lang</md-text> | <md-text type="field-type" >string</md-text> | 是 | 指定获取应用在该语言下的信息<br>**示例值**："zh_cn"<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| <md-text type="field-name" >user_id_type</md-text> | <md-text type="field-type" >string</md-text> | 否 | 用户 ID 类型<br>**示例值**："open_id"<br>**可选值有**：<br>- `open_id`：用户的 open id<br>- `union_id`：用户的 union id<br>- `user_id`：用户的 user id<br>**默认值**：`open_id`<br>**当值为 `user_id`，字段权限要求**：<br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |






## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app</md-text> | <md-text type="field-type" >application</md-text> | 应用数据 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 应用的 app_id |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >creator_id</md-text> | <md-text type="field-type" >string</md-text> | 应用创建者（所有者） |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >status</md-text> | <md-text type="field-type" >int</md-text> | 应用状态<br>**可选值有**：<br>- `0`：停用状态<br>- `1`：启用状态<br>- `2`：未启用状态<br>- `3`：未知状态 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scene_type</md-text> | <md-text type="field-type" >int</md-text> | 应用类型<br>**可选值有**：<br>- `0`：自建应用<br>- `1`：应用商店应用<br>- `2`：个人应用商店应用<br>- `3`：未知应用类型 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >redirect_urls</md-text> | <md-text type="field-type" >string\[\]</md-text> | 安全设置中的重定向 URL |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >online_version_id</md-text> | <md-text type="field-type" >string</md-text> | 发布在线上的应用版本 ID，若没有则为空 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >unaudit_version_id</md-text> | <md-text type="field-type" >string</md-text> | 在审核中的版本 ID，若没有则为空 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_name</md-text> | <md-text type="field-type" >string</md-text> | 应用名称 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >avatar_url</md-text> | <md-text type="field-type" >string</md-text> | 应用图标 url |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用默认描述 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scopes</md-text> | <md-text type="field-type" >app_scope\[\]</md-text> | 应用权限列表 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scope</md-text> | <md-text type="field-type" >string</md-text> | 应用权限 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用权限的国际化描述 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >level</md-text> | <md-text type="field-type" >string</md-text> | 权限等级描述<br>**可选值有**：<br>- `1`：普通权限<br>- `2`：高级权限<br>- `3`：超敏感权限<br>- `0`：未知等级 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >back_home_url</md-text> | <md-text type="field-type" >string</md-text> | 后台主页地址 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text> | <md-text type="field-type" >app_i18n_info\[\]</md-text> | 应用的国际化信息列表 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text> | <md-text type="field-type" >string</md-text> | 国际化语言的 key<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 应用国际化名称 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用国际化描述（副标题） |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >help_use</md-text> | <md-text type="field-type" >string</md-text> | 帮助国际化文档链接 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >primary_language</md-text> | <md-text type="field-type" >string</md-text> | 应用主语言<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >common_categories</md-text> | <md-text type="field-type" >string\[\]</md-text> | 应用分类的国际化描述 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "app": {
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
    }
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 210503 | invalid app_id | 请检查请求路径中的 app_id 是否合法 |
| 400 | 210504 | no such app in tenant | 请检查被查询应用与当前调用接口应用是否在同一企业内 |
| 400 | 210505 | target app not a custom app | 请检查被查询应用是否是自建应用 |
| 400 | 210506 | no such app | 请检查请求路径中的 app_id 是否存在 |
| 400 | 210508 | insufficient permission level | 请检查应用已申请权限与被查询 app_id，当被查询 app_id 非本应用且未申请[获取应用版本信息](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)权限时，返回该错误码 |





