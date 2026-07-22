---
document_id: '7070902815603015686'
directory_id: '7070770034936168453'
title: 查询班表信息
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/query
breadcrumb:
- Server API
- Attendance
- Attendance Schedule
- Query Schedule Information
document_type: ReferenceDocumentType
updated_at: 2022-03-16T08:28:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/query
---

# 查询班表信息

支持查询多个用户的排班情况，查询的时间跨度不能超过 30 天。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=user_daily_shift&method=query)

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
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_daily_shifts/query |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="attendance:task:readonly" desc="导出打卡数据" support_app_types="custom" tags="">导出打卡数据</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 请求体中的 user_ids 和响应体中的 user_id 的员工工号类型<br>**示例值**："employee_id"<br>**可选值有**：<br>- `employee_id`：员工 employee ID，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的用户 ID<br>- `employee_no`：员工工号，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的工号 |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >user_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 是 | employee_no 或 employee_id 列表<br>**示例值**：["abd754f7"] |
| <md-text type="field-name" >check_date_from</md-text> | <md-text type="field-type" >int</md-text> | 是 | 查询的起始工作日<br>**示例值**：20190817 |
| <md-text type="field-name" >check_date_to</md-text> | <md-text type="field-type" >int</md-text> | 是 | 查询的结束工作日<br>**示例值**：20190820 |




### 请求体示例

```json
{
    "user_ids": [
        "abd754f7"
    ],
    "check_date_from": 20190817,
    "check_date_to": 20190820
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_daily_shifts</md-text> | <md-text type="field-type" >user_daily_shift\[\]</md-text> | 班表信息列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >group_id</md-text> | <md-text type="field-type" >string</md-text> | 考勤组 ID，获取方式：1）[创建或修改考勤组](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/create) 2）[按名称查询考勤组](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/search) 3）[获取打卡结果](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task/query) |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >shift_id</md-text> | <md-text type="field-type" >string</md-text> | 班次 ID，获取方式：1）[按名称查询班次](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query) 2）[创建班次](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/create) |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >month</md-text> | <md-text type="field-type" >int</md-text> | 月份 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >day_no</md-text> | <md-text type="field-type" >int</md-text> | 日期 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "user_daily_shifts": [
            {
                "group_id": "6737202939523236110",
                "shift_id": "6753520403404030215",
                "month": 202101,
                "user_id": "abd754f7",
                "day_no": 21
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
| 400 | 1220004 | 用户不存在或没有权限 | 请检查用户 ID 是否正确 |
| 400 | 1220005 | 没有权限 | 请前往[考勤管理后台](https://oa.larksuite.com/attendance/manage/member/list)检查数据权限范围 |
| 500 | 1225000 | 系统错误 | 详见错误信息 |
| 500 | 1226000 | 班次服务系统错误 | 详见错误信息 |
| 500 | 1226003 | 班次不存在 | 请检查 shift_id 是否正确 |





