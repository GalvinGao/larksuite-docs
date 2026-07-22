---
document_id: '7070902815602900998'
directory_id: '7070770034936086533'
title: 获取考勤组详情
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/get
breadcrumb:
- Server API
- Attendance
- Attendance Group
- Get Attendance Group Details
document_type: ReferenceDocumentType
updated_at: 2022-03-03T15:54:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/get
---

# 获取考勤组详情

通过考勤组 ID 获取考勤组详情。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=group&method=get)

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
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/groups/:group_id |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="attendance:rule" desc="写入打卡管理规则" support_app_types="custom" tags="">写入打卡管理规则</md-perm><br><md-perm name="attendance:rule:readonly" desc="导出打卡管理规则" support_app_types="custom" tags="">导出打卡管理规则</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |




### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >group_id</md-text> | <md-text type="field-type" >string</md-text> | 考勤组 ID，获取方式：1）[创建或修改考勤组](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/create) 2）[按名称查询考勤组](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/search) 3）[获取打卡结果](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task/query)<br>**示例值**："6919358128597097404" |




### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >employee_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 用户 ID 的类型<br>**示例值**："employee_id"<br>**可选值有**：<br>- `employee_id`：员工 employeeId<br>- `employee_no`：员工工号 |
| <md-text type="field-name" >dept_type</md-text> | <md-text type="field-type" >string</md-text> | 是 | 部门 ID 的类型<br>**示例值**："od-fcb45c28a45311afd441b8869541ece8"<br>**可选值有**：<br>- `open_id`：暂时只支持部门的 openid |






## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >group</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >group_id</md-text> | <md-text type="field-type" >string</md-text> | 考勤组 ID（仅修改时提供）， 需要从“获取打卡结果”的接口中获取 groupId |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >group_name</md-text> | <md-text type="field-type" >string</md-text> | 考勤组名称 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >time_zone</md-text> | <md-text type="field-type" >string</md-text> | 时区 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >bind_dept_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 绑定的部门 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >except_dept_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 排除的部门 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >bind_user_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 绑定的用户 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >except_user_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 排除的用户 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >group_leader_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 考勤负责人 ID 列表，必选字段 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >allow_out_punch</md-text> | <md-text type="field-type" >boolean</md-text> | 是否允许外勤打卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >allow_pc_punch</md-text> | <md-text type="field-type" >boolean</md-text> | 是否允许 PC 端打卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >allow_remedy</md-text> | <md-text type="field-type" >boolean</md-text> | 是否限制补卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_limit</md-text> | <md-text type="field-type" >boolean</md-text> | 是否限制补卡次数 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_limit_count</md-text> | <md-text type="field-type" >int</md-text> | 补卡次数 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_date_limit</md-text> | <md-text type="field-type" >boolean</md-text> | 是否限制补卡时间 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_date_num</md-text> | <md-text type="field-type" >int</md-text> | 补卡时间，几天内补卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >show_cumulative_time</md-text> | <md-text type="field-type" >boolean</md-text> | 是否展示累计时长 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >show_over_time</md-text> | <md-text type="field-type" >boolean</md-text> | 是否展示加班时长 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >hide_staff_punch_time</md-text> | <md-text type="field-type" >boolean</md-text> | 是否隐藏员工打卡详情 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >face_punch</md-text> | <md-text type="field-type" >boolean</md-text> | 是否开启人脸识别打卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >face_punch_cfg</md-text> | <md-text type="field-type" >int</md-text> | 人脸识别打卡规则，1：每次打卡均需人脸识别，2：疑似作弊打卡时需要人脸识别 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >face_downgrade</md-text> | <md-text type="field-type" >boolean</md-text> | 人脸识别失败时是否允许普通拍照打卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >replace_basic_pic</md-text> | <md-text type="field-type" >boolean</md-text> | 人脸识别失败时是否允许替换基准图片 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >machines</md-text> | <md-text type="field-type" >machine\[\]</md-text> | 考勤机列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >machine_sn</md-text> | <md-text type="field-type" >string</md-text> | 考勤机序列号 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >machine_name</md-text> | <md-text type="field-type" >string</md-text> | 考勤机名称 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >gps_range</md-text> | <md-text type="field-type" >int</md-text> | GPS 打卡的有效范围（不建议使用） |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >locations</md-text> | <md-text type="field-type" >location\[\]</md-text> | 地址列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >location_id</md-text> | <md-text type="field-type" >string</md-text> | 地址 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >location_name</md-text> | <md-text type="field-type" >string</md-text> | 地址名称 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >location_type</md-text> | <md-text type="field-type" >int</md-text> | 地址类型，1：GPS，2：Wi-Fi，8：IP |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >latitude</md-text> | <md-text type="field-type" >float</md-text> | 地址纬度 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >longitude</md-text> | <md-text type="field-type" >float</md-text> | 地址经度 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ssid</md-text> | <md-text type="field-type" >string</md-text> | Wi-Fi 名称 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >bssid</md-text> | <md-text type="field-type" >string</md-text> | Wi-Fi 的 MAC 地址 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >map_type</md-text> | <md-text type="field-type" >int</md-text> | 地图类型，1：高德， 2：谷歌 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >address</md-text> | <md-text type="field-type" >string</md-text> | 地址名称 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ip</md-text> | <md-text type="field-type" >string</md-text> | IP 地址 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >feature</md-text> | <md-text type="field-type" >string</md-text> | 额外信息，例如：运营商信息 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >gps_range</md-text> | <md-text type="field-type" >int</md-text> | GPS 打卡的有效范围 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >group_type</md-text> | <md-text type="field-type" >int</md-text> | 考勤类型，0：固定班制，2：排班制， 3：自由班制 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_day_shift_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 固定班制必须填 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >free_punch_cfg</md-text> | <md-text type="field-type" >free_punch_cfg</md-text> | 配置自由班制 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >free_start_time</md-text> | <md-text type="field-type" >string</md-text> | 自由班制打卡开始时间 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >free_end_time</md-text> | <md-text type="field-type" >string</md-text> | 自由班制打卡结束时间 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_day</md-text> | <md-text type="field-type" >int</md-text> | 打卡的时间，为 7 位数字，每一位依次代表周一到周日，0 为不上班，1 为上班 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >work_day_no_punch_as_lack</md-text> | <md-text type="field-type" >boolean</md-text> | 工作日不打卡是否记为缺卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >calendar_id</md-text> | <md-text type="field-type" >int</md-text> | 国家日历  ID，0：不根据国家日历排休，1：中国大陆，2：美国，3：日本，4：印度，5：新加坡，默认 1 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >need_punch_special_days</md-text> | <md-text type="field-type" >punch_special_date_shift\[\]</md-text> | 必须打卡的特殊日期 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_day</md-text> | <md-text type="field-type" >int</md-text> | 打卡日期 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >shift_id</md-text> | <md-text type="field-type" >string</md-text> | 班次 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >no_need_punch_special_days</md-text> | <md-text type="field-type" >punch_special_date_shift\[\]</md-text> | 无需打卡的特殊日期 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_day</md-text> | <md-text type="field-type" >int</md-text> | 打卡日期 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >shift_id</md-text> | <md-text type="field-type" >string</md-text> | 班次 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >work_day_no_punch_as_lack</md-text> | <md-text type="field-type" >boolean</md-text> | 自由班制下工作日不打卡是否记为缺卡 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >effect_now</md-text> | <md-text type="field-type" >boolean</md-text> | 是否立即生效，默认 false |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_period_type</md-text> | <md-text type="field-type" >int</md-text> | 补卡周期类型 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_period_custom_date</md-text> | <md-text type="field-type" >int</md-text> | 补卡自定义周期起始日期 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_type</md-text> | <md-text type="field-type" >int</md-text> | 打卡类型，位运算。1：GPS 打卡，2：Wi-Fi 打卡，4：考勤机打卡，8：IP 打卡 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "group_id": "6919358128597097404",
        "group_name": "开心考勤",
        "time_zone": "Asia/Shanghai",
        "bind_dept_ids": [
            "a6a64f94gg768492"
        ],
        "except_dept_ids": [
            "f864f94dg73459845"
        ],
        "bind_user_ids": [
            "52aa1fa1"
        ],
        "except_user_ids": [
            "4fasdtc2"
        ],
        "group_leader_ids": [
            "52aa1fa1"
        ],
        "allow_out_punch": true,
        "allow_pc_punch": true,
        "allow_remedy": true,
        "remedy_limit": true,
        "remedy_limit_count": 3,
        "remedy_date_limit": true,
        "remedy_date_num": 3,
        "show_cumulative_time": true,
        "show_over_time": true,
        "hide_staff_punch_time": true,
        "face_punch": true,
        "face_punch_cfg": 1,
        "face_downgrade": true,
        "replace_basic_pic": true,
        "machines": [
            {
                "machine_sn": "FS0701",
                "machine_name": "创实 9 楼"
            }
        ],
        "gps_range": 300,
        "locations": [
            {
                "location_id": "6921213751454744578",
                "location_name": "浙江省杭州市余杭区五常街道木桥头西溪八方城",
                "location_type": 1,
                "latitude": 30.28994,
                "longitude": 120.04509,
                "ssid": "TP-Link-af12ca",
                "bssid": "08:00:20:0A:8C:6D",
                "map_type": 1,
                "address": "北京市海淀区中航广场",
                "ip": "122.224.123.146",
                "feature": "中国电信",
                "gps_range": 300
            }
        ],
        "group_type": 0,
        "punch_day_shift_ids": [
            "6919668824125513935"
        ],
        "free_punch_cfg": {
            "free_start_time": "7:00",
            "free_end_time": "18:00",
            "punch_day": 1111100,
            "work_day_no_punch_as_lack": true
        },
        "calendar_id": 1,
        "need_punch_special_days": [
            {
                "punch_day": 20190101,
                "shift_id": "6919668827865513935"
            }
        ],
        "no_need_punch_special_days": [
            {
                "punch_day": 20190101,
                "shift_id": "6919668827865513935"
            }
        ],
        "work_day_no_punch_as_lack": true,
        "effect_now": true,
        "remedy_period_type": 0,
        "remedy_period_custom_date": 1,
        "punch_type": 1
    }
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1220001 | 参数错误 | 请检查参数是否符合要求 |
| 400 | 1220002 | 租户不存在 | 请检查 tenant_access_token 是否正确 |
| 400 | 1220003 | employee_type 不存在 | employee_type，employee_id：员工的 employeeId，employee_no：员工工号 |
| 500 | 1225000 | 系统错误 | 详见错误信息 |
| 500 | 1227000 | 管理服务系统错误 | 详见错误信息 |





