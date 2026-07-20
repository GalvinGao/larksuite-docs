---
document_id: '7199928167141425157'
directory_id: '7186908899420815365'
title: 审批实例状态变更
full_path: /ukTMukTMukTM/uIDO24iM4YjLygjN/event/common-event/approval-instance-event
breadcrumb:
- Server API
- Approval
- Approval Events
- Common event
- Approval instance status change
document_type: GuideDocumentType
updated_at: 2023-02-15T03:04:07Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/common-event/approval-instance-event
---

# 审批实例状态变更
审批实例状态变更事件，会在实例状态变更后，向开发者推送审批实例状态消息。
1. 用户创建审批后，推送【PENDING】状态
2. 任一审批人拒绝后，推送【REJECTED】状态
3. 流程中所有人同意后，推送【APPROVED】状态
4. 发起人撤回审批后，推送【CANCELED】状态
5. 审批定义被管理员删除后，推送【DELETED】状态
6. 发起人撤销已通过的审批，推送【REVERTED】状态

**审批实例状态消息**:
```json
{
     "ts": "1502199207.7171419",
     "uuid": "bc447199585340d1f3728d26b1c0297a", 
     "token": "41a9425ea7df4536a7623e38fa321bae",
     "type": "event_callback",
     "event": { 
         "app_id": "cli_xxx", 
         "tenant_key":"xxx", 
         "type": "approval_instance", 
         "approval_code": "7C468A54-8745-2245-9675-08B7C63E7A85",
         "instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71", 
         "status": "PENDING",
         "instance_operate_time": "1666079207003",
         "uuid":"6525bffb"
    }
}
```
**消息说明**:
| 字段         | 类型           | 描述        |
| --------- | --------------- | --------- |
|`type` | `string` | approval_instance<br>固定字段 |
|`approval_code` | `string` | 审批定义 Code 
|`instance_code` | `string` | 审批实例 Code|
|`status` | `string` | 实例状态<br>PENDING - 进行中<br>APPROVED - 已通过<br>REJECTED - 已拒绝<br>CANCELED -  已撤回<br>DELETED - 已删除<br>REVERTED - 已撤销|
|`instance_operate_time` | `string` | 事件发生时间 |
|`uuid` | `string` | 审批实例自定义唯一ID，接口创建审批时候传入。 |

