---
document_id: '7070902815602802694'
directory_id: '7070770034936135685'
title: 导入打卡流水记录
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_flow/batch_create
breadcrumb:
- Server API
- Attendance
- Attendance Records
- Import Attendance Records
document_type: ReferenceDocumentType
updated_at: 2022-03-16T08:28:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_flow/batch_create
---

# 导入打卡流水记录

导入授权内员工的打卡流水记录。导入后，会根据员工所在的考勤组班次规则，计算最终的打卡状态与结果。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=user_flow&method=batch_create)

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
适用于考勤机数据导入等场景。
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/user_flows/batch_create |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="attendance:task" desc="写入打卡数据" support_app_types="custom" tags="">写入打卡数据</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 请求体和响应体中的 user_id 和 creator_id 的员工工号类型<br>**示例值**："employee_id"<br>**可选值有**：<br>- `employee_id`：员工 employee ID，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的用户 ID<br>- `employee_no`：员工工号，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的工号 |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >flow_records</md-text> | <md-text type="field-type" >user_flow\[\]</md-text> | 是 | 打卡流水记录列表 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户 ID<br>**示例值**："abd754f7" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >creator_id</md-text> | <md-text type="field-type" >string</md-text> | 是 | 记录创建者 ID<br>**示例值**："abd754f7" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >location_name</md-text> | <md-text type="field-type" >string</md-text> | 是 | 打卡位置名称信息<br>**示例值**："西溪八方城" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_time</md-text> | <md-text type="field-type" >string</md-text> | 是 | 打卡时间，精确到秒的时间戳<br>**示例值**："1611476284" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >comment</md-text> | <md-text type="field-type" >string</md-text> | 是 | 打卡备注<br>**示例值**："上班打卡" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >record_id</md-text> | <md-text type="field-type" >string</md-text> | 否 | 打卡记录 ID<br>**示例值**："6709359313699356941" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >longitude</md-text> | <md-text type="field-type" >float</md-text> | 否 | 打卡经度<br>**示例值**：30.28991 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >latitude</md-text> | <md-text type="field-type" >float</md-text> | 否 | 打卡纬度<br>**示例值**：120.04513 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ssid</md-text> | <md-text type="field-type" >string</md-text> | 否 | 打卡 Wi-Fi 的 SSID<br>**示例值**："b0:b8:67:5c:1d:72" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >bssid</md-text> | <md-text type="field-type" >string</md-text> | 否 | 打卡 Wi-Fi 的 MAC 地址<br>**示例值**："b0:b8:67:5c:1d:72" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_field</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 是否为外勤打卡<br>**示例值**：true |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_wifi</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 是否为 Wi-Fi 打卡<br>**示例值**：true |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >type</md-text> | <md-text type="field-type" >int</md-text> | 否 | 记录生成方式<br>**示例值**：0<br>**可选值有**：<br>- `0`：用户打卡<br>- `1`：管理员修改<br>- `2`：用户补卡<br>- `3`：系统自动生成<br>- `4`：下班免打卡<br>- `5`：考勤机<br>- `6`：极速打卡<br>- `7`：考勤开放平台导入 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >photo_urls</md-text> | <md-text type="field-type" >string\[\]</md-text> | 否 | 打卡照片列表<br>**示例值**：https://time.clockin.biz/manage/download/6840389754748502021 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >device_id</md-text> | <md-text type="field-type" >string</md-text> | 否 | 打卡设备 ID<br>**示例值**："99e0609ee053448596502691a81428654d7ded64c7bd85acd982d26b3636c37d" |




### 请求体示例

```json
{
    "flow_records": [
        {
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
        }
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
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >flow_records</md-text> | <md-text type="field-type" >user_flow\[\]</md-text> | 打卡流水记录列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >creator_id</md-text> | <md-text type="field-type" >string</md-text> | 记录创建者 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >location_name</md-text> | <md-text type="field-type" >string</md-text> | 打卡位置名称信息 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_time</md-text> | <md-text type="field-type" >string</md-text> | 打卡时间，精确到秒的时间戳 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >comment</md-text> | <md-text type="field-type" >string</md-text> | 打卡备注 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >record_id</md-text> | <md-text type="field-type" >string</md-text> | 打卡记录 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >longitude</md-text> | <md-text type="field-type" >float</md-text> | 打卡经度 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >latitude</md-text> | <md-text type="field-type" >float</md-text> | 打卡纬度 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ssid</md-text> | <md-text type="field-type" >string</md-text> | 打卡 Wi-Fi 的 SSID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >bssid</md-text> | <md-text type="field-type" >string</md-text> | 打卡 Wi-Fi 的 MAC 地址 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_field</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为外勤打卡 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_wifi</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为 Wi-Fi 打卡 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >type</md-text> | <md-text type="field-type" >int</md-text> | 记录生成方式<br>**可选值有**：<br>- `0`：用户打卡<br>- `1`：管理员修改<br>- `2`：用户补卡<br>- `3`：系统自动生成<br>- `4`：下班免打卡<br>- `5`：考勤机<br>- `6`：极速打卡<br>- `7`：考勤开放平台导入 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >photo_urls</md-text> | <md-text type="field-type" >string\[\]</md-text> | 打卡照片列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >device_id</md-text> | <md-text type="field-type" >string</md-text> | 打卡设备 ID |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "flow_records": [
            {
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





