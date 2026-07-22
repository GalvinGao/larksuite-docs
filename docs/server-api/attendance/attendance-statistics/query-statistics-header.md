---
document_id: '7070902815602917382'
directory_id: '7070770034936119301'
title: 查询统计表头
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_field/query
breadcrumb:
- Server API
- Attendance
- Attendance Statistics
- Query Statistics Header
document_type: ReferenceDocumentType
updated_at: 2022-03-03T15:54:13Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_field/query
---

# 查询统计表头

查询考勤统计支持的日度统计或月度统计的统计表头。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=user_stats_field&method=query)

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
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_stats_fields/query |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="attendance:task" desc="写入打卡数据" support_app_types="custom" tags="">写入打卡数据</md-perm><br><md-perm name="attendance:task:readonly" desc="导出打卡数据" support_app_types="custom" tags="">导出打卡数据</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 响应体中的 user_id 的员工工号类型<br>**示例值**："employee_id"<br>**可选值有**：<br>- `employee_id`：员工 employee ID，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的用户 ID<br>- `employee_no`：员工工号，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的工号 |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >locale</md-text> | <md-text type="field-type" >string</md-text> | 是 | 语言类型<br>**示例值**："zh"<br>**可选值有**：<br>- `en`：英语<br>- `ja`：日语<br>- `zh`：中文 |
| <md-text type="field-name" >stats_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 统计类型<br>**示例值**："daily"<br>**可选值有**：<br>- `daily`：日度统计<br>- `month`：月度统计 |
| <md-text type="field-name" >start_date</md-text> | <md-text type="field-type" >int</md-text> | 是 | 开始时间<br>**示例值**：20210316 |
| <md-text type="field-name" >end_date</md-text> | <md-text type="field-type" >int</md-text> | 是 | 结束时间（时间间隔不超过 40 天）<br>**示例值**：20210323 |




### 请求体示例

```json
{
    "locale": "zh",
    "stats_type": "daily",
    "start_date": 20210316,
    "end_date": 20210323
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_stats_field</md-text> | <md-text type="field-type" >user_stats_field</md-text> | 统计数据表头 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >stats_type</md-text> | <md-text type="field-type" >string</md-text> | 统计类型<br>**可选值有**：<br>- `daily`：日度统计<br>- `month`：月度统计 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >fields</md-text> | <md-text type="field-type" >field\[\]</md-text> | 字段列表 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 字段编号 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text> | <md-text type="field-type" >string</md-text> | 字段名称 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >child_fields</md-text> | <md-text type="field-type" >child_field\[\]</md-text> | 子字段列表 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 子字段编号 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text> | <md-text type="field-type" >string</md-text> | 子字段名称 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >time_unit</md-text> | <md-text type="field-type" >string</md-text> | 时间单位 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "",
    "data": {
        "user_stats_field": {
            "fields": [
                {
                    "child_fields": [
                        {
                            "code": "50103",
                            "title": "Employee ID"
                        }
                    ],
                    "code": "501",
                    "title": "Basic info"
                },
                {
                    "child_fields": [
                        {
                            "code": "52108",
                            "title": "Attendance group name"
                        },
                        {
                            "code": "52101",
                            "title": "Required attendance days"
                        },
                        {
                            "code": "52102",
                            "title": "Days of attendance"
                        },
                        {
                            "code": "52104",
                            "time_unit": "Minute",
                            "title": "Required attendance duration"
                        },
                        {
                            "code": "52105",
                            "time_unit": "Minute",
                            "title": "Actual attendance duration"
                        },
                        {
                            "code": "52107",
                            "title": "Overtime hours"
                        }
                    ],
                    "code": "521",
                    "title": "Attendance statistics"
                },
                {
                    "child_fields": [
                        {
                            "code": "52201",
                            "title": "Late in times"
                        },
                        {
                            "code": "52203",
                            "title": "Early out times"
                        },
                        {
                            "code": "52207",
                            "title": "No records"
                        }
                    ],
                    "code": "522",
                    "title": "Abnormal statistics"
                },
                {
                    "child_fields": [
                        {
                            "code": "2021-03-16",
                            "title": "2021-03-16 Tue"
                        },
                        {
                            "code": "2021-03-17",
                            "title": "2021-03-17 Wed"
                        },
                        {
                            "code": "2021-03-18",
                            "title": "2021-03-18 Thu"
                        },
                        {
                            "code": "2021-03-19",
                            "title": "2021-03-19 Fri"
                        },
                        {
                            "code": "2021-03-20",
                            "title": "2021-03-20 Sat"
                        },
                        {
                            "code": "2021-03-21",
                            "title": "2021-03-21 Sun"
                        },
                        {
                            "code": "2021-03-22",
                            "title": "2021-03-22 Mon"
                        },
                        {
                            "code": "2021-03-23",
                            "title": "2021-03-23 Tue"
                        }
                    ],
                    "code": "524",
                    "title": "Daily statistics"
                }
            ],
            "stats_type": "month",
            "user_id": "ec8ddg56"
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





