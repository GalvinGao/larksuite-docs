---
document_id: '6975751842354528261'
directory_id: '6975751873563557894'
title: 查询统计设置
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//task/query-user-statistics-settings
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Task
- Query Statistics Configuration
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//task/query-user-statistics-settings
---

# 查询统计设置
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/query)
</md-alert>
:::
查询日度统计或月度统计的统计设置信息。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_stats_views/query |
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


### 请求体示例

```json
{
    "locale": "zh",
    "stats_type": "month"
}
```

## 响应

### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view</md-text> | <md-text type="field-type" >user_stats_view</md-text> | 统计视图 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view_id</md-text> | <md-text type="field-type" >string</md-text> | 视图 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >stats_type</md-text> | <md-text type="field-type" >string</md-text> | 统计类型<br>**可选值有**：<br>- `daily`：日度统计<br>- `month`：月度统计 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >items</md-text> | <md-text type="field-type" >item\[\]</md-text> | 一级标题 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 标题编号 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text> | <md-text type="field-type" >string</md-text> | 标题名称 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >child_items</md-text> | <md-text type="field-type" >child_item\[\]</md-text> | 子标题 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text> | <md-text type="field-type" >string</md-text> | 标题编号 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text> | <md-text type="field-type" >string</md-text> | 是否开启<br>**可选值有**：<br>- `0`：关闭<br>- `1`：开启 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text> | <md-text type="field-type" >string</md-text> | 标题名称 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >column_type</md-text> | <md-text type="field-type" >int</md-text> | 标题类型 |
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
                            "code": "50101",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": true,
                            "title": "姓名",
                            "value": "1"
                        },
                        {
                            "code": "50102",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "部门",
                            "value": "0"
                        },
                        {
                            "code": "50111",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "组织架构",
                            "value": "0"
                        },
                        {
                            "code": "50103",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "工号",
                            "value": "1"
                        },
                        {
                            "code": "50104",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "邮箱",
                            "value": "0"
                        },
                        {
                            "code": "50105",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "雇员类型",
                            "value": "0"
                        },
                        {
                            "code": "50106",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "序列",
                            "value": "0"
                        },
                        {
                            "code": "50107",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "入职时间",
                            "value": "0"
                        },
                        {
                            "code": "50108",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "离职时间",
                            "value": "0"
                        },
                        {
                            "code": "50109",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "状态",
                            "value": "0"
                        },
                        {
                            "code": "50110",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "直属上级",
                            "value": "0"
                        }
                    ],
                    "code": "501",
                    "title": "基本信息"
                },
                {
                    "child_items": [
                        {
                            "code": "52108",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "考勤组名称",
                            "value": "1"
                        },
                        {
                            "code": "52101",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "应出勤天数",
                            "value": "1"
                        },
                        {
                            "code": "52102",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "工作日出勤天数",
                            "value": "1"
                        },
                        {
                            "code": "52103",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "休息日出勤天数",
                            "value": "0"
                        },
                        {
                            "code": "52104",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "应出勤时长",
                            "value": "1"
                        },
                        {
                            "code": "52105",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "实际出勤时长",
                            "value": "1"
                        },
                        {
                            "code": "52106",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "计薪工作时长",
                            "value": "0"
                        },
                        {
                            "code": "52107",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "加班工作时长",
                            "value": "1"
                        },
                        {
                            "code": "52109",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "加班时长(计加班费)\n",
                            "value": "0"
                        },
                        {
                            "code": "52110",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "加班时长(计为调休)\n",
                            "value": "0"
                        }
                    ],
                    "code": "521",
                    "title": "出勤统计"
                },
                {
                    "child_items": [
                        {
                            "code": "52201",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "迟到次数",
                            "value": "1"
                        },
                        {
                            "code": "52202",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "迟到时长",
                            "value": "0"
                        },
                        {
                            "code": "52203",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "早退次数",
                            "value": "1"
                        },
                        {
                            "code": "52204",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "早退时长",
                            "value": "0"
                        },
                        {
                            "code": "52205",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "上班缺卡次数",
                            "value": "0"
                        },
                        {
                            "code": "52206",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "下班缺卡次数",
                            "value": "0"
                        },
                        {
                            "code": "52207",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "缺勤",
                            "value": "1"
                        },
                        {
                            "code": "52208",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "请假时长",
                            "value": "0"
                        },
                        {
                            "code": "52209",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "出差时长",
                            "value": "0"
                        },
                        {
                            "code": "52211",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "换班天数",
                            "value": "0"
                        },
                        {
                            "code": "52212",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "补卡次数",
                            "value": "0"
                        },
                        {
                            "code": "52213",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "外勤次数",
                            "value": "0"
                        },
                        {
                            "code": "52214",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "外出时长\t",
                            "value": "0"
                        }
                    ],
                    "code": "522",
                    "title": "异常统计"
                },
                {
                    "child_items": [
                        {
                            "code": "52401",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "每日考勤结果",
                            "value": "1"
                        }
                    ],
                    "code": "524",
                    "title": "每日统计"
                }
            ],
            "stats_type": "month",
            "user_id": "ec8ddg56",
            "view_id": "TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09"
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



