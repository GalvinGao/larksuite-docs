---
document_id: '6975751842354298885'
directory_id: '6975751873563557894'
title: 查询统计表头
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//task/query-statistics-header
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Task
- Query the Statistics Table Header
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:24Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//task/query-statistics-header
---

# 查询统计表头
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_field/query)
</md-alert>
:::
查询日度统计或月度统计的统计表头。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_stats_fields/query |
| HTTP Method | POST |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm>写入打卡数据</md-perm><br><md-perm >导出打卡数据</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant-desc">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用 access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户 ID 类型<br>**可选值有**：<br>- `employee_id`<br>- `employee_no` |


### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >locale</md-text> | <md-text type="field-type" >string</md-text> | 是 | 语言类型<br>**可选值有**：<br>- `en`：英文<br>- `ja`：日文<br>- `zh`：中文 |
| <md-text type="field-name" >stats_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 统计类型<br>**可选值有**：<br>- `daily`：日度统计<br>- `month`：月度统计 |
| <md-text type="field-name" >start_date</md-text> | <md-text type="field-type" >int</md-text> | 是 | 开始时间<br>**示例值**：20210316<br>（时间间隔不超过 40 天） |
| <md-text type="field-name" >end_date</md-text> | <md-text type="field-type" >int</md-text> | 是 | 结束时间<br>**示例值**：20210323 |


### 请求体示例

```json
{
    "locale": "zh",
    "stats_type": "month",
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
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text> | <md-text type="field-type" >string</md-text> | 字段标题 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >child_fields</md-text> | <md-text type="field-type" >child_field\[\]</md-text> | 子字段列表 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 字段编号 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text> | <md-text type="field-type" >string</md-text> | 字段名称 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >time_unit</md-text> | <md-text type="field-type" >string</md-text> | 时间类型 |

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
                            "title": "工号"
                        }
                    ],
                    "code": "501",
                    "title": "基本信息"
                },
                {
                    "child_fields": [
                        {
                            "code": "52108",
                            "title": "考勤组名称"
                        },
                        {
                            "code": "52101",
                            "title": "应出勤天数"
                        },
                        {
                            "code": "52102",
                            "title": "工作日出勤天数"
                        },
                        {
                            "code": "52104",
                            "time_unit": "分钟",
                            "title": "应出勤时长"
                        },
                        {
                            "code": "52105",
                            "time_unit": "分钟",
                            "title": "实际出勤时长"
                        },
                        {
                            "code": "52107",
                            "title": "加班工作时长"
                        }
                    ],
                    "code": "521",
                    "title": "出勤统计"
                },
                {
                    "child_fields": [
                        {
                            "code": "52201",
                            "title": "迟到次数"
                        },
                        {
                            "code": "52203",
                            "title": "早退次数"
                        },
                        {
                            "code": "52207",
                            "title": "缺勤"
                        }
                    ],
                    "code": "522",
                    "title": "异常统计"
                },
                {
                    "child_fields": [
                        {
                            "code": "2021-03-16",
                            "title": "2021-03-16 星期二"
                        },
                        {
                            "code": "2021-03-17",
                            "title": "2021-03-17 星期三"
                        },
                        {
                            "code": "2021-03-18",
                            "title": "2021-03-18 星期四"
                        },
                        {
                            "code": "2021-03-19",
                            "title": "2021-03-19 星期五"
                        },
                        {
                            "code": "2021-03-20",
                            "title": "2021-03-20 星期六"
                        },
                        {
                            "code": "2021-03-21",
                            "title": "2021-03-21 星期日"
                        },
                        {
                            "code": "2021-03-22",
                            "title": "2021-03-22 星期一"
                        },
                        {
                            "code": "2021-03-23",
                            "title": "2021-03-23 星期二"
                        }
                    ],
                    "code": "524",
                    "title": "每日统计"
                }
            ],
            "stats_type": "month",
            "user_id": "ec8ddg56"
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



