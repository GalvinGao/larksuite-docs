---
document_id: '6967331158355410950'
directory_id: '6956151028698021894'
title: 获取机器人信息
full_path: /ukTMukTMukTM/uAjMxEjLwITMx4CMyETM
breadcrumb:
- Server API
- Bot
- Obtain Bot Info
document_type: GuideDocumentType
updated_at: 2022-03-10T13:24:36Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uAjMxEjLwITMx4CMyETM
---

# 获取机器人信息

获取机器人的基本信息。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bot&version=v3&resource=bot&method=get)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">
需要启用机器人能力
</md-alert>
:::

:::html
<md-alert type="tip">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/bot/v3/info |
| HTTP Method | GET |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | 无 |



### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |






## 响应



### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非 0 表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 错误描述 |
| <md-text type="field-name" >bot</md-text> | <md-text type="field-type" >bot_info</md-text> | 机器人信息 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >activate_status</md-text> | <md-text type="field-type" >int</md-text> | app 当前状态。<br>0: 初始化，租户待安装<br>1: 租户停用<br>2: 租户启用<br>3: 安装后待启用<br>4: 升级待启用<br>5: license过期停用<br>6: Lark套餐到期或降级停用 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_name</md-text> | <md-text type="field-type" >string</md-text> | app 名称 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >avatar_url</md-text> | <md-text type="field-type" >string</md-text> | app 图像地址 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ip_white_list</md-text> | <md-text type="field-type" >string\[\]</md-text> | app 的 IP 白名单地址 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_id</md-text> | <md-text type="field-type" >string</md-text> | 机器人的open_id |




### 响应体示例

```json
{
    "code":0,
    "msg":"ok",
    "bot":{
        "activate_status":2,
        "app_name":"name",
        "avatar_url":"https://xxxxxxxxxxxx",
        "ip_white_list":[

        ],
        "open_id":"ou_e6e14f667cfe239d7b129b521dce0569"
    }
}
```

