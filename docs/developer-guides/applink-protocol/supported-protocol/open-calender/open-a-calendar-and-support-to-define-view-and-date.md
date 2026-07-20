---
document_id: '7348410643215745029'
directory_id: '7073460768595230725'
title: 打开日历（支持定义视图和日期）
full_path: /uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-calender/open-a-calendar-and-support-to-define-view-and-date
breadcrumb:
- Developer Guides
- AppLink Protocol
- Supported protocol
- Open calender
- Open a calendar and support to define view and date
document_type: GuideDocumentType
updated_at: 2024-03-26T03:09:34Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-calender/open-a-calendar-and-support-to-define-view-and-date
---

# 打开日历（支持定义视图和日期）
::: note 
从Lark 3.40.0 版本开始支持。
:::

## 使用场景
打开日历tab，并支持定义跳转到具体视图和具体日期。

## 协议
[https://applink.larksuite.com/client/calendar/view](https://applink.larksuite.com/client/calendar/view)

##  参数


| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|type | 否         | 视图类型，枚举值包括：<br> `day`：日视图 <br>`three_day`：三日视图，仅移动端支持<br> `week`：周视图，仅PC端支持 <br>  `month`：月视图<br> `meeting`：会议室视图，仅PC端支持 <br> `list`：列表视图，仅移动端支持| 
|date|否         |日期，{unixTime}格式|


## 使用示例
#### 1. 以“周视图”打开日历

[https://applink.larksuite.com/client/calendar/view?type=week](https://applink.larksuite.com/client/calendar/view?type=week)
#### 2. 以“会议室视图”打开日历

[https://applink.larksuite.com/client/calendar/view?type=meeting](https://applink.larksuite.com/client/calendar/view?type=meeting)

#### 3. 打开某日期日历并显示“日视图”

[https://applink.larksuite.com/client/calendar/view?date=1581999948&type=day](https://applink.larksuite.com/client/calendar/view?date=1581999948&type=day)

