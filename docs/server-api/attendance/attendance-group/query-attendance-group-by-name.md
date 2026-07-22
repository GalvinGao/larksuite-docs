---
document_id: '7070902815602819078'
directory_id: '7070770034936086533'
title: 按名称查询考勤组
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/search
breadcrumb:
- Server API
- Attendance
- Attendance Group
- Query Attendance Group by Name
document_type: ReferenceDocumentType
updated_at: 2022-03-03T15:54:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/search
---

# 按名称查询考勤组

按考勤组名称查询考勤组摘要信息。查询条件支持名称精确匹配和模糊匹配两种方式。查询结果按考勤组修改时间 desc 排序，且最大记录数为 10 条。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=attendance&version=v1&resource=group&method=search)

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
该接口依赖的数据和考勤组主数据间存在数据同步延时（正常数据同步 2 秒以内），因此在使用该接口时需注意评估数据延迟潜在风险。
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/attendance/v1/groups/search |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="attendance:rule" desc="写入打卡管理规则" support_app_types="custom" tags="">写入打卡管理规则</md-perm><br><md-perm name="attendance:rule:readonly" desc="导出打卡管理规则" support_app_types="custom" tags="">导出打卡管理规则</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 请求体

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| <md-text type="field-name" >group_name</md-text> | <md-text type="field-type" >string</md-text> | 是 | 考勤组名称<br>**示例值**："考勤组1" |
| <md-text type="field-name" >exactly_matched</md-text> | <md-text type="field-type" >boolean</md-text> | 否 | 是否精准匹配，默认为 false：模糊匹配；true：精准匹配<br>**示例值**：true |




### 请求体示例

```json
{
    "group_name": "考勤组1",
    "exactly_matched": true
}
```



## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >group_list</md-text> | <md-text type="field-type" >group_meta\[\]</md-text> | 考勤组列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >group_id</md-text> | <md-text type="field-type" >string</md-text> | 考勤组 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >group_name</md-text> | <md-text type="field-type" >string</md-text> | 考勤组名称 |




### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "group_list": [
            {
                "group_id": "6919358128597097404",
                "group_name": "考勤组1"
            }
        ]
    }
}
```



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1220001 | 参数错误 | 请检查参数是否符合要求 |
| 400 | 1220002 | 租户不存在 | 请检查 tenant_access_token 是否正确 |





