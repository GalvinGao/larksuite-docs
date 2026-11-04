---
document_id: '7704258746084658627'
directory_id: '7680851565851299255'
title: 离开会议
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/leave
breadcrumb:
- Server API
- Video Conferencing
- bot
- LeaveMeeting
document_type: ReferenceDocumentType
updated_at: 2026-11-04T10:18:27Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/leave
---

# 离开会议

机器人可通过会议 ID 主动离开指定的视频会议。调用成功后，将返回该机器人对应的用户信息。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=vc&version=v1&resource=bot&method=leave)

:::html
<md-alert type="tip">
当前能力处于灰度开放阶段，暂未全量开放。如需申请开通或确认可用范围，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW)
</md-alert>
:::

:::html
<md-alert type="warn">
会议 ID 来源：请使用[加入会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join)接口调用成功后返回的长数字 `meeting_id`，而不是 9 位会议号。
</md-alert>
:::

:::html
<md-alert type="error">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/vc/v1/bots/leave |
| HTTP Method | POST |
| 接口频率限制 | [5 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="vc:meeting.bot.join:write" desc="加入、离开会议" support_app_types="custom" tags="">加入、离开会议</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 请求体

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;">描述</md-dt-th>
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
	是
	</md-dt-td>
	<md-dt-td>
	会议唯一标识，用于指定机器人需要退出的目标会议。请传入[加入会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join)接口调用成功后返回的长数字 `meeting_id`，而不是 9 位会议号。

**示例值**："mtg_202405201430_001"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "meeting_id": "mtg_202405201430_001"
}
</md-code-json>
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
	<md-text type="field-name" >leave_user</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >meeting_user</md-text>
	</md-dt-td>
	<md-dt-td>
	离开用户信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >user_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	用户类型

**可选值有**：
<md-enum>
<md-enum-item key="1" >lark用户</md-enum-item>
<md-enum-item key="2" >rooms用户</md-enum-item>
<md-enum-item key="3" >文档用户</md-enum-item>
<md-enum-item key="4" >neo单品用户</md-enum-item>
<md-enum-item key="5" >neo单品游客用户</md-enum-item>
<md-enum-item key="6" >pstn用户</md-enum-item>
<md-enum-item key="7" >sip用户</md-enum-item>
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
        "leave_user": {
            "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
            "user_type": 1
        }
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 121101 | internal service error | 服务内部异常。请使用响应中的 request_id / log_id 通过[技术支持](https://applink.larksuite.com/TLJpeNdW)联系Lark技术支持排查；请勿持续重试。 |
| 400 | 121102 | param is invalid | 请检查请求参数是否完整且格式正确，尤其确认 meeting_id 为有效的会议 ID。 |
| 403 | 121103 | no permission | 请确认应用为企业自建应用，且已申请并获授“机器人入离会”权限；调用身份需与应用和租户一致。 |
| 400 | 121104 | meeting status unexpected | 会议当前状态不支持机器人离会。请确认会议状态后重试。 |
| 400 | 121105 | meeting not exist | 未找到对应会议。请核对 meeting_id 是否正确，并确认会议属于当前Lark环境。 |
| 400 | 121106 | meeting locked | 会议已锁定。请联系主持人解锁会议或确认是否允许机器人离会后重试。 |
| 400 | 121107 | participant status unexpected | 参会人当前状态不支持离会。请确认机器人已在会，且其状态正常后重试。 |
| 403 | 121108 | This agent is temporarily unable to leave this meeting. Ask the meeting host to enable "AI Summary" and try again. If it is already enabled, confirm that "Allow agents to join meetings" is enabled in the current meeting's "Security" settings. See: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/leave | 该智能体暂时无法离开会议。请由会议主持人开启「AI 总结」后重试；如已开启，请在当前会议的「安全」中确认「允许智能体加入会议」已开启。<br>![当前会议中开启 AI 总结和允许智能体加入会议](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/eb1ea70801866df10b82ce7ad9576824_1Nt6JC2dI5.png?lazyload=true&width=2192&height=1550) |
| 403 | 121109 | user is not in the meeting | 发起调用的用户当前不在会议中。请使用已在会的授权用户调用，或先让该用户加入会议。 |
| 403 | 121110 | bot is not in the meeting | 机器人当前不在会议中。无需重复离会；请先确认入会接口成功，并使用返回的 meeting_id 继续执行后续操作。 |





