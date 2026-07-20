---
document_id: '6975751842354413573'
directory_id: '6975751873563557894'
title: 更新统计设置
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//task/update-user-stats-settings
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Task
- Update Statistics Settings
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:27Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//task/update-user-stats-settings
---

# 更新统计设置
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/update)
</md-alert>
:::
更新日度统计或月度统计的统计设置信息。

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
      <md-td>https://open.larksuite.com/open-apis/attendance/v1/user_stats_views/:user_stats_view_id</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>PUT</md-td>
    </md-tr>
    <md-tr>
      <md-th>
 权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
</md-th>
      <md-td>
             <md-perm>写入打卡数据</md-perm>
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
	<md-text type="field-name" >user_stats_view_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	用户视图 ID

**示例值**："TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09"
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
	用户 ID 类型

**可选值有**：
- `employee_id`：用户员工 ID
- `employee_no`：用户员工工号
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
      <md-th style="width: $$$attendance.v1.user_stats_view.method.update.request.body.table.param-column.width$$$;">名称</md-th>
      <md-th style="width: $$$attendance.v1.user_stats_view.method.update.request.body.table.type-column.width$$$;">类型</md-th>
      <md-th style="width: $$$attendance.v1.user_stats_view.method.update.request.body.table.required-column.width$$$;">必填</md-th>
      <md-th style="width: $$$attendance.v1.user_stats_view.method.update.request.body.table.desc-column.width$$$;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >view</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >user_stats_view</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	统计视图
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	视图 ID

**示例值**："TmpnNU5EQXpPVGN3TmpVMU16Y3lPVEEwTXl0dGIyNTBhQT09"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >stats_type</md-text>
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
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	用户 ID

	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >items</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >item\[\]</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	一级标题
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >code</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	编号

**示例值**："501"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	标题名称

**示例值**："基本信息"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >child_items</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >child_item\[\]</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	子标题
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
	是
	</md-td>
	<md-td>
	标题编号

**示例值**："50101"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	开关字段
      
**可选值有**：
- `0`：关闭
- `1`：开启

非开关字段场景
-  code = 51501  **可选值为1～6**
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::

### 请求体示例

```json
{
    "view": {
        "items": [
            {
                "child_items": [
                    {
                        "code": "50102",
                        "value": "0"
                    },
                    {
                        "code": "50111",
                        "value": "0"
                    },
                    {
                        "code": "50104",
                        "value": "0"
                    }
                ],
                "code": "501"
            }
        ],
        "stats_type": "month",
        "user_id": "ec8ddg56",
        "view_id": "TmpnNU5EQXpPVGN3TmpVMU16Y3lPVEEwTXl0dGIyNTBhQT09"
    }
}

```

## 响应

### 响应体
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: $$$attendance.v1.user_stats_view.method.update.response.body.table.param-column.width$$$;">名称</md-th>
      <md-th style="width: $$$attendance.v1.user_stats_view.method.update.response.body.table.type-column.width$$$;">类型</md-th>
      <md-th style="width: $$$attendance.v1.user_stats_view.method.update.response.body.table.desc-column.width$$$;">描述</md-th>
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
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >user_stats_view</md-text>
	</md-td>
	<md-td>
	用户视图
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >view_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
		统计视图 ID

**示例值**："TmpnNU5EQXpPVGN3TmpVMU16Y3lPVEEwTXl0dGIyNTBhQT09"
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
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >items</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >item\[\]</md-text>
	</md-td>
	<md-td>
一级标题
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
标题编码
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
标题名称
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >child_items</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >child_item\[\]</md-text>
	</md-td>
	<md-td>
子标题
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
标题编号
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是否开启
      
**可选值有**：
- `0`：关闭
- `1`：开启
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
        "view": {
            "items": [
                {
                    "child_items": [
                        {
                            "code": "50102",
                            "value": "0"
                        },
                        {
                            "code": "50111",
                            "value": "0"
                        },
                        {
                            "code": "50104",
                            "value": "0"
                        }
                    ],
                    "code": "501"
                }
            ],
            "stats_type": "month",
            "user_id": "ec8ddg56",
            "view_id": "TmpnNU5EQXpPVGN3TmpVMU16Y3lPVEEwTXl0dGIyNTBhQT09"
        }
    }
}
```

### 错误码
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: $$$attendance.v1.user_stats_view.method.update.error-mapping.table.http-status-code-column.width$$$;">HTTP 状态码</md-th>
            <md-th style="width: $$$attendance.v1.user_stats_view.method.update.error-mapping.table.code-column.width$$$;">错误码</md-th>
            <md-th style="width: $$$attendance.v1.user_stats_view.method.update.error-mapping.table.desc-column.width$$$;">描述</md-th>
            <md-th style="width: $$$attendance.v1.user_stats_view.method.update.error-mapping.table.suggestions-column.width$$$;">排查建议</md-th>
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


