---
document_id: '7085933590580494342'
directory_id: '7081519902443798534'
title: 获取录制文件
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/get
breadcrumb:
- Server API
- Video Conferencing
- Meeting record
- Obtain recording files
document_type: ReferenceDocumentType
updated_at: 2023-04-20T08:43:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/get
---

# 获取录制文件

获取一个会议的录制文件。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=vc&version=v1&resource=meeting.recording&method=get)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">
会议结束后并且收到了"录制完成"的事件方可获取录制文件；使用 user_access_token 只有会议owner（通过开放平台预约的会议即为预约人）有权限获取，使用 tenant_access_token 可以获取租户范围下的录制文件；录制时间太短(&lt;5s)有可能无法生成录制文件
</md-alert>
:::

:::html
<md-alert type="tip">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/vc/v1/meetings/:meeting_id/recording |
| HTTP Method | GET |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="vc:record:readonly" desc="获取会议录制信息" support_app_types="custom,isv" tags="">获取会议录制信息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




### 路径参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >meeting_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	会议ID（视频会议的唯一标识，视频会议开始后才会产生）

**示例值**："6911188411932033028"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





## 响应





### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	错误码，非 0 表示失败
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >msg</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	错误描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >data</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >\-</md-text>
	</md-dt-td>
	<md-dt-td>
	\-
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >recording</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >meeting.recording</md-text>
	</md-dt-td>
	<md-dt-td>
	录制文件数据
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	录制文件URL
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >duration</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	录制总时长（单位msec）
	</md-dt-td>
</md-dt-tr>


  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "msg": "success",
    "data": {
        "recording": {
            "url": "https://www.larksuite.com//minutes/obcn37dxcftoc3656rgyejm7",
            "duration": "30000"
        }
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 121001 | internal error | 服务器内部错误，如果重试无效可联系管理员 |
| 400 | 121002 | not support | 暂不支持该功能 |
| 400 | 121003 | param error | 参数错误，检查参数的取值范围（请按照上面字段说明自查） |
| 404 | 121004 | data not exist | 请求的数据不存在 |
| 403 | 121005 | no permission | 无权限进行该操作，建议检查token类型、操作者身份以及资源的归属 |
| 400 | 122001 | meeting status unexpected | 会议状态不正确（例如：会中操作而会议不在进行中状态） |
| 404 | 122002 | meeting not exist | 所操作的会议不存在 |
| 400 | 124002 | record processing | 录制文件生成中 |





