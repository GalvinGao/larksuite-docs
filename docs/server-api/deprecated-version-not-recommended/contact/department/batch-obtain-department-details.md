---
document_id: '6965400907875319814'
directory_id: '6907567266537242625'
title: 批量获取部门详情
full_path: /ukTMukTMukTM/uczN3QjL3czN04yN3cDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Department
- Batch Obtain Department Details
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:53Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uczN3QjL3czN04yN3cDN
---

# 批量获取部门详情
:::html

<md-alert type="error">

为了更好地提升该接口的安全性，我们对其进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/get)

</md-alert>

:::

该接口用于批量获取部门详情，只返回权限范围内的部门。<br>



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/contact/v1/department/detail/batch_get?department_ids=od-2efe30807a10608754862a63b108828f&department_ids=od-da6427b2adbceb91204d7fa6aeb7e8ff |
| GET | 方法 |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份访问通讯录（历史版本）</md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门基础信息 </md-perm><br><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取部门组织架构信息 </md-perm> |
| 字段权限要求<br><md-tooltip type="info">接口返回的部分字段受权限控制，开启字段权限才可获取对应字段数据；如无需获取这些字段，则无需开启。</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">根据要获取的字段开启相应权限</div> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户 user ID</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 查询参数
|参数|类型|必须|说明|
|-|-|-|-|
|department_ids|string|是|部门 ID，最多同时查询 100 条|

## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码的描述|
|data|-|返回业务信息|
|&emsp;∟department_infos|list|部门列表|
|&emsp;&emsp;∟chat_id|string|部门群 ID|
|&emsp;&emsp;∟id|string|部门 ID|
|&emsp;&emsp;∟name|string|部门名称|
|&emsp;&emsp;∟member_count|int|部门成员数量|
|&emsp;&emsp;∟parent_id|string|父部门 ID|
|&emsp;&emsp;∟status|int|部门状态，0 无效，1 有效|
|&emsp;&emsp;∟leader_employee_id|string|部门负责人 employee_id，申请了"获取用户 user_id"权限的应用返回该字段| 
|&emsp;&emsp;∟leader_open_id|string|部门负责人 open_id| 
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "department_infos": [
            {
                "chat_id": "oc_2a287650fc2928376b0f2a92c2a5431f",
                "id": "od-2091e298d8dffd6026b1670b90ca7f54",
                "leader_employee_id": "ed4g28gg",
                "leader_open_id": "ou_782168c229ccd7e4509ed09db13ba5d1",
                "member_count": 2,
                "name": "Finance",
                "parent_id": "od-2091e298d8dffd6026b1670b90ca7f66",
                "status": 1
            },
            {
                "chat_id": "oc_517b250144476ceeb8d325d5a86c8056",
                "id": "od-2efe30807a10608754862a63b108828f",
                "member_count": 11,
                "name": "Human Resources",
                "parent_id": "0",
                "status": 1
            }
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
