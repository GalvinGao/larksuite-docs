---
document_id: '6975751842354561029'
directory_id: '6975751873563574278'
title: 删除班次
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance//shift_delete
breadcrumb:
- Server API
- Attendance
- Attendance（Historical Version）
- API Reference
- Rule
- Delete a Shift
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance//shift_delete
---

# 删除班次
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/delete)
</md-alert>
:::

通过班次 ID 删除班次。

## 请求
|基本||
|---|---|
|HTTP URL|https://open.larksuite.com/open-apis/attendance/v1/shifts/:shift_id|
|HTTP Method|DELETE|
|HTTP Content-Type|application/json; charset=utf-8|
|凭证要求|tenant_access_token|
|权限要求|打卡管理规则写入|
### 头部
key|value
--|--
Authorization|Bearer tenant_access_token
Content-Type|application/json
### 路径参数
|名称|类型|必填|描述|
|---|---|---|---|
|shift_id|string|是|班次 ID，示例值："6919358778597097404"|
## 响应
### 响应体
|名称|类型|描述|
|---|---|---|
|code|int|错误码，非 0 表示失败|
|msg|string|错误描述|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success"
}
```
### 错误码
|HTTP 状态码|错误码|描述|排查建议|
|---|---|---|---|
|400|1220001|参数错误|请检查参数是否符合要求|
|400|1220002|租户不存在|请检查 tenant_access_token 是否正确|
|400|1220005|没有权限|请前往考勤管理后台检查数据权限范围|
|500|1225000|系统错误|详见错误信息|
|500|1226000|班次服务系统错误|详见错误信息|
|400|1226003|班次不存在|请检查 shift_id 是否正确|

