---
document_id: '7070902815602720774'
directory_id: '7070770034936152069'
title: 删除班次
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/delete
breadcrumb:
- Server API
- Attendance
- Attendance Shift
- Delete Attendance Shift
document_type: ReferenceDocumentType
updated_at: 2022-03-16T08:28:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/delete
---

# 删除班次

通过班次 ID 删除班次。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=shift&method=delete)

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
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/shifts/:shift_id |
| HTTP Method | DELETE |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="attendance:rule" desc="写入打卡管理规则" support_app_types="custom" tags="">写入打卡管理规则</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |




### 路径参数

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >shift_id</md-text> | <md-text type="field-type" >string</md-text> | 班次 ID，获取方式：1）[按名称查询班次](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query) 2）[创建班次](/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/create)<br>**示例值**："6919358778597097404" |






## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {

    }
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1220001 | 参数错误 | 请检查参数是否符合要求 |
| 400 | 1220002 | 租户不存在 | 请检查 tenant_access_token 是否正确 |
| 400 | 1220005 | 没有权限 | 请前往[考勤管理后台](https://oa.larksuite.com/attendance/manage/member/list)检查数据权限范围 |
| 500 | 1225000 | 系统错误 | 详见错误信息 |
| 500 | 1226000 | 班次服务系统错误 | 详见错误信息 |
| 500 | 1226001 | 班次已被使用 | 请修改班次名称 |
| 400 | 1226003 | 班次不存在 | 请检查 shift_id 是否正确 |





