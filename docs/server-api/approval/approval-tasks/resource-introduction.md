---
document_id: '7139727755097554949'
directory_id: '7072711453267181573'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/introduction
breadcrumb:
- Server API
- Approval
- Approval tasks
- Resource introduction
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:45Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/introduction
---

# 资源介绍
## 资源定义
审批任务依赖于审批节点存在，每一个审批节点可能包含有一或多个审批任务，每一个任务表明当前审批节点的审批人是谁。如果当前节点包含有不止一个审批人，则有多个审批任务，每一个审批任务对应不同的审批人。当某一节点有多个审批任务时，某些任务可能会因为其他任务状态的改变而改变，并不单独依赖于审批人的操作（例如一个或签节点对应多个审批任务，当其中一个任务通过之后，其他任务会自动变为已完成状态而不需要对应审批人进行操作）。每当审批实例流转到新的节点时创建该节点的任务。

## 字段说明
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: $$$approval.v4.instance.method.create.request.body.table.param-column.width$$$;">名称</md-dt-th>
      <md-dt-th style="width: $$$approval.v4.instance.method.create.request.body.table.type-column.width$$$;">类型</md-dt-th>
      <md-dt-th style="width: $$$approval.v4.instance.method.create.request.body.table.desc-column.width$$$;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	task id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批人的用户id，自动通过、自动拒绝 时为空
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >open_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批人 open id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	任务状态

**可选值有**：
<md-enum>
<md-enum-item key="PENDING" >审批中</md-enum-item>
<md-enum-item key="APPROVED" >同意</md-enum-item>
<md-enum-item key="REJECTED" >拒绝</md-enum-item>
<md-enum-item key="TRANSFERRED" >已转交</md-enum-item>
<md-enum-item key="DONE" >完成</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >node_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	task 所属节点 id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >node_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
		task 所属节点名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >custom_node_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
		task 所属节点自定义 id, 如果没设置自定义 id, 则不返回该字段
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批方式

**可选值有**：
<md-enum>
<md-enum-item key="AND" >会签</md-enum-item>
<md-enum-item key="OR" >或签</md-enum-item>
<md-enum-item key="AUTO_PASS" >自动通过</md-enum-item>
<md-enum-item key="AUTO_REJECT" >自动拒绝</md-enum-item>
<md-enum-item key="SEQUENTIAL" >按顺序</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >start_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	task 开始时间
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >end_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	task 完成时间, 未完成为 0
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>

:::

## 数据示例
```json
{
    "id": "1234",
    "user_id": "f7cb567e",
    "open_id": "ou_123457",
    "status": "PENDING",
    "node_id": "46e6d96cfa756980907209209ec03b64",
    "node_name": "开始",
    "custom_node_id": "manager",
    "type": "AND",
    "start_time": "1564590532967",
    "end_time": "0"
}
```
## 用户ID说明
了解user_id，open_id，union_id的区别和用途，参见教程 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)
