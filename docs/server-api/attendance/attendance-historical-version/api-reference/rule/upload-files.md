---
document_id: '7070902815602688006'
directory_id: '6975751873563574278'
title: 文件上传
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//rule/file_upload
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Rule
- Upload Files
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//rule/file_upload
---

# 文件上传
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/file/upload)
</md-alert>
:::
上传文件并获取文件 ID，可用于“修改用户设置”接口的 face_key 参数。

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
      <md-td>https://open.larksuite.com/open-apis/attendance/v1/files/upload</md-td>
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
            <md-perm>写入打卡管理规则</md-perm>
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
      <md-td>**示例值**："multipart/form-data"</md-td>
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
	<md-text type="field-name" >file_name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	文件名
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
      <md-th style="width: $$$attendance.v1.file.method.upload.request.body.table.param-column.width$$$;">名称</md-th>
      <md-th style="width: $$$attendance.v1.file.method.upload.request.body.table.type-column.width$$$;">类型</md-th>
      <md-th style="width: $$$attendance.v1.file.method.upload.request.body.table.required-column.width$$$;">必填</md-th>
      <md-th style="width: $$$attendance.v1.file.method.upload.request.body.table.desc-column.width$$$;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >file</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >file</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
文件
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 请求体示例

```HTTP
Content-Disposition: form-data; name="file"
Content-Type: application/octet-stream
```


## 响应

### 响应体
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: $$$attendance.v1.file.method.upload.response.body.table.param-column.width$$$;">名称</md-th>
      <md-th style="width: $$$attendance.v1.file.method.upload.response.body.table.type-column.width$$$;">类型</md-th>
      <md-th style="width: $$$attendance.v1.file.method.upload.response.body.table.desc-column.width$$$;">描述</md-th>
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
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >file</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >file</md-text>
	</md-td>
	<md-td>
	文件
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >file_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
文件 ID
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
        "file": {
            "file_id": "6b30e7636a38861bbe02869c726a4612"
        }
    }
}
```

### 错误码
|HTTP 状态码|错误码|描述|排查建议|
|---|---|---|---|
|400|1220001|参数错误|请检查参数是否符合要求|
|400|1220002|租户不存在|请检查 tenant_access_token 是否正确|
|500|1225000|系统错误|详见错误信息|
|500|1227000|管理服务系统错误|详见错误信息|


