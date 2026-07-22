---
document_id: '7070902815602786310'
directory_id: '7070770034936135685'
title: 获取打卡结果
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task/query
breadcrumb:
- Server API
- Attendance
- Attendance Records
- Get Attendance Result
document_type: ReferenceDocumentType
updated_at: 2022-03-16T08:28:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task/query
---

# 获取打卡结果

获取企业内员工的实际打卡结果，包括上班打卡结果和下班打卡结果。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=user_task&method=query)

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
- 如果企业给一个员工设定的班次是上午 9 点和下午 6 点各打一次上下班卡，即使员工在这期间打了多次卡，该接口也只会返回 1 条记录。
- 如果要获取打卡的详细数据，如打卡位置等信息，可使用“获取打卡流水记录”或“批量查询打卡流水记录”的接口。
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_tasks/query |
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
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 员工工号类型<br>**示例值**："employee_id"<br>**可选值有**：<br>- `employee_id`：员工 employee ID，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的用户 ID<br>- `employee_no`：员工工号，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的工号 |
| <md-text type="field-name" >ignore_invalid_users</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 是否忽略无效和没有权限的用户。如果 true，则返回有效用户的信息，并告知无效和没有权限的用户信息；如果 false，且 user_ids 中存在无效或没有权限的用户，则返回错误<br>**示例值**：true |
| <md-text type="field-name" >include_terminated_user</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 由于新入职员工可以复用已离职员工的 employee_no/employee_id，如果 true，则返回 employee_no/employee_id 对应的所有在职 + 离职员工的数据；如果 false，则只返回 employee_no/employee_id 对应的在职或最近一个离职员工的数据<br>**示例值**：true |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >user_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 是 | employee_no 或 employee_id 列表<br>**示例值**：abd754f7 |
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
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_task_results</md-text> | <md-text type="field-type" >user_task\[\]</md-text> | 打卡任务列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >result_id</md-text> | <md-text type="field-type" >string</md-text> | 打卡记录 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >employee_name</md-text> | <md-text type="field-type" >string</md-text> | 用户姓名 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >day</md-text> | <md-text type="field-type" >int</md-text> | 日期 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >group_id</md-text> | <md-text type="field-type" >string</md-text> | 考勤组 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >shift_id</md-text> | <md-text type="field-type" >string</md-text> | 班次 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >records</md-text> | <md-text type="field-type" >task_result\[\]</md-text> | 用户考勤记录 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_in_record_id</md-text> | <md-text type="field-type" >string</md-text> | 上班打卡记录 ID |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_in_record</md-text> | <md-text type="field-type" >user_flow</md-text> | 上班打卡记录 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >creator_id</md-text> | <md-text type="field-type" >string</md-text> | 记录创建者 ID |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >location_name</md-text> | <md-text type="field-type" >string</md-text> | 打卡位置名称信息 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_time</md-text> | <md-text type="field-type" >string</md-text> | 打卡时间，精确到秒的时间戳 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >comment</md-text> | <md-text type="field-type" >string</md-text> | 打卡备注 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >record_id</md-text> | <md-text type="field-type" >string</md-text> | 打卡记录 ID |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >longitude</md-text> | <md-text type="field-type" >float</md-text> | 打卡经度 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >latitude</md-text> | <md-text type="field-type" >float</md-text> | 打卡纬度 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ssid</md-text> | <md-text type="field-type" >string</md-text> | 打卡 Wi-Fi 的 SSID |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >bssid</md-text> | <md-text type="field-type" >string</md-text> | 打卡 Wi-Fi 的 MAC 地址 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_field</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为外勤打卡 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_wifi</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为 Wi-Fi 打卡 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >type</md-text> | <md-text type="field-type" >int</md-text> | 记录生成方式<br>**可选值有**：<br>- `0`：用户打卡<br>- `1`：管理员修改<br>- `2`：用户补卡<br>- `3`：系统自动生成<br>- `4`：下班免打卡<br>- `5`：考勤机<br>- `6`：极速打卡<br>- `7`：考勤开放平台导入 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >photo_urls</md-text> | <md-text type="field-type" >string\[\]</md-text> | 打卡照片列表 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >device_id</md-text> | <md-text type="field-type" >string</md-text> | 打卡设备 ID |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_out_record_id</md-text> | <md-text type="field-type" >string</md-text> | 下班打卡记录 ID |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_out_record</md-text> | <md-text type="field-type" >user_flow</md-text> | 下班打卡记录 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >creator_id</md-text> | <md-text type="field-type" >string</md-text> | 记录创建者 ID |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >location_name</md-text> | <md-text type="field-type" >string</md-text> | 打卡位置名称信息 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_time</md-text> | <md-text type="field-type" >string</md-text> | 打卡时间，精确到秒的时间戳 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >comment</md-text> | <md-text type="field-type" >string</md-text> | 打卡备注 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >record_id</md-text> | <md-text type="field-type" >string</md-text> | 打卡记录 ID |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >longitude</md-text> | <md-text type="field-type" >float</md-text> | 打卡经度 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >latitude</md-text> | <md-text type="field-type" >float</md-text> | 打卡纬度 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ssid</md-text> | <md-text type="field-type" >string</md-text> | 打卡 Wi-Fi 的 SSID |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >bssid</md-text> | <md-text type="field-type" >string</md-text> | 打卡 Wi-Fi 的 MAC 地址 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_field</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为外勤打卡 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_wifi</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为 Wi-Fi 打卡 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >type</md-text> | <md-text type="field-type" >int</md-text> | 记录生成方式<br>**可选值有**：<br>- `0`：用户打卡<br>- `1`：管理员修改<br>- `2`：用户补卡<br>- `3`：系统自动生成<br>- `4`：下班免打卡<br>- `5`：考勤机<br>- `6`：极速打卡<br>- `7`：考勤开放平台导入 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >photo_urls</md-text> | <md-text type="field-type" >string\[\]</md-text> | 打卡照片列表 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >device_id</md-text> | <md-text type="field-type" >string</md-text> | 打卡设备 ID |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_in_result</md-text> | <md-text type="field-type" >string</md-text> | 上班打卡结果<br>**可选值有**：<br>- `NoNeedCheck`：无需打卡<br>- `SystemCheck`：系统打卡<br>- `Normal`：正常<br>- `Early`：早退<br>- `Late`：迟到<br>- `Lack`：缺卡 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_out_result</md-text> | <md-text type="field-type" >string</md-text> | 下班打卡结果<br>**可选值有**：<br>- `NoNeedCheck`：无需打卡<br>- `SystemCheck`：系统打卡<br>- `Normal`：正常<br>- `Early`：早退<br>- `Late`：迟到<br>- `Lack`：缺卡 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_in_result_supplement</md-text> | <md-text type="field-type" >string</md-text> | 上班打卡结果补充<br>**可选值有**：<br>- `None`：无<br>- `ManagerModification`：管理员修改<br>- `CardReplacement`：补卡通过<br>- `ShiftChange`：换班<br>- `Travel`：出差<br>- `Leave`：请假<br>- `GoOut`：外出<br>- `CardReplacementApplication`：补卡申请中<br>- `FieldPunch`：外勤打卡 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_out_result_supplement</md-text> | <md-text type="field-type" >string</md-text> | 下班打卡结果补充<br>**可选值有**：<br>- `None`：无<br>- `ManagerModification`：管理员修改<br>- `CardReplacement`：补卡通过<br>- `ShiftChange`：换班<br>- `Travel`：出差<br>- `Leave`：请假<br>- `GoOut`：外出<br>- `CardReplacementApplication`：补卡申请中<br>- `FieldPunch`：外勤打卡 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_in_shift_time</md-text> | <md-text type="field-type" >string</md-text> | 上班打卡时间 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_out_shift_time</md-text> | <md-text type="field-type" >string</md-text> | 下班打卡时间 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >invalid_user_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 无效用户 ID 列表 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >unauthorized_user_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 没有权限用户 ID 列表 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "user_task_results": [
            {
                "result_id": "6709359313699356941",
                "user_id": "abd754f7",
                "employee_name": "张三",
                "day": 20190819,
                "group_id": "6737202939523236110",
                "shift_id": "6753520403404030215",
                "records": [
                    {
                        "check_in_record_id": "6709359313699356941",
                        "check_in_record": {
                            "user_id": "abd754f7",
                            "creator_id": "abd754f7",
                            "location_name": "西溪八方城",
                            "check_time": "1611476284",
                            "comment": "上班打卡",
                            "record_id": "6709359313699356941",
                            "longitude": 30.28991,
                            "latitude": 120.04513,
                            "ssid": "b0:b8:67:5c:1d:72",
                            "bssid": "b0:b8:67:5c:1d:72",
                            "is_field": true,
                            "is_wifi": true,
                            "type": 0,
                            "photo_urls": [
                                "https://time.clockin.biz/manage/download/6840389754748502021"
                            ],
                            "device_id": "99e0609ee053448596502691a81428654d7ded64c7bd85acd982d26b3636c37d"
                        },
                        "check_out_record_id": "6709359313699356942",
                        "check_out_record": {
                            "user_id": "abd754f7",
                            "creator_id": "abd754f7",
                            "location_name": "西溪八方城",
                            "check_time": "1611476284",
                            "comment": "上班打卡",
                            "record_id": "6709359313699356941",
                            "longitude": 30.28991,
                            "latitude": 120.04513,
                            "ssid": "b0:b8:67:5c:1d:72",
                            "bssid": "b0:b8:67:5c:1d:72",
                            "is_field": true,
                            "is_wifi": true,
                            "type": 0,
                            "photo_urls": [
                                "https://time.clockin.biz/manage/download/6840389754748502021"
                            ],
                            "device_id": "99e0609ee053448596502691a81428654d7ded64c7bd85acd982d26b3636c37d"
                        },
                        "check_in_result": "SystemCheck",
                        "check_out_result": "SystemCheck",
                        "check_in_result_supplement": "None",
                        "check_out_result_supplement": "None",
                        "check_in_shift_time": "1609722000",
                        "check_out_shift_time": "1609754400"
                    }
                ]
            }
        ],
        "invalid_user_ids": [
            "abd754f7"
        ],
        "unauthorized_user_ids": [
            "abd754f7"
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
| 500 | 1227500 | 组织架构服务系统错误 | 详见错误信息 |





