---
document_id: '7704258746084707779'
directory_id: '7680851565851299255'
title: 加入会议
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join
breadcrumb:
- Server API
- Video Conferencing
- bot
- JoinMeeting
document_type: ReferenceDocumentType
updated_at: 2026-11-04T10:18:32Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join
---

# 加入会议

通过会议号将机器人加入指定的视频会议。调用成功后会返回会议 ID，该 ID 可用于后续的[离开会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/leave)、[发送会中消息](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/message)等操作。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=vc&version=v1&resource=bot&method=join)

:::html
<md-alert type="tip">
当前能力处于灰度开放阶段，暂未全量开放。如需申请开通或确认可用范围，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW)
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
| HTTP URL | https://open.larksuite.com/open-apis/vc/v1/bots/join |
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
	<md-text type="field-name" >join_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	机器人入会方式。取值范围为 1~100；当前仅支持取值 1，表示通过会议号入会，join_identify 中需传入 meeting_no 字段。取值 2~100 暂不支持。

**示例值**：1

**数据校验规则**：

- 取值范围：`1` ～ `100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >join_identify</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >join_identify</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	入会标识信息。当 join_type=1 时，需传入 meeting_no 字段。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >meeting_no</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	会议号，用于定位需加入的目标会议。可通过会议创建接口或会议列表查询接口获取。

**示例值**："123456789"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >password</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	会议密码。目标会议未设置密码时无需传入；若会议设置了密码，未传入或传入错误密码将无法入会。可通过会议创建接口或会议详情查询接口获取。

**示例值**："******"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >call_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	邀请-入会链路的关联标识。从[邀请机器人入会](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/supportEventsList/meeting_invited)事件获取；仅在响应邀请入会时原样回传。未传入时按普通入会链路处理，不进行邀请关联。

**示例值**："7c9e6679-7425-40de-944b-e07fc1f90ae7"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "join_type": 1,
    "join_identify": {
        "meeting_no": "123456789"
    },
    "password": "******",
    "call_id": "7c9e6679-7425-40de-944b-e07fc1f90ae7"
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
	<md-text type="field-name" >meeting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >bot_meeting_info</md-text>
	</md-dt-td>
	<md-dt-td>
	机器人加入的会议详情信息，包含会议唯一标识、会议号、开始时间及会议主题等核心数据
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
	会议唯一标识ID，系统自动生成，全局唯一。可通过会议创建接口或会议列表查询接口获取
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >meeting_no</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	会议号，用于用户快速加入会议，支持数字或字母组合格式。可在会议创建时指定或由系统自动生成
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >start_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	会议开始时间，格式为YYYY-MM-DD HH:mm:ss，时区为UTC+8
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >topic</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	会议主题，用于标识会议内容，支持中英文及特殊字符，长度不超过128个字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >join_user</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >meeting_user</md-text>
	</md-dt-td>
	<md-dt-td>
	成功入会的机器人用户信息，包含用户唯一标识及用户类型
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
    "meeting": {
      "id": "7628568141510692381",
      "meeting_no": "123456789",
      "start_time": "2024-05-20 10:30:00",
      "topic": "产品规划评审会"
    },
    "join_user": {
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
| 500 | 121001 | internal service error | 服务内部异常。请使用响应中的 request_id / log_id 通过[技术支持](https://applink.larksuite.com/TLJpeNdW)联系Lark技术支持排查；请勿持续重试。 |
| 400 | 121002 | param is invalid | 请检查请求参数是否完整且格式正确：会议号、join_type、密码及其他必填字段应符合接口定义。 |
| 403 | 121003 | no permission | 请确认应用为企业自建应用，且已申请并获授“机器人入离会”权限；调用身份需与应用和租户一致。 |
| 400 | 121004 | meeting status unexpected | 会议当前状态不支持机器人入会。请确认会议未结束、未被取消，且处于允许加入的状态后重试。 |
| 400 | 121005 | meeting not exist | 未找到对应会议。请核对会议号是否正确、未过期，并确认会议属于当前Lark环境。 |
| 400 | 121006 | participant count reach limit | 会议人数已达上限，暂无法加入。请等待参会人离开，或请主持人调整会议人数限制。 |
| 400 | 121007 | callee is busy | 机器人当前正忙或已处于其他通话中。请确认机器人已离开其他会议后再重试。 |
| 400 | 121008 | meeting locked | 会议已锁定。请联系主持人解锁会议或允许机器人加入后重试。 |
| 400 | 121009 | need password | 会议设置了密码。请在请求中提供正确的 password。 |
| 400 | 121010 | wrong password | 会议密码错误。请向会议组织者确认密码后重试。 |
| 400 | 121011 | password try exceed limit | 密码尝试次数已超限。请停止重试，等待限制解除或联系会议组织者确认后再操作。 |
| 429 | 121012 | too many failed attempts | 短时间内失败次数过多，已触发限流。请采用指数退避重试，避免并发或高频重复调用。 |
| 403 | 121013 | no permission to join | 当前用户无权加入该会议。请确认用户受邀或具备加入权限，必要时请主持人调整入会权限。 |
| 400 | 121014 | meeting not started | 会议尚未开始。请等待会议开始后再调用；不要针对同一会议号高频轮询。 |
| 403 | 121015 | lark security forbidden | 命中Lark安全策略，禁止机器人入会。请检查企业/会议安全策略与应用配置；不要通过反复重试规避限制。 |
| 403 | 121016 | This agent is temporarily unable to join the meeting. Ask the meeting host to enable "AI Summary" and try again. If it is already enabled, confirm that "Allow agents to join meetings" is enabled in the current meeting's "Security" settings. See: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join | 该智能体暂时无法加入会议。请由会议主持人开启「AI 总结」后重试；如已开启，请在当前会议的「安全」中确认「允许智能体加入会议」已开启。<br>![当前会议中开启 AI 总结和允许智能体加入会议](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/eb1ea70801866df10b82ce7ad9576824_1Nt6JC2dI5.png?lazyload=true&width=2192&height=1550) |
| 403 | 121017 | user is not in the meeting | 发起调用的用户当前不在会议中。请使用已在会的授权用户调用，或先让该用户加入会议。 |
| 403 | 121018 | bot is not in the meeting | 机器人当前不在会议中。请先确认入会接口成功，并使用返回的 meeting_id 继续执行后续操作。 |





