---
document_id: '6966503468250415109'
directory_id: '6965452230078922758'
title: 查询日历信息
full_path: /uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/get
breadcrumb:
- Server API
- Calendar
- Calendar management
- Query calendar information
document_type: ReferenceDocumentType
updated_at: 2025-01-14T02:35:05Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/get
---

# 查询日历信息

调用该接口以当前身份（应用或用户）查询指定日历的信息。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=calendar&version=v4&resource=calendar&method=get)

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
- 当前身份由 Header Authorization 的 Token 类型决定。tenant_access_token 指应用身份，user_access_token 指用户身份。
- 使用应用身份调用该接口前，需要确保该应用开启了[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
- 当前身份必须对日历有访问权限。
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/calendar/v4/calendars/:calendar_id |
| HTTP Method | GET |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="calendar:calendar" desc="更新日历及日程信息" support_app_types="custom,isv" tags="">更新日历及日程信息</md-perm><br><md-perm name="calendar:calendar:read" desc="读取日历信息" support_app_types="custom,isv" tags="">读取日历信息</md-perm><br><md-perm name="calendar:calendar:readonly" desc="获取日历、日程及忙闲信息" support_app_types="custom,isv" tags="">获取日历、日程及忙闲信息</md-perm> |

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
	<md-text type="field-name" >calendar_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	日历 ID。

创建共享日历时会返回日历 ID。你也可以调用以下接口获取某一日历的 ID。
- [查询主日历信息](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primary)
- [查询日历列表](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/list)
- [搜索日历](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/search)

**示例值**："larksuite.com_xxxxxxxxxx@group.calendar.larksuite.com"
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
	<md-text type="field-type" >calendar</md-text>
	</md-dt-td>
	<md-dt-td>
	\-
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >calendar_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	日历 ID。后续可以通过该 ID 查询、更新或删除日历信息。更多信息可参见[日历 ID 字段说明](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/introduction)。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >summary</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	日历标题。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >description</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	日历描述。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >permissions</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	日历公开范围。

**可选值有**：
<md-enum>
<md-enum-item key="private" >私密</md-enum-item>
<md-enum-item key="show_only_free_busy" >仅展示忙闲信息</md-enum-item>
<md-enum-item key="public" >公开，他人可查看日程详情</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >color</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	日历颜色，由颜色 RGB 值的 int32 表示。实际在客户端展示时会映射到色板上最接近的一种颜色，且该字段仅对当前身份生效。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	日历类型。

**可选值有**：
<md-enum>
<md-enum-item key="unknown" >未知类型</md-enum-item>
<md-enum-item key="primary" >用户或应用的主日历</md-enum-item>
<md-enum-item key="shared" >由用户或应用创建的共享日历</md-enum-item>
<md-enum-item key="google" >用户绑定的谷歌日历</md-enum-item>
<md-enum-item key="resource" >会议室日历</md-enum-item>
<md-enum-item key="exchange" >用户绑定的 Exchange 日历</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >summary_alias</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	日历备注名，仅对当前身份生效。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >is_deleted</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	对于当前身份，日历是否已经被标记为删除。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >is_third_party</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	当前日历是否是第三方数据。三方日历及日程只支持读，不支持写入。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >role</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	当前身份对于该日历的访问权限。

**可选值有**：
<md-enum>
<md-enum-item key="unknown" >未知权限</md-enum-item>
<md-enum-item key="free_busy_reader" >游客，只能看到忙碌、空闲信息</md-enum-item>
<md-enum-item key="reader" >订阅者，可查看所有日程详情</md-enum-item>
<md-enum-item key="writer" >编辑者，可创建及修改日程</md-enum-item>
<md-enum-item key="owner" >管理员，可管理日历及共享设置</md-enum-item>
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
        "calendar_id": "larksuite.com_xxxxxxxxxx@group.calendar.larksuite.com",
        "summary": "测试日历",
        "description": "使用开放接口创建日历",
        "permissions": "private",
        "color": -1,
        "type": "shared",
        "summary_alias": "日历备注名",
        "is_deleted": false,
        "is_third_party": false,
        "role": "owner"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 190002 | invalid parameters in request | 无效的请求参数。排查建议如下：<br>- 确认请求参数的字段名称、传参类型正确。<br>- 确认已经申请了相应资源的权限。<br>- 确认相应资源未被删除。 |
| 500 | 190003 | internal service error | 内部服务错误，请咨询[技术支持](https://applink.larksuite.com/TLJpeNdW)。 |
| 429 | 190004 | method rate limited | 方法频率限制。建议稍后再试，并适当减小请求 QPS。 |
| 429 | 190005 | app rate limited | 应用频率限制。建议稍后再试，并适当减小请求 QPS。 |
| 403 | 190006 | wrong unit for app tenant | 请求错误，检查应用 App ID 和 App Secret 是否正确。如仍无法解决请咨询[技术支持](https://applink.larksuite.com/TLJpeNdW)。 |
| 404 | 190007 | app bot_id not found | 应用的 bot_id 没有找到。你需要确保应用开启了[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。如仍未解决请咨询[技术支持](https://applink.larksuite.com/TLJpeNdW)。 |
| 429 | 190010 | current operation rate limited | 当前操作被限流，原因一般为公用资源并发抢占失败。你可以适当降低当前操作频率，然后重试。 |
| 404 | 191000 | calendar not found | 日历没有找到。你需要检查并改为正确的日历 ID。 |
| 400 | 191001 | invalid calendar_id | calendar_id 无效。你需要检查并改为正确的日历 ID。 |
| 403 | 191002 | no calendar access_role | 当前身份没有日历的访问权限。如需查询日历信息，则需要确保当前身份拥有该日历的访问权限。 |
| 403 | 191003 | calendar is deleted | 日历已经被删除。你需要检查并改为正确的日历 ID。 |
| 403 | 191004 | invalid calendar type | 日历类型错误。你可以调用[查询日历信息](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/get)接口获取日历类型信息，然后确保日历类型适用于当前接口。 |
| 404 | 195100 | user is dismiss or not exist in the tenant | 当前身份或指定用户已经离职，或者不在该租户内。请检查并改为正确的身份来调用接口。 |


更多错误码信息，参见[通用错误码](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)。


