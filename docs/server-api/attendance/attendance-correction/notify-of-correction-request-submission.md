---
document_id: '7070902815602851846'
directory_id: '7070770034936102917'
title: 通知补卡审批发起
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/create
breadcrumb:
- Server API
- Attendance
- Attendance Correction
- Notify of Correction Request Submission
document_type: ReferenceDocumentType
updated_at: 2022-03-16T08:28:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/create
---

# 通知补卡审批发起

对于只使用Lark考勤系统而未使用Lark审批系统的企业，可以通过该接口，将在三方审批系统中发起的补卡审批数据，写入到Lark考勤系统中。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=user_task_remedy&method=create)

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
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/attendance/v1/user_task_remedys</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
      </md-th>
      <md-td>
            <md-perm name="attendance:task" desc="写入打卡数据" support_app_types="custom" tags="">写入打卡数据</md-perm>
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 18%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
       <md-th style="width: 15%;">必填</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"

[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)

</md-td>
</md-tr>
<md-tr>
<md-td>Content-Type</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
</md-tbody>
</md-table>
:::



### 查询参数
:::html
<md-table>
  <md-thead>
      <tr>
      <md-th style="width: 30%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
      <md-th style="width: 15%;">必填</md-th>
      <md-th >描述</md-th>
      </tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >employee_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	请求体和响应体中的 user_id 的员工工号类型

**示例值**："employee_id"

**可选值有**：
- `employee_id`：员工 employee ID，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的用户 ID
- `employee_no`：员工工号，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的工号
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 请求体

:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: 40%;">名称</md-th>
      <md-th style="width: 20%;">类型</md-th>
      <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 30%;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >user_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	用户 ID

**示例值**："abd754f7"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >remedy_date</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	补卡日期

**示例值**：20210701
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >punch_no</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	第几次上下班，0：第 1 次上下班，1：第 2 次上下班，2：第 3 次上下班，自由班制填 0

**示例值**：0
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >work_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	上班 / 下班，1：上班，2：下班，自由班制填 0

**示例值**：1
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >approval_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	审批 ID

**示例值**："6737202939523236113"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >remedy_time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	补卡时间，时间格式为 yyyy-MM-dd HH:mm

**示例值**："2021-07-01 08:00"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >status</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	补卡状态

**示例值**：2

**可选值有**：
- `0`：审批中
- `2`：已通过
- `3`：已取消
- `4`：通过后撤回
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >reason</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	补卡原因

**示例值**："忘记打卡"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	补卡时间，精确到秒的时间戳

**示例值**："1611476284"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >time_zone</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	补卡时考勤组时区

**示例值**："Asia/Shanghai"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >create_time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	补卡发起时间，精确到秒的时间戳

**示例值**："1611476284"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >update_time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	补卡状态更新时间，精确到秒的时间戳

**示例值**："1611476284"
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 请求体示例

```json
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
```



## 响应



### 响应体
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: 40%;">名称</md-th>
      <md-th style="width: 20%;">类型</md-th>
      <md-th style="width: 30%;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >code</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	错误码，非 0 表示失败
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >msg</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	错误描述
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >data</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >\-</md-text>
	</md-td>
	<md-td>
	\-
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_remedy</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >user_task_remedy</md-text>
	</md-td>
	<md-td>
	补卡审批信息
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	用户 ID
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_date</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	补卡日期
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >punch_no</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	第几次上下班，0：第 1 次上下班，1：第 2 次上下班，2：第 3 次上下班，自由班制填 0
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >work_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	上班 / 下班，1：上班，2：下班，自由班制填 0
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >approval_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	审批 ID
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remedy_time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	补卡时间，时间格式为 yyyy-MM-dd HH:mm
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >status</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	补卡状态

**可选值有**：
- `0`：审批中
- `2`：已通过
- `3`：已取消
- `4`：通过后撤回
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >reason</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	补卡原因
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	补卡时间，精确到秒的时间戳
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >time_zone</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	补卡时考勤组时区
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >create_time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	补卡发起时间，精确到秒的时间戳
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >update_time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	补卡状态更新时间，精确到秒的时间戳
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "user_remedy": {
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
    }
}
```



### 错误码
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">HTTP状态码</md-th>
            <md-th style="width: 15%;">错误码</md-th>
            <md-th style="width: 30%;">描述</md-th>
            <md-th style="width: 30%;">排查建议</md-th>
        </md-tr>
    </md-thead>
  <md-tbody>

<md-tr>
  <md-td>400</md-td>
  <md-td>1220001</md-td>
  <md-td>参数错误</md-td>
  <md-td>请检查参数是否符合要求</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1220002</md-td>
  <md-td>租户不存在</md-td>
  <md-td>请检查 tenant_access_token 是否正确</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1220004</md-td>
  <md-td>用户不存在或没有权限</md-td>
  <md-td>请检查用户 ID 是否正确</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1220005</md-td>
  <md-td>没有权限</md-td>
  <md-td>请前往[考勤管理后台](https://oa.larksuite.com/attendance/manage/member/list)检查数据权限范围</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1225000</md-td>
  <md-td>系统错误</md-td>
  <md-td>详见错误信息</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1225001</md-td>
  <md-td>写入部分成功</md-td>
  <md-td>详见错误信息</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1226501</md-td>
  <md-td>没有异常考勤</md-td>
  <md-td>当天没有异常考勤，无需补卡</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1226502</md-td>
  <md-td>不允许补卡</md-td>
  <md-td>考勤组设置不允许补卡</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1226503</md-td>
  <md-td>补卡日期限制</md-td>
  <md-td>考勤组设置只允许补过去多少天的卡，超出可补卡日期</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1226504</md-td>
  <md-td>超出补卡次数</md-td>
  <md-td>当前周期的补卡次数已用完</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




