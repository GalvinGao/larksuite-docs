---
document_id: '7072724425749184517'
directory_id: '7122028361539026950'
title: 预览审批流程
full_path: /ukTMukTMukTM/ukTM5UjL5ETO14SOxkTN/approval-preview
breadcrumb:
- Server API
- Approval
- Approval instances
- Preview approval instances
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:41Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukTM5UjL5ETO14SOxkTN/approval-preview
---

# 预览审批流程

提交审批前，预览审批流程。或者发起审批后，在某一审批节点预览后续流程。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/approval/v4/instances/preview |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 查询参数

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >user_id_type</md-text> | <md-text type="field-type" >string</md-text> | 否 | 用户 ID 类型<br>**示例值**："open_id"<br>**可选值有**：<br>- `open_id`：用户的 open id<br>- `union_id`：用户的 union id<br>- `user_id`：用户的 user id<br>**默认值**：`open_id`<br>**当值为 `user_id`，字段权限要求**：<br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |


### 请求体
|参数|类型|必须|说明|
|-|-|-|-|
|approval_code|string|否|审批定义 Code|
|user_id|string|是|发起审批用户，employeid或者openid|
|department_id|string|否|发起审批用户部门，如果用户只属于一个部门，可以不填，如果属于多个部门，必须填其中一个部门|
|form|string|否| JSON字符串，控件值。提交审批之前，查看预览流程时，该字段必填|
|&emsp;∟id|string|是|控件ID，也可以使用自定义 ID custom_id 的值|
|&emsp;∟type|string|是|控件类型|
|&emsp;∟value|string|是|控件值，不同类型的值格式不一样|
|instance_code|string|否|审批实例code|
|task_id|string|否|若审批实例已存在，则传递当前审批任务对应的task_id, 并且user_id需要传task的指派人|

#### 说明:

-   请求参数和"创建审批实例"接口类似（没有uuid字段）。如果用户还未发起审批实例，则类似创建审批实例接口的传值，比如user_id、表单form、审批定义approval_code等数据。如果已经发起了，此时流程是已经固定的，只需要user_id和instance_code、task_id。

### 请求体示例

case1: 发起审批之前

```json
{
    "approval_code":"C2CAAA90-70D9-3214-906B-B6FFF947F00D",
    "user_id":"f7cb567e",
    "department_id":"",
    "form":"[{\"id\":\"widget16256287451710001\", \"type\": \"number\", \"value\":\"43\"}]"

}
```

case2:发起审批之后

```json
{
    "instance_code":"12345CA6-97AC-32BB-8231-47C33FFFCCFD",
    "user_id":"f7cb567e",
    "task_id": "6982332863116876308"
}
```

## 响应

### 响应体

|参数|类型|必须|说明|
|-|-|-|-|
|code|int|是|错误码，非0表示失败|
|msg|string|是|返回码的描述|
|data|json|是| 返回业务信息|
|∟preview_nodes|list|是|预览节点信息|
|&emsp;∟user_id_list|list|是|审批人id列表|
|&emsp;∟end_cc_id_list| list| 是  |审批结束抄送人id列表|
|&emsp;∟node_id| string | 是  |节点id|
|&emsp;∟node_name      | string | 是  |节点名称|
|&emsp;∟node_type      | string | 是  |节点类型：<br>AND：会签<br>OR: 或签|
|&emsp;∟custom_node_id | string | 是  |用户自定义节点id|
|&emsp;∟comments       | list   | 是  |节点的说明信息|
|&emsp;∟is_empty_logic | bool   | 是  |审批人是否为空，若为空，则user_id_list为兜底审批人id列表|
|&emsp;∟is_approver_type_free | bool   | 是  |是否发起人自选节点|
|&emsp;∟has_cc_type_free | bool   | 是  |节点是否支持抄送人自选|

### 响应体示例

```json
{ 
    "code":0, 
    "msg":"success", 
    "data": { 
        "preview_nodes":[
            {
                "user_id_list":["ffffffff"],
                "end_cc_id_list":[],
                "node_id":"b078ffd28db767c502ac367053f6e0ac",
                "node_name":"发起",
                "node_type":"",
                "comments":[],
                "custom_node_id":""
            },
            {
                "user_id_list":["ffffffff"],
                "end_cc_id_list":[],
                "node_id":"e6ce10282a3cc3bf4a408feffd678dcf",
                "node_name":"审批",
                "node_type":"AND",
                "comments":[],
                "custom_node_id":""，
                "is_empty_logic":false，
                "is_approver_type_free":false，
                "has_cc_type_free":false
            },
            {
                "user_id_list":[],
                "end_cc_id_list":[],
                "node_id":"b1a326c06d88bf042f73d70f50197905",
                "node_name":"结束",
                "node_type":"",
                "comments":[],
                "custom_node_id":""
             }
         ]
    }        
}  
```

###
