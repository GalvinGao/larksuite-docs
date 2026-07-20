---
document_id: '7348410643215761413'
directory_id: '7073460768595230725'
title: 打开日程创建页
full_path: /uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-calender/open-the-schedule-creation-page
breadcrumb:
- Developer Guides
- AppLink Protocol
- Supported protocol
- Open calender
- Open the schedule creation page
document_type: GuideDocumentType
updated_at: 2024-03-26T03:09:34Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-calender/open-the-schedule-creation-page
---

# 打开日程创建页
::: note 
从Lark 3.40.0 版本开始支持。
:::

## 使用场景
跳转日历 tab 并进入日程创建页面，用户可新建日程。

## 协议
[https://applink.larksuite.com/client/calendar/event/create](https://applink.larksuite.com/client/calendar/event/create)

##  参数
| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|startTime|否         |开始日期，{unixTime}格式|
|endTime|否         |结束日期，{unixTime}格式|
|summary|否         |日程主题，中文可使用encodeURIComponent方法生成|
`startTime`和`endTime`参数说明：

1.  若`startTime`和`endTime`均有效，则无需验证两者大小，若`startTime` > `endTime`，则APPLink正常跳转，但会提示错误。
2.  若`startTime`无效，则`endTime`作废，创建以`now`为开始时间、以`now + defaultDuration`为结束时间的日程。`defaultDuration`为日历默认时长，见日历「设置」。
3.  若`startTime`有效、`endTime`无效，则创建以`startTime`为开始时间、以`startTime + defaultDuration`为结束时间的日程。`defaultDuration`为日历默认时长，见日历「设置」。

## 使用示例
#### 1. 打开日程创建页并设置日程开始时间

[https://applink.larksuite.com/client/calendar/event/create?startTime=1581950880](https://applink.larksuite.com/client/calendar/event/create?startTime=1581950880)

#### 2. 打开日程创建页并设置日程开始时间和结束时间

[https://applink.larksuite.com/client/calendar/event/create?startTime=1581950880&endTime=1581951000](https://applink.larksuite.com/client/calendar/event/create?startTime=1581950880&endTime=1581951000)

#### 3. 打开日程创建页并设置日程开始时间和日程主题
[https://applink.larksuite.com/client/calendar/event/create?startTime=1581950880&summary=%E6%88%91%E7%9A%84%E6%96%B0%E6%97%A5%E7%A8%8B](https://applink.larksuite.com/client/calendar/event/create?startTime=1581950880&summary=%E6%88%91%E7%9A%84%E6%96%B0%E6%97%A5%E7%A8%8B)


