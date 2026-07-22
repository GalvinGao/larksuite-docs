---
document_id: '6975751842354282501'
directory_id: '6975751873563557894'
title: 查询统计数据
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//task/query-statistics-data
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Task
- Query Statistics
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:22Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//task/query-statistics-data
---

# 查询统计数据
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/query)
</md-alert>
:::
查询日度统计或月度统计的统计数据。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_stats_datas/query |
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
| <md-text type="field-name" >start_date</md-text> | <md-text type="field-type" >int</md-text> | 是 | 开始时间<br>**示例值**：20210316 |
| <md-text type="field-name" >end_date</md-text> | <md-text type="field-type" >int</md-text> | 是 | 结束时间<br>**示例值**：20210323<br>（时间间隔不超过 40 天） |
| <md-text type="field-name" >user_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 否 | 查询的用户 ID 列表<br>（用户数量不超过 20） |
| <md-text type="field-name" >need_history</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 是否包含历史数据<br>**示例值**：true |
| <md-text type="field-name" >current_group_only</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 是否只包含当前考勤组<br>**示例值**：true |


### 请求体示例

```json
{
    "current_group_only": true,
    "end_date": 20210323,
    "locale": "zh",
    "need_history": true,
    "start_date": 20210316,
    "stats_type": "month",
    "user_ids": [
        "ec8ddg56",
        "4dbb52f2",
        "4167842e"
    ]
}
```

## 响应

### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_datas</md-text> | <md-text type="field-type" >user_stats_data\[\]</md-text> | 用户统计数据 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 姓名 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >datas</md-text> | <md-text type="field-type" >user_stats_data_cell\[\]</md-text> | 数据 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 字段编号 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text> | <md-text type="field-type" >string</md-text> | 数据值 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >features</md-text> | <md-text type="field-type" >user_stats_data_feature\[\]</md-text> | 数据属性 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >key</md-text> | <md-text type="field-type" >string</md-text> | 属性名 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text> | <md-text type="field-type" >string</md-text> | 属性值 |

### 响应体示例

```json
{
    "code": 0,
    "msg": "",
    "data": {
        "user_datas": [
            {
                "datas": [
                    {
                        "code": "2021-03-19",
                        "features": [
                            {
                                "key": "Abnormal",
                                "value": "true"
                            }
                        ],
                        "value": "缺卡(-), 缺卡(-)"
                    },
                    {
                        "code": "2021-03-22",
                        "features": [
                            {
                                "key": "Abnormal",
                                "value": "true"
                            }
                        ],
                        "value": "缺卡(-), 缺卡(-)"
                    },
                    {
                        "code": "2021-03-16",
                        "features": [
                            {
                                "key": "Abnormal",
                                "value": "true"
                            }
                        ],
                        "value": "缺卡(-), 缺卡(-)"
                    },
                    {
                        "code": "2021-03-21",
                        "features": [
                            {
                                "key": "Abnormal",
                                "value": "false"
                            }
                        ],
                        "value": "无需打卡(-), 无需打卡(-)"
                    },
                    {
                        "code": "50103",
                        "features": [],
                        "value": "3766663"
                    },
                    {
                        "code": "52107",
                        "features": [],
                        "value": "0.0小时"
                    },
                    {
                        "code": "2021-03-20",
                        "features": [
                            {
                                "key": "Abnormal",
                                "value": "false"
                            }
                        ],
                        "value": "无需打卡(-), 无需打卡(-)"
                    },
                    {
                        "code": "52203",
                        "features": [],
                        "value": "0"
                    },
                    {
                        "code": "52101",
                        "features": [],
                        "value": "5 天"
                    },
                    {
                        "code": "52105",
                        "features": [],
                        "value": "0"
                    },
                    {
                        "code": "52201",
                        "features": [],
                        "value": "0"
                    },
                    {
                        "code": "52207",
                        "features": [],
                        "value": "5 天"
                    },
                    {
                        "code": "2021-03-18",
                        "features": [
                            {
                                "key": "Abnormal",
                                "value": "true"
                            }
                        ],
                        "value": "缺卡(-), 缺卡(-)"
                    },
                    {
                        "code": "52108",
                        "features": [],
                        "value": "排班测试"
                    },
                    {
                        "code": "52102",
                        "features": [],
                        "value": "0 天"
                    },
                    {
                        "code": "52104",
                        "features": [],
                        "value": "2700"
                    },
                    {
                        "code": "2021-03-23",
                        "features": [
                            {
                                "key": "Abnormal",
                                "value": "false"
                            }
                        ],
                        "value": "无需打卡(-), 无需打卡(-)"
                    },
                    {
                        "code": "2021-03-17",
                        "features": [
                            {
                                "key": "Abnormal",
                                "value": "true"
                            }
                        ],
                        "value": "缺卡(-), 缺卡(-)"
                    }
                ],
                "name": "小李",
                "user_id": "ec8ddg56"
            }
        ]
    }
}
```

### 错误码

| HTTP 状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1220001 | 参数错误 | 请检查参数是否符合要求 |
| 400 | 1220002 | 租户不存在 | 请检查 tenant_access_token 是否正确 |
| 500 | 1228000 | 统计服务系统错误 | 详见错误信息 |



