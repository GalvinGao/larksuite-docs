---
document_id: '6967331173081694213'
directory_id: '7122028361538912262'
title: 三方审批定义创建
full_path: /ukTMukTMukTM/uIDNyYjLyQjM24iM0IjN
breadcrumb:
- Server API
- Approval
- Approval（history version）
- v2
- Third-party approval integration
- External Approval Create
document_type: GuideDocumentType
updated_at: 2022-07-20T09:39:22Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uIDNyYjLyQjM24iM0IjN
---

# 三方审批定义创建
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/create)
</md-alert>
:::
审批定义是审批的描述，包括审批名称、图标、描述等基础信息。创建好审批定义，用户就可以在审批应用的发起页中看到审批，如果用户点击发起，则会跳转到配置的发起三方系统地址去发起审批。
<br>
另外，审批定义还配置了审批操作时的回调地址：审批人在待审批列表中进行【同意】【拒绝】操作时，审批中心会调用回调地址通知三方系统。


:::html
<md-alert type="tip">
注意，审批中心不负责审批流程的流转，只负责展示、操作、消息通知。因此审批定义创建时没有审批流程的信息。
</md-alert>
:::

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://www.larksuite.com/approval/openapi/v3/external/approval/create |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |



### 请求体

| 名称 | 类型 | 说明 | 示例 |
| --- | --- | --- | --- |
| <md-text type="field-name" >approval_name</md-text><br><md-tag mode="block" type="field-required" >必选</md-tag> | <md-text type="field-type" >string</md-text> | 审批定义名称， 创建审批定义返回的值，表示该实例属于哪个流程；该字段会影响到列表中该实例的标题，标题取自对应定义的 name 字段。 | @i18n@1 |
| <md-text type="field-name" >approval_code</md-text><br><md-tag mode="block" type="field-required" >必选</md-tag> | <md-text type="field-type" >string</md-text> | 审批定义 code，用户自定义，定义的唯一标识，如果不存在该 code，则创建，否则更新 | permission_test |
| <md-text type="field-name" >group_code</md-text><br><md-tag mode="block" type="field-required" >必选</md-tag> | <md-text type="field-type" >string</md-text> | 审批定义所属审批分组，用户自定义；<br><br>如果group_code当前不存在，则会新建审批分组；<br><br>如果group_code已经存在，则会使用group_name更新审批分组名称 | work_group |
| <md-text type="field-name" >group_name</md-text> | <md-text type="field-type" >string</md-text> | 分组名称，值的格式是 i18n key，文案放在 i18n_resource；<br>如果是 group_code 当前不存在，则该 group_name 必填，否则，如果填写了则会更新分组名称，不填则不更新分组名称；<br>审批发起页 审批定义的分组名称来自该字段 | @i18n@2 |
| <md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 审批定义的说明，值的格式是 i18n key，文案放在 i18n_resource；<br>审批发起页 审批定义的说明内容来自该字段 | @i18n@2 |
| <md-text type="field-name" >external<br><md-tag mode="block" type="field-required" >必选</md-tag><br></md-text> | <md-text type="field-type" >map</md-text> | 三方审批相关 |  |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >biz_name</md-text> | <md-text type="field-type" >string</md-text> | 列表中用于提示审批来自哪里，i18n key， 注意不需要“来自”前缀，审批中心会拼上前缀 | @i18n@3 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" > biz_type</md-text> | <md-text type="field-type" >string</md-text> | 审批定义业务类别 | permission |
| &emsp;<span style="color: #8F959E">∟ </span>&nbsp;<md-text type="field-name" >create_link_mobile<br><md-tag mode="block" type="field-required" >必选</md-tag><br></md-text> | <md-text type="field-type" >string</md-text> | 移动端发起链接，如果设置了该链接，则会在移动端审批发起页展示该审批，用户点击后会跳转到该链接进行发起；<br><br>如果不填，则在mobile端不显示该审批 | https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/approval-form/index?id=9999 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >     create_link_pc </md-text> | <md-text type="field-type" >string</md-text> | PC端发起链接，如果设置了该链接，则会在PC端审批发起页展示该审批，用户点击后会跳转到该链接进行发起；<br>如果不填，则在PC端不显示该审批； | https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc/pages/create-form/index?id=9999 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >support_pc</md-text> | <md-text type="field-type" >bool</md-text> | 审批实例、审批任务、审批抄送是否要在PC端展示，如果为 true，则PC端列表会展示该定义下的实例信息，否则，不展示<br> | true |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" ><br>support_mobile</md-text> | <md-text type="field-type" >bool</md-text> | 审批实例、审批任务、审批抄送是否要在移动端展示，如果为 true，则移动端列表会展示该定义下的实例信息，否则，不展示；<br>support_pc和support_mobile不可都为false，否则不展示<br> | true |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" ><br>support_batch_read</md-text> | 是否支持批量已读 | true |  |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" ><br>enable_mark_readed</md-text> | 是否支持标注可读 | true |  |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" ><br>enable_quick_operate</md-text> | 是否支持快速操作 | true |  |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" ><br>allow_batch_operate</md-text> | 是否支持批量审批 | true |  |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" ><br>action_callback_url</md-text> | 三方系统的操作回调 url，【待审批】列表的任务审批人点同意或拒绝操作后，审批中心调用该地址通知三方系统，回调地址相关信息可参见：/document/ukTMukTMukTM/ukjNyYjL5YjM24SO2IjN/quick-approval-callback | http://www.larksuite.com/approval/openapi/instanceOperate |  |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" ><br>action_callback_token</md-text> | 回调时带的 token， 用于业务系统验证请求来自审批,具体参考 [开放平台文档](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM) | sdjkljkx9lsadf110 |  |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" ><br>action_callback_key</md-text> | 请求参数加密密钥，如果配置了该参数，则会对请求参数进行加密，业务需要对请求进行解密，加解密算法参考[关联外部选项说明](/document/ukTMukTMukTM/uADM4QjLwADO04CMwgDN) | gfdqedvsadfgfsd |  |
| <md-text type="field-name" >viewers</md-text> | <md-text type="field-type" >list</md-text> | 可见人列表，可通知配置多个可见人，只有在配置的范围内用户可以在审批发起也看到该审批，默认不传，则是任何人不可见 |  |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >viewer_type</md-text><br><md-tag mode="block" type="field-required" ></md-tag> | <md-text type="field-type" >string</md-text> | 可见人类型<br>**可选值有：**<br>- `TENANT`：租户内可见<br>- `DEPARTMENT`：指定部门<br>- `USER`：指定用户<br>- `NONE`：任何人都不可见 | USER |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >viewer_id</md-text><br><md-tag mode="block" type="field-required" ></md-tag> | <md-text type="field-type" >string</md-text> | 可见人 ID，如果 view_type 是 TENANT 和 NONE, viewer_id 可为空;<br> 如果 view_type 为DEPARTMENT，viewer_id 是 open_department_id;<br>如果 view_type 是 USER，viewer_id 为 user_id | 19a294c2 |
| <md-text type="field-name" >i18n_resources</md-text><br><md-tag mode="block" type="field-required" >必选</md-tag> | <md-text type="field-type" >list</md-text> | 国际化文案 |  |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >locale</md-text><br><md-tag mode="block" type="field-required" >必选</md-tag> | <md-text type="field-type" >string</md-text> | 语言<br>**可选值有：**<br>- `zh-CN`：中文<br>- `en-US`：英文<br>- `ja-JP`：日文 | zh-CN |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_default</md-text><br><md-tag mode="block" type="field-required" >必选</md-tag> | <md-text type="field-type" >bool</md-text> | 是否默认语言，默认语言需要包含所有key，非默认语言如果key不存在会使用默认语言代替 | true |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >texts</md-text><br><md-tag mode="block" type="field-required" >必选</md-tag> | <md-text type="field-type" >map</md-text> | 文案 key, value,  i18n key 以  **@i18n@** 开头；<br>该字段主要用于做国际化，允许用户同时传多个语言的文案，审批中心会根据用户当前的语音环境使用对应的文案，如果没有传用户当前的语音环境文案，则会使用默认的语言文案。 | {<br>"@i18n@1": "权限申请",  <br>    "@i18n@2": "OA审批",<br>"@i18n@3": "Permission"<br>} |


### 请求体示例

```json
{
    "approval_name": "@i18n@1",
    "approval_code": "permission_test",
    "group_code": "work_group",
    "group_name": "@i18n@2",
    "external": {
        "create_link_pc": "https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc%2Fpages%2Fcreate-form%2Findex%3Fid%3D9999",
        "create_link_mobile": "https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages%2Fapproval-form%2Findex%3Fid%3D9999",
        "support_pc": true,
        "support_mobile": true,
        "support_batch_read": false,
        "action_callback_url":"http://larksuite.com/approval/openapi/operate",
        "action_callback_token":"sdjkljkx9lsadf110",
        "action_callback_key":"gfdqedvsadfgfsd",
        "enable_mark_readed": false,
        "enable_quick_operate": false,
        "allow_batch_operate": false,
        "key": "",
        "token":""
    },
    "i18n_resources":[
     {
        "locale":"zh-CN",
        "is_default":true,
         "texts":{
            "@i18n@1":"people",
             "@i18n@2":"hr",
             "@i18n@3":"HR"
         }
      }
    ],
    "viewers": [
        {
            "viewer_type": "TENANT"
        }
    ]
}
```

## 响应

### 响应体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >code</md-text> | <md-text type="field-type" >int</md-text> | 错误码，非0表示失败 |
| <md-text type="field-name" >msg</md-text> | <md-text type="field-type" >string</md-text> | 返回码的描述 |
| <md-text type="field-name" >data</md-text> | <md-text type="field-type" >map</md-text> | 返回业务信息 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >approval_code</md-text> | <md-text type="field-type" >string</md-text> | 审批定义 Code，用于发起实例 |

### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "approval_code": "C30381C8-7A5F-4717-A9CF-C233BF0202D4"
    }
}
```
