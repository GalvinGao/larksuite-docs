---
document_id: '7070902815602982918'
directory_id: '6975751873563557894'
title: 通知审批状态更新
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//task/notify-approval-status-update
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Task
- Notify Approval Status Update
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//task/notify-approval-status-update
---

# 通知审批状态更新
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/Attendance//task/notify-approval-status-update2)
</md-alert>
:::
对于只使用Lark考勤系统而未使用Lark审批系统的企业，可以通过该接口更新写入Lark考勤系统中的三方系统审批状态，例如请假、加班、外出、出差、补卡等审批，状态包括通过、不通过、撤销等。
:::note
发起状态的审批才可以被更新为通过、不通过，已经通过的审批才可以被更新为撤销。
:::

## 请求

| 基本                |                                                                         |
| ----------------- | ----------------------------------------------------------------------- |
| HTTP URL          | <https://open.larksuite.com/open-apis/attendance/v1/approval_infos/process> |
| HTTP Method       | POST                                                                    |
| HTTP Content-Type | application/json; charset=utf-8                                         |
| 凭证要求              | tenant_access_token                                                     |
| 权限要求              | 打卡数据导入                                                                  |

### 头部

| key           | value                      |
| ------------- | -------------------------- |
| Authorization | Bearer tenant_access_token |
| Content-Type  | application/json           |

### 请求体

| 名称            | 类型     | 必填 | 描述                                                 |
| ------------- | ------ | -- | -------------------------------------------------- |
| approval_id   | string | 是  | 审批实例 ID                                             |
| approval_type | string | 是  | 审批类型，leave：请假，out：外出，overtime：加班，trip：出差，remedy：补卡 |
| status        | int    | 是  | 审批状态，1：不通过，2：通过，4：撤销                               |

### 请求体示例

```json
{
        "approval_id" : "6737202939523236113",
        "approval_type" : "remedy",
        "status" : 4
}
```
## 响应
### 响应体

| 名称             | 类型            | 描述                     |
| -------------- | ------------- | ---------------------- |
| code           | int           | 错误码，非 0 表示失败           |
| msg            | string        | 错误描述                   |
| data           | -             | -                      |
|&emsp;∟approval_info | approval_info |                        |
|&emsp;&emsp;∟approval_id   | string        | 审批实例 ID                 |
|&emsp;&emsp;∟approval_type | string        | 审批类型，leave：请假，out：外出，overtime：加班，trip：出差，remedy：补卡         |
|&emsp;&emsp;∟status        | int           | 审批状态，1：不通过，2：通过，4：已撤销 |

### 响应体示例

```json
{ 

    "code": 0, 
    "msg": "success",
    "data": {
        "approval_info" : {
            "approval_id" : "6737202939523236113",
            "approval_type" : "remedy",
            "status" : 4
        }
    }
}
```

### 错误码

| HTTP 状态码 | 错误码     | 描述         | 排查建议                         |
| -------- | ------- | ---------- | ---------------------------- |
| 400      | 1220001 | 参数错误       | 请检查参数是否符合要求                  |
| 400      | 1220002 | 租户不存在      | 请检查 tenant_access_token 是否正确 |
| 400      | 1220004 | 用户不存在或没有权限 | 请检查用户 ID 是否正确                |
| 400      | 1220005 | 没有权限       | 请前往考勤管理后台检查数据权限范围            |
| 500      | 1225000 | 系统错误       | 详见错误信息
