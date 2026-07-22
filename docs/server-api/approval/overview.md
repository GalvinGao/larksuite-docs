---
document_id: '7139727755097653253'
directory_id: '7072711453267165189'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval-overview
breadcrumb:
- Server API
- Approval
- Overview
document_type: GuideDocumentType
updated_at: 2024-01-30T13:34:34Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval-overview
---

# 概述
## 业务介绍
Lark审批通过提供一站式、高效率的审批解决方案，帮助企业解决各种审批难题，轻松解锁高效愉悦的审批人体验。Lark审批可以快速建立企业内部审批流程，如请假、出差等。审批开放接口可以对审批实例进行查询和创建，可用于企业原有平台与审批打通。我们提供了一系列安全、可靠的 API，来方便你对审批信息进行操作，通过审批 API，你可以实现多种功能，例如：
- Lark原生审批接入
- 三方审批系统接入
- Lark应用接入审批



### 接入流程
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 10% ">编号</md-dt-th>
      <md-dt-th style="width: 30%">步骤</md-dt-th>
      <md-dt-th style="width: 60%">介绍</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>
    
<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >1</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >创建一个应用</md-text>
	</md-dt-td>

	<md-dt-td>
	如需创建企业自建应用，可参考 [自建应用的开发流程](/document/home/introduction-to-custom-app-development/self-built-application-development-process)如需创建应用商店应用，可参考 [开发和上架应用商店应用](/document/uMzNwEjLzcDMx4yM3ATM/uYzNwEjL2cDMx4iN3ATM)

	</md-dt-td>
</md-dt-tr>
    
<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >2</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >调用API，对审批进行操作</md-text>
	</md-dt-td>

	<md-dt-td>
	调用API前，你需要先获取访问凭证并开启对应的权限，详情参见 [如何调用服务端API](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)。 

	</md-dt-td>
</md-dt-tr>
    
<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >3</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >监听事件，获知审批状态的变化</md-text>
	</md-dt-td>

	<md-dt-td>
	监听事件前，你需要先申请相应的权限，[审批事件监听开发指南](/document/ukTMukTMukTM/ugDNyUjL4QjM14CO0ITN)

	</md-dt-td>
</md-dt-tr>
  </md-dt-tbody>
</md-dt-table>
:::


### 开发教程
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 50% ">教程名称</md-dt-th>
      <md-dt-th style="width: 50%">教程步骤拆解</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>
    
<md-dt-tr level="0">
	<md-dt-td>
	[快速开发三方审批](/document/home/quickly-develop-three-party-approvals/introduction)
	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7e3dbfdaaeac897ce0949076621a69c0_QqSOUFSBky.jpeg?height=600&lazyload=true&width=1128)
	</md-dt-td>
	<md-dt-td>
	教程步骤拆解
1. [简介](/document/home/quickly-develop-three-party-approvals/introduction)
2. [准备工作](/document/home/quickly-develop-three-party-approvals/prep-work)
3. [创建应用和申请权限](/document/home/quickly-develop-three-party-approvals/creating-applications-and-requesting-permissions)
4. [获取访问凭证](/document/home/quickly-develop-three-party-approvals/get-access-token)
5. [创建和更新三方审批定义](/document/home/quickly-develop-three-party-approvals/create-and-update-three-party-approval-definitions)
6. [三方审批实例同步](/document/home/quickly-develop-three-party-approvals/three-party-approval-instance-synchronization)
7. [发送与更新审批bot消息](/document/home/quickly-develop-three-party-approvals/send-and-update-approval-bot-messages)
8. [三方快捷审批](/document/home/quickly-develop-three-party-approvals/three-party-expedited-approval)
9. [三方审批实例校验](/document/home/quickly-develop-three-party-approvals/three-party-approval-example-verification)
	</md-dt-td>
 
</md-dt-tr>
    
    
      </md-dt-tbody>
</md-dt-table>
:::
    
## 资源介绍
资源的定义如下：
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 20% ">资源</md-dt-th>
      <md-dt-th style="width: 70%">资源定义</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>
    
<md-dt-tr level="0">
	<md-dt-td>
	[审批定义](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/overview-of-approval-resources)
	</md-dt-td>
	<md-dt-td>
	单个审批流，由表单和审批流程组成，创建后可以让员工在发起审批时填写各个控件的值并形成[审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/overview-approval-instance)。
	</md-dt-td>
 
</md-dt-tr>
    
    <md-dt-tr level="0">
	<md-dt-td>
	[审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/overview-approval-instance)
	</md-dt-td>
	<md-dt-td>
	员工发起审批时产生的审批流。包括多个[审批任务](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/introduction)
	</md-dt-td>
 
</md-dt-tr>
    
    <md-dt-tr level="0">
	<md-dt-td>
	[审批任务](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/introduction)
	</md-dt-td>
	<md-dt-td>
	审批任务依赖于审批节点存在，每一个审批节点可能包含有一或多个审批任务，每一个任务表明当前审批节点的审批人是谁
	</md-dt-td>
 
</md-dt-tr>
<md-dt-tr level="0">
	<md-dt-td>
	[审批评论](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/overview)
	</md-dt-td>
	<md-dt-td>
	员工在审批实例中进行的评论或评论回复。
	</md-dt-td>
 
</md-dt-tr>
    <md-dt-tr level="0">
	<md-dt-td>
	[文件](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/file/overview)
	</md-dt-td>
	<md-dt-td>
	当审批表单中有图片或附件控件时，开发者需在创建审批实例前通过审批上传文件接口将文件上传到审批系统。
	</md-dt-td>
 
</md-dt-tr>
    
    <md-dt-tr level="0">
	<md-dt-td>
[三方审批定义](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/overview)
	</md-dt-td>
	<md-dt-td>
	三方审批定义是审批的描述，包括审批名称、图标、描述、分组等基础信息。三方将根据三方审批定义来创建和同步[三方审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/overview)

	</md-dt-td>
 
</md-dt-tr>
    
    <md-dt-tr level="0">
	<md-dt-td>
[三方审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/overview)
	</md-dt-td>
	<md-dt-td>
	员工发起审批时产生的审批流。包括多个审批任务、审批抄送等信息
	</md-dt-td>
 
</md-dt-tr>
    
    <md-dt-tr level="0">
	<md-dt-td>
[三方审批任务](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_task/overview)
	</md-dt-td>
	<md-dt-td>
	审批人每一个审批的操作对应着一个审批任务
	</md-dt-td>
 
</md-dt-tr>

      </md-dt-tbody>
</md-dt-table>
:::

以下将详细介绍每个资源的字段、方法、事件。

### 资源：审批定义 Approval
查看资源 [字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/overview-of-approval-resources)

#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)** |
| --- | --- | --- |
| <md-text type="field-name" >[创建审批定义](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create)</md-text><br>`POST` /open-apis/approval/v4/approval | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[查看审批定义](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get)</md-text><br>`GET` /open-apis/approval/v4/approvals/:approval_code | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[订阅审批事件](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/subscribe)</md-text><br>`POST` /open-apis/approval/v4/:approval_code/subscribe | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[取消订阅审批事件](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/unsubscribe)</md-text><br>`POST` /open-apis/approval/v4/:approval_code/unsubscribe | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |



### 资源：审批实例 Instance
查看资源 [字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/overview-approval-instance)

#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)** |
| --- | --- | --- |
| <md-text type="field-name" >[创建审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/create)</md-text><br>`POST` /open-apis/approval/v4/instances | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[批量获取审批实例ID](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/list)</md-text><br>`GET` /open-apis/approval/v4/instances | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[获取单个审批实例详情](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/get)</md-text><br>`GET` /open-apis/approval/v4/instances/:instance_id | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[审批实例抄送](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/cc)</md-text><br>`POST` /open-apis/approval/v4/instances/cc | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[审批实例撤回](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/cancel)</md-text><br>`POST` /open-apis/approval/v4/instances/cancel | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[审批流程预览](/document/ukTMukTMukTM/ukTM5UjL5ETO14SOxkTN/approval-preview)</md-text><br>`POST` /open-apis/approval/v4/instances/preview | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |


#### 事件列表

| **[事件（event）](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)** | 权限要求 | **触发时机** |
| --- | --- | --- |
| <md-text type="field-name" >[审批定义更新](/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/custom-approval-event)</md-text> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-text type="field-name" >审批定义变更</md-text> |
| <md-text type="field-name" >[审批实例更新](/document/ukTMukTMukTM/ugDNyUjL4QjM14CO0ITN)</md-text> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-text type="field-name" >审批实例状态变更</md-text> |
| <md-text type="field-name" >[审批任务更新](/document/ukTMukTMukTM/ugDNyUjL4QjM14CO0ITN)</md-text> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-text type="field-name" >审批任务状态变更</md-text> |
| <md-text type="field-name" >[请假审批](/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/leave)</md-text> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-text type="field-name" >「审批」应用的表单里如果包含 请假控件组，则在此表单审批通过后触发此事件</md-text> |
| <md-text type="field-name" >[加班审批](/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/overtime)</md-text> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-text type="field-name" >「审批」应用的表单里如果包含 加班控件组，则在此表单审批通过后触发此事件</md-text> |
| <md-text type="field-name" >[换班审批](/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/shift-change)</md-text> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-text type="field-name" >「审批」应用的表单包含换班控件组的，换班申请审批通过后触发此事件</md-text> |
| <md-text type="field-name" >[补卡审批](/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/attendance-record-correction)</md-text> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-text type="field-name" >补卡申请审批通过后触发此事件。 你可以在「打卡」应用里提交补卡申请</md-text> |
| <md-text type="field-name" >[出差审批](/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/business-trip)</md-text> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-text type="field-name" >「审批」应用的表单里如果包含 出差控件组，则在此表单审批通过后触发此事件</md-text> |
| <md-text type="field-name" >[外出审批事件](/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/out-of-office)</md-text> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-text type="field-name" >「审批」应用的表单里如果包含 外出控件组，则在此表单审批通过后触发此事件</md-text> |


### 资源：审批任务 Task
查看资源 [字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/introduction)


#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)** |
| --- | --- | --- |
| <md-text type="field-name" >[审批任务同意](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/approve)</md-text><br>`POST` /open-apis/approval/v4/tasks/approve | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[审批任务拒绝](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/reject)</md-text><br>`POST` /open-apis/approval/v4/tasks/reject | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[审批任务转交](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/transfer)</md-text><br>`POST` /open-apis/approval/v4/tasks/transfer | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[审批任务退回](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/specified_rollback)</md-text><br>`POST` /open-apis/approval/v4/tasks/specified_rollback | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[审批任务加签](/document/ukTMukTMukTM/ukTM5UjL5ETO14SOxkTN/approval-task-addsign)</md-text><br>`POST` /open-apis/approval/v4/tasks/add_sign | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |


### 资源：审批评论 Comment
查看资源 [字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/overview)
#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)** |
| --- | --- | --- |
| <md-text type="field-name" >[创建评论](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/create)</md-text><br>`POST` /open-apis/approval/v4/instances/:instance_id/comments | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[获取评论](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/list)</md-text><br>`GET` /open-apis/approval/v4/instances/:instance_id/comments | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm><br><md-perm name="approval:approval:readonly" desc="Access Approval" support_app_types="custom,isv" tags="">Access Approval</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[删除评论](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/delete)</md-text><br>`DELETE` /open-apis/approval/v4/instances/:instance_id/comments/:comment_id | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[清空评论](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/remove)</md-text><br>`POST` /open-apis/approval/v4/instances/:instance_id/comments/remove | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |



### 资源：三方审批定义 External Approval
查看资源 [字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/overview)


#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)** |
| --- | --- | --- |
| <md-text type="field-name" >[三方审批定义创建](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/create)</md-text><br>`POST` /open-apis/approval/v4/external_approvals | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |


### 资源：三方审批实例 External Instance
查看资源 [字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/overview)

#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)** |
| --- | --- | --- |
| <md-text type="field-name" >[三方审批实例同步](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/create)</md-text><br>`POST` /open-apis/approval/v4/external_instances | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |
| <md-text type="field-name" >[三方审批实例校验](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/check)</md-text><br>`POST` /open-apis/approval/v4/external_instances/check | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |


### 资源：三方审批任务 Exteranl Task
查看资源 [字段及示例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_task/overview)


#### 方法列表

| **[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)** | 权限要求 | **[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)** |
| --- | --- | --- |
| <md-text type="field-name" >[获取三方审批任务状态](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_task/list)</md-text><br>`GET` /open-apis/approval/v4/external_tasks | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> | <md-tag type="token-tenant">tenant_access_token</md-tag> |



