---
document_id: '7199928167141359621'
directory_id: '7186908899420815365'
title: 审批抄送状态变更
full_path: /ukTMukTMukTM/uIDO24iM4YjLygjN/event/common-event/approval-cc-event
breadcrumb:
- Server API
- Approval
- Approval Events
- Common event
- Approval cc status change
document_type: GuideDocumentType
updated_at: 2023-02-15T03:04:07Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/common-event/approval-cc-event
---

# 审批抄送状态变更
创建抄送或者被抄送人已读抄送后，会向开发者推送消息。
1. 创建抄送，推送 "operate" 为 "CREATE" 的事件。
1. 被抄送人已读抄送，推送 "operate" 为 "READ" 的事件。

**审批抄送消息**:
```json
{    
     "ts": "1502199207.7171419",
     "uuid": "bc447199585340d1f3728d26b1c0297a",
     "token": "41a9425ea7df4536a7623e38fa321bae",
     "type": "event_callback",
     "event": { 
         "app_id": "cli_xxx", 
         "tenant_key":"xxx", 
         "type": "approval_cc", 
         "approval_code": "7C468A54-8745-2245-9675-08B7C63E7A85",
         "instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71", 
         "id": "12345",
         "user_id": "b613t51g",
         "create_time": 1502199207000, 
         "operate": "CREATE",
         "from": "b613t51g"
    }
}
```
**消息说明**:
| 字段         | 类型           | 描述        |
| --------- | --------------- | --------- |
|`type` | `string` | approval_cc<br>固定字段 |
|`approval_code` | `string` | 审批定义 Code |
|`instance_code` | `string` | 审批实例 Code|
|`id` | `string` |抄送 ID|
|`user_id` | `string` | 被抄送人|
|`create_time` | `long` | 抄送时间|
|`operate` | `string` | 操作类型<br> CREATE: 抄送<br> READ: 已读 |
|`from` | `string` | 抄送人, 可能为空|
