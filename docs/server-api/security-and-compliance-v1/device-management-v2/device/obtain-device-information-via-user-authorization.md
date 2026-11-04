---
document_id: '7696363549205384644'
directory_id: '7432195859482099718'
title: 通过授权获取设备信息
full_path: /uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_record/mine
breadcrumb:
- Server API
- security_and_compliance
- Device Management V2
- Device
- Obtain device information via user authorization
document_type: ReferenceDocumentType
updated_at: 2026-10-14T03:41:26Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_record/mine
---

# 通过授权获取设备信息

通过用户授权获取对应授权端的设备信息，包含设备认证编码、当前设备归属、当前可信状态{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=security_and_compliance&version=v2&resource=device_record&method=mine)

:::html
<md-alert type="tip">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="error">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/security_and_compliance/v2/device_records/mine |
| HTTP Method | GET |
| 接口频率限制 | [50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="security_and_compliance:device_record:read" desc="获取设备信息" support_app_types="custom" tags="">获取设备信息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |






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
	<md-text type="field-name" >device_record_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	设备认证编码
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >device_ownership</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	设备归属

**可选值有**：
<md-enum>
<md-enum-item key="0" >未知设备</md-enum-item>
<md-enum-item key="1" >个人设备</md-enum-item>
<md-enum-item key="2" >企业设备</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >device_status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	可信状态

**可选值有**：
<md-enum>
<md-enum-item key="0" >未知状态</md-enum-item>
<md-enum-item key="1" >信任设备</md-enum-item>
<md-enum-item key="2" >非信任设备</md-enum-item>
</md-enum>
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
        "device_record_id": "7089353870308032531",
        "device_ownership": 0,
        "device_status": 0
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 1780001 | Internal server error. | 内部接口处理失败。若无法解决，请通过【Lark客服入口】寻求帮助，https://applink.larkoffice.com/TLJpeNdW |
| 400 | 1780101 | Request param error. | 检查参数是否正确 |
| 400 | 1780102 | The API is in beta testing. Please contact customer support to request beta access. | 请联系服务提供方排查，https://applink.larkoffice.com/TLJpeNdW |
| 400 | 1785011 | The current user identity has not completed device authentication. | 客户端未完成设备认证，建议提示用户重新登录或升级客户端至7.42及以上后重试 |
| 400 | 1785012 | The third-party device integration is not enabled. | 未开启允许三方应用获取客户端设备认证信息，请联系管理员前往 【Lark管理后台-安全-终端安全-设备管理-设备集成配置】进行配置 |





