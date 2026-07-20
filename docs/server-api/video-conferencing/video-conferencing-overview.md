---
document_id: '7085933590580658182'
directory_id: '7081519902443880454'
title: 视频会议概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/video-conferencing-overview
breadcrumb:
- Server API
- Video Conferencing
- Video Conferencing overview
document_type: GuideDocumentType
updated_at: 2022-04-13T04:01:07Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/video-conferencing-overview
---

# 视频会议概述


## 业务介绍

视频会议（Video Conferencing，简称 VC）是指Lark音视频会议业务域，为不同区域的用户提供完善、便捷的高质量音视频交互体验，满足线上实时沟通协作的需求。通过视频会议 API，你可以实现多种功能，例如：

- 会议预约，如预约会议、更新会议等
- 会议操作，如邀请参会人、设置主持人、结束会议等
- 会议录制，如开启/停止录制、获取录制文件等
- 会议报告，如获取会议数据报告、Top 用户列表等
- 会议室配置，如设置背景图、设置数字标牌等


### 接入流程

:::html

<md-table>

<md-thead>

<md-tr>

<md-th style="width: 5%;"></md-th>
<md-th style="width: 25%;">步骤</md-th>

<md-th style="width: 70%;">介绍</md-th>

</md-tr>

</md-thead>

<md-tbody>

<md-tr>
<md-td>1</md-td>
<md-td>创建一个应用</md-td>

<md-td>
- 如需创建企业自建应用，可参考 [自建应用的开发流程](/document/home/introduction-to-custom-app-development/self-built-application-development-process) 
- 如需创建应用商店应用，可参考 [开发和上架应用商店应用](/document/uMzNwEjLzcDMx4yM3ATM/uYzNwEjL2cDMx4iN3ATM)
</md-td>
</md-tr>
  
<md-tr>
<md-td>2</md-td>
<md-td>调用 API，对视频会议进行操作</md-td>

<md-td>
调用 API 前，你需要先获取访问凭证并开启对应的权限，详情参见 [如何调用服务端API](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)

</md-td>
</md-tr>
  
  
<md-tr>
<md-td>3</md-td>
<md-td>监听事件，获知视频会议的变化</md-td>

<md-td>
监听事件前，你需要先申请相应的权限，详情参见 [事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)。目前仅通过开放平台进行预约的会议能够监听到相关事件，详情参见[预约会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/apply)。
</md-td>
</md-tr>

</md-tbody>

</md-table>

:::

## 资源介绍

视频会议业务域以“资源”为中心进行开放，资源的关系图如下：
  


![20220411-183316.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1670765a1c80b0db5bbfdbbd1db7fd3c_43ZMqwb16g.png?lazyload=true&width=2480&height=2114)


:::html

视频会议的相关资源定义如下：
<md-table>
<md-thead>
<tr>
<md-th style="width: 20%;">资源</md-th>
<md-th style="width: 80%;">资源定义</md-th>
</tr>
</md-thead>
<md-tbody>
  
<md-tr>
<md-td>
[预约会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/schedule-meeting-overview)
</md-td>
<md-td>
用户可以预约会议，提前设置参会成员和会议权限，并获取会议信息
</md-td>
</md-tr>

<md-tr>
<md-td>
[会议操作](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/meeting-overview)
</md-td>
<md-td>
用户可以在会议中进行邀请参会成员、移除参会成员和设置主持人等操作
</md-td>
</md-tr>

<md-tr>
<md-td>
[会议录制](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/recording-overview)
</md-td>
<md-td>
用户可以录制一场会议，在会议结束后获得会议录制文件链接
</md-td>
</md-tr>
  
  
<md-tr>
<md-td>
[会议报告](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/meeting-report-overview)
</md-td>
<md-td>
会议报告用于记录一段时间内租户会议的使用情况，包括会议数、会议时长和参会人数等信息
</md-td>
</md-tr>  
  
  

</md-tbody>

</md-table>

:::

以下将详细介绍每个资源的字段、方法、事件。

### 资源：预约会议
查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/schedule-meeting-overview)

#### 方法列表
>  “商店”代表 [应用商店应用](/document/home/app-types-introduction/overview)；“自建”代表 [企业自建应用](/document/home/app-types-introduction/overview)
:::html

<md-table>

<md-thead>

<tr>

<md-th style="width: 70%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 10%;">权限要求（满足任一）</md-th>

<md-th style="width: 10%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）**</md-td></md-th>

<md-th style="width: 5%;">商店</md-th>
<md-th style="width: 5%;">自建</md-th>

</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" >[预约会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/apply)</md-text>
  
`POST` /open-apis/vc/v1/reserves/apply
>预约一场视频会议

  </md-text>

</md-td>

<md-td><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新会议预约信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>
</md-td>

<md-td>
**✓**
</md-td>
<md-td>
**✓**
</md-td>


</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[更新会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/update)</md-text>
  
`PUT` /open-apis/vc/v1/reserves/:reserve_id
>更新一场已预约的视频会议配置，包括会议主题、会议权限配置等
</md-td>


<md-td><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新会议预约信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-user" >user_access_token</md-tag>

</md-td>

<md-td>
**✓**
</md-td>
<md-td>
**✓**
</md-td>


</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[删除预约](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/delete)</md-text>
  
`DELETE`/open-apis/vc/v1/reserves/:reserve_id
>删除一场已预约的视频会议
</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新会议预约信息</md-perm>


</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>
**✓**
</md-td>
<md-td>
**✓**
</md-td>


</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[获取预约](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get)</md-text>
  
`GET`/open-apis/vc/v1/reserves/:reserve_id
>获取一场已预约的视频会议的详情，包括会议 ID、会议链接、会议权限
</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议预约信息</md-perm>


</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>
**✓**
</md-td>
<md-td>
**✓**
</md-td>


</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[获取活跃会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get_active_meeting)</md-text>
  
`GET`/open-apis/vc/v1/reserves/:reserve_id/get_active_meeting
>获取一场进行中的预约会议详情，包括参会人数、参会人详情等

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议预约信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>
**✓**
</md-td>
<md-td>
**✓**
</md-td>


</md-tr>


</md-tbody>

</md-table>

:::


### 资源：会议

查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/meeting-overview)

#### 方法列表

:::html

<md-table>

<md-thead>

<tr>

<md-th style="width: 70%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 10%;">权限要求（满足任一）</md-th>

<md-th style="width: 10%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）**</md-td></md-th>

<md-th style="width: 5%;">商店</md-th>
<md-th style="width: 5%;">自建</md-th>

</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" >[获取会议详情](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/get)</md-text>

`GET` /open-apis/vc/v1/meetings/:meeting_id
  
>获取一个会议的详细数据，包括会议主题、会议 ID、会议链接、开始时间、会议状态、参会人列表等

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>
   <md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[获取与会议号相关联的会议列表](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/list_by_no)</md-text>

  `GET` /open-apis/vc/v1/meetings/list_by_no
  
>获取指定时间范围会议号关联的会议简要信息列表

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

<md-tag ype="token-tenant">tenant_access_token</md-tag>
  <md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[邀请参会人](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/invite)</md-text>

  `PATCH` /open-apis/vc/v1/meetings/:meeting_id/invite
  
>邀请参会人加入会议

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新会议信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[移除参会人](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/kickout)</md-text>

`POST` /open-apis/vc/v1/meetings/:meeting_id/kickout
  
>将参会人从会议中移除

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新会议信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[设置主持人](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/set_host)</md-text>

  `PATCH` /open-apis/vc/v1/meetings/:meeting_id/set_host
  
>将某一参会人设置为主持人

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新会议信息</md-perm>

</md-td>

<md-td>
  
<md-tag type="token-tenant">tenant_access_token</md-tag>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[结束会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/end)</md-text>

`PATCH` /open-apis/vc/v1/meetings/:meeting_id/end
>结束一场正在进行的会议

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新会议信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

</md-tbody>

</md-table>

:::

#### 事件列表

:::html

<md-table>

<md-thead>

<tr>

<md-th style="width: 20%;"><md-td>**[事件 (Event)](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)**</md-td></md-th>

<md-th style="width: 20%;">触发时机</md-th>

<md-th style="width: 25%;">权限要求（满足任一）</md-th>

<md-th style="width: 25%;">事件类型</md-th>

<md-th style="width: 5%;">商店</md-th>

<md-th style="width: 5%;">自建</md-th>

</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" >[会议开始](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/meeting_started)</md-text>

</md-td>

<md-td>会议被开始时</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

vc.meeting.meeting_started_v1

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[会议结束](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/meeting_ended)</md-text>

</md-td>

<md-td>会议被结束时</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

vc.meeting.meeting_ended_v1

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[加入会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/join_meeting)</md-text>

</md-td>

<md-td>有参会人加入会议时</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

vc.meeting.join_meeting_v1

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>
  <md-tr>

<md-td>

<md-text type="field-name" >[离开会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/leave_meeting)</md-text>

</md-td>

<md-td>有参会人离开会议时</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

vc.meeting.leave_meeting_v1

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>
<md-tr>

<md-td>

<md-text type="field-name" >[录制开始](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/recording_started)</md-text>

</md-td>

<md-td>录制开始时</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

vc.meeting.recording_started_v1

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>
<md-tr>

<md-td>

<md-text type="field-name" >[录制停止](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/recording_ended)</md-text>

</md-td>

<md-td>录制停止时</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

vc.meeting.recording_ended_v1

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>
<md-tr>

<md-td>

<md-text type="field-name" >[录制完成](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/recording_ready)</md-text>

</md-td>

<md-td>录制文件上传完毕时</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

vc.meeting.recording_ready_v1

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>
<md-tr>

<md-td>

<md-text type="field-name" >[屏幕共享开始](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/share_started)</md-text>

</md-td>

<md-td>屏幕共享开始时</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

vc.meeting.share_started_v1

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>
<md-tr>

<md-td>

<md-text type="field-name" >[屏幕共享结束](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/share_ended)</md-text>

</md-td>

<md-td>屏幕共享结束时</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议信息</md-perm>

</md-td>

<md-td>

vc.meeting.share_ended_v1

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>


</md-tbody>

</md-table>

:::

### 资源：录制

查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/recording-overview)

#### 方法列表

:::html

<md-table>

<md-thead>

<tr>

<md-th style="width: 70%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 10%;">权限要求（满足任一）</md-th>

<md-th style="width: 10%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）**</md-td></md-th>

<md-th style="width: 5%;">商店</md-th>
<md-th style="width: 5%;">自建</md-th>

</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" >[开始录制](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/start)</md-text>

`PATCH` /open-apis/vc/v1/meetings/:meeting_id/recording/start
  
>在会议中开始录制

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">更新会议录制信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[停止录制](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/stop)</md-text>

  `PATCH` /open-apis/vc/v1/meetings/:meeting_id/recording/stop
  
>在会议中停止录制

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">更新会议录制信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[获取录制文件](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/get)</md-text>

  `GET` /open-apis/vc/v1/meetings/:meeting_id/recording
  
>获取一个会议的录制文件

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">获取会议录制信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>


</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[授权录制文件](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/set_permission)</md-text>

`PATCH` /open-apis/vc/v1/meetings/:meeting_id/recording/set_permission
  
>将一个会议的录制文件授权给组织、用户或公开到公网

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#29943da9">更新会议录制信息</md-perm>


</md-td>

<md-td>

<md-tag type="token-user">user_access_token</md-tag>


</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

</md-tbody>

</md-table>

:::



### 资源：会议报告
查看[资源字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/meeting-report-overview)

#### 方法列表

:::html

<md-table>

<md-thead>

<tr>

<md-th style="width: 70%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 10%;">权限要求（满足任一）</md-th>

<md-th style="width: 10%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)（选择其一）**</md-td></md-th>

<md-th style="width: 5%;">商店</md-th>
<md-th style="width: 5%;">自建</md-th>

</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" >[获取会议报告](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/get_daily)</md-text>

`GET` /open-apis/vc/v1/reports/get_daily
  
>获取一段时间内组织的每日会议使用报告，包括总会议数量、总会议时长、总参会人数等

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议报告</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[获取 Top 用户列表](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/get_top_user)</md-text>

  `GET` /open-apis/vc/v1/reports/get_top_user
  
>获取一段时间内组织内会议使用的 Top 用户列表

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取会议报告</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>

<md-td>

**✓**

</md-td>

<md-td>

**✓**

</md-td>

</md-tr>

</md-tbody>

</md-table>

:::


