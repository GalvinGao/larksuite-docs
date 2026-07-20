---
document_id: '7122386910857412614'
directory_id: '7122028361539026950'
title: 获取单个审批实例详情
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/get
breadcrumb:
- Server API
- Approval
- Approval instances
- Obtain details of an approval instance
document_type: ReferenceDocumentType
updated_at: 2023-01-31T12:16:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/get
---

# 获取单个审批实例详情

通过审批实例 Instance Code  获取审批实例详情。Instance Code 由 [批量获取审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/list) 接口获取。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=approval&version=v4&resource=instance&method=get)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="tip">

</md-alert>
:::



## 请求
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/approval/v4/instances/:instance_id</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom,isv"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
            
      </md-th>
      <md-td>
            <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm>
            <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm>
            <md-perm name="approval:instance" desc="查看、创建、更新、删除原生审批实例相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除原生审批实例相关信息</md-perm>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            字段权限要求
      </md-th>
      <md-td>
        <md-alert type="tip" icon="none">
        该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请
        </md-alert>
        <md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 35%;">名称</md-th>
      <md-th style="width: 13%;">类型</md-th>
       <md-th style="width: 15%;" filters="是,否" >必填</md-th>
      <md-th  style="width: 37%;">描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"

[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)

</md-td>
</md-tr>
</md-tbody>
</md-table>
:::



### 路径参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >instance_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批实例 Code, 若在创建的时候传了uuid, 也可以通过传uuid获取

**示例值**："81D31358-93AF-92D6-7425-01A5D67C4E71"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 查询参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;" >描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >locale</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	语言

**示例值**："zh-CN"

**可选值有**：
<md-enum>
<md-enum-item key="zh-CN" >中文</md-enum-item>
<md-enum-item key="en-US" >英文</md-enum-item>
<md-enum-item key="ja-JP" >日文</md-enum-item>
</md-enum>
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
	否
	</md-dt-td>
	<md-dt-td>
	发起审批用户id，仅自建应用可返回

**示例值**："f7cb567e"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >user_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户 ID 类型

**示例值**："open_id"

**可选值有**：
<md-enum>
<md-enum-item key="open_id" >标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)</md-enum-item>
<md-enum-item key="union_id" >标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)</md-enum-item>
<md-enum-item key="user_id" >标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)</md-enum-item>
</md-enum>

**默认值**：`open_id`

**当值为 `user_id`，字段权限要求**：
<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





## 响应



### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	错误码，非 0 表示失败
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >msg</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	错误描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >data</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >\-</md-text>
	</md-dt-td>
	<md-dt-td>
	\-
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >approval_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >start_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批创建时间
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >end_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批完成时间，未完成为 0
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	发起审批用户
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >open_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	发起审批用户 open id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >serial_number</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批单编号
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >department_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	发起审批用户所在部门
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批实例状态

**可选值有**：
<md-enum>
<md-enum-item key="PENDING" >审批中</md-enum-item>
<md-enum-item key="APPROVED" >通过</md-enum-item>
<md-enum-item key="REJECTED" >拒绝</md-enum-item>
<md-enum-item key="CANCELED" >撤回</md-enum-item>
<md-enum-item key="DELETED" >删除</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >uuid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户的唯一标识id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >form</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	json字符串，控件值详情见下方
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >task_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >instance_task\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	审批任务列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
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


<md-dt-tr level="2">
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


<md-dt-tr level="2">
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


<md-dt-tr level="2">
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


<md-dt-tr level="2">
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


<md-dt-tr level="2">
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


<md-dt-tr level="2">
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


<md-dt-tr level="2">
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


<md-dt-tr level="2">
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


<md-dt-tr level="2">
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


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >comment_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >instance_comment\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	评论列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	评论 id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	发表评论用户
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >open_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	发表评论用户 open id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >comment</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	评论内容
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >create_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	1564590532967
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >files</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >file\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	评论附件
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	附件路径
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >file_size</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	附件大小
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >title</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	附件标题
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	附件类别
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >timeline</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >instance_timeline\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	审批动态
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	动态类型，不同类型 ext 内的 user_id_list 含义不一样

**可选值有**：
<md-enum>
<md-enum-item key="START" >审批开始</md-enum-item>
<md-enum-item key="PASS" >通过</md-enum-item>
<md-enum-item key="REJECT" >拒绝</md-enum-item>
<md-enum-item key="AUTO_PASS" >自动通过</md-enum-item>
<md-enum-item key="AUTO_REJECT" >自动拒绝</md-enum-item>
<md-enum-item key="REMOVE_REPEAT" >去重</md-enum-item>
<md-enum-item key="TRANSFER" >转交</md-enum-item>
<md-enum-item key="ADD_APPROVER_BEFORE" >前加签</md-enum-item>
<md-enum-item key="ADD_APPROVER" >并加签</md-enum-item>
<md-enum-item key="ADD_APPROVER_AFTER" >后加签</md-enum-item>
<md-enum-item key="DELETE_APPROVER" >减签</md-enum-item>
<md-enum-item key="ROLLBACK_SELECTED" >指定回退</md-enum-item>
<md-enum-item key="ROLLBACK" >全部回退</md-enum-item>
<md-enum-item key="CANCEL" >撤回</md-enum-item>
<md-enum-item key="DELETE" >删除</md-enum-item>
<md-enum-item key="CC" >抄送</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >create_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	发生时间
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	动态产生用户
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >open_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	动态产生用户 open id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >user_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	被抄送人列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >open_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	被抄送人列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >task_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	产生动态关联的task_id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >comment</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	理由
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >cc_user_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >instance_cc_user\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	抄送人列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	抄送人 user id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >cc_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批实例内抄送唯一标识
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >open_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	抄送人 open id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >ext</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	动态其他信息，json格式，目前包括 user_id_list, user_id，open_id_list，open_id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >node_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	产生task的节点key
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >files</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >file\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	审批附件
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	附件路径
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >file_size</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	附件大小
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >title</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	附件标题
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	附件类别
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >modified_instance_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	修改的原实例 code,仅在查询修改实例时显示该字段
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >reverted_instance_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	撤销的原实例 code,仅在查询撤销实例时显示该字段
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >approval_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批定义 Code
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >reverted</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	单据是否被撤销
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >instance_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批实例 Code
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "msg": "success",
    "data": {
        "approval_name": "Payment",
        "start_time": "1564590532967",
        "end_time": "1564590532967",
        "user_id": "f3ta757q",
        "open_id": "ou_3cda9c969f737aaa05e6915dce306cb9",
        "serial_number": "202102060002",
        "department_id": "od-8ec33ffec336c3a39a278bc25e931676",
        "status": "PENDING",
        "uuid": "1234567",
        "form": "[{\"id\": \"widget1\",\"custom_id\": \"user_info\",\"name\": \"Item application\",\"type\": \"textarea\"},\"value\":\"aaaa\"]",
        "task_list": [
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
        ],
        "comment_list": [
            {
                "id": "1234",
                "user_id": "f7cb567e",
                "open_id": "ou_123456",
                "comment": "ok",
                "create_time": "评论时间",
                "files": [
                    {
                        "url": "https://p3-approval-sign.byteimg.com/lark-approval-attachment/image/20220714/1/332f3596-0845-4746-a4bc-818d54ad435b.png~tplv-ottatrvjsm-image.image?x-expires=1659033558&x-signature=6edF3k%2BaHeAuvfcBRGOkbckoUl4%3D#.png",
                        "file_size": 186823,
                        "title": "e018906140ed9388234bd03b0.png",
                        "type": "image"
                    }
                ]
            }
        ],
        "timeline": [
            {
                "type": "PASS",
                "create_time": "1564590532967",
                "user_id": "f7cb567e",
                "open_id": "ou_123456",
                "user_id_list": [
                    "eea5gefe"
                ],
                "open_id_list": [
                    "ou_123456"
                ],
                "task_id": "1234",
                "comment": "ok",
                "cc_user_list": [
                    {
                        "user_id": "eea5gefe",
                        "cc_id": "123445",
                        "open_id": "ou_12345"
                    }
                ],
                "ext": "{\"user_id\":\"62d4a44c\",\"open_id\":\"ou_123456\"}",
                "node_key": "APPROVAL_240330_4058663",
                "files": [
                    {
                        "url": "https://p3-approval-sign.byteimg.com/lark-approval-attachment/image/20220714/1/332f3596-0845-4746-a4bc-818d54ad435b.png~tplv-ottatrvjsm-image.image?x-expires=1659033558&x-signature=6edF3k%2BaHeAuvfcBRGOkbckoUl4%3D#.png",
                        "file_size": 186823,
                        "title": "e018906140ed9388234bd03b0.png",
                        "type": "image"
                    }
                ]
            }
        ],
        "modified_instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71",
        "reverted_instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71",
        "approval_code": "7C468A54-8745-2245-9675-08B7C63E7A85",
        "reverted": false,
        "instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71"
    }
}
</md-code-json>
:::

**控件值**: 
|类型|说明|
|-|-|
|input|--|
|textarea|--|
|date|RFC3339格式，2019-10-01T08:12:01+08:00|
|radio/radioV2|option 中的 text 字段|
|address|China/Beijing/Beijing/Chaoyang Qu/chang an jie<br>如果地址控件允许输入详细地址，则最后一项为用户输入的详细地址|
```json
{
    "id": "widget1",
    "custom_id": "user_info",
    "name": "Item application",
    "type": "input",
    "value": "data"
}
```
|类型|说明|
|-|-|
|number|--|
|amount|--|
|formula|--|
```json
{
    "id": "widget1",
    "name": "Item application",
    "type": "number",
    "value": 1234.56
}
```
|类型|说明|
|-|-|
|contact|`user_id`|
```json
{
    "id": "widget1",
    "name": "Item application",
    "type": "contact",
    "value": ["f8ca557e"],
    "open_ids": ["ou_12345"]
}
```

|类型|说明|
|-|-|
|connect|`instance_code`|
|attachmentV2|附件控件, ext为附件名字,value为附件地址, 按逗号分隔|且链接有效时长为12小时
|image/imageV2|图片控件, ext为图片名字,value为图片地址, 按逗号分隔|且链接有效时长为12小时

```json
{
    "id": "widget1",
    "name": "Item application",
    "type": "attachmentV2",
    "ext": "'Item 1 name','Item 2 name'",
    "value": ["Item 1", "Item 2"]
}
```

|类型|说明|
|-|-|
|connect|`instance_code`|
|attachment|附件控件,建议用attachmentV2|
|checkbox/checkboxV2|option中的text字段|

```json
{
    "id": "widget1",
    "name": "Item application",
    "type": "checkbox",
    "value": ["Item 1"]
}
```
|类型|说明|
|-|-|
|dateInterval|start和end满足RFC3339格式|
```json
{
    "id": "widget1",
    "name": "Item application",
    "type": "dateInterval",
    "value": {
         "start": "2019-10-01T08:12:01+08:00",
         "end": "2019-10-02T08:12:01+08:00",
         "interval": 2.0
     }
}
```

|类型|说明|
|-|-|
|fieldList|value是二维数组|

```json
{
    "id": "widget1",
    "name": "Item application",
    "type": "fieldList",
    "value": [
         [   
            {
                "id": "widget1",
                "type": "checkbox",
                "value": ["jxpsebqp-0"]
            }
         ]
     ]
}
```
|类型|说明|
|-|-|
|document|--|

```json
{
    "id":"widget1",
    "type":"document",
    "value":
        {
          "token": "doxcx7B8OzLFHExkiwYuPGAwf",
          "type": "doc",
          "title": "title",
          "url": "https://bytedance..larksuite.com/docx/doxcx7B8OzLFHExkiwYuPGAwf"
       }
}
```


|类型|说明|
|-|-|
|department|--|

```json
{
    "id":"widget1",
    "type":"department",
    "value":[
        {
            "open_id":"od-xxx"
        }
    ]
}
```
|字段|说明|
|-|-|
|id|控件 ID|
|name|控件名称|
|type|leaveGroup|
|value|--|
|&emsp;∟name|假期名称|
|&emsp;∟start|开始时间，满足 RFC3339 格式|
|&emsp;∟end|结束时间，满足 RFC3339 格式|
|&emsp;∟interval|时长，单位天，有些假期类型用户手动输入请假时长|

```json
{
    "id": "widget1",
    "name": "leave",
    "type": "leaveGroup",
    "value": {
         "name": "annual leave"
         "start": "2019-10-01T00:00:00+08:00",
         "end": "2019-10-02T00:0:00+08:00",
         "interval": 2.0
     }
}
```

|字段|说明|
|-|-|
|id|控件 ID|
|name|控件名称|
|type|leaveGroupV2|
|value|--|
|&emsp;∟name|假期名称|
|&emsp;∟start|开始时间，满足 RFC3339 格式|
|&emsp;∟end|结束时间，满足 RFC3339 格式|
|&emsp;∟interval|时长，有些假期类型用户手动输入请假时长|
|&emsp;∟unit|时长单位, DAY/HOUR ，有些假期类型用户手动输入请假时长|
|&emsp;∟reason|原因|

```json
{
    "id": "widget1",
    "name": "leave",
    "type": "widgetLeaveGroupV2",
    "value": {
         "name": "annual leave"
         "start": "2019-10-01T00:00:00+08:00",
         "end": "2019-10-02T00:0:00+08:00",
         "interval": 2.0,
         "unit": "DAY",
         "reason": "out going"
     }
}
```


|类型|说明|
|-|-|
|remedyGroup|time满足RFC3339格式|

```json
{
    "id": "widget1",
    "name": "remedy",
    "type": "remedyGroup",
    "value": {
         "time": "2019-10-01T08:12:01+08:00",
         "reason": "forgot"
     }
}
```


|类型|说明|
|-|-|
|shiftGroup|shiftTime和returnTime满足RFC3339格式|

```json
{
    "id": "widget1",
    "name": "shift",
    "type": "shiftGroup",
    "value": {
         "shiftTime": "2019-10-01T08:12:01+08:00",
         "returnTime": "2019-10-02T08:12:01+08:00",
         "reason": "ask for leave"
     }
}
```


|类型|说明|
|-|-|
|workGroup|start和end满足RFC3339格式|

```json
{
    "id": "widget1",
    "name": "work",
    "type": "workGroup",
    "value": {
         "name": "Overtime pay"
         "start": "2019-10-01T08:12:01+08:00",
         "end": "2019-10-02T08:12:01+08:00",
         "interval": 2.0,
         "reason": "ask for leave"
     }
}
```


|类型|说明|
|-|-|
|tripGroup|start和end满足RFC3339格式|

```json
{
    "id": "widget1",
    "name": "trip",
    "type": "tripGroup",
    "value": {
         "schedule": [{
                "start": "2019-10-0T00:00:00Z+08:00",
                "end": "2019-10-01T00:00:00Z+08:00",
                "interval": 123.45,
                "departure": "China/Beijing/Beijing",
                "destination": "China/Shanghai/Shanghai",
                "transportation": "Airplane",
                "oneRound": "One Way",
                "remark": "business",
         }],
         "interval": 2.0,
         "reason": "business",
         "peer": ["f7cb567e"],
    }
}
```
|类型|说明|
|-|-|
|telephone|电话|
```json
{
    "id":"widget1",
    "type":"telephone",
    "value": {
        "country_code":"+86",
        "national_number":"13122222222"
    }
}
```

### 错误码
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">HTTP状态码</md-th>
            <md-th style="width: 15%;">错误码</md-th>
            <md-th style="width: 30%;">描述</md-th>
            <md-th style="width: 30%;">排查建议</md-th>
        </md-tr>
    </md-thead>
  <md-tbody>

<md-tr>
  <md-td>400</md-td>
  <md-td>1390001</md-td>
  <md-td>param is invalid</md-td>
  <md-td>参数错误</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1395001</md-td>
  <md-td>There have been some errors. Please try again later</md-td>
  <md-td>服务出现错误，请稍候再试</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1390003</md-td>
  <md-td>instance code not found</md-td>
  <md-td>检查审批实例code是否正确</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1390009</md-td>
  <md-td>no operation permission</md-td>
  <md-td>检查操作权限是否正确</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1390004</md-td>
  <md-td>user_id or open_id not found</md-td>
  <md-td>检查user_id,open_id是否正确</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




