---
document_id: '7070902815602933766'
directory_id: '7070770034936135685'
title: 获取打卡流水记录
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_flow/get
breadcrumb:
- Server API
- Attendance
- Attendance Records
- Get Attendance Records
document_type: ReferenceDocumentType
updated_at: 2022-03-16T08:28:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_flow/get
---

# 获取打卡流水记录

通过打卡记录 ID 获取用户的打卡流水记录。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=user_flow&method=get)

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
      <md-td>https://open.larksuite.com/open-apis/attendance/v1/user_flows/:user_flow_id</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
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
            
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
            
      </md-th>
      <md-td>
            <md-perm name="attendance:task" desc="写入打卡数据" support_app_types="custom" tags="">写入打卡数据</md-perm>
            <md-perm name="attendance:task:readonly" desc="导出打卡数据" support_app_types="custom" tags="">导出打卡数据</md-perm>
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
</md-tbody>
</md-table>
:::



### 路径参数
:::html
<md-table>
  <md-thead>
      <tr>
      <md-th style="width: 30%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
      <md-th >描述</md-th>
      </tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >user_flow_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	打卡流水记录 ID，获取方式：1）[批量查询打卡流水记录](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_flow/query) 2）[获取打卡结果](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task/query) 3）[导入打卡流水记录](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_flow/batch_create)

**示例值**："6708236686834352397"
	</md-td>
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
	响应体中的 user_id 和 creator_id 的员工工号类型

**示例值**："employee_id"

**可选值有**：
- `open_id`：开放 openID
- `employee_id`：员工 employee ID，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的用户 ID
- `employee_no`：员工工号，即Lark管理后台 > 组织架构 > 成员与部门 > 成员详情中的工号
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::





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
	<md-text type="field-type" >user_flow</md-text>
	</md-td>
	<md-td>
	\-
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text>
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
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >creator_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	记录创建者 ID
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >location_name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	打卡位置名称信息
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >check_time</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	打卡时间，精确到秒的时间戳
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >comment</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	打卡备注
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >record_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	打卡记录 ID
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >longitude</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >float</md-text>
	</md-td>
	<md-td>
	打卡经度
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >latitude</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >float</md-text>
	</md-td>
	<md-td>
	打卡纬度
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ssid</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	打卡 Wi-Fi 的 SSID
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >bssid</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	打卡 Wi-Fi 的 MAC 地址
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_field</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	是否为外勤打卡
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_wifi</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	是否为 Wi-Fi 打卡
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	记录生成方式

**可选值有**：
- `0`：用户打卡
- `1`：管理员修改
- `2`：用户补卡
- `3`：系统自动生成
- `4`：下班免打卡
- `5`：考勤机
- `6`：极速打卡
- `7`：考勤开放平台导入
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >photo_urls</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-td>
	<md-td>
	打卡照片列表
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >device_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	打卡设备 ID
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
  <md-td>1226500</md-td>
  <md-td>打卡服务系统错误</md-td>
  <md-td>详见错误信息</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1227500</md-td>
  <md-td>组织架构服务系统错误</md-td>
  <md-td>详见错误信息</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




