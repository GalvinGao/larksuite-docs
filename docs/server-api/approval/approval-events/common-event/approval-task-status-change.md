---
document_id: '7199928167141408773'
directory_id: '7186908899420815365'
title: 审批任务状态变更
full_path: /ukTMukTMukTM/uIDO24iM4YjLygjN/event/common-event/approval-task-event
breadcrumb:
- Server API
- Approval
- Approval Events
- Common event
- Approval task status change
document_type: GuideDocumentType
updated_at: 2023-02-15T03:04:07Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/common-event/approval-task-event
---

# 审批任务状态变更
审批人同意/拒绝/转交/退回 审批任务后，会向开发者推送审批任务状态消息。
1. 用户创建审批后，推送第一个审批节点的审批任务【PENDING】状态
2. 如果当前节点是会签(AND)节点
   - 	任一审批任务被同意，推送该任务的【APPROVED】状态
   - 	任一审批任务被拒绝，推送该任务的【REJECTED】状态，当前节点其他剩余任务的【DONE】状态
3. 如果当前节点是或签(OR)节点
   -    任一审批任务被同意，推送该任务的【APPROVED】状态，当前节点其他剩余任务的【DONE】状态，下一个节点所有任务的【PENDING】状态
   -    任一审批任务被拒绝，推送该任务的【REJECTED】状态，当前节点其他所有任务的【DONE】状态
4. 如果用户对审批任务进行转交，推送该任务的【TRANSFERRED】状态，和被转交人任务的【PENDING】状态
5. 发起人撤回审批后，推送剩余所有任务的【DONE】状态
6. 审批定义被管理员删除后，推送剩余所有任务的【DONE】状态
7. 如果用户对审批任务进行退回，推送该任务的【ROLLBACK】状态，和被退回人任务的【PENDING】状态

**审批任务状态消息**:
```json
{    
     "ts": "1502199207.7171419",
     "uuid": "bc447199585340d1f3728d26b1c0297a",
     "token": "41a9425ea7df4536a7623e38fa321bae",
     "type": "event_callback",
     "event": { 
         "app_id": "cli_xxx", 
         "open_id": "ou_123456",
         "tenant_key":"xxx", 
         "type": "approval_task", 
         "approval_code": "7C468A54-8745-2245-9675-08B7C63E7A85",
         "instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71", 
         "task_id": "12345",
         "user_id": "b613t51g",
         "status": "PENDING", 
         "operate_time": "1502199207000",
         "custom_key": "xxx",
         "def_key": "xxx",
         "extra":"{\"rollback_node_ids\":[\"nodeid\"],\"rollback_custom_node_ids\":[\"customnodeid\"]}"
    }
}
```
**消息说明**:
| 字段         | 类型           | 描述        |
| --------- | --------------- | --------- |
|`type` | `string` | approval_task<br>固定字段 |
|`approval_code` | `string` | 审批定义 Code 
|`instance_code` | `string` | 审批实例 Code|
|`task_id` | `string` | 审批任务 ID|
|`user_id` | `string` | 操作人 ID（当 task 为自动通过类型时，user_id 为空）|
|`status` | `string` | 任务状态<br>REVERTED - 已还原<br>PENDING - 进行中<br>APPROVED - 已通过<br>REJECTED - 已拒绝<br>TRANSFERRED - 已转交<br>ROLLBACK - 已退回<br>DONE - 已完成|
|`operate_time` | `string` | 事件发生时间 |
|`extra` | `string` | 扩展数据, 当前只有退回事件才有此字段，rollback_node_ids退回的节点列表，rollback_custom_node_ids用户自定义配置的节点列表|
|`custom_key` | `string` | 节点自定义ID |
|`def_key` | `string` | 节点系统生成唯一ID |


