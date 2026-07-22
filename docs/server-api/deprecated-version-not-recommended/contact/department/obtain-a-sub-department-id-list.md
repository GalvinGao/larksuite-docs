---
document_id: '6965400907878006789'
directory_id: '6907567266537242625'
title: 获取子部门 ID 列表
full_path: /ukTMukTMukTM/ukjNz4SO2MjL5YzM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Department
- Obtain a Sub-department ID List
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:47Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukjNz4SO2MjL5YzM
---

# 获取子部门 ID 列表
该接口用于获取子部门 ID 列表，只返回授权的部门列表。  <br>
企业根部门 ID 为 0，当参数指定的部门为根部门时，如果授权范围为全员，返回该企业的所有一级部门；否则返回管理员在设置通讯录授权范围时勾选的部门（不包含子部门）。<br>
指定获取特定部门（部门 ID 非 0）的部门列表时，需要具有该部门的通讯录授权，返回部门列表为该部门所有的子部门。<br>



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v1/department/list?department_id=od-c02cc3b685a711db3a0f14fc4cdb76dc |
| HTTP Method | GET |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 获取部门组织架构信息 </md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 读取通讯录 </md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 查询参数
|参数|类型|必须|说明|
|-|-|-|-|
|department_id|string|是|部门 ID|

## 响应

### 响应体

|参数|说明|
|-|-|
|code|错误码，非 0 表示失败|
|msg|返回码的描述|
|data|返回业务数据|
|&emsp;∟departments_list|部门列表|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "departments_list": [
            "od-a140b4eeb892b90a0ab3e616fc2054d6",
            "od-c02cc3b685a711db3a0f14fc4cdb76dc",
            "od-5a921e75b6adfb9f607bbaa40e50ba24",
            "od-fa83d3690de01787618b85bb27a013bc"
        ]
    }

}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

