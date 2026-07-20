---
document_id: '7070902815602966534'
directory_id: '6975751873563574278'
title: 修改用户设置
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//rule/user-setting-modify
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Rule
- Modify User Settings
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:18Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//rule/user-setting-modify
---

# 修改用户设置
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/modify)
</md-alert>
:::
修改授权内员工的用户设置信息，包括人脸照片文件 ID。

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
      <md-td>https://open.larksuite.com/open-apis/attendance/v1/user_settings/modify</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    <md-tr>
      <md-th>
 权限要求
 <md-tooltip type="info">调用该 API 所需的权限，开启其中任意一项权限即可调用</md-tooltip>
</md-th>
      <md-td>
            <md-perm >写入打卡管理规则</md-perm>
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
	用户类型
      
**可选值有**：
- `employee_id`： 员工 ID
- `employee_no`： 员工工号
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
      <md-th style="width: $$$attendance.v1.user_setting.method.modify.request.body.table.param-column.width$$$;">名称</md-th>
      <md-th style="width: $$$attendance.v1.user_setting.method.modify.request.body.table.type-column.width$$$;">类型</md-th>
      <md-th style="width: $$$attendance.v1.user_setting.method.modify.request.body.table.required-column.width$$$;">必填</md-th>
      <md-th style="width: $$$attendance.v1.user_setting.method.modify.request.body.table.desc-column.width$$$;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >user_setting</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >user_setting</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
		用户信息
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
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >face_key</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	人脸照片 key（通过文件上传接口得到）
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::


### 请求体示例

```json
{
    "user_setting": {
        "user_id": "61gc44e1",
        "face_key": "1013d6cb59555a26ff3e5f721342a2a7"
    }
}
```

## 响应


### 响应体
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: $$$attendance.v1.user_setting.method.modify.response.body.table.param-column.width$$$;">名称</md-th>
      <md-th style="width: $$$attendance.v1.user_setting.method.modify.response.body.table.type-column.width$$$;">类型</md-th>
      <md-th style="width: $$$attendance.v1.user_setting.method.modify.response.body.table.desc-column.width$$$;">描述</md-th>
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
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_setting</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >user_setting</md-text>
	</md-td>
	<md-td>
	用户设置
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
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >face_key</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	人脸照片 key
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
        "user_setting": {
            "user_id": "61gc44e1",
            "face_key": "1013d6cb59555a26ff3e5f721342a2a7",
        }
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
|500|1227000|管理服务系统错误|详见错误信息|



