---
document_id: '7295682588634923013'
directory_id: '7294197569669185541'
title: 更多管理日程的开放能力
full_path: /home/quickly-build-a-feishu-calendar-schedule/more-schedule-api
breadcrumb:
- Home
- Quickly Build Calendar Events
- More open capabilities for managing schedules
document_type: GuideDocumentType
updated_at: 2023-10-31T08:33:02Z
source_url: https://open.larksuite.com/document/home/quickly-build-a-feishu-calendar-schedule/more-schedule-api
---

# 更多管理日程的开放能力

- 对于已创建好的日程，你可以：
    
    - 调用[更新日程](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/patch)接口，更新日程起止时间、公开范围、参与人权限等配置。
    
    - 调用[删除日程](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/delete)接口，删除不再进行的日程。
    
    - 调用[添加日程参与人](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee/create)接口，继续为日程添加用户、群组、会议室或者邮箱。
    
    - 调用[删除日程参与人](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee/batch_delete)接口，删除不再参与日程的用户、群组、会议室或者邮箱。

- 对于已开启第三方会议室同步的会议室，日程预定结果会由用户的会议室管理系统决定。因此，在预定日程后还需要调用[回复会议室日程实例](/document/ukTMukTMukTM/uYzN4UjL2cDO14iN3gTN)接口，确认此会议室预订成功。

- 如果你的企业针对多个三方系统创建了多个Lark日历，则可以调用[获取日程列表](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/list)接口，基于不同的日历 ID 管理不同系统对应的日程。
