---
document_id: '7122386910857609222'
directory_id: '7122028361539043334'
title: 同步三方审批实例
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/create
breadcrumb:
- Server API
- Approval
- Third-party approval instances
- Sync instances
document_type: ReferenceDocumentType
updated_at: 2025-07-18T02:48:57Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/create
---

# 同步三方审批实例

审批中心不负责审批的流转，审批的流转在三方系统，三方系统在审批流转后生成的审批实例、审批任务、审批抄送数据同步到审批中心。

用户可以在审批中心中浏览三方系统同步过来的实例、任务、抄送信息，并且可以跳转回三方系统进行更详细的查看和操作，其中实例信息在【已发起】列表，任务信息在【待审批】和【已审批】列表，抄送信息在【抄送我】列表。

:::html
<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9dff4434afbeb0ef69de7f36b9a6e995_z5iwmTzEgg.png" alt="" style="zoom:17%;" />


<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ca6e0e984a7a6d64e1b16a0bac4bf868_tfqjCiaJQM.png" alt="" style="zoom:17%;" />


<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/529377e238df78d391bbd22e962ad195_T7eefLI1GA.png" alt="" style="zoom:17%;" />
:::

对于审批任务，三方系统也可以配置审批任务的回调接口，这样审批人可以在审批中心中直接进行审批操作，审批中心会回调三方系统，三方系统收到回调后更新任务信息，并将新的任务信息同步回审批中心，形成闭环。

:::html
<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/721c35428bc1187db3318c572f9979ad_je75QpElcg.png" alt=""  style="zoom:25%;" />
:::
<br>{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=approval&version=v4&resource=external_instance&method=create)

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

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/approval/v4/external_instances |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm><br><md-perm name="approval:external_instance" desc="查看、创建、更新、删除三方审批实例相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除三方审批实例相关信息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 请求体

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >approval_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	审批定义 code， 创建审批定义返回的值，表示该实例属于哪个流程；该字段会影响到列表中该实例的标题，标题取自对应定义的 name 字段

**示例值**："81D31358-93AF-92D6-7425-01A5D67C4E71"
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
	是
	</md-dt-td>
	<md-dt-td>
	审批实例状态

**示例值**："PENDING"

**可选值有**：
<md-enum>
<md-enum-item key="PENDING" >审批中</md-enum-item>
<md-enum-item key="APPROVED" >审批流程结束，结果为同意</md-enum-item>
<md-enum-item key="REJECTED" >审批流程结束，结果为拒绝</md-enum-item>
<md-enum-item key="CANCELED" >审批发起人撤回</md-enum-item>
<md-enum-item key="DELETED" >审批被删除</md-enum-item>
<md-enum-item key="HIDDEN" >状态隐藏(不显示状态)</md-enum-item>
<md-enum-item key="TERMINATED" >审批终止</md-enum-item>
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
	否
	</md-dt-td>
	<md-dt-td>
	审批实例扩展 JSON。单据编号通过传business_key字段来实现

**示例值**："{\"xxx\":\"xxx\",\"business_key\":\"xxx\"}"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >instance_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	审批实例唯一标识，用户自定义，需确保证租户下唯一

**示例值**："24492654"
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
	是
	</md-dt-td>
	<md-dt-td>
	审批实例链接集合 ，用于【已发起】列表的跳转，跳转回三方系统； pc_link 和 mobile_link 必须填一个，填写的是哪一端的链接，即会跳转到该链接，不受平台影响
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >pc_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	pc 端的跳转链接，当用户使用Lark pc 端时，使用该字段进行跳转

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
	否
	</md-dt-td>
	<md-dt-td>
	移动端 跳转链接，当用户使用Lark 移动端时，使用该字段进行跳转

**示例值**："https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/detail?id=1234"
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
	否
	</md-dt-td>
	<md-dt-td>
	审批展示名称，如果填写了该字段，则审批列表中的审批名称使用该字段，如果不填该字段，则审批名称使用审批定义的名称

**示例值**："@i18n@1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >form</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >external_instance_form\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户提交审批时填写的表单数据，用于所有审批列表中展示。可传多个值，但审批中心pc展示前2个,移动端展示前3个,长度不超过2048字符
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
	否
	</md-dt-td>
	<md-dt-td>
	表单字段名称

**示例值**："@i18n@2"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	表单值

**示例值**："@i18n@3"
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
	审批发起人 user_id，发起人可在【已发起】列表中看到所有已发起的审批; 在【待审批】，【已审批】【抄送我】列表中，该字段展示审批是谁发起的。审批发起人 open id，和 user id 二者至少填一个。

**示例值**："a987sf9s"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >user_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	审批发起人 用户名，如果发起人不是真实的用户（例如是某个部门），没有 user_id，则可以使用该字段传名称

**示例值**："@i18n@9"
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
	否
	</md-dt-td>
	<md-dt-td>
	审批发起人 open id，和 user id 二者至少填一个

**示例值**："ou_be73cbc0ee35eb6ca54e9e7cc14998c1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >department_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	发起人部门，用于列表中展示发起人所属部门。不传则不展示。如果用户没加入任何部门，传 ""，将展示租户名称传 department_name 展示部门名称

**示例值**："od-8ec33278bc2"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >department_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	审批发起人 部门，如果发起人不是真实的用户（例如是某个部门），没有 department_id，则可以使用该字段传名称

**示例值**："@i18n@10"
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
	是
	</md-dt-td>
	<md-dt-td>
	审批发起时间，Unix毫秒时间戳

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
	是
	</md-dt-td>
	<md-dt-td>
	审批实例结束时间：未结束的审批为 0，Unix毫秒时间戳

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
	是
	</md-dt-td>
	<md-dt-td>
	审批实例最近更新时间，Unix毫秒时间戳，用于推送数据版本控制如果 update_mode 值为 UPDATE，则只有传过来的 update_time 有变化时（变大），才会更新审批中心中的审批实例信息。使用该字段主要用来避免并发时老的数据更新了新的数据

**示例值**："1556468012678"
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
	否
	</md-dt-td>
	<md-dt-td>
	列表页打开审批实例的方式

**示例值**："BROWSER"

**可选值有**：
<md-enum>
<md-enum-item key="BROWSER" >跳转系统默认浏览器打开</md-enum-item>
<md-enum-item key="SIDEBAR" >Lark中侧边抽屉打开</md-enum-item>
<md-enum-item key="NORMAL" >Lark内嵌页面打开</md-enum-item>
<md-enum-item key="TRUSTEESHIP" >以托管打开</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >update_mode</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	更新方式， 当 update_mode=REPLACE时，每次都以当前推送的数据为最终数据，会删掉审批中心中多余的任务、抄送数据（不在这次推送的数据中）; 当 update_mode=UPDATE时，则不会删除审批中心的数据，而只是进行新增和更新实例、任务数据

**示例值**："UPDATE"

**可选值有**：
<md-enum>
<md-enum-item key="REPLACE" >全量替换，默认值</md-enum-item>
<md-enum-item key="UPDATE" >增量更新</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >task_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >external_instance_task_node\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	任务列表

**数据校验规则**：

- 最大长度：`200`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >task_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	审批实例内的唯一标识，用于更新审批任务时定位数据

**示例值**："112534"
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
	否
	</md-dt-td>
	<md-dt-td>
	审批人 user_id ，和 open_id 二者至少填一个。该任务会出现在审批人的【待审批】或【已审批】列表中

**示例值**："a987sf9s"
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
	否
	</md-dt-td>
	<md-dt-td>
	审批人 open_id，和 user_id 二者至少填一个

**示例值**："ou_be73cbc0ee35eb6ca54e9e7cc14998c1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >title</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	审批任务名称

**示例值**："i18n1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >links</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >external_instance_link</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	【待审批】或【已审批】中使用的跳转链接，用于跳转回三方系统pc_link 和 mobile_link 必须填一个，填写的是哪一端的链接，即会跳转到该链接，不受平台影响
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >pc_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	pc 端的跳转链接，当用户使用Lark pc 端时，使用该字段进行跳转

**示例值**："https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc/pages/detail?id=1234"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >mobile_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	移动端 跳转链接，当用户使用Lark 移动端时，使用该字段进行跳转

**示例值**："https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/detail?id=1234"
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
	是
	</md-dt-td>
	<md-dt-td>
	任务状态

**示例值**："PENDING"

**可选值有**：
<md-enum>
<md-enum-item key="PENDING" >待审批</md-enum-item>
<md-enum-item key="APPROVED" >任务同意</md-enum-item>
<md-enum-item key="REJECTED" >任务拒绝</md-enum-item>
<md-enum-item key="TRANSFERRED" >任务转交</md-enum-item>
<md-enum-item key="DONE" >任务通过但审批人未操作；审批人看不到这个任务, 若想要看到, 可以通过抄送该人.</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >extra</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	扩展 json，任务结束原因需传complete_reason字段。枚举值与对应说明：
  - approved：同意
  - rejected：拒绝
  - node_auto_reject：（因逻辑判断产生的）自动拒绝
  - specific_rollback：退回（包括退回到发起人、退回到中间任一审批人）
  - add：并加签（添加新审批人，和我一起审批）
  - add_pre：前加签（添加新审批人，在我之前审批）
  - add_post：后加签（添加新审批人，在我之后审批）
  - delete_assignee：减签
  - forward_resign：转交（转给其他人审批）
  - recall：撤销（撤回单据，单据失效）
  - delete ：删除审批单
  - admin_forward：管理员在后台操作转交
  - system_forward：系统自动转交
  - auto_skip：自动通过
  - manual_skip：手动跳过
  - submit_again：重新提交任务
  - restart：重新启动流程
  - others：其他（作为兜底）

**示例值**："{\"xxx\":\"xxx\",\"complete_reason\":\"approved\"}"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >create_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	任务创建时间，Unix 毫秒时间戳

**示例值**："1556468012678"
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
	是
	</md-dt-td>
	<md-dt-td>
	任务完成时间：未结束的审批为 0，Unix 毫秒时间戳

**示例值**："1556468012678"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >update_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	task最近更新时间，Unix毫秒时间戳，用于推送数据版本控制； 更新策略同 instance 中的 update_time

**示例值**："1556468012678"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >action_context</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	操作上下文，当用户操作时，回调请求中带上该参数，用于传递该任务的上下文数据

**示例值**："123456"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >action_configs</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >action_config\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	任务级别操作配置,快捷审批目前支持移动端操作
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >action_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	操作类型，每个任务都可以配置2个操作，会展示审批列表中，当用户操作时，回调请求会带上该字段，表示用户进行了同意操作还是拒绝操作

**可选值有**：
<md-enum>
<md-enum-item key="APPROVE" >同意</md-enum-item>
<md-enum-item key="REJECT" >拒绝</md-enum-item>
<md-enum-item key="{KEY}" >任意字符串，如果使用任意字符串，则需要提供 action_name</md-enum-item>
</md-enum>

**示例值**："APPROVE"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >action_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	操作名称，i18n key 用于前台展示，如果 action_type 不是 APPROVAL和REJECT，则必须提供该字段，用于展示特定的操作名称

**示例值**："@i18n@5"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >is_need_reason</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	是否需要意见, 如果为true,则用户操作时，会跳转到 意见填写页面

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >is_reason_required</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	审批意见是否必填

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >is_need_attachment</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	意见是否支持上传附件

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >display_method</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	列表页打开审批任务的方式

**示例值**："BROWSER"

**可选值有**：
<md-enum>
<md-enum-item key="BROWSER" >跳转系统默认浏览器打开</md-enum-item>
<md-enum-item key="SIDEBAR" >Lark中侧边抽屉打开</md-enum-item>
<md-enum-item key="NORMAL" >Lark内嵌页面打开</md-enum-item>
<md-enum-item key="TRUSTEESHIP" >以托管模式打开</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >exclude_statistics</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	三方任务支持不纳入效率统计。
false：纳入效率统计。
true：不纳入效率统计

**示例值**：false

**默认值**：`false`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >node_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	节点id：必须同时满足
- 一个流程内，每个节点id唯一。如一个流程下「直属上级」、「隔级上级」等每个节点的Node_id均不一样
- 同一个流程定义内，不同审批实例中的相同节点，Node_id要保持不变。例如张三和李四分别发起了请假申请，这2个审批实例中的「直属上级」节点的node_id应该保持一致

**示例值**："node"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >node_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	节点名称，如「财务审批」「法务审批」，支持中英日三种语言。示例：i18n@name。需要在i18n_resources中传该名称对应的国际化文案

**示例值**："i18n@name"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >cc_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >cc_node\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	抄送列表

**数据校验规则**：

- 最大长度：`200`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >cc_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	审批实例内唯一标识

**示例值**："123456"
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
	否
	</md-dt-td>
	<md-dt-td>
	抄送人 employee id

**示例值**："12345"
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
	否
	</md-dt-td>
	<md-dt-td>
	抄送人 open id，和user id 二者至少填一个

**示例值**："ou_be73cbc0ee35eb6ca54e9e7cc14998c1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >links</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >external_instance_link</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	跳转链接，用于【抄送我的】列表中的跳转pc_link 和 mobile_link 必须填一个，填写的是哪一端的链接，即会跳转到该链接，不受平台影响
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >pc_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	pc 端的跳转链接，当用户使用Lark pc 端时，使用该字段进行跳转

**示例值**："https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc/pages/detail?id=1234"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >mobile_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	移动端 跳转链接，当用户使用Lark 移动端时，使用该字段进行跳转

**示例值**："https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/detail?id=1234"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >read_status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	阅读状态，空值表示不支持已读未读：

**示例值**："READ"

**可选值有**：
<md-enum>
<md-enum-item key="READ" >已读</md-enum-item>
<md-enum-item key="UNREAD" >未读</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >extra</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	扩展 json

**示例值**："{\"xxx\":\"xxx\"}"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >title</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	抄送任务名称

**示例值**："xxx"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >create_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	抄送发起时间，Unix 毫秒时间戳

**示例值**："1556468012678"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >update_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	抄送最近更新时间，Unix毫秒时间戳，用于推送数据版本控制更新策略同 instance 的update_time

**示例值**："1556468012678"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >display_method</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	列表页打开审批任务的方式

**示例值**："BROWSER"

**可选值有**：
<md-enum>
<md-enum-item key="BROWSER" >跳转系统默认浏览器打开</md-enum-item>
<md-enum-item key="SIDEBAR" >Lark中侧边抽屉打开</md-enum-item>
<md-enum-item key="NORMAL" >Lark内嵌页面打开</md-enum-item>
<md-enum-item key="TRUSTEESHIP" >以托管模式打开</md-enum-item>
</md-enum>
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
	是
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
	是
	</md-dt-td>
	<md-dt-td>
	语言可选值有： zh-CN：中文 en-US：英文 ja-JP：日文

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
	是
	</md-dt-td>
	<md-dt-td>
	文案 key, value, i18n key 以 @i18n@ 开头； 该字段主要用于做国际化，允许用户同时传多个语言的文案，审批中心会根据用户当前的语言环境使用对应的文案，如果没有传用户当前的语言环境文案，则会使用默认的语言文案。
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
	是
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
	是
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
	是
	</md-dt-td>
	<md-dt-td>
	是否默认语言，默认语言需要包含所有key，非默认语言如果key不存在会使用默认语言代替

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >trusteeship_url_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	单据托管认证token，托管回调会附带此token，帮助业务方认证

**示例值**："788981c886b1c28ac29d1e68efd60683d6d90dfce80938ee9453e2a5f3e9e306"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >trusteeship_user_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户的类型，会影响请求参数用户标识域的选择，包括加签操作回传的目标用户， 目前仅支持 "user_id"

**示例值**："user_id"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >trusteeship_urls</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >trusteeship_urls</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	单据托管回调接入方的接口的URL地址
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >form_detail_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	获取表单schema相关数据的url地址

**示例值**："https://#{your_domain}/api/form_detail"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >action_definition_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	表示获取审批操作区数据的url地址

**示例值**："https://#{your_domain}/api/action_definition"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >approval_node_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	获取审批记录相关数据的url地址

**示例值**："https://#{your_domain}/api/approval_node"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >action_callback_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	进行审批操作时回调的url地址

**示例值**："https://#{your_domain}/api/action_callback"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >pull_business_data_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	获取托管动态数据url 地址，使用该接口时必须要保证历史托管单据的数据中都同步了该接口地址，如果历史单据中没有该接口需要重新同步历史托管单据的数据来更新该URL。该接口用于Lark审批前端和业务线进行交互使用,只有使用审批前端的特定组件(由Lark审批前端提供的组件，并且需要和业务线进行接口交互的组件)才会需要

**示例值**："https://#{your_domain}/api/pull_business_data"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::

:::html
<md-alert type="warn">**注意：**

需确保审批实例内各类唯一ID在审批实例内的唯一性，即使不属于同一实体。实体包括：实例(instance)、任务(task)、抄送(cc)。如果实例ID、任务ID、抄送ID重复，则会导致在审批中心任务看不到对应的待办、已办、抄送、已发起数据。</md-alert>
:::

### 请求体示例
:::html
<md-code-json>
{
    "approval_code": "84C18825-A3D2-41D0-891F-E7A2424C5D48",
    "instance_id": "3162634",
    "status": "PENDING",
    "extra": "",
    "links": {
        "pc_link": "http://applink.larksuite.com/sso/common?redirectUrl=/seeyon/main.do?method=main&client=pc",
        "mobile_link": "http://applink.larksuite.com/sso/common?redirectUrl=/seeyon/main.do?method=main&client=pc"
    },
    "title": "@i18n@1",
    "form": [{
        "name": "@i18n@2",
        "value": "@i18n@3"
    }],
    "user_id": "16fb9ff3",
    "user_name": "张三",
    "open_id": "123",
    "department_id": "",
    "department_name": "hr",
    "start_time": "1657093395000",
    "update_time": "1657093395000",
    "end_time": 0,
    "update_mode": "REPLACE",
    "task_list": [{
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
    }],
    "cc_list": [{
        "cc_id": "1231243",
        "user_id": "16fb9ff3",
        "open_id": "",
        "links": {
            "pc_link": "http://",
            "mobile_link": "http://"
        },
        "read_status": "READ",
        "extra": "",
        "title": "XXX",
        "create_time": "1657093395000",
        "update_time": "1657093395000"
    }],
    "i18n_resources": [{
        "locale": "zh-CN",
        "is_default": true,
        "texts": [ {"key":"@i18n@1", "value":"测试"}, {"key":"@i18n@2", "value":"天"},{"key":"@i18n@3", "value":"2022-07-06"}]
    }]
}
</md-code-json>
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
	<md-text type="field-name" >data</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >external_instance</md-text>
	</md-dt-td>
	<md-dt-td>
	同步的实例数据
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >approval_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批定义 code， 创建审批定义返回的值，表示该实例属于哪个流程；该字段会影响到列表中该实例的标题，标题取自对应定义的 name 字段
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
	审批实例状态

**可选值有**：
<md-enum>
<md-enum-item key="PENDING" >审批中</md-enum-item>
<md-enum-item key="APPROVED" >审批流程结束，结果为同意</md-enum-item>
<md-enum-item key="REJECTED" >审批流程结束，结果为拒绝</md-enum-item>
<md-enum-item key="CANCELED" >审批发起人撤回</md-enum-item>
<md-enum-item key="DELETED" >审批被删除</md-enum-item>
<md-enum-item key="HIDDEN" >状态隐藏(不显示状态)</md-enum-item>
<md-enum-item key="TERMINATED" >审批终止</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >extra</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批实例扩展 JSON。单据编号通过传business_key字段来实现
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >instance_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批实例唯一标识，用户自定义，需确保证租户下唯一
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >links</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >external_instance_link</md-text>
	</md-dt-td>
	<md-dt-td>
	审批实例链接集合 ，用于【已发起】列表的跳转，跳转回三方系统； pc_link 和 mobile_link 必须填一个，填写的是哪一端的链接，即会跳转到该链接，不受平台影响
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >pc_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	pc 端的跳转链接，当用户使用Lark pc 端时，使用该字段进行跳转
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >mobile_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	移动端 跳转链接，当用户使用Lark 移动端时，使用该字段进行跳转
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >title</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批展示名称，如果填写了该字段，则审批列表中的审批名称使用该字段，如果不填该字段，则审批名称使用审批定义的名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >form</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >external_instance_form\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	用户提交审批时填写的表单数据，用于所有审批列表中展示。可传多个值，但审批中心pc展示前2个,移动端展示前3个,长度不超过2048字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	表单字段名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	表单值
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
	审批发起人 user_id，发起人可在【已发起】列表中看到所有已发起的审批; 在【待审批】，【已审批】【抄送我】列表中，该字段展示审批是谁发起的。审批发起人 open id，和 user id 二者至少填一个。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >user_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批发起人 用户名，如果发起人不是真实的用户（例如是某个部门），没有 user_id，则可以使用该字段传名称
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
	审批发起人 open id，和 user id 二者至少填一个
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >department_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	发起人部门，用于列表中展示发起人所属部门。不传则不展示。如果用户没加入任何部门，传 ""，将展示租户名称传 department_name 展示部门名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >department_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批发起人 部门，如果发起人不是真实的用户（例如是某个部门），没有 department_id，则可以使用该字段传名称
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
	审批发起时间，Unix毫秒时间戳
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
	审批实例结束时间：未结束的审批为 0，Unix毫秒时间戳
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >update_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批实例最近更新时间，Unix毫秒时间戳，用于推送数据版本控制如果 update_mode 值为 UPDATE，则只有传过来的 update_time 有变化时（变大），才会更新审批中心中的审批实例信息。使用该字段主要用来避免并发时老的数据更新了新的数据
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >display_method</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	列表页打开审批实例的方式

**可选值有**：
<md-enum>
<md-enum-item key="BROWSER" >跳转系统默认浏览器打开</md-enum-item>
<md-enum-item key="SIDEBAR" >Lark中侧边抽屉打开</md-enum-item>
<md-enum-item key="NORMAL" >Lark内嵌页面打开</md-enum-item>
<md-enum-item key="TRUSTEESHIP" >以托管打开</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >update_mode</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	更新方式， 当 update_mode=REPLACE时，每次都以当前推送的数据为最终数据，会删掉审批中心中多余的任务、抄送数据（不在这次推送的数据中）; 当 update_mode=UPDATE时，则不会删除审批中心的数据，而只是进行新增和更新实例、任务数据

**可选值有**：
<md-enum>
<md-enum-item key="REPLACE" >全量替换，默认值</md-enum-item>
<md-enum-item key="UPDATE" >增量更新</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >task_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >external_instance_task_node\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	任务列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >task_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批实例内的唯一标识，用于更新审批任务时定位数据
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
	审批人 user_id ，和 open_id 二者至少填一个。该任务会出现在审批人的【待审批】或【已审批】列表中
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
	审批人 open_id，和 user_id 二者至少填一个
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
	审批任务名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
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


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >pc_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	pc 端的跳转链接，当用户使用Lark pc 端时，使用该字段进行跳转
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >mobile_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	移动端 跳转链接，当用户使用Lark 移动端时，使用该字段进行跳转
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
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
<md-enum-item key="PENDING" >待审批</md-enum-item>
<md-enum-item key="APPROVED" >任务同意</md-enum-item>
<md-enum-item key="REJECTED" >任务拒绝</md-enum-item>
<md-enum-item key="TRANSFERRED" >任务转交</md-enum-item>
<md-enum-item key="DONE" >任务通过但审批人未操作；审批人看不到这个任务, 若想要看到, 可以通过抄送该人.</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >extra</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	扩展 json，任务结束原因需传complete_reason字段。枚举值与对应说明：
  - approved：同意
  - rejected：拒绝
  - node_auto_reject：（因逻辑判断产生的）自动拒绝
  - specific_rollback：退回（包括退回到发起人、退回到中间任一审批人）
  - add：并加签（添加新审批人，和我一起审批）
  - add_pre：前加签（添加新审批人，在我之前审批）
  - add_post：后加签（添加新审批人，在我之后审批）
  - delete_assignee：减签
  - forward_resign：转交（转给其他人审批）
  - recall：撤销（撤回单据，单据失效）
  - delete ：删除审批单
  - admin_forward：管理员在后台操作转交
  - system_forward：系统自动转交
  - auto_skip：自动通过
  - manual_skip：手动跳过
  - submit_again：重新提交任务
  - restart：重新启动流程
  - others：其他（作为兜底）
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >create_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	任务创建时间，Unix 毫秒时间戳
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >end_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	任务完成时间：未结束的审批为 0，Unix 毫秒时间戳
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >update_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	task最近更新时间，Unix毫秒时间戳，用于推送数据版本控制； 更新策略同 instance 中的 update_time
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >action_context</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	操作上下文，当用户操作时，回调请求中带上该参数，用于传递该任务的上下文数据
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
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


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >action_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	操作类型，每个任务都可以配置2个操作，会展示审批列表中，当用户操作时，回调请求会带上该字段，表示用户进行了同意操作还是拒绝操作

**可选值有**：
<md-enum>
<md-enum-item key="APPROVE" >同意</md-enum-item>
<md-enum-item key="REJECT" >拒绝</md-enum-item>
<md-enum-item key="{KEY}" >任意字符串，如果使用任意字符串，则需要提供 action_name</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >action_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	操作名称，i18n key 用于前台展示，如果 action_type 不是 APPROVAL和REJECT，则必须提供该字段，用于展示特定的操作名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >is_need_reason</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否需要意见, 如果为true,则用户操作时，会跳转到 意见填写页面
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >is_reason_required</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	审批意见是否必填
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >is_need_attachment</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	意见是否支持上传附件
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >display_method</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	列表页打开审批任务的方式

**可选值有**：
<md-enum>
<md-enum-item key="BROWSER" >跳转系统默认浏览器打开</md-enum-item>
<md-enum-item key="SIDEBAR" >Lark中侧边抽屉打开</md-enum-item>
<md-enum-item key="NORMAL" >Lark内嵌页面打开</md-enum-item>
<md-enum-item key="TRUSTEESHIP" >以托管模式打开</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >exclude_statistics</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	三方任务支持不纳入效率统计。
false：纳入效率统计。
true：不纳入效率统计
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >node_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	节点id：必须同时满足
- 一个流程内，每个节点id唯一。如一个流程下「直属上级」、「隔级上级」等每个节点的Node_id均不一样
- 同一个流程定义内，不同审批实例中的相同节点，Node_id要保持不变。例如张三和李四分别发起了请假申请，这2个审批实例中的「直属上级」节点的node_id应该保持一致
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >node_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	节点名称，如「财务审批」「法务审批」，支持中英日三种语言。示例：i18n@name。需要在i18n_resources中传该名称对应的国际化文案
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >cc_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >cc_node\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	抄送列表
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
	审批实例内唯一标识
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
	抄送人 employee id
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
	抄送人 open id，和user id 二者至少填一个
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >links</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >external_instance_link</md-text>
	</md-dt-td>
	<md-dt-td>
	跳转链接，用于【抄送我的】列表中的跳转pc_link 和 mobile_link 必须填一个，填写的是哪一端的链接，即会跳转到该链接，不受平台影响
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >pc_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	pc 端的跳转链接，当用户使用Lark pc 端时，使用该字段进行跳转
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >mobile_link</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	移动端 跳转链接，当用户使用Lark 移动端时，使用该字段进行跳转
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >read_status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	阅读状态，空值表示不支持已读未读：

**可选值有**：
<md-enum>
<md-enum-item key="READ" >已读</md-enum-item>
<md-enum-item key="UNREAD" >未读</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >extra</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	扩展 json
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
	抄送任务名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >create_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	抄送发起时间，Unix 毫秒时间戳
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >update_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	抄送最近更新时间，Unix毫秒时间戳，用于推送数据版本控制更新策略同 instance 的update_time
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >display_method</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	列表页打开审批任务的方式

**可选值有**：
<md-enum>
<md-enum-item key="BROWSER" >跳转系统默认浏览器打开</md-enum-item>
<md-enum-item key="SIDEBAR" >Lark中侧边抽屉打开</md-enum-item>
<md-enum-item key="NORMAL" >Lark内嵌页面打开</md-enum-item>
<md-enum-item key="TRUSTEESHIP" >以托管模式打开</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
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


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >locale</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	语言可选值有： zh-CN：中文 en-US：英文 ja-JP：日文

**可选值有**：
<md-enum>
<md-enum-item key="zh-CN" >中文</md-enum-item>
<md-enum-item key="en-US" >英文</md-enum-item>
<md-enum-item key="ja-JP" >日文</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >texts</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_resource_text\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	文案 key, value, i18n key 以 @i18n@ 开头； 该字段主要用于做国际化，允许用户同时传多个语言的文案，审批中心会根据用户当前的语言环境使用对应的文案，如果没有传用户当前的语言环境文案，则会使用默认的语言文案。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	文案key
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	文案
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >is_default</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否默认语言，默认语言需要包含所有key，非默认语言如果key不存在会使用默认语言代替
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >trusteeship_url_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	单据托管认证token，托管回调会附带此token，帮助业务方认证
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >trusteeship_user_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户的类型，会影响请求参数用户标识域的选择，包括加签操作回传的目标用户， 目前仅支持 "user_id"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >trusteeship_urls</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >trusteeship_urls</md-text>
	</md-dt-td>
	<md-dt-td>
	单据托管回调接入方的接口的URL地址
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >form_detail_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	获取表单schema相关数据的url地址
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >action_definition_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	表示获取审批操作区数据的url地址
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >approval_node_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	获取审批记录相关数据的url地址
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >action_callback_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	进行审批操作时回调的url地址
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >pull_business_data_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	获取托管动态数据url 地址，使用该接口时必须要保证历史托管单据的数据中都同步了该接口地址，如果历史单据中没有该接口需要重新同步历史托管单据的数据来更新该URL。该接口用于Lark审批前端和业务线进行交互使用,只有使用审批前端的特定组件(由Lark审批前端提供的组件，并且需要和业务线进行接口交互的组件)才会需要
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
        "data": {
            "approval_code": "81D31358-93AF-92D6-7425-01A5D67C4E71",
            "status": "PENDING",
            "extra": "{\"xxx\":\"xxx\",\"business_key\":\"xxx\"}",
            "instance_id": "24492654",
            "links": {
                "pc_link": "https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc/pages/detail?id=1234",
                "mobile_link": "https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/detail?id=1234"
            },
            "title": "@i18n@1",
            "form": [
                {
                    "name": "@i18n@2",
                    "value": "@i18n@3"
                }
            ],
            "user_id": "a987sf9s",
            "user_name": "@i18n@9",
            "open_id": "ou_be73cbc0ee35eb6ca54e9e7cc14998c1",
            "department_id": "od-8ec33278bc2",
            "department_name": "@i18n@10",
            "start_time": "1556468012678",
            "end_time": "1556468012678",
            "update_time": "1556468012678",
            "display_method": "BROWSER",
            "update_mode": "UPDATE",
            "task_list": [
                {
                    "task_id": "112534",
                    "user_id": "a987sf9s",
                    "open_id": "ou_be73cbc0ee35eb6ca54e9e7cc14998c1",
                    "title": "i18n1",
                    "links": {
                        "pc_link": "https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc/pages/detail?id=1234",
                        "mobile_link": "https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/detail?id=1234"
                    },
                    "status": "PENDING",
                    "extra": "{\"xxx\":\"xxx\",\"complete_reason\":\"approved\"}",
                    "create_time": "1556468012678",
                    "end_time": "1556468012678",
                    "update_time": "1556468012678",
                    "action_context": "123456",
                    "action_configs": [
                        {
                            "action_type": "APPROVE",
                            "action_name": "@i18n@5",
                            "is_need_reason": false,
                            "is_reason_required": false,
                            "is_need_attachment": false
                        }
                    ],
                    "display_method": "BROWSER",
                    "exclude_statistics": false,
                    "node_id": "node",
                    "node_name": "i18n@name"
                }
            ],
            "cc_list": [
                {
                    "cc_id": "123456",
                    "user_id": "12345",
                    "open_id": "ou_be73cbc0ee35eb6ca54e9e7cc14998c1",
                    "links": {
                        "pc_link": "https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc/pages/detail?id=1234",
                        "mobile_link": "https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/detail?id=1234"
                    },
                    "read_status": "READ",
                    "extra": "{\"xxx\":\"xxx\"}",
                    "title": "xxx",
                    "create_time": "1556468012678",
                    "update_time": "1556468012678",
                    "display_method": "BROWSER"
                }
            ],
            "i18n_resources": [
                {
                    "locale": "zh-CN",
                    "texts": [
                        {
                            "key": "@i18n@1",
                            "value": "people"
                        }
                    ],
                    "is_default": true
                }
            ],
            "trusteeship_url_token": "788981c886b1c28ac29d1e68efd60683d6d90dfce80938ee9453e2a5f3e9e306",
            "trusteeship_user_id_type": "user_id",
            "trusteeship_urls": {
                "form_detail_url": "https://#{your_domain}/api/form_detail",
                "action_definition_url": "https://#{your_domain}/api/action_definition",
                "approval_node_url": "https://#{your_domain}/api/approval_node",
                "action_callback_url": "https://#{your_domain}/api/action_callback",
                "pull_business_data_url": "https://#{your_domain}/api/pull_business_data"
            }
        }
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1390001 | param is invalid | 参数错误 |
| 400 | 1395001 | There have been some errors. Please try again later | 服务出现错误。排查方案：<br>1. 参考接口文档的参数说明，检查请求时传入的参数是否正确。如果传入的有表单参数（form），则需要检查传入的表单控件数据是否正确。<br>2. 降低请求频率，短时间内不要同步相同的审批实例，请稍后重试。如果重试仍然报错，请联系技术支持。 |
| 400 | 1390014 | tenant_id not found | 未找到指定租户。 |





