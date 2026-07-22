---
document_id: '7085933590580690950'
directory_id: '7081519902443798534'
title: 录制概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/recording-overview
breadcrumb:
- Server API
- Video Conferencing
- Meeting record
- Recording overview
document_type: GuideDocumentType
updated_at: 2022-04-13T04:01:18Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/recording-overview
---

#  录制概述
##  资源定义
用户可以录制一场会议，在会议结束后获得会议录制文件链接，包括：[开始录制](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/start)、[停止录制](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/stop)、[获取录制文件](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/get)、[授权录制文件](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/set_permission)。

##  字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >meeting_id</md-text> | <md-text type="field-type" >string</md-text> | 会议ID（视频会议的唯一标识，视频会议开始后才会产生）<br>**示例值**："6911188411932033028" |
| <md-text type="field-name" >user_id_type</md-text> | <md-text type="field-type" >string</md-text> | 用户 ID 类型<br>**示例值**："open_id"<br>**可选值有**：<br>- `open_id`：用户的 open id<br>- `union_id`：用户的 union id<br>- `user_id`：用户的 user id<br>**默认值**：`open_id`<br>**当值为** `user_id` **，字段权限要求**：<br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |
| <md-text type="field-name" >timezone</md-text> | <md-text type="field-type" >int</md-text> | 录制文件时间显示使用的时区[-12,12]<br>**示例值**：8 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >-</md-text> | <md-text type="field-type" >-</md-text> |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >recording</md-text> | <md-text type="field-type" >meeting.recording</md-text> | 录制文件数据 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >url</md-text> | <md-text type="field-type" >string</md-text> | 录制文件URL |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >duration</md-text> | <md-text type="field-type" >string</md-text> | 录制总时长（单位msec） |
| <md-text type="field-name" >permission_objects</md-text> | <md-text type="field-type" >recording_permission_object[]</md-text> | 授权对象列表 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 授权对象ID<br>**示例值**：<br>"ou_3ec3f6a28a0d08c45d895276e8e5e19b" |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >type</md-text> | <md-text type="field-type" >int</md-text> | 授权对象类型<br>**示例值**：1<br>**可选值有**：<br>- `1`：用户授权<br>- `2`：群组授权<br>- `3`：租户内授权（id字段不填）<br>- `4`：公网授权（id字段不填） |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >permission</md-text> | <md-text type="field-type" >int</md-text> | 权限<br>**示例值**：1<br>**可选值有**：<br>- `1`：查看 |

###  数据示例
```json
{
    "meeting_id":"6911188411932033028",
    "user_id_type":"open_id",
    "timezone":8,
    "data":{
        "recording":{
            "url":"https://meetings.larksuite.com/minutes/obcn37dxcftoc3656rgyejm7",
            "duration":"30000",
        }
    },
    "permission_objects":[
    {
        "id":"ou_3ec3f6a28a0d08c45d895276e8e5e19b",
        "type":1,
        "permission":1
    }
    ]
}
```
