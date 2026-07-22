---
document_id: '7245184492319031302'
directory_id: '7244811844195221510'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-room-data/resource-introduction
breadcrumb:
- Server API
- Video Conferencing
- Meeting data
- Resource introduction
document_type: GuideDocumentType
updated_at: 2023-07-14T09:16:14Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-room-data/resource-introduction
---

#  资源介绍
##  资源定义
用于分页查询一段时间内租户的会议数据，包括：[查询会议明细](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting_list/get)、[查询参会人明细](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/participant_list/get)、[查询参会人会议质量数据](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/participant_quality_list/get)、[查询会议室预定数据](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/resource_reservation_list/get)。

## 权限
:::html
<md-alert type="tip">
**「查询会议明细、查询参会人明细、查询参会人会议质量数据、查询会议室预定数据」** 四个查询接口均需要创建自建应用的用户至少拥有==Lark后台-视频会议-会议管理==模块的权限！
</md-alert>
:::

##  资源：会议明细

###  字段说明

| 名称 | 描述 |
| --- | --- |
| <md-text type="field-name" >会议ID</md-text> | 9位会议号<br>**示例值**："123456789" |
| <md-text type="field-name" >会议主题</md-text> | 会议主题<br>**示例值**："xx的视频会议" |
| <md-text type="field-name" >会议组织者</md-text> | - 日程会议：会议组织者<br>- 即时会议（包含1v1通话）：会议/通话 发起人/会议室；<br>- 面试会议：面试官<br>- 开放平台的预约会议：会议预约者<br>**示例值**："xx" |
| <md-text type="field-name" >部门</md-text> | 部门名称<br>**示例值**："人事部" |
| <md-text type="field-name" >用户ID</md-text> | 内部员工ID<br>**示例值**："1a2b3c4d" |
| <md-text type="field-name" >工号</md-text> | 工号<br>**示例值**："12345670000" |
| <md-text type="field-name" >邮箱</md-text> | 邮箱<br>**示例值**："xx@email.com" |
| <md-text type="field-name" >手机</md-text> | 手机号<br>**示例值**："+86123****8910" |
| <md-text type="field-name" >会议开始时间</md-text> | 会议开始时间<br>**示例值**："2022.07.21 11:01:40 (GMT+08:00)" |
| <md-text type="field-name" >会议结束时间</md-text> | 会议结束时间<br>**示例值**："2022.07.21 12:01:40 (GMT+08:00)" |
| <md-text type="field-name" >会议持续时间</md-text> | 会议持续时间<br>**示例值**："01:00:00" |
| <md-text type="field-name" >参会人数量</md-text> | 参会人数量<br>**示例值**："2" |
| <md-text type="field-name" >音频</md-text> | 会中是否使用过麦克风/扬声器<br>**示例值**："是" |
| <md-text type="field-name" >视频</md-text> | 会中是否使用过摄像头<br>**示例值**："是" |
| <md-text type="field-name" >共享</md-text> | 会中是否使用过共享屏幕/magic-share<br>**示例值**："否" |
| <md-text type="field-name" >录制</md-text> | 会中是否开启过录制<br>**示例值**："是" |
| <md-text type="field-name" >电话</md-text> | 会中是否使用过pstn<br>**示例值**："否" |


##  资源：参会人明细

###  字段说明

| 名称 | 描述 |
| --- | --- |
| <md-text type="field-name" >参会人</md-text> | 参会人名称<br>**示例值**："xx" |
| <md-text type="field-name" >部门</md-text> | 部门名称<br>**示例值**："人事部" |
| <md-text type="field-name" >用户ID</md-text> | 内部员工ID<br>**示例值**："1a2b3c4d" |
| <md-text type="field-name" >工号</md-text> | 工号<br>**示例值**："12345670000" |
| <md-text type="field-name" >电话</md-text> | 手机号<br>**示例值**："+86123****8910" |
| <md-text type="field-name" >邮箱</md-text> | 邮箱<br>**示例值**："xx@email.com" |
| <md-text type="field-name" >设备</md-text> | 设备类型<br>**示例值**："WINDOWS" |
| <md-text type="field-name" >客户端版本</md-text> | 应用版本<br>**示例值**："5.18.0" |
| <md-text type="field-name" >公网IP</md-text> | 公网IP地址<br>**示例值**："170.40.80.80" |
| <md-text type="field-name" >内网IP</md-text> | 内网IP地址<br>**示例值**："192.168.1.1" |
| <md-text type="field-name" >代理服务</md-text> | 参会人是否开启代理服务设置<br>**示例值**："否" |
| <md-text type="field-name" >位置</md-text> | 参会人所在地理位置<br>**示例值**："中国大陆" |
| <md-text type="field-name" >网络类型</md-text> | 包括LAN、wifi等<br>**示例值**："wifi" |
| <md-text type="field-name" >麦克风</md-text> | 麦克风设备名<br>**示例值**："与系统一致" |
| <md-text type="field-name" >扬声器</md-text> | 扬声器设备名<br>**示例值**："与系统一致" |
| <md-text type="field-name" >摄像头</md-text> | 摄像头设备名<br>**示例值**："Integrated Camera" |
| <md-text type="field-name" >音频</md-text> | 参会人是否使用过麦克风/扬声器<br>**示例值**："是" |
| <md-text type="field-name" >视频</md-text> | 参会人是否使用过摄像头<br>**示例值**："是" |
| <md-text type="field-name" >共享</md-text> | 参会人是否使用过共享屏幕/magic-share<br>**示例值**："否" |
| <md-text type="field-name" >入会时间</md-text> | 参会人首次入会时间<br>**示例值**："2022.07.21 11:04:52 (GMT+08:00)" |
| <md-text type="field-name" >离会时间</md-text> | 参会人最后一次离会时间<br>**示例值**："2022.07.21 12:04:52 (GMT+08:00)" |
| <md-text type="field-name" >参会时长</md-text> | 参会人在会议中的累计时长<br>**示例值**："00:01:58" |
| <md-text type="field-name" >离会原因</md-text> | 离会原因说明<br>**示例值**："主持人结束会议" |


##  资源：参会人会议质量数据

###  字段说明

| 名称 | 描述 |
| --- | --- |
| <md-text type="field-name" >时间</md-text> | 时间点，按分钟统计<br>**示例值**："2022.07.21 11:30:00 (GMT+08:00)" |
| <md-text type="field-name" >音频-码率（接收）</md-text> | 接收端的音频码率<br>**示例值**："50kbps" |
| <md-text type="field-name" >音频-延迟（接收）</md-text> | 接收端的音频延迟<br>**示例值**："5ms" |
| <md-text type="field-name" >音频-抖动（接收）</md-text> | 接收端的音频抖动<br>**示例值**："1ms" |
| <md-text type="field-name" >音频-丢包平均（接收）</md-text> | 接收端的音频丢包平均值<br>**示例值**："2%" |
| <md-text type="field-name" >音频-丢包最大（接收）</md-text> | 接收端的音频丢包最大值<br>**示例值**："5%" |
| <md-text type="field-name" >音频-码率（发送）</md-text> | 发送端的音频码率<br>**示例值**："50kbps" |
| <md-text type="field-name" >音频-延迟（发送）</md-text> | 发送端的音频延迟<br>**示例值**："5ms" |
| <md-text type="field-name" >音频-抖动（发送）</md-text> | 发送端的音频抖动<br>**示例值**："1ms" |
| <md-text type="field-name" >音频-丢包平均（发送）</md-text> | 发送端的音频丢包平均值<br>**示例值**："2%" |
| <md-text type="field-name" >音频-丢包最大（发送）</md-text> | 发送端的音频丢包最大值<br>**示例值**："5%" |
| <md-text type="field-name" >视频-码率（接收）</md-text> | 接收端的视频码率<br>**示例值**："50kbps" |
| <md-text type="field-name" >视频-延迟（接收）</md-text> | 接收端的视频延迟<br>**示例值**："5ms" |
| <md-text type="field-name" >视频-抖动（接收）</md-text> | 接收端的视频抖动<br>**示例值**："1ms" |
| <md-text type="field-name" >视频-丢包平均（接收）</md-text> | 接收端的视频丢包平均值<br>**示例值**："2%" |
| <md-text type="field-name" >视频-丢包最大（接收）</md-text> | 接收端的视频丢包最大值<br>**示例值**："5%" |
| <md-text type="field-name" >视频-码率（发送）</md-text> | 发送端的视频码率<br>**示例值**："50kbps" |
| <md-text type="field-name" >视频-延迟（发送）</md-text> | 发送端的视频延迟<br>**示例值**："5ms" |
| <md-text type="field-name" >视频-抖动（发送）</md-text> | 发送端的视频抖动<br>**示例值**："1ms" |
| <md-text type="field-name" >视频-丢包平均（发送）</md-text> | 发送端的视频丢包平均值<br>**示例值**："2%" |
| <md-text type="field-name" >视频-丢包最大（发送）</md-text> | 发送端的视频丢包最大值<br>**示例值**："5%" |
| <md-text type="field-name" >共享屏幕-码率（接收）</md-text> | 接收端的共享屏幕码率<br>**示例值**："50kbps" |
| <md-text type="field-name" >共享屏幕-延迟（接收）</md-text> | 接收端的共享屏幕延迟<br>**示例值**："5ms" |
| <md-text type="field-name" >共享屏幕-抖动（接收）</md-text> | 接收端的共享屏幕抖动<br>**示例值**："1ms" |
| <md-text type="field-name" >共享屏幕-丢包平均（接收）</md-text> | 接收端的共享屏幕丢包平均值<br>**示例值**："2%" |
| <md-text type="field-name" >共享屏幕-丢包最大（接收）</md-text> | 接收端的共享屏幕丢包最大值<br>**示例值**："5%" |
| <md-text type="field-name" >共享屏幕-码率（发送）</md-text> | 发送端的共享屏幕码率<br>**示例值**："50kbps" |
| <md-text type="field-name" >共享屏幕-延迟（发送）</md-text> | 发送端的共享屏幕延迟<br>**示例值**："5ms" |
| <md-text type="field-name" >共享屏幕-抖动（发送）</md-text> | 发送端的共享屏幕抖动<br>**示例值**："1ms" |
| <md-text type="field-name" >共享屏幕-丢包平均（发送）</md-text> | 发送端的共享屏幕丢包平均值<br>**示例值**："2%" |
| <md-text type="field-name" >共享屏幕-丢包最大（发送）</md-text> | 发送端的共享屏幕丢包最大值<br>**示例值**："5%" |
| <md-text type="field-name" >客户端最小 CPU 占用</md-text> | 客户端最小 CPU 占用<br>**示例值**："2%" |
| <md-text type="field-name" >客户端平均 CPU 占用</md-text> | 客户端平均 CPU 占用<br>**示例值**："5%" |
| <md-text type="field-name" >客户端最大 CPU 占用</md-text> | 客户端最大 CPU 占用<br>**示例值**："10%" |
| <md-text type="field-name" >系统最大 CPU 占用</md-text> | 系统最大 CPU 占用<br>**示例值**："20%" |

##  资源：会议室预定数据

###  字段说明

| 名称 | 描述 |
| --- | --- |
| <md-text type="field-name" >会议室名称</md-text> | 会议室名称<br>**示例值**："测试专用rooms" |
| <md-text type="field-name" >会议主题</md-text> | 会议主题<br>**示例值**："xx的视频会议" |
| <md-text type="field-name" >预定人</md-text> | 会议室预定人<br>**示例值**："xx" |
| <md-text type="field-name" >预定人所属部门</md-text> | 部门名称<br>**示例值**："人事部" |
| <md-text type="field-name" >邀请人数</md-text> | 邀请参加该会议的人数<br>**示例值**："10" |
| <md-text type="field-name" >接受人数</md-text> | 接受该会议邀请的人数<br>**示例值**："8" |
| <md-text type="field-name" >会议开始时间</md-text> | 会议开始时间<br>**示例值**："2022.07.21 11:01:40 (GMT+08:00)" |
| <md-text type="field-name" >会议结束时间</md-text> | 会议结束时间<br>**示例值**："2022.07.21 12:01:40 (GMT+08:00)" |
| <md-text type="field-name" >会议时长</md-text> | 会议时长<br>**示例值**："01:00:00" |
| <md-text type="field-name" >会议室预定状态</md-text> | 会议室预定状态<br>可选值：<br>- 审批中<br>- 预定成功<br>- 已释放（手动释放）<br>- 已释放（会议结束后自动释放）<br>- 已释放（未签到被释放）<br>**示例值**："预定成功" |
| <md-text type="field-name" >签到设备</md-text> | 签到设备<br>可选值：<br>- 签到板<br>- 签到二维码<br>- 控制器<br>- 触屏版<br>- 设备离线，自动签到<br>**示例值**："签到板" |
| <md-text type="field-name" >会议室签到状态</md-text> | 会议室签到状态<br>可选值：<br>- 无需签到（即时预定）<br>- 无需签到（未开启签到设置）<br>- 无需签到<br>- 超时未签到<br>- 签到时间未结束<br>- 无需签到（未安装签到设备）<br>- 已签到<br>**示例值**："无需签到" |
| <md-text type="field-name" >会议室签到时间</md-text> | 会议室签到时间<br>**示例值**："2022.07.04 21:00:00 (GMT+08:00)" |
| <md-text type="field-name" >释放人</md-text> | 释放人<br>**示例值**："xx" |
| <md-text type="field-name" >释放时间</md-text> | 释放时间<br>**示例值**："2022.07.04 21:00:00 (GMT+08:00)" |


