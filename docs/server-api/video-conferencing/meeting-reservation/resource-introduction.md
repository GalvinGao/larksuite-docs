---
document_id: '7085933590580756486'
directory_id: '7081519902443864070'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/schedule-meeting-overview
breadcrumb:
- Server API
- Video Conferencing
- Meeting reservation
- Resource introduction
document_type: GuideDocumentType
updated_at: 2023-08-23T10:41:54Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/schedule-meeting-overview
---

#  资源介绍
##  资源定义
用户可以预约会议（创建会议预约），提前设置参会成员和会议权限，并获取会议信息，包括：[预约会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/apply)、[更新预约会议信息](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/update)、[删除预约会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/delete)、[获取预约会议详情](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get)、[获取正在进行的会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get_active_meeting)。

##  字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >reserve_id</md-text> | <md-text type="field-type" >string</md-text> | 预约ID（预约的唯一标识）<br>**示例值**："6911188411932033028" |
| <md-text type="field-name" >user_id_type</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID 类型<br>**示例值**："open_id"<br>**可选值有**：<br>- `open_id`：用户的 open id<br>- `union_id`：用户的 union id<br>- `user_id`：用户的 user id<br>**默认值**：`open_id`<br>**当值为** `user_id`**，字段权限要求**：<br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |
| <md-text type="field-name" >end_time</md-text> | <md-text type="field-type" >string</md-text> | 预约到期时间（unix时间，单位sec），多人会议必填<br>**示例值**："1608888867" |
| <md-text type="field-name" >meeting_settings</md-text> | <md-text type="field-type" >reserve_meeting_setting</md-text> | 会议设置 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >topic</md-text> | <md-text type="field-type" >string</md-text> | 会议主题<br>**示例值**："my meeting" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >action_permissions</md-text> | <md-text type="field-type" >reserve_action_permission[]</md-text> | 会议权限配置列表，如果存在相同的权限配置项则它们之间为"逻辑或"的关系（即 有一个为true则拥有该权限） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >permission</md-text> | <md-text type="field-type" >int</md-text> | 权限项<br>**示例值**：1<br>**可选值有**：<br>- `1`：是否能成为主持人<br>- `2`：是否能邀请参会人<br>- `3`：是否能加入会议 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >permission_checkers</md-text> | <md-text type="field-type" >reserve_permission_checker[]</md-text> | 权限检查器列表，权限检查器之间为"逻辑或"的关系（即 有一个为true则拥有该权限） |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >check_field</md-text> | <md-text type="field-type" >int</md-text> | 检查字段类型<br>**示例值**：1<br>**可选值有**：<br>- `1`：用户ID<br>- `2`：用户类型<br>- `3`：租户ID |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >check_mode</md-text> | <md-text type="field-type" >int</md-text> | 检查方式<br>**示例值**：1<br>**可选值有**：<br>- `1`：在check_list中为有权限（白名单）<br>- `2`：不在check_list中为有权限（黑名单） |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >check_list</md-text> | <md-text type="field-type" >string[]</md-text> | 检查字段列表<br>**示例值**：123 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >meeting_initial_type</md-text> | <md-text type="field-type" >int</md-text> | 会议初始类型<br>**示例值**：1<br>**可选值有**：<br>- `1`：多人会议<br>- `2`：1v1呼叫 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >auto_record</md-text> | <md-text type="field-type" >boolean</md-text> | 使用Lark视频会议时，是否开启自动录制，默认false<br>**示例值**：true |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >assign_host_list</md-text> | <md-text type="field-type" >assign_host_list[]</md-text> | 指定主持人列表 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<br><md-text type="field-name" >user_type</md-text> | <md-text type="field-type" >int</md-text> | 用户类型，仅支持设置同租户下的 Lark 用户<br>**示例值**：1<br>**可选值有**：<br>- `1`：Lark用户 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<br><md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 用户ID<br>**示例值**："ou_3ec3f6a28a0d08c45d895276e8e5e19b" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >call_setting</md-text> | <md-text type="field-type" >reserve_call_setting</md-text> | 1v1呼叫相关参数 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >callee</md-text> | <md-text type="field-type" >reserve_callee</md-text> | 被呼叫的用户 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 用户ID<br>**示例值**："ou_3ec3f6a28a0d08c45d895276e8e5e19b" |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >user_type</md-text> | <md-text type="field-type" >int</md-text> | 用户类型，当前仅支持用户类型6(pstn用户)<br>**示例值**：1<br>**可选值有**：<br>- `1`：lark用户<br>- `2`：rooms用户<br>- `3`：文档用户<br>- `4`：neo单品用户<br>- `5`：neo单品游客用户<br>- `6`：pstn用户<br>- `7`：sip用户 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >pstn_sip_info</md-text> | <md-text type="field-type" >pstn_sip_info</md-text> | pstn/sip信息 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >nickname</md-text> | <md-text type="field-type" >string</md-text> | 给pstn/sip用户设置的临时昵称<br>**示例值**："dodo" |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >main_address</md-text> | <md-text type="field-type" >string</md-text> | pstn/sip主机号，格式为：[国际冠字]-[电话区号][电话号码]，当前仅支持国内手机及固定电话号码<br>**示例值**："+86-02187654321" |
| <md-text type="field-name" >with_participants</md-text> | <md-text type="field-type" >boolean</md-text> | <md-text type="field-type" >是否需要参会人列表，默认为false<br>**示例值**：false</md-text> |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >-</md-text> | <md-text type="field-type" >-</md-text> |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >meeting</md-text> | <md-text type="field-type" >meeting</md-text> | 会议数据 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 会议ID（视频会议的唯一标识，视频会议开始后才会产生） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >topic</md-text> | <md-text type="field-type" >string</md-text> | 会议主题 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >url</md-text> | <md-text type="field-type" >string</md-text> | 会议链接（Lark用户可通过点击会议链接快捷入会） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >meeting_no</md-text> | <md-text type="field-type" >string</md-text> | 会议号 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >create_time</md-text> | <md-text type="field-type" >string</md-text> | 会议创建时间（unix时间，单位sec） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >start_time</md-text> | <md-text type="field-type" >string</md-text> | 会议开始时间（unix时间，单位sec） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >end_time</md-text> | <md-text type="field-type" >string</md-text> | 会议结束时间（unix时间，单位sec） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >host_user</md-text> | <md-text type="field-type" >meeting_user</md-text> | 主持人 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >user_type</md-text> | <md-text type="field-type" >int</md-text> | 用户类型<br>**可选值有**：<br>- `1`：lark用户<br>- `2`：rooms用户<br>- `3`：文档用户<br>- `4`：neo单品用户<br>- `5`：neo单品游客用户<br>- `6`：pstn用户<br>- `7`：sip用户 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >status</md-text> | <md-text type="field-type" >int</md-text> | 会议状态<br>**可选值有**：<br>- `1`：会议呼叫中<br>- `2`：会议进行中<br>- `3`：会议已结束 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >participant_count</md-text> | <md-text type="field-type" >string</md-text> | 参会人数 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >participant_count_accumulated</md-text> | <md-text type="field-type" >string</md-text> | 累计参会人数 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >participants</md-text> | <md-text type="field-type" >meeting_participant[]</md-text> | 参会人列表 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 用户ID |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >first_join_time</md-text> | <md-text type="field-type" >string</md-text> | 首次入会时间，秒级Unix时间戳 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >final_leave_time</md-text> | <md-text type="field-type" >string</md-text> | 最终离会时间，秒级Unix时间戳 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" > in_meeting_duration</md-text> | <md-text type="field-type" >string</md-text> | 累计在会中时间，时间单位：秒 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >user_type</md-text> | <md-text type="field-type" >int</md-text> | 用户类型<br>**可选值有**：<br>- `1`：lark用户<br>- `2`：rooms用户<br>- `3`：文档用户<br>- `4`：neo单品用户<br>- `5`：neo单品游客用户<br>- `6`：pstn用户<br>- `7`：sip用户 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >is_host</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为主持人 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >is_cohost</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为联席主持人 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >is_external</md-text> | <md-text type="field-type" >boolean</md-text> | 是否为外部参会人 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >status</md-text> | <md-text type="field-type" >int</md-text> | 参会人状态<br>**可选值有**：<br>- `1`：呼叫中<br>- `2`：在会中<br>- `3`：正在响铃<br>- `4`：不在会中或已经离开会议 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ability</md-text> | <md-text type="field-type" >meeting_ability</md-text> | 会中使用的能力 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" > use_video</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用视频 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >use_audio</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用音频 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >use_share_screen</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用共享屏幕 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >use_follow_screen</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用妙享（magic share） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >use_recording</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用录制 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >use_pstn</md-text> | <md-text type="field-type" >boolean</md-text> | 是否使用PSTN |

###  数据示例
```json
{
    "reserve_id":"6911188411932033028",
    "user_id_type":"open_id",
    "end_time":"1608888867",
    "meeting_settings":{
        "topic":"my meeting",
        "action_permissions":[
            {
                "permission":1,
                "permission_checkers":[
                    {
                        "check_field":1,
                        "check_mode":1,
                        "check_list": [
                            "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
                        ]
                    }
                ]
            }
        ],
         "meeting_initial_type":1,
         "auto_record":true,
         "call_setting":
         {
         "callee":
             {
                 "id":"ou_3ec3f6a28a0d08c45d895276e8e5e19b",
                 "user_type":1,
                 "pstn_sip_info":{
                     "nickname":"dodo",
                     "main_address":"+86-02187654321"
                 }
             }
         }
    },
    "with_participants":false,
    "data":{
        "meeting":{
            "id": "6911188411934433028",
            "topic": "my meeting",
            "url":"https://vc.larksuite.com/j/337736498",
            "meeting_no": "235812466",
            "create_time":"1608885566",
            "start_time": "1608883322",
            "end_time": "1608883899",
            "host_user": {
                "id":"ou_3ec3f6a28a0d08c45d895276e8e5e19b",
                "user_type": 1
            },
        },
        "status": 2,
            "participant_count": "10",
            "participant_count_accumulated":"15",
            "participants": [
                {
                    "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
                    "first_join_time":"1624438144",
                    "final_leave_time":"1624438144",
                    "in_meeting_duration":"123",
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
```
## 会议 ID 说明
了解会议号和会议 ID 的区别和用途，以及获取方法。
- 什么是会议 ID？

会议 ID 即 meetingID 是一个会议的唯一标识，用户在客户端看到的 9 位会议号不是 meetingID，使用会议相关的 API 需要以 meetingID 作为唯一标识来进行操作。
- 如何获取会议ID？

通过 [预约会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/apply) API 预约的会议，会议开始后可通过 [获取活跃会议](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get_active_meeting) API 进行获取，或监听 [会议相关的事件](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/events/meeting_started) 也可获取。
