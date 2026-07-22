---
document_id: '7085933590580609030'
directory_id: '7081519902443831302'
title: 会议概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/meeting-overview
breadcrumb:
- Server API
- Video Conferencing
- Meeting management
- Meeting overview
document_type: GuideDocumentType
updated_at: 2022-04-13T04:01:14Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/meeting-overview
---

#  会议概述
##  资源定义
用户可以在会议中进行邀请参会成员、移除参会成员和设置主持人等操作。其中，方法包括：[获取会议详情](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/get)、[获取与会议号相关联的会议列表](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/list_by_no)、[邀请参会人](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/invite)、[移除参会人](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/kickout)、[设置主持人](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/set_host)、[结束会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/end)。事件包括：[会议开始](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/meeting_started)、[会议结束](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/meeting_ended)、[加入会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/join_meeting)、[离开会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/leave_meeting)、[录制开始](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/recording_started)、[录制停止](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/recording_ended)、[录制完成](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/recording_ready)、[屏幕共享开始](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/share_started)、[屏幕共享结束](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/share_ended)。

##  字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >-</md-text> | <md-text type="field-type" >-</md-text> |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >meeting</md-text> | <md-text type="field-type" >meeting</md-text> | 会议数据 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 会议ID（视频会议的唯一标识，视频会议开始后才会产生） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >topic</md-text> | <md-text type="field-type" >string</md-text> | 会议主题 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >url</md-text> | <md-text type="field-type" >string</md-text> | 会议链接（Lark用户可通过点击会议链接快捷入会） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >create_time</md-text> | <md-text type="field-type" >string</md-text> | 会议创建时间（unix时间，单位sec） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >start_time</md-text> | <md-text type="field-type" >string</md-text> | 会议开始时间（unix时间，单位sec） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >end_time</md-text> | <md-text type="field-type" >string</md-text> | 会议结束时间（unix时间，单位sec） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >host_user</md-text> | <md-text type="field-type" >meeting_user</md-text> | 主持人 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >user_type</md-text> | <md-text type="field-type" >int</md-text> | 用户类型<br>**可选值有**：<br>- `1`：lark用户<br>- `2`：rooms用户<br>- `3`：文档用户<br>- `4`：neo单品用户<br>- `5`：neo单品游客用户<br>- `6`：pstn用户<br>- `7`：sip用户 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >status</md-text> | <md-text type="field-type" >int</md-text> | 会议状态<br>**可选值有**：<br>- `1`：会议呼叫中<br>- `2`：会议进行中<br>- `3`：会议已结束 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >participant_count</md-text> | <md-text type="field-type" >string</md-text> | 参会人数 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >participants</md-text> | <md-text type="field-type" >meeting_participant[]</md-text> | 参会人列表 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 用户ID |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >user_type</md-text> | <md-text type="field-type" >int</md-text> | 用户类型<br>**可选值有**：<br>- `1`：lark用户<br>- `2`：rooms用户<br>- `3`：文档用户<br>- `4`：neo单品用户<br>- `5`：neo单品游客用户<br>- `6`：pstn用户<br>- `7`：sip用户 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >is_host</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为主持人 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >is_cohost</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为联席主持人 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >is_external</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为外部参会人 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >status</md-text> | <md-text type="field-type" >int</md-text> | 参会人状态<br>**可选值有**：<br>- `1`：呼叫中<br>- `2`：在会中<br>- `3`：正在响铃<br>- `4`：不在会中或已经离开会议 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >ability</md-text> | <md-text type="field-type" >meeting_ability</md-text> | 会中使用的能力 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" > use_video</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用视频 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >use_audio</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用音频 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >use_share_screen</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用共享屏幕 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >use_follow_screen</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用妙享（magic share） |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >use_recording</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用录制 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >use_pstn</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用PSTN |

###  数据示例
```json
{
    "data": {
        "meeting": {
            "id": "6911188411934433028",
            "topic": "my meeting",
            "url": "https://vc.larksuite.com/j/337736498",
            "create_time": "1608885566",
            "start_time": "1608883322",
            "end_time": "1608888867",
            "host_user": {
                "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
                "user_type": 1
            },
            "status": 2,
            "participant_count": "10",
            "participants": [
                {
                    "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
                    "user_type": 1,
                    "is_host": true,
                    "is_cohost": false,
                    "is_external": false,
                    "status": 2
                }
            ],
            "ability": {
                "use_video": true,
                "use_audio": true,
                "use_share_screen": true,
                "use_follow_screen": true,
                "use_recording": true,
                "use_pstn": true
            }
        }
    }
}
```
