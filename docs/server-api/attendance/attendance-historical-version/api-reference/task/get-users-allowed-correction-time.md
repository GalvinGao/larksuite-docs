---
document_id: '7070902815603097606'
directory_id: '6975751873563557894'
title: 获取用户可补卡时间
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//task/query-user-allowed-remedys
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Task
- Get Users’ Allowed Correction Time
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//task/query-user-allowed-remedys
---

# 获取用户可补卡时间
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query_user_allowed_remedys)
</md-alert>
:::
获取用户某天可以补第几次上/下班卡的时间。

## 请求
|基本||
|---|---|
|HTTP URL|https://open.larksuite.com/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys|
|HTTP Method|POST|
|HTTP Content-Type|application/json; charset=utf-8|
|凭证要求|tenant_access_token|
|权限要求| 打卡数据导出|

### 头部
| key           | value                      |
| ------------- | -------------------------- |
| Authorization | Bearer tenant_access_token |
| Content-Type  | application/json           |

### 查询参数
|名称|类型|必填|描述|
|---|---|---|---|
| employee_type | string | 是  | 请求体中的 user_id 的员工工号类型，必选字段，可用值：【employee_id（员工employeeId），employee_no（员工工号）】，示例值："employee_id" |

### 请求体

| 名称          | 类型     | 必填 | 描述      |
| ----------- | ------ | -- | ------- |
| user_id     | string | 是  | 用户 ID    |
| remedy_date | int    | 是  | 查询补卡的日期 |

### 请求体示例

```json
{ 
    "user_id" : "abd754f7",
    "remedy_date" : 20210704
}
```

## 响应
### 响应体

|名称|类型|描述|
|---|---|---|
|code|int|错误码，非 0 表示失败|
|msg|string|错误描述|
|data|-|-|
|&emsp;∟user_allowed_remedys|user_allowed_remedy[]||
|&emsp;&emsp;∟user_id| string| 用户 ID|
|&emsp;&emsp;∟remedy_date|int| 补卡日期|
|&emsp;&emsp;∟is_free_punch|bool|是否为自由班次，若为自由班次，则不用选择考虑第几次上下班，直接选择补卡时间即可|
|&emsp;&emsp;∟punch_no|int|第几次上下班，可用值：【0（第 1 次上下班），1（第 2 次上下班），2（第 3 次上下班）】|
|&emsp;&emsp;∟work_type|int|上班/下班，1：上班，2：下班|
|&emsp;&emsp;∟punch_status|string| 打卡状态，可用值【Early（早退），Late（迟到），Lack（缺卡）】|
|&emsp;&emsp;∟normal_punch_time|string|正常的应打卡时间，时间格式为 yyyy-MM-dd HH:mm|
|&emsp;&emsp;∟remedy_start_time|string|可选的补卡时间的最小值，时间格式为 yyyy-MM-dd HH:mm|
|&emsp;&emsp;∟remedy_end_time|string|可选的补卡时间的最大值，时间格式为 yyyy-MM-dd HH:mm|

### 响应体示例

```json
{ 
    "code": 0, 
    "msg": "success",
    "data": {
        "user_allowed_remedy_times" : [
            {
                "user_id" : "abd754f7",
                "remedy_date" : 20210104,
                "is_free_punch" : false,
                "punch_no" : 0,
                "work_type" : 1,
                "punch_status" : "Lack",
                "normal_punch_time" : "2021-07-01 09:00",
                "remedy_start_time" : "2021-07-01 08:00",
                "remedy_end_time" : "2021-07-01 10:00"
            }
        ]
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
