---
document_id: '7122386910857314310'
directory_id: '7122028361538945030'
title: 查看指定审批定义
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get
breadcrumb:
- Server API
- Approval
- Approval definition
- View approval definitions
document_type: ReferenceDocumentType
updated_at: 2023-01-31T12:16:35Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get
---

# 查看指定审批定义

根据 Approval Code 获取某个审批定义的详情，用于构造创建审批实例的请求。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=approval&version=v4&resource=approval&method=get)

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
| HTTP URL | https://open.larksuite.com/open-apis/approval/v4/approvals/:approval_code |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm><br><md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm><br><md-perm name="approval:definition" desc="查看、创建、更新、删除原生审批定义相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除原生审批定义相关信息</md-perm> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




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
	<md-text type="field-name" >approval_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批定义 Code

**示例值**："7C468A54-8745-2245-9675-08B7C63E7A85"
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
	语言可选值

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
	<md-text type="field-name" >with_admin_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	可选是否返回有数据权限审批流程管理员ID列表

**示例值**：false
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
	<md-text type="field-name" >status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	审批定义状态

**可选值有**：
<md-enum>
<md-enum-item key="ACTIVE" >已启用</md-enum-item>
<md-enum-item key="INACTIVE" >已停用</md-enum-item>
<md-enum-item key="DELETED" >已删除</md-enum-item>
<md-enum-item key="UNKNOWN" >未知</md-enum-item>
</md-enum>
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
	控件信息，见下方form字段说明
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >node_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_node_info\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	节点信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	节点名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >need_approver</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否发起人自选节点 true - 发起审批时需要提交审批人
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
	节点 ID
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
	节点自定义 ID，如果没有设置则不返回
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >node_type</md-text>
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
<md-enum-item key="SEQUENTIAL" >依次审批</md-enum-item>
<md-enum-item key="CC_NODE" >抄送节点</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >approver_chosen_multi</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否支持多选：true-支持，发起、结束节点该值无意义
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >approver_chosen_range</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approver_chosen_range\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	自选范围
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >approver_range_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	指定范围：0-all，1-指定角色，2-指定人员

**可选值有**：
<md-enum>
<md-enum-item key="0" >全公司范围</md-enum-item>
<md-enum-item key="1" >指定角色范围</md-enum-item>
<md-enum-item key="2" >指定用户范围</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >approver_range_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	根据上面的type，分别存放角色id与userid，type为0时本字段为空列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >require_signature</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否签名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >viewers</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_viewer_info\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	可见人列表
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
	可见人类型

**可选值有**：
<md-enum>
<md-enum-item key="TENANT" >租户内可见</md-enum-item>
<md-enum-item key="DEPARTMENT" >指定部门</md-enum-item>
<md-enum-item key="USER" >指定用户</md-enum-item>
<md-enum-item key="ROLE" >指定角色</md-enum-item>
<md-enum-item key="USER_GROUP" >指定用户组</md-enum-item>
<md-enum-item key="NONE" >任何人都不可见</md-enum-item>
</md-enum>
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
	在可见人类型为DEPARTMENT时，id为部门的id ；在可见人类型为USER时，id为用户的id ；在可见人类型为ROLE时，id为角色的id ；在可见人类型为USER_GROUP时，id为用户组的id
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
	在可见人类型为USER时，表示可见人用户id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >approval_admin_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	有数据管理权限的审批流程管理员ID
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::

**form字段说明**: 
|参数|类型|必须|说明|
|-|-|-|-|
form|String|是|json 数组，**控件信息**|
&emsp;∟id|String|是|控件 ID|
&emsp;∟custom_id|String|否|控件自定义 ID|
&emsp;∟name|String|是|控件名称|
&emsp;∟type|String|是|控件类型|
&emsp;∟enable_default_value|bool|是|此控件是否启用了默认值|
&emsp;∟widget_default_value|String|是|控件的默认值|
&emsp;∟default_value_type|String|是|控件的默认值类型|
&emsp;∟display_condition|String|否|控件显隐条件|
&emsp;&emsp;∟conditional|String|否||
&emsp;&emsp;∟conditions|list|否||
&emsp;&emsp;&emsp;∟conditional|String|否|多个条件同时满足|
&emsp;&emsp;&emsp;∟expressions|list|否||
&emsp;&emsp;&emsp;&emsp;∟source_widget|String|否||
&emsp;&emsp;&emsp;&emsp;∟compare_type|String|否|判断规则|
&emsp;&emsp;&emsp;&emsp;∟standard_value|String|否|条件值|

**控件信息**:
|名称|类型|
|-|-|
|单行文本|input|
|多行文本|textarea|
|数字|number|
|金额|amount|
|日期|date|
|日期区间|dateInterval|
|计算公式|formula|
|附件|attachment/attachmentV2|
|图片| image/imageV2|
|联系人|contact|
|关联审批|connect|
|地址|address|
|电话|telephone|
|请假控件组|leaveGroup|
|加班控件组|workGroup|
|换班控件组|shiftGroup|
|补卡控件组|remedyGroup|
|出差控件组|tripGroup|
|外出控件组|outGroup|
```json
{
    "id": "widget1",
    "custom_id": "user_name",
    "name": "Item application",
    "type": "input",
    "printable": true,
    "required":true
}
```
|名称|类型|
|-|-|
|单选|radio/radioV2|
|多选|checkbox/checkboxV2|
```json
{
    "id": "widget1",
    "name": "Item application",
    "type": "radio",
    "printable": true,
    "required":true,
    "option": [
        {
            "text": "1",
            "value": "jxpsebqp-0"
        }
    ]
}
```
|名称|类型|
|-|-|
|明细|fieldList|
```json
{
 "id": "widget1",
 "name": "Item application",
 "type": "fieldList",
 "printable": true,
 "required": true,
 "option":{
 	"input_type": "LIST",
 	"print_type": "LIST"
 },
 "children":[
 	{
 	"id": "widget2",
 	"name": "Item name",
 	"type": "input"
 }
 ]
}
```
|名称|类型|
|-|-|
|电话|telephone|
```json
{
    "id":"widget1",
    "name":"Item application",
    "type":"telephone",
    "printable": true,
    "required":true,
    "option":{
        "available_type": "FIXED_LINE_OR_MOBILE"
      }
}
```

### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "msg": "success",
    "data": {
        "approval_name": "Payment",
        "status": "ACTIVE",
        "form": "[{\"id\": \"widget1\", \"custom_id\": \"user_name\",\"name\": \"Item application\",\"type\": \"textarea\",\"printable\": true,\"required\": true}\"]",
        "node_list": [
            {
                "name": "Approval",
                "need_approver": true,
                "node_id": "46e6d96cfa756980907209209ec03b64",
                "custom_node_id": "46e6d96cfa756980907209209ec03b64",
                "node_type": "AND",
                "approver_chosen_multi": true,
                "approver_chosen_range": [
                    {
                        "approver_range_type": 2,
                        "approver_range_ids": [
                            "ou_e03053f0541cecc3269d7a9dc34a0b21"
                        ]
                    }
                ],
                "require_signature": false
            }
        ],
        "viewers": [
            {
                "type": "TENANT",
                "id": "ou_e03053f0541cecc3269d7a9dc34a0b21",
                "user_id": "f7cb567e"
            }
        ],
        "approval_admin_ids": [
            "ou_3cda9c969f737aaa05e6915dce306cb9"
        ]
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1390001 | param is invalid | 参数错误 |
| 400 | 1390002 | approval code not found | 检查审批定义code是否正确 |
| 400 | 1395001 | There have been some errors. Please try again later | 服务出现错误，请稍候再试 |
| 400 | 1390016 | approval is deleted | 检查审批定义是否存在 |





