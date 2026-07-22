---
document_id: '7245184492770459654'
directory_id: '7244811844195172358'
title: 会议室配置概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_config/rooms-configuration-overview
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Rooms
- Rooms configuration
- Rooms configuration overview
document_type: GuideDocumentType
updated_at: 2023-06-16T07:36:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_config/rooms-configuration-overview
---

#  会议室配置概述
##  资源定义
会议室配置用于对Lark会议室的背景设置、资源管理等进行配置，包括：[查询会议室配置](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_config/query)、[设置会议室配置](/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_config/set)。

##  字段说明

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >scope</md-text> | <md-text type="field-type" >int</md-text> | 查询节点范围<br>**示例值**："5"<br>**可选值有**：<br>- `1`：租户<br>- `2`：国家/地区<br>- `3`：城市<br>- `4`：建筑<br>- `5`：楼层<br>- `6`：会议室 |
| <md-text type="field-name" >country_id</md-text> | <md-text type="field-type" >string</md-text> | 国家/地区ID scope为2，3时需要此参数<br>**示例值**："086" |
| <md-text type="field-name" >district_id</md-text> | <md-text type="field-type" >string</md-text> | 城市ID scope为3时需要此参数<br>**示例值**："001" |
| <md-text type="field-name" >building_id</md-text> | <md-text type="field-type" >string</md-text> | 建筑ID scope为4，5时需要此参数<br>**示例值**："22" |
| <md-text type="field-name" >floor_name</md-text> | <md-text type="field-type" >string</md-text> | 楼层 scope为5时需要此参数<br>**示例值**："4" |
| <md-text type="field-name" >room_id</md-text> | <md-text type="field-type" >string</md-text> | 会议室ID scope为6时需要此参数<br>**示例值**："6383786266263" |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >room_config</md-text> | <md-text type="field-type" >-</md-text> |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >room_background</md-text> | <md-text type="field-type" >string</md-text> | Lark会议室背景图 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >display_background</md-text> | <md-text type="field-type" >string</md-text> | Lark签到板背景图 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >digital_signage</md-text> | <md-text type="field-type" >room_digital_signage</md-text> | Lark会议室数字标牌 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >enable</md-text> | <md-text type="field-type" >boolean</md-text> | 是否开启数字标牌功能 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >mute</md-text> | <md-text type="field-type" >boolean</md-text> | 是否静音播放 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >start_display</md-text> | <md-text type="field-type" >int</md-text> | 日程会议开始前n分钟结束播放 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >stop_display</md-text> | <md-text type="field-type" >int</md-text> | 会议结束后n分钟开始播放 |
| &emsp;<span style="color: #8F959E"> ∟</span>&nbsp;<md-text type="field-name" >materials</md-text> | <md-text type="field-type" >room_digital_signage_material[]</md-text> | 素材列表 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >id</md-text> | <md-text type="field-type" >string</md-text> | 素材ID |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 素材名称 |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >material_type</md-text> | <md-text type="field-type" >int</md-text> | 素材类型<br>**可选值有**：<br>- `1`：图片<br>- `2`：视频<br>- `3`：GIF |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >url</md-text> | <md-text type="field-type" >string</md-text> | 素材url |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >duration</md-text> | <md-text type="field-type" >int</md-text> | 播放时长（单位sec） |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >cover</md-text> | <md-text type="field-type" >string</md-text> | 素材封面url |
| &emsp;<span style="color: #8F959E">  ∟</span>&nbsp;<md-text type="field-name" >md5</md-text> | <md-text type="field-type" >string</md-text> | 素材文件md5 |

###  数据示例
```json
{
    "scope":5,
    "country_id":"086",
    "district_id":"001",
    "building_id":"22",
    "floor_name":"4",
    "room_id":"6383786266263",
    "code":0,
    "msg":"success",
    "data": {
        "room_background": "https://lf1-ttcdn-tos.pstatp.com/obj/xxx",
        "display_background": "https://lf1-ttcdn-tos.pstatp.com/obj/xxx",
        "digital_signage": {
            "enable": true,
            "mute": true,
            "start_display": 3,
            "stop_display": 3,
            "materials": [
                {
                    "id": "7847784676276",
                    "name": "name",
                    "material_type": 0,
                    "url": "url",
                    "duration": 15,
                    "cover": "url",
                    "md5": "md5"
                }
            ]
        }
    }
}
```
