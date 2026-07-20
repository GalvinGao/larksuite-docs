---
document_id: '7070902815602655238'
directory_id: '6975751873563557894'
title: 通知补卡审批发起
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//task/notify-remedy-approval-initiation
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Task
- Notify Correction Request Submission
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//task/notify-remedy-approval-initiation
---

# 通知补卡审批发起
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/create)
</md-alert>
:::
对于只使用Lark考勤系统而未使用Lark审批系统的企业，可以通过该接口，将在三方审批系统中发起的补卡审批数据，写入到Lark的考勤系统中。
## 请求
|基本||
|---|---|
|HTTP URL|https://open.larksuite.com/open-apis/attendance/v1/user_task_remedys|
|HTTP Method|POST|
|HTTP Content-Type|application/json; charset=utf-8|
|凭证要求|tenant_access_token|
|权限要求|打卡数据写入|
### 头部
key|value
--|--
Authorization|Bearer tenant_access_token
Content-Type|application/json
### 查询参数
|名称|类型|必填|描述|
|---|---|---|---|
|employee_type|string|是|请求体中的 user_id 的员工工号类型，必选字段，可用值：【employee_id（员工employeeId），employee_no（员工工号）】，示例值："employee_id"|
### 请求体
|名称|类型|必填|描述|
|---|---|---|---|
|user_id|string|是|用户 ID|
|remedy_date|int|是|补卡日期|
|punch_no|int|是|第几次上下班，可用值【0（第 1 次上下班），1（第 2 次上下班），2（第 3 次上下班）】，自由班次时填 0|
|work_type|int|是|上班/下班，1：上班，2：下班，自由班次时填 0|
|remedy_time|string|是|补卡时间，时间格式为 yyyy-MM-dd HH:mm|
|reason|string|是|补卡原因|
### 请求体示例

```json
{ 
    "user_id" : "abd754f7",
    "remedy_date" : 20210701,
    "punch_no" : 0,
    "work_type" : 1,
    "remedy_time" : "2021-07-01 08:00",
    "reason" : ""
}
```
## 响应
### 响应体

|名称|类型|描述|
|---|---|---|
|code|int| 错误码，非 0 表示失败|
|msg|string|错误描述|
|data|-|-|
|&emsp;∟user_remedy|user_task_remedy||
|&emsp;&emsp;∟user_id|string|用户 ID|
|&emsp;&emsp;∟approval_id|string|审批实例 ID，可用于通知审批状态更新|
|&emsp;&emsp;∟remedy_date|int|补卡日期|
|&emsp;&emsp;∟punch_no|int|第几次上下班，可用值【0（第 1 次上下班），1（第2次上下班），2（第3次上下班）】，自由班次时填 0|
|&emsp;&emsp;∟work_type|int| 上班/下班，1：上班，2：下班|
|&emsp;&emsp;∟remedy_time|string|补卡时间，时间格式为 yyyy-MM-dd HH:mm|
|&emsp;&emsp;∟reason|string|补卡原因|

### 响应体示例

```json
{ 
    "code": 0, 
    "msg": "success",
    "data": {
        "user_remedy" : {
            "user_id" : "abd754f7",
            "approval_id" : "6737202939523236113",
            "remedy_date" : 20210701,
            "punch_no" : 0,
            "work_type" : 1,
            "remedy_time" : "2021-07-01 08:00",
            "reason" : ""
        }
    }
}
```

### 错误码

| HTTP 状态码 | 错误码     | 描述         | 排查建议                         |
| -------- | ------- | ---------- | ---------------------------- |
| 400      | 1220001 | 参数错误       | 请检查参数是否符合要求                  |
| 400      | 1220002 | 租户不存在      | 请检查 tenant_access_token 是否正确 |
| 400      | 1220004 | 用户不存在或没有权限 | 请检查用户 ID 是否正确                |
| 400      | 1220005 | 没有权限       | 请前往考勤管理后台检查数据权限范围            |
| 400      | 1226501 | 没有异常考勤     | 当天没有异常考勤，无需补卡                |
| 400      | 1226502 | 不允许补卡      | 考勤组设置不允许补卡                   |
| 400      | 1226503 | 补卡日期限制     | 考勤组设置只允许补过去多少天的卡，超出可补卡日期     |
| 400      | 1226504 | 超出补卡次数     | 当前周期的补卡次数已用完                  |
| 500      | 1225000 | 系统错误       | 详见错误信息
