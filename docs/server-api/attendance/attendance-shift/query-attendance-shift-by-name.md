---
document_id: '7070902815602999302'
directory_id: '7070770034936152069'
title: 按名称查询班次
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query
breadcrumb:
- Server API
- Attendance
- Attendance Shift
- Query Attendance Shift by Name
document_type: ReferenceDocumentType
updated_at: 2022-03-03T15:54:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query
---

# 按名称查询班次

通过班次的名称查询班次信息。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=shift&method=query)

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
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/shifts/query |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="attendance:task:readonly" desc="导出打卡数据" support_app_types="custom" tags="">导出打卡数据</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >shift_name</md-text> | <md-text type="field-type" >string</md-text> | 是 | 班次名称<br>**示例值**："早班" |






## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >shift</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >shift_id</md-text> | <md-text type="field-type" >string</md-text> | 班次 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >shift_name</md-text> | <md-text type="field-type" >string</md-text> | 班次名称 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_times</md-text> | <md-text type="field-type" >int</md-text> | 打卡次数 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_flexible</md-text> | <md-text type="field-type" >boolean</md-text> | 是否弹性打卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >flexible_minutes</md-text> | <md-text type="field-type" >int</md-text> | 弹性打卡的时间 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >no_need_off</md-text> | <md-text type="field-type" >boolean</md-text> | 不需要打下班卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_time_rule</md-text> | <md-text type="field-type" >punch_time_rule\[\]</md-text> | 打卡规则 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >on_time</md-text> | <md-text type="field-type" >string</md-text> | 上班时间 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >off_time</md-text> | <md-text type="field-type" >string</md-text> | 下班时间 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >late_minutes_as_late</md-text> | <md-text type="field-type" >int</md-text> | 晚到多久记为迟到 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >late_minutes_as_lack</md-text> | <md-text type="field-type" >int</md-text> | 晚到多久记为缺卡 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >on_advance_minutes</md-text> | <md-text type="field-type" >int</md-text> | 最早多久可打上班卡 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >early_minutes_as_early</md-text> | <md-text type="field-type" >int</md-text> | 早退多久记为早退 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >early_minutes_as_lack</md-text> | <md-text type="field-type" >int</md-text> | 早退多久记为缺卡 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >off_delay_minutes</md-text> | <md-text type="field-type" >int</md-text> | 最晚多久可打下班卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >late_off_late_on_rule</md-text> | <md-text type="field-type" >late_off_late_on_rule\[\]</md-text> | 晚走晚到规则 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >late_off_minutes</md-text> | <md-text type="field-type" >int</md-text> | 晚走多久 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >late_on_minutes</md-text> | <md-text type="field-type" >int</md-text> | 晚到多久 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >rest_time_rule</md-text> | <md-text type="field-type" >rest_rule\[\]</md-text> | 休息规则 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >rest_begin_time</md-text> | <md-text type="field-type" >string</md-text> | 休息开始 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >rest_end_time</md-text> | <md-text type="field-type" >string</md-text> | 休息结束 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "shift_id": "6919358778597097404",
        "shift_name": "早班",
        "punch_times": 1,
        "is_flexible": false,
        "flexible_minutes": 60,
        "no_need_off": true,
        "punch_time_rule": [
            {
                "on_time": "9:00",
                "off_time": "18:00， 第二天凌晨2点， 26:00",
                "late_minutes_as_late": 30,
                "late_minutes_as_lack": 60,
                "on_advance_minutes": 60,
                "early_minutes_as_early": 30,
                "early_minutes_as_lack": 60,
                "off_delay_minutes": 60
            }
        ],
        "late_off_late_on_rule": [
            {
                "late_off_minutes": 60,
                "late_on_minutes": 30
            }
        ],
        "rest_time_rule": [
            {
                "rest_begin_time": "13:00",
                "rest_end_time": "14:00"
            }
        ]
    }
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1220001 | 参数错误 | 请检查参数是否符合要求 |
| 400 | 1220002 | 租户不存在 | 请检查 tenant_access_token 是否正确 |
| 500 | 1225000 | 系统错误 | 详见错误信息 |
| 500 | 1226000 | 班次服务系统错误 | 详见错误信息 |
| 400 | 1226003 | 班次不存在 | 请检查 shift_id 是否正确 |





