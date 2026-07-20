---
document_id: '7139727756258000902'
directory_id: '7122028361538830342'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_task/overview
breadcrumb:
- Server API
- Approval
- Third-party approval tasks
- Resource introduction
document_type: GuideDocumentType
updated_at: 2023-01-31T12:17:05Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_task/overview
---

# 资源介绍

审批人每一个审批的操作对应着一个审批任务
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: ;">名称</md-dt-th>
      <md-dt-th style="width: ;">类型</md-dt-th>
      <md-dt-th style="width: ;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>
   
  <md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >task_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批实例内的唯一标识，用于更新审批任务时定位数据

**示例值**："112534"
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
	审批人 user_id，该任务会出现在审批人的【待审批】或【已审批】列表中

**示例值**："a987sf9s"
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
	审批人 open id，和 user id 二选一

**示例值**："ou_be73cbc0ee35eb6ca54e9e7cc14998c1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >title</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批任务名称

**示例值**："i18n1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >links</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >external_instance_link</md-text>
	</md-dt-td>

	<md-dt-td>
		
【待审批】或【已审批】中使用的跳转链接，用于跳转回三方系统pc_link 和 mobile_link 必须填一个，填写的是哪一端的链接，即会跳转到该链接，不受平台影响
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >pc_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
pc 端的跳转链接，当用户使用Lark pc 端时，使用该字段进行跳转
	<md-dt-td>
	

**示例值**："https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc/pages/detail?id=1234"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >mobile_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
		
移动端 跳转链接，当用户使用Lark 移动端时，使用该字段进行跳转

**示例值**："https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/detail?id=1234"
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

**示例值**："PENDING待审批"

**可选值有**：
<md-enum>
<md-enum-item key="PENDING" >待审批</md-enum-item>
<md-enum-item key="APPROVED" >任务同意</md-enum-item>
<md-enum-item key="REJECTED" >任务拒绝</md-enum-item>
<md-enum-item key="TRANSFERRED" >任务转交</md-enum-item>
<md-enum-item key="DONE" >任务通过但审批人未操作；审批人看不到这个任务, 若想要看到, 可以通过抄送该人</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >extra</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
		
扩展 json

**示例值**："{\"xxx\":\"xxx\"}"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >create_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	任务创建时间，Unix 毫秒时间戳

**示例值**："1556468012678"
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
		
任务完成时间：未结束的审批为 0，Unix 毫秒时间戳

**示例值**："1556468012678"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >update_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
		
task最近更新时间，用于推送数据版本控制； 更新策略同 instance 中的 update_time

**示例值**："1556468012678"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >action_context</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	操作上下文，当用户操作时，回调请求中带上该参数，用于传递该任务的上下文数据

**示例值**："123456"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >action_configs</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >action_config\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
	任务级别操作配置,快捷审批目前支持移动端操作
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >action_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	操作类型，每个任务都可以配置2个操作，会展示审批列表中，当用户操作时，回调请求会带上该字段，表示用户进行了同意操作还是拒绝操作

**示例值**："APPROVE"

**可选值有**：
<md-enum>
<md-enum-item key="APPROVE" >同意</md-enum-item>
<md-enum-item key="REJECT" >拒绝</md-enum-item>
<md-enum-item key="{KEY}" >任意字符串，如果使用任意字符串，则需要提供 action_name</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >action_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	操作名称，i18n key 用于前台展示，如果 action_type 不是 APPROVAL和REJECT，则必须提供该字段，用于展示特定的操作名称

**示例值**："@i18n@5"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >is_need_reason</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	是否需要意见, 如果为true,则用户操作时，会跳转到 意见填写页面



**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >is_reason_required</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	审批意见是否必填

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >is_need_attachment</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	意见是否支持上传附件

**示例值**：false
	</md-dt-td>
</md-dt-tr>
    
<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >display_method</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
		
列表页打开审批任务的方式

**示例值**："BROWSER"

**可选值有**：
<md-enum>
<md-enum-item key="BROWSER" >跳转系统默认浏览器打开</md-enum-item>
<md-enum-item key="SIDEBAR" >Lark中侧边抽屉打开</md-enum-item>
<md-enum-item key="NORMAL" >Lark内嵌页面打开</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>
    
  </md-dt-tbody>
</md-dt-table>
:::


## 数据示例
```json
{
    "task_id": "112253",
    "user_id": "16fb9ff3",
    "links": {
      "pc_link": "http://",
      "mobile_link": "http://"
    },
    "status": "PENDING",
    "extra": "",
    "title": "同意",
    "create_time": "1638468921000",
    "end_time": 0,
    "update_time": "1638468921000",
    "action_context": "123456",
    "action_configs": [{
      "action_type": "APPROVE",
      "action_name": "@i18n@1",
      "is_need_reason": true,
      "is_reason_required": true,
      "is_need_attachment": true
    }]
}
```

## 用户ID说明
了解user_id，open_id，union_id的区别和用途，参见教程 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)
