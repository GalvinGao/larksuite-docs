---
document_id: '7704258746084724163'
directory_id: '7680851565851299255'
title: 获取用户活跃会议列表
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/user_active_meeting
breadcrumb:
- Server API
- Video Conferencing
- bot
- GetUserActiveMeeting
document_type: ReferenceDocumentType
updated_at: 2026-11-04T10:18:40Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/user_active_meeting
---

# 获取用户活跃会议列表

查询指定用户当前正在参与的所有会议，返回该用户处于活跃状态的会议列表，包含会议号、会议 ID 及会议标题等核心信息。

{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=vc&version=v1&resource=bot&method=user_active_meeting)

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
| HTTP URL | https://open.larksuite.com/open-apis/vc/v1/bots/user_active_meeting |
| HTTP Method | GET |
| 接口频率限制 | [5 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="vc:meeting.meetingevent:read" desc="获取会议信息" support_app_types="custom,isv" tags="">获取会议信息</md-perm><br><md-perm name="vc:meeting.bot.join:write" desc="加入、离开会议" support_app_types="custom" tags="">加入、离开会议</md-perm><br><md-alert type="tip" icon="none"><br>**推荐权限申请**：用户身份申请「获取会议信息」，应用身份申请「机器人入离会」<br></md-alert> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom,isv" tags="">获取用户 user ID</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




### 查询参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;" >描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	目标用户的 ID。其类型由 user_id_type 指定：open_id（以 ou_ 开头）、union_id 或 user_id。user_id 的格式必须与 user_id_type 的取值一致；应用身份调用时必填。

**示例值**：
ou_3ec3f6a28a0d08c45d895276e8e5e19b
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >user_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户 ID 类型

**示例值**：open_id

**可选值有**：
<md-enum>
<md-enum-item key="open_id" >标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)</md-enum-item>
<md-enum-item key="union_id" >标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)</md-enum-item>
<md-enum-item key="user_id" >标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)</md-enum-item>
</md-enum>

**默认值**：`open_id`

**当值为 `user_id`，字段权限要求**：
<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
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
	<md-text type="field-name" >meetings</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >user_active_meeting_info\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	用户当前参与的活跃会议集合，每个元素包含会议号、会议ID及会议标题信息。
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
	会议号
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >meeting_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	会议ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >meeting_title</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	会议标题
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
        "meetings": [
            {
                "meeting_no": "987654321",
                "meeting_id": "7628568141510692381",
                "meeting_title": "Q3季度产品迭代评审会"
            }
        ]
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 121001 | internal error | 服务器内部错误，请稍后重试，若重试无效可联系Lark技术支持：https://applink.larksuite.com/TLJpeNdW |
| 400 | 121002 | not support | 当前能力暂不支持。请确认接口版本、应用类型和使用场景是否符合要求；如需确认能力开放范围，请通过 https://applink.larksuite.com/TLJpeNdW 联系Lark技术支持。 |
| 400 | 121003 | param error | 参数错误。请检查参数的类型、格式、取值范围和必填项是否符合接口文档要求，尤其确认 user_id 的格式与 user_id_type 的取值一致。 |
| 404 | 121004 | data not exist | 数据不存在，确认会议是否存在或仍在进行中 |
| 403 | 121005 | no permission | 请在开发者后台的“权限管理”中确认当前应用已申请并获授所需权限（包括“机器人入离会”）；使用与目标用户同一租户的正确 access token，并确认用户与会议数据均属于当前租户。 |
| 403 | 121009 | This user is temporarily unable to retrieve the user's active meetings. Ask the meeting host to enable "AI Summary" first, then confirm that "Allow agents to join meetings" is enabled in the meeting's "Security" settings. See: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/user_active_meeting | 用户当前无法获取用户活跃会议列表。请由会议主持人开启「AI 总结」后重试；如已开启，请在当前会议的「安全」中确认「允许智能体加入会议」已开启。<br>![当前会议中开启 AI 总结和允许智能体加入会议](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/eb1ea70801866df10b82ce7ad9576824_1Nt6JC2dI5.png?lazyload=true&width=2192&height=1550) |
| 403 | 121010 | user is not in the meeting | 用户不在会议中，请确认用户已加入该会议后重新发起请求 |
| 403 | 121011 | bot is not in the meeting | 应用机器人不在会议中，请先将应用机器人加入该会议后重新发起请求 |





