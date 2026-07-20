---
document_id: '7070902815602917382'
directory_id: '7070770034936119301'
title: 查询统计表头
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_field/query
breadcrumb:
- Server API
- Attendance
- Attendance Statistics
- Query Statistics Header
document_type: ReferenceDocumentType
updated_at: 2022-03-03T15:54:13Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_field/query
---

# 查询统计表头

查询考勤统计支持的日度统计或月度统计的统计表头。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=user_stats_field&method=query)

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
      <md-td>https://open.larksuite.com/open-apis/attendance/v1/user_stats_fields/query</md-td>
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
	响应体中的 user_id 的员工工号类型

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
	<md-text type="field-name" >locale</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	语言类型

**示例值**："zh"

**可选值有**：
- `en`：英语
- `ja`：日语
- `zh`：中文
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >stats_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	统计类型

**示例值**："daily"

**可选值有**：
- `daily`：日度统计
- `month`：月度统计
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >start_date</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	开始时间

**示例值**：20210316
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >end_date</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	结束时间（时间间隔不超过 40 天）

**示例值**：20210323
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 请求体示例

```json
{
    "locale": "zh",
    "stats_type": "daily",
    "start_date": 20210316,
    "end_date": 20210323
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
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_stats_field</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >user_stats_field</md-text>
	</md-td>
	<md-td>
	统计数据表头
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >stats_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	统计类型

**可选值有**：
- `daily`：日度统计
- `month`：月度统计
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
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >fields</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >field\[\]</md-text>
	</md-td>
	<md-td>
	字段列表
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	字段编号
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	字段名称
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >child_fields</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >child_field\[\]</md-text>
	</md-td>
	<md-td>
	子字段列表
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	子字段编号
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	子字段名称
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >time_unit</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	时间单位
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 响应体示例

```json
{
    "code": 0,
    "msg": "",
    "data": {
        "user_stats_field": {
            "fields": [
                {
                    "child_fields": [
                        {
                            "code": "50103",
                            "title": "Employee ID"
                        }
                    ],
                    "code": "501",
                    "title": "Basic info"
                },
                {
                    "child_fields": [
                        {
                            "code": "52108",
                            "title": "Attendance group name"
                        },
                        {
                            "code": "52101",
                            "title": "Required attendance days"
                        },
                        {
                            "code": "52102",
                            "title": "Days of attendance"
                        },
                        {
                            "code": "52104",
                            "time_unit": "Minute",
                            "title": "Required attendance duration"
                        },
                        {
                            "code": "52105",
                            "time_unit": "Minute",
                            "title": "Actual attendance duration"
                        },
                        {
                            "code": "52107",
                            "title": "Overtime hours"
                        }
                    ],
                    "code": "521",
                    "title": "Attendance statistics"
                },
                {
                    "child_fields": [
                        {
                            "code": "52201",
                            "title": "Late in times"
                        },
                        {
                            "code": "52203",
                            "title": "Early out times"
                        },
                        {
                            "code": "52207",
                            "title": "No records"
                        }
                    ],
                    "code": "522",
                    "title": "Abnormal statistics"
                },
                {
                    "child_fields": [
                        {
                            "code": "2021-03-16",
                            "title": "2021-03-16 Tue"
                        },
                        {
                            "code": "2021-03-17",
                            "title": "2021-03-17 Wed"
                        },
                        {
                            "code": "2021-03-18",
                            "title": "2021-03-18 Thu"
                        },
                        {
                            "code": "2021-03-19",
                            "title": "2021-03-19 Fri"
                        },
                        {
                            "code": "2021-03-20",
                            "title": "2021-03-20 Sat"
                        },
                        {
                            "code": "2021-03-21",
                            "title": "2021-03-21 Sun"
                        },
                        {
                            "code": "2021-03-22",
                            "title": "2021-03-22 Mon"
                        },
                        {
                            "code": "2021-03-23",
                            "title": "2021-03-23 Tue"
                        }
                    ],
                    "code": "524",
                    "title": "Daily statistics"
                }
            ],
            "stats_type": "month",
            "user_id": "ec8ddg56"
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
  <md-td>500</md-td>
  <md-td>1228000</md-td>
  <md-td>统计服务系统错误</md-td>
  <md-td>详见错误信息</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




