---
document_id: '7139727755097587717'
directory_id: '7122028361538863110'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/overview
breadcrumb:
- Server API
- Approval
- Third-party approval definition
- Resource introduction
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:58Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/overview
---

# 资源介绍
三方审批定义是审批的描述，包括审批名称、图标、描述、分组等基础信息。三方将根据三方审批定义来创建和同步[三方审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/overview)

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: ;">名称</md-dt-th>
      <md-dt-th style="width: ;">类型</md-dt-th>

      <md-dt-th style="width: ;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>
    <md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >approval_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
		
审批定义名称，创建审批定义返回的值，表示该实例属于哪个流程；该字段会影响到列表中该实例的标题，标题取自对应定义的 name 字段。

**示例值**："@i18n@1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >approval_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批定义 code，用户自定义，定义的唯一标识，如果不存在该 code，则创建，否则更新

**示例值**："permission_test"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >group_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
		
审批定义所属审批分组，用户自定义； 如果group_code当前不存在，则会新建审批分组； 如果group_code已经存在，则会使用group_name更新审批分组名称

**示例值**："work_group"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >group_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
		
分组名称，值的格式是 i18n key，文案放在 i18n_resource； 如果是 group_code 当前不存在，则该 group_name 必填，否则，如果填写了则会更新分组名称，不填则不更新分组名称； 审批发起页 审批定义的分组名称来自该字段

**示例值**："@i18n@2"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >description</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	审批定义的说明，值的格式是 i18n key，文案放在 i18n_resource； 审批发起页 审批定义的说明内容来自该字段

**示例值**："@i18n@2"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >external</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_create_external</md-text>
	</md-dt-td>

	<md-dt-td>
	三方审批相关
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >biz_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	列表中用于提示审批来自哪里，i18n key， 注意不需要“来自”前缀，审批中心会拼上前缀

**示例值**："@i18n@3"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >biz_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
		
审批定义业务类别

**示例值**："permission"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >create_link_mobile</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
		
移动端发起链接，如果设置了该链接，则会在移动端审批发起页展示该审批，用户点击后会跳转到该链接进行发起； 如果不填，则在mobile端不显示该审批

**示例值**："https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/approval-form/index?id=9999"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >create_link_pc</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	PC端发起链接，如果设置了该链接，则会在PC端审批发起页展示该审批，用户点击后会跳转到该链接进行发起； 如果不填，则在PC端不显示该审批；

**示例值**："https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc/pages/create-form/index?id=9999"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >support_pc</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
		
审批实例、审批任务、审批抄送是否要在PC端展示，如果为 true，则PC端列表会展示该定义下的实例信息，否则，不展示

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >support_mobile</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
		
审批实例、审批任务、审批抄送是否要在移动端展示，如果为 true，则移动端列表会展示该定义下的实例信息，否则，不展示； support_pc和support_mobile不可都为false，否则不展示

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >support_batch_read</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
		
是否支持批量已读

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >enable_mark_readed</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	是否支持标注可读（该字段无效）

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >enable_quick_operate</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	是否支持快速操作

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >action_callback_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	三方系统的操作回调 url，【待审批】列表的任务审批人点同意或拒绝操作后，审批中心调用该地址通知三方系统，回调地址相关信息可参见：[三方审批快捷审批回调](/document/ukTMukTMukTM/ukjNyYjL5YjM24SO2IjN/quick-approval-callback)

**示例值**："http://www.larksuite.com/approval/openapi/instanceOperate"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >action_callback_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	回调时带的 token， 用于业务系统验证请求来自审批,具体参考 [开放平台文档](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)

**示例值**：""
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >action_callback_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	请求参数加密密钥，如果配置了该参数，则会对请求参数进行加密，业务需要对请求进行解密，加解密算法参考 [关联外部选项说明](/document/ukTMukTMukTM/uADM4QjLwADO04CMwgDN)

**示例值**："sdjkljkx9lsadf110"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >allow_batch_operate</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	是否支持批量审批

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >viewers</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >approval_create_viewers\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
	可见人列表，可通知配置多个可见人，只有在配置的范围内用户可以在审批发起也看到该审批，默认不传，则是任何人不可见
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >viewer_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	可见人类型

**示例值**："USER"

**可选值有**：
<md-enum>
<md-enum-item key="TENANT" >租户内可见</md-enum-item>
<md-enum-item key="DEPARTMENT" >指定部门</md-enum-item>
<md-enum-item key="USER" >指定用户</md-enum-item>
<md-enum-item key="NONE" >任何人都不可见</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >viewer_user_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	当 view_type 是 USER，填写user_id，open_id或者union_id

**示例值**："19a294c2"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >viewer_department_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	当 view_type 为DEPARTMENT，填写department_id或者open_department_id

**示例值**："od-ac9d697abfa990b715dcc33d58a62a9d"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >i18n_resources</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_resource\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
	国际化文案
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >locale</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	语言可选值有： zh-CN：中文 en-US：英文 ja-JP：日文

**示例值**："zh-CN"

**可选值有**：
<md-enum>
<md-enum-item key="zh-CN" >中文</md-enum-item>
<md-enum-item key="en-US" >英文</md-enum-item>
<md-enum-item key="ja-JP" >日文</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >texts</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_resource_text\[\]</md-text>
	</md-dt-td>

	<md-dt-td>
	文案 key, value, i18n key 以 @i18n@ 开头； 该字段主要用于做国际化，语序用户同时传多个语言的文案，审批中心会根据用户当前的语音环境使用对应的文案，如果没有传用户当前的语音环境文案，则会使用默认的语言文案。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	文案key

**示例值**："@i18n@1"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	文案

**示例值**："people"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >is_default</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>

	<md-dt-td>
	是否默认语言，默认语言需要包含所有key，非默认语言如果key不存在会使用默认语言代替

**示例值**：true
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::


## 数据示例

```json
{
    "approval_name": "@i18n@1",
    "approval_code": "permission_test",
    "group_code": "work_group",
    "group_name": "@i18@2",
    "external": {
        "create_link_pc": "https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc%2Fpages%2Fcreate-form%2Findex%3Fid%3D9999",
        "create_link_mobile": "https://applink.larksuite.com/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages%2Fapproval-form%2Findex%3Fid%3D9999",
        "support_pc": true,
        "support_mobile": true,
        "support_batch_read": false,
        "action_callback_url":"http://www.larksuite.com/approval/openapi/operate",
        "action_callback_token":"sdjkljkx9lsadf110",
        "action_callback_key":"gfdqedvsadfgfsd",
        "enable_mark_readed": false,
        "key": "",
        "token":""
    },
    "i18n_resources":[
     {
        "locale":"zh-CN",
        "is_default":true,
         "texts":[
             {
             "key": "@i18n@1",
             "value":"people"  
            },
            {
             "key": "@i18n@2",
             "value":"hr"  
            }
         ]
      }
    ],
    "viewers": [
        {
            "viewer_type": "TENANT"
        }
    ]
}

```

