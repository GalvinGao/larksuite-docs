---
document_id: '7070902815603195910'
directory_id: '7070770034936102917'
title: 获取补卡记录
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query
breadcrumb:
- Server API
- Attendance
- Attendance Correction
- Get Correction Records
document_type: ReferenceDocumentType
updated_at: 2022-03-16T08:28:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query
---

# 获取补卡记录

获取授权内员工的补卡记录。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=user_task_remedy&method=query)

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
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_task_remedys/query |
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
| <md-text type="field-name" >check_time_from</md-text> | <md-text type="field-type" >string</md-text> | 是 | 查询的起始时间，精确到秒的时间戳<br>**示例值**："1566641088" |
| <md-text type="field-name" >check_time_to</md-text> | <md-text type="field-type" >string</md-text> | 是 | 查询的结束时间，精确到秒的时间戳<br>**示例值**："1592561088" |




### 请求体示例

```json
{
    "user_ids": [
        "abd754f7"
    ],
    "check_time_from": "1566641088",
    "check_time_to": "1592561088"
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_remedys</md-text> | <md-text type="field-type" >user_task_remedy\[\]</md-text> | 补卡记录列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_date</md-text> | <md-text type="field-type" >int</md-text> | 补卡日期 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_no</md-text> | <md-text type="field-type" >int</md-text> | 第几次上下班，0：第 1 次上下班，1：第 2 次上下班，2：第 3 次上下班，自由班制填 0 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >work_type</md-text> | <md-text type="field-type" >int</md-text> | 上班 / 下班，1：上班，2：下班，自由班制填 0 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >approval_id</md-text> | <md-text type="field-type" >string</md-text> | 审批 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_time</md-text> | <md-text type="field-type" >string</md-text> | 补卡时间，时间格式为 yyyy-MM-dd HH:mm |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >status</md-text> | <md-text type="field-type" >int</md-text> | 补卡状态<br>**可选值有**：<br>- `0`：审批中<br>- `2`：已通过<br>- `3`：已取消<br>- `4`：通过后撤回 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >reason</md-text> | <md-text type="field-type" >string</md-text> | 补卡原因 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >time</md-text> | <md-text type="field-type" >string</md-text> | 补卡时间，精确到秒的时间戳 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >time_zone</md-text> | <md-text type="field-type" >string</md-text> | 补卡时考勤组时区 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >create_time</md-text> | <md-text type="field-type" >string</md-text> | 补卡发起时间，精确到秒的时间戳 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >update_time</md-text> | <md-text type="field-type" >string</md-text> | 补卡状态更新时间，精确到秒的时间戳 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "user_remedys": [
            {
                "user_id": "abd754f7",
                "remedy_date": 20210701,
                "punch_no": 0,
                "work_type": 1,
                "approval_id": "6737202939523236113",
                "remedy_time": "2021-07-01 08:00",
                "status": 2,
                "reason": "忘记打卡",
                "time": "1611476284",
                "time_zone": "Asia/Shanghai",
                "create_time": "1611476284",
                "update_time": "1611476284"
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





