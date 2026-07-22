---
document_id: '7070902815603032070'
directory_id: '7070770034936102917'
title: 获取可补卡时间
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query_user_allowed_remedys
breadcrumb:
- Server API
- Attendance
- Attendance Correction
- Get Allowed Correction Time
document_type: ReferenceDocumentType
updated_at: 2022-03-16T08:28:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query_user_allowed_remedys
---

# 获取用户可补卡时间

获取用户某天可以补的第几次上 / 下班卡的时间。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=user_task_remedy&method=query_user_allowed_remedys)

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
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys |
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
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 请求体和响应体中的 user_id 的员工工号类型<br>**示例值**："employee_id"<br>**可选值有**：<br>- `employee_id`：员工 employee ID，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的用户 ID<br>- `employee_no`：员工工号，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的工号 |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户 ID<br>**示例值**："abd754f7" |
| <md-text type="field-name" >remedy_date</md-text> | <md-text type="field-type" >int</md-text> | 是 | 补卡日期<br>**示例值**：20210104 |




### 请求体示例

```json
{
    "user_id": "abd754f7",
    "remedy_date": 20210104
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_allowed_remedys</md-text> | <md-text type="field-type" >user_allowed_remedy\[\]</md-text> | 用户可补卡时间 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_date</md-text> | <md-text type="field-type" >int</md-text> | 补卡日期 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_free_punch</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为自由班次，若为自由班次，则不用选择考虑第几次上下班，直接选择补卡时间即可 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_no</md-text> | <md-text type="field-type" >int</md-text> | 第几次上下班，0：第 1 次上下班，1：第 2 次上下班，2：第 3 次上下班 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >work_type</md-text> | <md-text type="field-type" >int</md-text> | 上班 / 下班，1：上班，2：下班 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_status</md-text> | <md-text type="field-type" >string</md-text> | 打卡状态，Early：早退，Late：迟到，Lack：缺卡 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >normal_punch_time</md-text> | <md-text type="field-type" >string</md-text> | 正常的应打卡时间，时间格式为 yyyy-MM-dd HH:mm |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_start_time</md-text> | <md-text type="field-type" >string</md-text> | 可选的补卡时间的最小值，时间格式为 yyyy-MM-dd HH:mm |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_end_time</md-text> | <md-text type="field-type" >string</md-text> | 可选的补卡时间的最大值，时间格式为 yyyy-MM-dd HH:mm |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "user_allowed_remedys": [
            {
                "user_id": "abd754f7",
                "remedy_date": 20210104,
                "is_free_punch": false,
                "punch_no": 0,
                "work_type": 1,
                "punch_status": "Lack",
                "normal_punch_time": "2021-07-01 09:00",
                "remedy_start_time": "2021-07-01 08:00",
                "remedy_end_time": "2021-07-01 10:00"
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
| 500 | 1226500 | 打卡服务系统错误 | 详见错误信息 |
| 400 | 1226501 | 没有异常考勤 | 当天没有异常考勤，无需补卡 |
| 400 | 1226502 | 不允许补卡 | 考勤组设置不允许补卡 |
| 400 | 1226503 | 补卡日期限制 | 考勤组设置只允许补过去多少天的卡，超出可补卡日期 |
| 400 | 1226504 | 超出补卡次数 | 当前周期的补卡次数已用完 |
| 500 | 1227500 | 组织架构服务系统错误 | 详见错误信息 |





