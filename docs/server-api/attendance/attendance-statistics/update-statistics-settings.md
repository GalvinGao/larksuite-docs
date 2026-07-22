---
document_id: '7070902815602753542'
directory_id: '7070770034936119301'
title: 更新统计设置
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/update
breadcrumb:
- Server API
- Attendance
- Attendance Statistics
- Update Statistics Settings
document_type: ReferenceDocumentType
updated_at: 2022-03-03T15:54:13Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/update
---

# 更新统计设置

更新开发者定制的日度统计或月度统计的统计报表表头设置信息。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=user_stats_view&method=update)

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
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_stats_views/:user_stats_view_id |
| HTTP Method | PUT |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="attendance:task" desc="写入打卡数据" support_app_types="custom" tags="">写入打卡数据</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >user_stats_view_id</md-text> | <md-text type="field-type" >string</md-text> | 用户视图 ID，获取方式：1）[查询统计设置](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/query)<br>**示例值**："TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 员工工号类型<br>**示例值**："employee_id"<br>**可选值有**：<br>- `employee_id`：员工 employee ID，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的用户 ID<br>- `employee_no`：员工工号，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的工号 |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >view</md-text> | <md-text type="field-type" >user_stats_view</md-text> | 是 | 统计设置 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 视图 ID<br>**示例值**："TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >stats_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 视图类型<br>**示例值**："month"<br>**可选值有**：<br>- `daily`：日度统计<br>- `month`：月度统计 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户 ID<br>**示例值**："ec8ddg56" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >items</md-text> | <md-text type="field-type" >item\[\]</md-text> | 否 | 用户设置字段 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 是 | 标题编号<br>**示例值**："522" |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >child_items</md-text> | <md-text type="field-type" >child_item\[\]</md-text> | 否 | 子标题 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 是 | 子标题编号<br>**示例值**："50101" |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text> | <md-text type="field-type" >string</md-text> | 是 | 开关字段，0：关闭，1：开启（非开关字段场景：code = 51501 可选值为1-6）<br>**示例值**："0" |




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
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view</md-text> | <md-text type="field-type" >user_stats_view</md-text> | 视图 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view_id</md-text> | <md-text type="field-type" >string</md-text> | 视图 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >stats_type</md-text> | <md-text type="field-type" >string</md-text> | 视图类型<br>**可选值有**：<br>- `daily`：日度统计<br>- `month`：月度统计 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >items</md-text> | <md-text type="field-type" >item\[\]</md-text> | 用户设置字段 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 标题编号 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text> | <md-text type="field-type" >string</md-text> | 标题名称 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >child_items</md-text> | <md-text type="field-type" >child_item\[\]</md-text> | 子标题 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 子标题编号 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text> | <md-text type="field-type" >string</md-text> | 开关字段，0：关闭，1：开启（非开关字段场景：code = 51501 可选值为1-6） |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text> | <md-text type="field-type" >string</md-text> | 子标题名称 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >column_type</md-text> | <md-text type="field-type" >int</md-text> | 列类型 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >read_only</md-text> | <md-text type="field-type" >boolean</md-text> | 是否只读 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >min_value</md-text> | <md-text type="field-type" >string</md-text> | 最小值 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >max_value</md-text> | <md-text type="field-type" >string</md-text> | 最大值 |




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

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1220001 | 参数错误 | 请检查参数是否符合要求 |
| 400 | 1220002 | 租户不存在 | 请检查 tenant_access_token 是否正确 |
| 500 | 1228000 | 统计服务系统错误 | 详见错误信息 |





