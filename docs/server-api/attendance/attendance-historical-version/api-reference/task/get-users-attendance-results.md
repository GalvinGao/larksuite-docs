---
document_id: '6975751842354544645'
directory_id: '6975751873563557894'
title: 获取打卡结果
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//GetCheckinResults
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Task
- Get Users’ Attendance Results
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//GetCheckinResults
---

# 获取打卡结果

:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task/query)
</md-alert>
:::
获取企业内员工的实际打卡结果，包括上班打卡结果和下班打卡结果。

:::note
* 如果企业给一个员工设定的班次是上午 9 点和下午 6 点各打一次上下班卡，即使员工在这期间打了多次卡，该接口也只会返回 1 条记录。
* 如果要获取打卡的详细数据，如打卡位置等信息，可使用“获取打卡流水记录”或“批量查询打卡流水记录”的接口。
:::
## 请求
|基本||
|---|---|
|HTTP URL|https://open.larksuite.com/open-apis/attendance/v1/user_tasks/query|
|HTTP Method|POST|
|HTTP Content-Type|application/json; charset=utf-8|
|凭证要求|tenant_access_token|
|权限要求|打卡数据导出|
### 头部
key|value
--|--
Authorization|Bearer tenant_access_token
Content-Type|application/json
### 查询参数
|名称|类型|必填|描述|
|---|---|---|---|
|employee_type|string|是|请求体中的 user_ids 的员工工号类型，可用值：【employee_id（员工的 employeeId），employee_no（员工工号）】，示例值：“employee_id”|
|ignore_invalid_users|boolean|否|是否忽略无效和没有权限的用户。如果 true，返回有效用户的数据，并告知无效和没有权限的用户；如果 false，且 user_ids 中存在无效或没有权限的用户，返回错误|

### 请求体
|名称|类型|必填|描述|
|---|---|---|---|
|user_ids|string\[\]|是|employee_no 或 employee_id 列表|
|check_date_from|int|是|查询的起始工作日|
|check_date_to|int|是|查询的结束工作日，与 check_date_from 的时间间隔不超过30天|
### 请求体示例
```json
{
    "user_ids": [
        "abd754f7"
    ],
    "check_date_from": 20210104,
    "check_date_to": 20210105
}
```
## 响应
### 响应体
|名称|类型|描述|
|---|---|---|
|code|int|错误码，非 0 表示失败|
|msg|string|错误描述|
|data|-|-|
|&emsp;∟user_task_results|user_task\[\]|打卡任务列表|
|&emsp;&emsp;∟result_id|string|打卡记录 ID|
|&emsp;&emsp;∟user_id|string|employee ID|
|&emsp;&emsp;∟employee_name|string|employee 姓名|
|&emsp;&emsp;∟day|int|日期|
|&emsp;&emsp;∟group_id|string|考勤组 ID|
|&emsp;&emsp;∟shift_id|string|班次 ID|
|&emsp;&emsp;∟records|task_result\[\]|用户考勤记录|
|&emsp;&emsp;&emsp;∟check_in_record_id|string|上班打卡记录 ID|
|&emsp;&emsp;&emsp;∟check_in_record|user_flow\[\]|上班打卡记录|
|&emsp;&emsp;&emsp;&emsp;∟user_id|string|员工工号|
|&emsp;&emsp;&emsp;&emsp;∟creator_id|string|打卡记录创建者的 employee_no|
|&emsp;&emsp;&emsp;&emsp;∟location_name|string|打卡位置名称信息|
|&emsp;&emsp;&emsp;&emsp;∟check_time|string|打卡时间，精确到秒的时间戳|
|&emsp;&emsp;&emsp;&emsp;∟comment|string|打卡备注|
|&emsp;&emsp;&emsp;&emsp;∟record_id|string|打卡记录 ID|
|&emsp;&emsp;&emsp;&emsp;∟longitude|float|打卡经度|
|&emsp;&emsp;&emsp;&emsp;∟latitude|float|打卡纬度|
|&emsp;&emsp;&emsp;&emsp;∟ssid|string|打卡 Wi-Fi 的 SSID|
|&emsp;&emsp;&emsp;&emsp;∟bssid|string|打卡 Wi-Fi 的 MAC 地址|
|&emsp;&emsp;&emsp;&emsp;∟is_field|boolean|是否为外勤打卡|
|&emsp;&emsp;&emsp;&emsp;∟is_wifi|boolean|是否为 Wi-Fi 打卡|
|&emsp;&emsp;&emsp;&emsp;∟type|int|记录生成方式，可用值：【0（用户自己打卡），1（管理员修改），2（用户补卡），3（系统自动生成），4（下班免打卡），5（考勤机打卡），6（极速打卡），7（考勤开放平台导入），8（Lark自研考勤机），9（Lark门禁考勤机）】|
|&emsp;&emsp;&emsp;&emsp;∟photo_urls|string\[\]|打卡照片列表|
|&emsp;&emsp;&emsp;&emsp;∟device_id|string|手机打卡设备ID|
|&emsp;&emsp;&emsp;∟check_out_record_id|string|下班打卡记录 ID|
|&emsp;&emsp;&emsp;∟check_out_record|user_flow\[\]|下班打卡记录|
|&emsp;&emsp;&emsp;&emsp;∟user_id|string|员工工号|
|&emsp;&emsp;&emsp;&emsp;∟creator_id|string|打卡记录创建者的 employee_no|
|&emsp;&emsp;&emsp;&emsp;∟location_name|string|打卡位置名称信息|
|&emsp;&emsp;&emsp;&emsp;∟check_time|string|打卡时间，精确到秒的时间戳|
|&emsp;&emsp;&emsp;&emsp;∟comment|string|打卡备注|
|&emsp;&emsp;&emsp;&emsp;∟record_id|string|打卡记录 ID|
|&emsp;&emsp;&emsp;&emsp;∟longitude|float|打卡经度|
|&emsp;&emsp;&emsp;&emsp;∟latitude|float|打卡纬度|
|&emsp;&emsp;&emsp;&emsp;∟ssid|string|打卡 Wi-Fi 的 SSID|
|&emsp;&emsp;&emsp;&emsp;∟bssid|string|打卡 Wi-Fi 的 MAC 地址|
|&emsp;&emsp;&emsp;&emsp;∟is_field|boolean|是否为外勤打卡|
|&emsp;&emsp;&emsp;&emsp;∟is_wifi|boolean|是否为 Wi-Fi 打卡|
|&emsp;&emsp;&emsp;&emsp;∟type|int|记录生成方式，可用值：【0（用户自己打卡），1（管理员修改），2（用户补卡），3（系统自动生成），4（下班免打卡），5（考勤机打卡），6（极速打卡），7（考勤开放平台导入），8（Lark自研考勤机），9（Lark门禁考勤机）】|
|&emsp;&emsp;&emsp;&emsp;∟photo_urls|string\[\]|打卡照片列表|
|&emsp;&emsp;&emsp;&emsp;∟device_id|string|手机打卡设备ID|
|&emsp;&emsp;&emsp;∟check_in_result|string|上班打卡结果，可用值：【NoNeedCheck（无需打卡），SystemCheck（系统打卡），Normal（正常），Early（早退），Late（迟到），Lack（缺卡）】|
|&emsp;&emsp;&emsp;∟check_out_result|string|下班打卡结果，可用值：【NoNeedCheck（无需打卡），SystemCheck（系统打卡），Normal（正常），Early（早退），Late（迟到），Lack（缺卡）】|
|&emsp;&emsp;&emsp;∟check_in_result_supplement|string|上班打卡结果补充，可用值：【None（无），ManagerModification（管理员修改），CardReplacement（补卡通过），ShiftChange（换班），Travel（出差），Leave（请假），GoOut（外出），CardReplacementApplication（补卡申请中），FieldPunch（外勤打卡）】|
|&emsp;&emsp;&emsp;∟check_out_result_supplement|string|下班打卡结果补充，可用值：【None（无），ManagerModification（管理员修改），CardReplacement（补卡通过），ShiftChange（换班），Travel（出差），Leave（请假），GoOut（外出），CardReplacementApplication（补卡申请中），FieldPunch（外勤打卡）】|
|&emsp;&emsp;&emsp;∟check_in_shift_time|string|正常默认上班时间，精确到秒的时间戳|
|&emsp;&emsp;&emsp;∟check_out_shift_time|string|正常默认下班时间，精确到秒的时间戳|
|&emsp;∟invalid_user_ids|string\[\]|无效用户工号列表|
|&emsp;∟unauthorized_user_ids|string\[\]|没有权限用户工号列表|
### 响应体示例
```json
{
	"code": 0,
	"msg": "success",
	"data": {
		"user_task_results": [{
			"result_id": "6709359313699356941",
			"user_id": "abd754f7",
			"employee_name": "张三",
			"day": 20210104,
			"group_id": "6737202939523236110",
			"shift_id": "6753520403404030215",
			"records": [{
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
				}
				"check_out_record_id": "6709359313699356942",
				"check_out_record": {
					"user_id": "abd754f7",
					"creator_id": "abd754f7",
					"location_name": "西溪八方城",
					"check_time": "1611476284",
					"comment": "下班打卡",
					"record_id": "6709359313699356942",
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
				"check_in_result": "SystemCheck",
				"check_out_result": "SystemCheck",
				"check_in_result_supplement": "None",
				"check_out_result_supplement": "None",
				"check_in_shift_time": "1611476284",
				"check_out_shift_time": "1611476284"
			}]
		}],
		"invalid_user_ids": ["abd754xx"],
		"unauthorized_user_ids": ["abd75xxx"]
	}
}
```
### 错误码
|HTTP 状态码|错误码|描述|排查建议|
|---|---|---|---|
|400|1220001|参数错误|请检查参数是否符合要求|
|400|1220002|租户不存在|请检查 tenant_access_token 是否正确|
|400|1220004|用户不存在或没有权限|请检查用户 ID 是否正确|
|400|1220005|没有权限|请前往考勤管理后台检查数据权限范围|
|500|1225000|系统错误|详见错误信息|
|500|1226500|打卡服务系统错误|详见错误信息|
|500|1227500|组织架构服务系统错误|详见错误信息|
