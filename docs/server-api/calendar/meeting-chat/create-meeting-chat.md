---
document_id: '7459650906272710662'
directory_id: '7456715520965935116'
title: 创建会议群
full_path: /uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-meeting_chat/create
breadcrumb:
- Server API
- Calendar
- Meeting Chat
- Create meeting chat
document_type: ReferenceDocumentType
updated_at: 2025-01-14T06:14:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-meeting_chat/create
---

# 创建会议群

调用该接口以当前身份（应用或用户）为指定日程创建一个会议群。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=calendar&version=v4&resource=calendar.event.meeting_chat&method=create)

:::html
<md-alert type="tip">
- 当前身份由 Header Authorization 的 Token 类型决定。tenant_access_token 指应用身份，user_access_token 指用户身份。
- 如果使用应用身份调用该接口，则需要确保应用开启了[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
- 日程所在的日历需要是当前身份的主日历，且具有日历的 writer 权限。你可以调用[查询主日历信息](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primary)接口，获取当前身份的主日历信息。
- 日程需要添加了至少 2 个参与人，且不隐藏参与人列表。你可以调用[获取日程参与人列表](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee/list)接口获取日程的参与人情况；可以调用[获取日程](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/get)接口，查看日程参与人权限信息（attendee_ability）。
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
| HTTP URL | https://open.larksuite.com/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_chat |
| HTTP Method | POST |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="calendar:calendar" desc="更新日历及日程信息" support_app_types="custom,isv" tags="">更新日历及日程信息</md-perm><br><md-perm name="calendar:calendar.event:update" desc="更新日程" support_app_types="custom,isv" tags="">更新日程</md-perm> |

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
	日程所在的日历 ID。了解更多，参见[日历 ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/introduction)。

**示例值**："larksuite.com_xxx@group.calendar.larksuite.com"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >event_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	日程 ID。

创建日程时会返回日程 ID。你也可以调用以下接口获取某一日历的 ID。
- [获取日程列表](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/list)
- [搜索日程](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/search)

**示例值**："75d28f9b-e35c-4230-8a83-123_0"
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
	<md-text type="field-name" >meeting_chat_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	会议群 ID。后续可用于解绑会议群。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >applink</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群分享链接。
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
        "meeting_chat_id": "oc_xxx",
        "applink": "https://example.cn?openChatId=oc_xxx"
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
| 403 | 190011 | tenant encrypt key has been deleted | 加解密状态的自主密钥被删除，被该秘钥加密的数据不可用。 |
| 403 | 190012 | tenant decrypt key has been deleted | 仅解密状态的自主密钥被删除，被该秘钥加密的数据不可用。 |
| 404 | 191000 | calendar not found | 日历没有找到。你需要检查并改为正确的日历 ID。 |
| 400 | 191001 | invalid calendar_id | calendar_id 无效。你需要检查并改为正确的日历 ID。 |
| 403 | 191002 | no calendar access_role | 当前身份没有日历的访问权限。如需查询某一日历信息，则需要确保当前身份拥有该日历的访问权限。 |
| 403 | 191003 | calendar is deleted | 日历已经被删除。你需要检查并改为正确的日历 ID。 |
| 403 | 191004 | invalid calendar type | 日历类型错误。你可以调用[查询日历信息](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/get)接口获取日历类型信息，然后确保日历类型适用于当前接口。 |
| 404 | 193001 | event not found | 日程未找到。你需要确保传入了正确的日程 ID。 |
| 403 | 193002 | no permission to operate event | 无权限操作。你需要确保有日历以及日程的编辑权限。 |
| 403 | 193003 | event is deleted | 日程已经被删除。你需要检查并改为正确的日程 ID。 |
| 404 | 195100 | user is dismiss or not exist in the tenant | 当前身份或指定用户已经离职，或者不在该租户内。请检查并改为正确的身份来调用接口。 |
| 400 | 195109 | event not support chat creation | 日程不支持创建会议群。你需要检查日历是否为当前身份的主日历且有 writer 权限，还需要检查日程参与人是否至少为 2 个，且有查看参与人列表的权限。 |


更多错误码信息，参见[通用错误码](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)。


