---
document_id: '6975751842354413573'
directory_id: '6975751873563557894'
title: 更新统计设置
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//task/update-user-stats-settings
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Task
- Update Statistics Settings
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:27Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//task/update-user-stats-settings
---

# 更新统计设置
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/update)
</md-alert>
:::
更新日度统计或月度统计的统计设置信息。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_stats_views/:user_stats_view_id |
| HTTP Method | PUT |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm>写入打卡数据</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant-desc">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用 access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |

### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >user_stats_view_id</md-text> | <md-text type="field-type" >string</md-text> | 用户视图 ID<br>**示例值**："TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09" |


### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户 ID 类型<br>**可选值有**：<br>- `employee_id`：用户员工 ID<br>- `employee_no`：用户员工工号 |


### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >view</md-text> | <md-text type="field-type" >user_stats_view</md-text> | 是 | 统计视图 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 视图 ID<br>**示例值**："TmpnNU5EQXpPVGN3TmpVMU16Y3lPVEEwTXl0dGIyNTBhQT09" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >stats_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 统计类型<br>**可选值有**：<br>- `daily`：日度统计<br>- `month`：月度统计 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >items</md-text> | <md-text type="field-type" >item\[\]</md-text> | 否 | 一级标题 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 是 | 编号<br>**示例值**："501" |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text> | <md-text type="field-type" >string</md-text> | 否 | 标题名称<br>**示例值**："基本信息" |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >child_items</md-text> | <md-text type="field-type" >child_item\[\]</md-text> | 否 | 子标题 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 是 | 标题编号<br>**示例值**："50101" |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text> | <md-text type="field-type" >string</md-text> | 是 | 开关字段<br>**可选值有**：<br>- `0`：关闭<br>- `1`：开启<br>非开关字段场景<br>-  code = 51501  **可选值为1～6** |


### 请求体示例

```json
{
    "view": {
        "items": [
            {
                "child_items": [
                    {
                        "code": "50102",
                        "value": "0"
                    },
                    {
                        "code": "50111",
                        "value": "0"
                    },
                    {
                        "code": "50104",
                        "value": "0"
                    }
                ],
                "code": "501"
            }
        ],
        "stats_type": "month",
        "user_id": "ec8ddg56",
        "view_id": "TmpnNU5EQXpPVGN3TmpVMU16Y3lPVEEwTXl0dGIyNTBhQT09"
    }
}

```

## 响应

### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view</md-text> | <md-text type="field-type" >user_stats_view</md-text> | 用户视图 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view_id</md-text> | <md-text type="field-type" >string</md-text> | 统计视图 ID<br>**示例值**："TmpnNU5EQXpPVGN3TmpVMU16Y3lPVEEwTXl0dGIyNTBhQT09" |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >stats_type</md-text> | <md-text type="field-type" >string</md-text> | 统计类型<br>**可选值有**：<br>- `daily`：日度统计<br>- `month`：月度统计 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >items</md-text> | <md-text type="field-type" >item\[\]</md-text> | 一级标题 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 标题编码 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text> | <md-text type="field-type" >string</md-text> | 标题名称 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >child_items</md-text> | <md-text type="field-type" >child_item\[\]</md-text> | 子标题 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 标题编号 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text> | <md-text type="field-type" >string</md-text> | 是否开启<br>**可选值有**：<br>- `0`：关闭<br>- `1`：开启 |

### 响应体示例

```json
{
    "code": 0,
    "msg": "",
    "data": {
        "view": {
            "items": [
                {
                    "child_items": [
                        {
                            "code": "50102",
                            "value": "0"
                        },
                        {
                            "code": "50111",
                            "value": "0"
                        },
                        {
                            "code": "50104",
                            "value": "0"
                        }
                    ],
                    "code": "501"
                }
            ],
            "stats_type": "month",
            "user_id": "ec8ddg56",
            "view_id": "TmpnNU5EQXpPVGN3TmpVMU16Y3lPVEEwTXl0dGIyNTBhQT09"
        }
    }
}
```

### 错误码

| HTTP 状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1220001 | 参数错误 | 请检查参数是否符合要求 |
| 400 | 1220002 | 租户不存在 | 请检查 tenant_access_token 是否正确 |
| 500 | 1228000 | 统计服务系统错误 | 详见错误信息 |



