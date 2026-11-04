---
document_id: '7704258746084642243'
directory_id: '7680851565851299255'
title: 会中倒计时
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/countdown
breadcrumb:
- Server API
- Video Conferencing
- bot
- CountDown
document_type: ReferenceDocumentType
updated_at: 2026-11-04T10:18:23Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/countdown
---

# 操作会议倒计时

该接口用于会中操作倒计时，支持自定义时长、设置/延长/提前结束倒计时、关闭倒计时窗口。适用于会议控场场景{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=vc&version=v1&resource=bot&method=countdown)

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
| HTTP URL | https://open.larksuite.com/open-apis/vc/v1/bots/countdown |
| HTTP Method | POST |
| 接口频率限制 | [100 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="vc:meeting.interaction:write" desc="使用会中交互功能" support_app_types="custom" tags="">使用会中交互功能</md-perm> |

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
	会议唯一标识，必须传入长数字 meeting_id，不是 9 位会议号。可通过创建会议接口或会议列表查询接口获取。


**示例值**："7628568141510692381"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >action</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	倒计时操作类型。可选值：set、prolong、end_in_advance、close_window。set 表示设置倒计时，prolong 表示延长倒计时，end_in_advance 表示提前结束，close_window 表示关闭会中倒计时窗口。

**示例值**："set"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >duration</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	倒计时时长，单位为分钟。action 为 set 或 prolong 时必填；set 时超过 24 小时会报错，prolong 时超过 24 小时会按 24 小时处理。

**示例值**："10"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >need_play_audio_at_end</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	倒计时结束时是否播放提示音。仅 action=set 时生效。默认为false。

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >reminder_before_end</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	倒计时结束前的提醒时间，单位为分钟。仅 action=set 时生效；只支持设置一次提醒，且必须大于 0 并小于 duration。

**示例值**："1"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "meeting_id": "7628568141510692381",
    "action": "set",
    "duration": "10",
    "need_play_audio_at_end": false,
    "reminder_before_end": "1"
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


  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "msg": "success",
    "data": {}
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 121001 | internal service error | 服务内部异常。请使用响应中的 request_id / log_id 通过[技术支持](https://applink.larksuite.com/TLJpeNdW)联系Lark技术支持排查；请勿持续重试。 |
| 400 | 121002 | param is invalid | 操作参数不正确。请确认倒计时时长、会议、操作是否选择正确，然后重新发起。 |
| 403 | 121003 | no permission | 当前账号没有权限操作倒计时。会议人数>50人后，仅主持人、联席主持人有权限操作倒计时。 |
| 403 | 121004 | user is not in the meeting | 你当前不在会议中，无法操作倒计时。请先加入会议，再重新操作。 |
| 403 | 121005 | bot is not in the meeting | 机器人不在会议中，无法执行该操作。请确认会议助手/机器人已加入当前会议，必要时重新邀请机器人入会。 |
| 400 | 121006 | meeting status unexpected | 当前会议状态不支持倒计时。请确认会议是否已经开始、是否已经结束，或是否处于可操作状态。 |
| 400 | 121007 | meeting not exist | 没有找到对应会议。请确认会议链接或会议号是否正确，必要时重新进入会议后再操作。 |
| 403 | 121008 | lark security forbidden | 当前操作被安全策略拦截。请确认你是否使用了公司允许的账号、网络或应用环境；如果仍失败，联系管理员处理。 |
| 400 | 121009 | countdown has been triggered | 这个会议已经设置了倒计时，不能重复设置。请查看会议中是否已有倒计时，避免重复点击。 |
| 400 | 121010 | no countdown to operate | 当前没有可操作的倒计时。请确认是否已经创建或启动倒计时，再进行操作。 |
| 400 | 121011 | countdown is in use | 倒计时正在使用中，暂时不能执行当前操作。请等待当前倒计时结束，或提前结束后再重新操作。 |





