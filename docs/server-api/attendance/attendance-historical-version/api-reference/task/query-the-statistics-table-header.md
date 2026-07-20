---
document_id: '6975751842354298885'
directory_id: '6975751873563557894'
title: 查询统计表头
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//task/query-statistics-header
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Task
- Query the Statistics Table Header
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:24Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//task/query-statistics-header
---

# 查询统计表头
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_field/query)
</md-alert>
:::
查询日度统计或月度统计的统计表头。

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
      <md-th>
 权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
</md-th>
      <md-td>
                  <md-perm>写入打卡数据</md-perm>
           		 <md-perm >导出打卡数据</md-perm>
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
<md-tag mode="inline" type="token-tenant-desc">tenant_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"

[了解更多：获取与使用 access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)
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
		用户 ID 类型

**可选值有**：
- `employee_id`
- `employee_no`
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
      <md-th style="width: $$$attendance.v1.user_stats_field.method.query.request.body.table.param-column.width$$$;">名称</md-th>
      <md-th style="width: $$$attendance.v1.user_stats_field.method.query.request.body.table.type-column.width$$$;">类型</md-th>
      <md-th style="width: $$$attendance.v1.user_stats_field.method.query.request.body.table.required-column.width$$$;">必填</md-th>
      <md-th style="width: $$$attendance.v1.user_stats_field.method.query.request.body.table.desc-column.width$$$;">描述</md-th>
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

**可选值有**：
- `en`：英文
- `ja`：日文
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
      
      （时间间隔不超过 40 天）
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
	结束时间

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
    "stats_type": "month",
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
      <md-th style="width: $$$attendance.v1.user_stats_field.method.query.response.body.table.param-column.width$$$;">名称</md-th>
      <md-th style="width: $$$attendance.v1.user_stats_field.method.query.response.body.table.type-column.width$$$;">类型</md-th>
      <md-th style="width: $$$attendance.v1.user_stats_field.method.query.response.body.table.desc-column.width$$$;">描述</md-th>
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
字段标题
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
字段编号
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
字段名称
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
时间类型
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
                            "title": "工号"
                        }
                    ],
                    "code": "501",
                    "title": "基本信息"
                },
                {
                    "child_fields": [
                        {
                            "code": "52108",
                            "title": "考勤组名称"
                        },
                        {
                            "code": "52101",
                            "title": "应出勤天数"
                        },
                        {
                            "code": "52102",
                            "title": "工作日出勤天数"
                        },
                        {
                            "code": "52104",
                            "time_unit": "分钟",
                            "title": "应出勤时长"
                        },
                        {
                            "code": "52105",
                            "time_unit": "分钟",
                            "title": "实际出勤时长"
                        },
                        {
                            "code": "52107",
                            "title": "加班工作时长"
                        }
                    ],
                    "code": "521",
                    "title": "出勤统计"
                },
                {
                    "child_fields": [
                        {
                            "code": "52201",
                            "title": "迟到次数"
                        },
                        {
                            "code": "52203",
                            "title": "早退次数"
                        },
                        {
                            "code": "52207",
                            "title": "缺勤"
                        }
                    ],
                    "code": "522",
                    "title": "异常统计"
                },
                {
                    "child_fields": [
                        {
                            "code": "2021-03-16",
                            "title": "2021-03-16 星期二"
                        },
                        {
                            "code": "2021-03-17",
                            "title": "2021-03-17 星期三"
                        },
                        {
                            "code": "2021-03-18",
                            "title": "2021-03-18 星期四"
                        },
                        {
                            "code": "2021-03-19",
                            "title": "2021-03-19 星期五"
                        },
                        {
                            "code": "2021-03-20",
                            "title": "2021-03-20 星期六"
                        },
                        {
                            "code": "2021-03-21",
                            "title": "2021-03-21 星期日"
                        },
                        {
                            "code": "2021-03-22",
                            "title": "2021-03-22 星期一"
                        },
                        {
                            "code": "2021-03-23",
                            "title": "2021-03-23 星期二"
                        }
                    ],
                    "code": "524",
                    "title": "每日统计"
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
            <md-th style="width: $$$attendance.v1.user_stats_field.method.query.error-mapping.table.http-status-code-column.width$$$;">HTTP 状态码</md-th>
            <md-th style="width: $$$attendance.v1.user_stats_field.method.query.error-mapping.table.code-column.width$$$;">错误码</md-th>
            <md-th style="width: $$$attendance.v1.user_stats_field.method.query.error-mapping.table.desc-column.width$$$;">描述</md-th>
            <md-th style="width: $$$attendance.v1.user_stats_field.method.query.error-mapping.table.suggestions-column.width$$$;">排查建议</md-th>
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


