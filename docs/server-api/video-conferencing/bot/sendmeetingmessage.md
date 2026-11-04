---
document_id: '7704258746084691395'
directory_id: '7680851565851299255'
title: 发送会中消息
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/message
breadcrumb:
- Server API
- Video Conferencing
- bot
- SendMeetingMessage
document_type: ReferenceDocumentType
updated_at: 2026-11-04T10:18:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/message
---

# 发送会中消息

向指定的视频会议发送会中消息。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=vc&version=v1&resource=bot&method=message)

:::html
<md-alert type="tip">
当前能力处于灰度开放阶段，暂未全量开放。如需申请开通或确认可用范围，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW)
</md-alert>
:::

:::html
<md-alert type="warn">
调用前提：机器人需已加入目标会议。请先调用[加入会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join)接口。
</md-alert>
:::

:::html
<md-alert type="error">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/vc/v1/bots/message |
| HTTP Method | POST |
| 接口频率限制 | [100 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="vc:meeting.message:write" desc="发送会中消息和表情" support_app_types="custom" tags="">发送会中消息和表情</md-perm> |

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
	会议唯一标识，必须传入长数字 meeting_id，不是 9 位会议号。可通过“获取用户活跃会议列表”等会议查询接口获取。

**示例值**："7628568141510692381"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >msg_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	消息类型。可选值：text、reaction。text 表示会中文本消息，reaction 表示会中反馈表情。

**示例值**："text"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	消息内容。当 msg_type=text 时表示文本内容；当 msg_type=reaction 时表示反馈表情 key。表情可选值参考：[表情参考说明](https://open.larksuite.com/document/server-docs/vc-v1/bot/user-guide/meeting-emoji-reference)

**示例值**："请大家看一下这个问题"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >uuid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	幂等去重 ID，对应底层 Cid。不传时服务端自动生成并在响应中返回。

**示例值**："7f3b7fd2-1b64-4f5d-9e8b-2f2d8d3b0c11"
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
    "msg_type": "text",
    "content": "请大家看一下这个问题",
    "uuid": "7f3b7fd2-1b64-4f5d-9e8b-2f2d8d3b0c11"
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
	<md-text type="field-name" >uuid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	本次实际使用的幂等去重 ID。若请求未传 uuid，则返回服务端生成的 uuid。
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
        "uuid": "a1b2c3d4-5678-90ef-ghij-klmnopqrstuv"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 120001 | param is invalid | 请检查请求参数是否完整且格式正确，尤其确认 meeting_id、msg_type、content 和 uuid 符合接口定义。 |





