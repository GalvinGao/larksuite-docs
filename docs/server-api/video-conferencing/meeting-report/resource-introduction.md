---
document_id: '7085933590580592646'
directory_id: '7083761332315258886'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/meeting-report-overview
breadcrumb:
- Server API
- Video Conferencing
- Meeting report
- Resource introduction
document_type: GuideDocumentType
updated_at: 2023-08-02T02:55:21Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/meeting-report-overview
---

#  资源介绍
##  资源定义
会议报告用于记录一段时间内租户会议的使用情况，包括：[获取会议报告](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/get_daily)、[获取 Top 用户列表](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/get_top_user)。

##  字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >start_time</md-text> | <md-text type="field-type" >string</md-text> | 开始时间（unix时间，单位sec）<br>**示例值**："1608888867" |
| <md-text type="field-name" >end_time</md-text> | <md-text type="field-type" >string</md-text> | 结束时间（unix时间，单位sec）<br>**示例值**："1608888966" |
| <md-text type="field-name" >limit</md-text> | <md-text type="field-type" >int</md-text> | 取前多少位<br>**示例值**：10 |
| <md-text type="field-name" >order_by</md-text> | <md-text type="field-type" >int</md-text> | 排序依据（降序）<br>**示例值**：1<br>**可选值有**：<br>- `1`：会议数量<br>- `2`：会议时长 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >-</md-text> | <md-text type="field-type" >-</md-text> |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >meeting_report</md-text> | <md-text type="field-type" >report</md-text> | 会议报告 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >total_meeting_count</md-text> | <md-text type="field-type" >string</md-text> | 总会议数量 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >total_meeting_duration</md-text> | <md-text type="field-type" >string</md-text> | 总会议时长（单位sec） |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >total_participant_count</md-text> | <md-text type="field-type" >string</md-text> | 总参会人数 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >daily_report</md-text> | <md-text type="field-type" >report_meeting_daily[]</md-text> | 每日会议报告列表 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >date</md-text> | <md-text type="field-type" >string</md-text> | 日期（unix时间，单位sec） |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >meeting_count</md-text> | <md-text type="field-type" >string</md-text> | 会议数量 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >meeting_duration</md-text> | <md-text type="field-type" >string</md-text> | 会议时长（单位sec） |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >participant_count</md-text> | <md-text type="field-type" >string</md-text> | 参会人数 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >-</md-text> | <md-text type="field-type" >-</md-text> |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >top_user_report</md-text> | <md-text type="field-type" >report_top_user[]</md-text> | top用户列表 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 用户ID |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 用户名 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >user_type</md-text> | <md-text type="field-type" >int</md-text> | 用户类型<br>**可选值有**：<br>- `1`：lark用户<br>- `2`：rooms用户<br>- `3`：文档用户<br>- `4`：neo单品用户<br>- `5`：neo单品游客用户<br>- `6`：pstn用户<br>- `7`：sip用户 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >meeting_count</md-text> | <md-text type="field-type" >string</md-text> | 会议数量 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >meeting_duration</md-text> | <md-text type="field-type" >string</md-text> | 会议时长（单位sec） |

###  数据示例
```json
{
    "start_time":"1608888867",
    "end_time":"1608888966",
    "limit":10,
    "order_by":1,
    "data": {
        "meeting_report": {
            "total_meeting_count": "100",
            "total_meeting_duration": "300000",
            "total_participant_count": "20000",
            "daily_report": [
                {
                    "date": "1609113600",
                    "meeting_count": "100",
                    "meeting_duration": "147680",
                    "participant_count": "2000"
                }
            ]
        }
    }
    "data": {
        "top_user_report": [
            {
                "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
                "name": "name",
                "user_type": 1,
                "meeting_count": "100",
                "meeting_duration": "3000"
            }
        ]
    }
}
```
