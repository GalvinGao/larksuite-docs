---
document_id: '7139727755097636869'
directory_id: '7122028361538945030'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/overview-of-approval-resources
breadcrumb:
- Server API
- Approval
- Approval definition
- Resource introduction
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:35Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/overview-of-approval-resources
---

# 资源介绍
## 资源定义
单个审批流，由表单和审批流程组成，创建后可以让员工在发起审批时填写各个控件的值并形成[审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/overview-approval-instance)。

## 字段说明
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: ">名称</md-dt-th>
      <md-dt-th style="width: ">类型</md-dt-th>
      <md-dt-th style="width: ">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >approval_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批名称的国际化文案 Key，以 @i18n@ 开头，长度不得少于 9 个字符

**示例值**："@i18n@approval_name"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >approval_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批定义code，传空表示新建

**示例值**："7C468A54-8745-2245-9675-08B7C63E7A85"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >description</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批描述的国际化文案 Key，以 @i18n@ 开头，长度不得少于 9 个字符

**示例值**：@i18n@description
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >viewers</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_create_viewers\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
		
viewers 字段指定了哪些人能从审批应用的前台发起该审批。

1.当 type 为 USER 时，需要填写 user_id 、 open_id、union_id 中的一个，用于指定对哪个用户可见；

2.当 type 为 DEPARTMENT 时，需要填写部门的 open_id，用于指定对哪个部门可见
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >viewer_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	可见人类型

**示例值**："USER"

**可选值有**：
<md-enum>
<md-enum-item key="TENANT" >租户内可见</md-enum-item>
<md-enum-item key="DEPARTMENT" >指定部门</md-enum-item>
<md-enum-item key="USER" >指定用户</md-enum-item>
<md-enum-item key="NONE" >任何人都不可见</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >viewer_user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	当 view_type 是 USER，需要填写 user_id 、 open_id、union_id 中的一个

**示例值**：19a294c2
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >viewer_department_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	当 view_type 为DEPARTMENT，需要填写 department_id 、 open_department_id 中的一个

**示例值**："od-ac9d697abfa990b715dcc33d58a62a9d"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >form</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_form</md-text>
	</md-dt-td>

	<md-dt-td>
		审批定义表单
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >form_content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批定义表单，json 数组

**示例值**："[{"id":"user_name", "type": "input", "required":true, "name":"@i18n@widget1"}]"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >node_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_node\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
		审批定义节点，需要将开始节点作为 list 第一个元素，结束节点作为最后一个元素
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	节点 ID，开始节点的 ID 为 START，结束节点的 ID 为 END，开始和结束节点不需要指定 name、node_type 以及 approver

**示例值**："START"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	节点名称的国际化文案 Key，以 @i18n@ 开头，长度不得少于 9 个字符

**示例值**："@i18n@node_name"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >node_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批类型枚举,当 node_type 为依次审批时，审批人必须为『发起人自选』

**示例值**："AND"

**可选值有**：
<md-enum>
<md-enum-item key="AND" >会签</md-enum-item>
<md-enum-item key="OR" >或签</md-enum-item>
<md-enum-item key="SEQUENTIAL" >依次审批</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >approver</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_approver_ccer\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
	审批人列表
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
	审批节点上的审批人，1.当 type 为 Supervisor、SupervisorTopDown、DepartmentManager 、DepartmentManagerTopDown 这 4 种时，需要在 user_id 中填写对应的级数，例如：由下往上三级主管审批，user_id = 3；

2.当 type 为 Personal 时，需要填写user_id 或 open_id 中的一个，用于指定用户；

3.当 approver 为 Free 发起人自选时，不需要指定 user_id 或 open_id；

ccer不支持 Free 发起人自选

**示例值**："Supervisor"

**可选值有**：
<md-enum>
<md-enum-item key="Supervisor" >主管审批（由下往上）</md-enum-item>
<md-enum-item key="SupervisorTopDown" >主管审批（从上往下）</md-enum-item>
<md-enum-item key="DepartmentManager" >部门负责人审批（由下往上）</md-enum-item>
<md-enum-item key="DepartmentManagerTopDown" >部门负责人审批（从上往下）</md-enum-item>
<md-enum-item key="Personal" >指定成员</md-enum-item>
<md-enum-item key="Free" >发起人自选</md-enum-item>
</md-enum>
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
		用户id，根据user_id_type填写

**示例值**："f7cb567e"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >level</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批级数，当 type 为 Supervisor、SupervisorTopDown、DepartmentManager 、DepartmentManagerTopDown 这 4 种时，需要在 level 中填写对应的级数，例如：由下往上三级主管审批，level = 3

**示例值**："3"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >ccer</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_approver_ccer\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
	抄送人列表
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
	审批节点上的审批人，1.当 type 为 Supervisor、SupervisorTopDown、DepartmentManager 、DepartmentManagerTopDown 这 4 种时，需要在 user_id 中填写对应的级数，例如：由下往上三级主管审批，user_id = 3；

2.当 type 为 Personal 时，需要填写user_id 或 open_id 中的一个，用于指定用户；

3.当 approver 为 Free 发起人自选时，不需要指定 user_id 或 open_id；

ccer不支持 Free 发起人自选

**示例值**："Supervisor"

**可选值有**：
<md-enum>
<md-enum-item key="Supervisor" >主管审批（由下往上）</md-enum-item>
<md-enum-item key="SupervisorTopDown" >主管审批（从上往下）</md-enum-item>
<md-enum-item key="DepartmentManager" >部门负责人审批（由下往上）</md-enum-item>
<md-enum-item key="DepartmentManagerTopDown" >部门负责人审批（从上往下）</md-enum-item>
<md-enum-item key="Personal" >指定成员</md-enum-item>
<md-enum-item key="Free" >发起人自选</md-enum-item>
</md-enum>
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
		用户id，根据user_id_type填写

**示例值**："f7cb567e"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >level</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批级数，当 type 为 Supervisor、SupervisorTopDown、DepartmentManager 、DepartmentManagerTopDown 这 4 种时，需要在 level 中填写对应的级数，例如：由下往上三级主管审批，level = 3

**示例值**："3"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >privilege_field</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >field_group</md-text>
	</md-dt-td>

	<md-dt-td>
	表单项的控件权限
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >writable</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
	可写权限的表单项的 id列表

**示例值**：9293493
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >readable</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
	可读权限的表单项的 id列表

**示例值**：9293493
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >settings</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_setting</md-text>
	</md-dt-td>

	<md-dt-td>
	审批定义其他设置
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >revert_interval</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>

	<md-dt-td>
	审批实例通过后允许撤回的时间，以秒为单位，默认 31 天，0 为不可撤回

**示例值**：0
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >revert_option</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>

	<md-dt-td>
	是否支持审批通过第一个节点后撤回，默认为1，0为不支持

**示例值**：0
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >config</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_config</md-text>
	</md-dt-td>

	<md-dt-td>
	审批定义配置项，用于配置对应审批定义是否可以由用户在审批后台进行修改
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >can_update_viewer</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	允许用户修改可见范围

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >can_update_form</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	允许用户更新表单

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >can_update_process</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	允许用户更新流程定义

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >can_update_revert</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	允许用户更新撤回设置

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >help_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	帮助文档链接

**示例值**："https://www.baidu.com"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >icon</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>

	<md-dt-td>
	审批图标枚举，默认为 0

**示例值**：0

**默认值**：`0`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >i18n_resources</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_resource\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
	国际化文案
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >locale</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	语言可选值有

**示例值**："zh-CN"

**可选值有**：
<md-enum>
<md-enum-item key="zh-CN" >中文</md-enum-item>
<md-enum-item key="en-US" >英文</md-enum-item>
<md-enum-item key="ja-JP" >日文</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >texts</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_resource_text\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
		文案 key, value, i18n key 以 @i18n@ 开头； 该字段主要用于做国际化，语序用户同时传多个语言的文案，审批中心会根据用户当前的语音环境使用对应的文案，如果没有传用户当前的语音环境文案，则会使用默认的语言文案。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	文案key

**示例值**："@i18n@1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	文案

**示例值**："people"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >is_default</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	是否默认语言，默认语言需要包含所有key，非默认语言如果key不存在会使用默认语言代替

**示例值**：true
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::


## 数据示例

```json
{
    "approval_name": "@i18n@approval_name",
    "approval_code": "813718CE-F38D-45CA-A5C1-ACF4F564B526",
    "viewers":[
        {
            "viewer_type":"TENANT",
            "viewer_user_id":""
        }
    ],
    "form": {
        "form_content": "[{\"id\":\"111\",\"name\":\"@i18n@event_name\",\"required\":true,\"type\":\"input\"},{\"id\":\"222\",\"name\":\"@i18n@time_interval\",\"required\":true,\"type\":\"dateInterval\",\"value\":{\"format\":\"YYYY-MM-DD hh:mm\",\"intervalAllowModify\":false}},{\"id\":\"333\",\"name\":\"@i18n@event_type\",\"type\":\"radioV2\",\"value\":[{\"key\":\"1\",\"text\":\"@i18n@recurrence_event\"},{\"key\":\"2\",\"text\":\"@i18n@single_event\"}]},{\"id\":\"444\",\"name\":\"@i18n@attende_count\",\"required\":true,\"type\":\"number\"},{\"id\":\"555\",\"name\":\"@i18n@apply_reason\",\"required\":true,\"type\":\"textarea\"}]"
        },

    "node_list": [{
          "id": "START",
          "privilege_field":{ 
				 "writable": ["111","222"],
				 "readable": ["111","222"]
		  }
        },{
          "id": "7106864726566",
          "privilege_field":{ 
				 "writable": ["111","222"],
				 "readable": ["111","222"]
		  },
          "name": "@i18n@node_name",
          "node_type": "AND",
          "approver": [
            {
              "type": "Personal",
              "user_id": "59a92c4a"
            }
          ],
          "ccer": [
            {
              "type": "Supervisor",
              "level": "2"
            }
          ]
        },{
          "id": "END"
        }],
    "settings" : {
          "revert_interval":0
        },
    "config" : {
          "can_update_viewer": false,
          "can_update_form": true,
          "can_update_process": true,
          "can_update_revert": true,
          "help_url":"https://www.baidu.com"
        },
    "icon": 1,
    "i18n_resources" : [{
          "locale": "zh-CN",
          "texts" : [
              {"key":"@i18n@approval_name","value":"审批名称"},
              {"key":"@i18n@event_name","value":"日程名称"},
              {"key":"@i18n@node_name","value":"审批"},
              {"key":"@i18n@time_interval","value":"日程名称"},
              {"key":"@i18n@event_type","value":"日程类型"},
              {"key":"@i18n@recurrence_event","value":"重复性日程"},
              {"key":"@i18n@single_event","value":"单次日程"},
              {"key":"@i18n@attende_count","value":"参与人数量"},
              {"key":"@i18n@apply_reason","value":"申请原因"}
            ],
          "is_default": true
        }]
}
```


## 什么是Approval Code

审批定义对应的唯一编码，可在编辑审批时从浏览器 url 中找到。点击以下链接可以进入管理后台：

[https://www.larksuite.com/approval/admin/approvalList?devMode=on](https://www.larksuite.com/approval/admin/approvalList?devMode=on)


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e7fa9ad3633e76a27c0f4cd753bd5c37_jpCqoviq5g.png?lazyload=true&width=1001&height=136)

点击上图中的编辑按钮，可以在URL中找到Approval Code，格式为：`definitionCode=E3254848-D172-4169-B03E-744E7CD11F06`


## 什么是表单（Form）

表单包括一或多个控件，员工发起审批时需要填写的表单页面，每个审批定义都有自己的表单。

### 1.审批控件（Widget）

组成审批的部分。一个表单由一或多个审批控件组成。每个控件都有一个基础属性：

```js 
struct{
    String Id; //标识在一个审批定义中某一个控件的唯一 ID
    String CustomId; //自定义控件 ID，标识在一个审批定义中某一个控件的唯一 ID
    String Name; //该控件的名称
    String Type; //该控件的类型
} 
```
### 2.自定义控件ID

可以通过在审批管理后台编辑定义时指定一个 ID，之后可以用指定的 ID 唯一确定一个在该定义中的控件。

### 3.控件类型说明

|控件类型|说明|
|-|-|
|单行文本|用于填写一个单行文本|
|多行文本|用于填写一个多行文本|
|数字|用于填写一个数字|
|金额|用于填写审批金额数量及单位，默认单位为CNY|
|日期|用于填写详细时间|
|日期区间|用于填写一个日期区间，包括有开始时间、结束时间以及持续时间|
|单选|用于选择单个选择|
|多选框|用于选择多个选择|
|地址|用于填写一个地址|
|联系人|用于在审批中添加联系人|
|说明|用于在审批定义中添加说明（如填写规范、注意事项），在发起审批时不可编辑|
|明细|用于填写明细信息，在明细中可以添加其他控件比如数字、金额等。在创建审批定义时设计一个明细控件表示明细的一个条目中包括哪些控件，发起人可以根据自身需求增加条目，每一个条目都和创建审批定义时所设明细控件一致|
|计算公式|在创建审批定义时设计计算方式，表示该控件的值依赖于其他控件（数字、金额）计算得出|
|图片|用于在审批中添加图片|
|附件|用于在审批中添加附件，如文件等|
|关联|用于在当前审批中关联其他审批，使审批人能够在审批时查看所关联审批的概况|
|请假|用于填写请假审批的相关内容，包括选择请假类型（如病假、产假等，请假类型需要管理员提前在假期管理中设置并在创建审批定义时选择当前审批定义发起实例时可选种类有哪些），填写请假开始时间，填写请假结束时间以及请假时长|
|加班|用于填写加班申请的相关内容，包括选择加班类型（比如调休、带薪、不带薪等，由管理员在创建审批定义时设定），填写加班开始/结束时间和时长以及填写加班原因|
|换班|用于填写换班审批的相关内容，包括填写换班时间以及填写换班原因|
|出差|用于填写出差审批的相关内容，包括填写行程（一段行程中包括行程开始时间、行程结束时间、行程时长、出发地、目的地、交通方式、单程/往返以及备注，其中行程时长以自然日为单位计算，并且倘若出差有多段行程，可以由发起人手动增加新的一段行程），出差总时长，出差原因以及同行人|
|补卡|用于填写补卡审批的相关内容，包括填写补卡时间和填写补卡原因

## 什么是审批流程（Process）

管理员在创建审批定义同时会设计一个审批流转方式，这个方式包括了一个由该定义产生的实例每一步如何进行。流程由审批节点构成，一个审批定义对应一个审批流程。一个审批流程对应一或多个审批节点。员工在发起审批实例后，按照设计好的审批流程进行流转。设计审批流程时，需要指定在流程的每一步（审批节点，详情见下）的审批操作。包括有每一步的审批人或者到当前步骤时系统自动通过或系统自动拒绝。如果当前步骤不是由系统自动进行操作，则需要设置审批人或者设置由发起人自选审批人。

### 1.发起人自选审批人

即创建审批定义时在某一步不指定审批人，设置了由发起人自选，那么是由操作人在发起审批实例时自己选择审批人，审批定义创建时只需指定当前步骤是由所有操作人所选审批人或签或者会签即可。**如果在定义设置了发起人自选审批人，在通过 API 发起审批实例时必须要指定该步骤发起人所选审批人**。

### 2.审批节点

每一个审批流程由一或多个审批节点构成，每一个节点表示审批流转过程中的一个步骤，在当前节点完成后进入下一个节点。每当实例创建时生成该实例对应的各个审批节点，并由审批节点构成审批流程。

### 3.自定义节点 ID

和控件类似的，节点 ID 也可以指定，指定后可以用指定的 ID 唯一确定一个在该流程中的节点。


## 用户ID说明
了解user_id，open_id，union_id的区别和用途，参见教程 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)

## 部门ID说明
了解department_id，open_department_id的区别和用途，参见教程 
[部门 Department 资源概述](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview)
