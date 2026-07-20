---
document_id: '7026663890609160198'
directory_id: '7136113559830396933'
title: 创建群
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create
breadcrumb:
- Server API
- Group Chat
- Group management
- Create a group
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:09:07Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create
---

# 创建群

创建群并设置群头像、群名、群描述等。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=chat&method=create)

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
注意事项：
 - 应用需要开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
- 本接口支持在创建群的同时拉用户或机器人进群；如果仅需要拉用户或者机器人入群参考 [将用户或机器人拉入群聊](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/create)接口
- 商店应用无法使用 user_id。因此使用商店应用调用本接口时用户 ID 类型请选择 open_id 或者 union_id。
</md-alert>
:::



## 请求
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/im/v1/chats</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    <md-tr>
      <md-th>接口频率限制</md-th>
      <md-td>[1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom,isv"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
            
      </md-th>
      <md-td>
            <md-perm name="im:chat" desc="获取与更新群组信息" support_app_types="custom,isv" tags="">获取与更新群组信息</md-perm>
            <md-perm name="im:chat:create" desc="创建群" support_app_types="custom,isv" tags="">创建群</md-perm>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            字段权限要求
      </md-th>
      <md-td>
        <md-alert type="tip" icon="none">
        该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请
        </md-alert>
        <md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 35%;">名称</md-th>
      <md-th style="width: 13%;">类型</md-th>
       <md-th style="width: 15%;" filters="是,否" >必填</md-th>
      <md-th  style="width: 37%;">描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"

[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)

</md-td>
</md-tr>
<md-tr>
<md-td>Content-Type</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
</md-tbody>
</md-table>
:::



### 查询参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;" >描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >user_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户 ID 类型

**示例值**：open_id

**可选值有**：
<md-enum>
<md-enum-item key="open_id" >标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)</md-enum-item>
<md-enum-item key="union_id" >标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)</md-enum-item>
<md-enum-item key="user_id" >标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)</md-enum-item>
</md-enum>

**默认值**：`open_id`

**当值为 `user_id`，字段权限要求**：
<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >set_bot_manager</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	如果在请求体的 ==owner_id== 字段指定了某个用户为群主，可以选择是否同时设置创建此群的机器人为管理员，此标志位用于标记是否设置创建群的机器人为管理员

**示例值**：false

**默认值**：`false`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >uuid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	由开发者生成的唯一字符串序列，用于创建群组请求去重；持有相同uuid + owner_id（若有） 的请求10小时内只可成功创建1个群聊

**示例值**：b13g2t38-1jd2-458b-8djf-dtbca5104204

**数据校验规则**：

- 最大长度：`50` 字符
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 请求体

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >avatar</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	群头像对应的 Image Key，可通过[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)获取（注意：上传图片的 ==image_type== 需要指定为 ==avatar==）

**示例值**："default-avatar_44ae0ca3-e140-494b-956f-78091e348435"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	群名称

 **注意：** 
- 公开群名称的长度不得少于2个字符
- 私有群若未填写群名称，群名称默认设置为 ”`(无主题)`“

**示例值**："测试群名称"
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
	否
	</md-dt-td>
	<md-dt-td>
	群描述

**示例值**："测试群描述"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >i18n_names</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_names</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	群国际化名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >zh_cn</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	中文名

**示例值**："群聊"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >en_us</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	英文名

**示例值**："group chat"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >ja_jp</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	日文名

**示例值**："グループチャット"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >owner_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	创建群时指定的群主，不填时指定建群的机器人为群主。群主 ID值应与查询参数中的 ==user_id_type== 对应；推荐使用 OpenID，获取方式可参考文档[如何获取 Open ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)

**示例值**："ou_7d8a6e6df7621556ce0d21922b676706ccs"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >user_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	创建群时邀请的群成员，ID 类型在查询参数 ==user_id_type== 中指定；推荐使用 OpenID，获取方式可参考文档[如何获取 Open ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)

**注意**：
- 最多同时邀请 50 个用户
- 为便于在客户端查看效果，建议调试接口时加入开发者自身ID

**示例值**：["ou_7d8a6e6df7621556ce0d21922b676706ccs"]

**数据校验规则**：

- 最大长度：`50`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >bot_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	创建群时邀请的群机器人；可参考[如何获取应用的 App ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-app-id)来获取应用的App ID
 
**注意：**
- 操作此接口的机器人会自动入群，无需重复填写
- 拉机器人入群请使用`app_id`
- 最多同时邀请5个机器人，且邀请后群组中机器人数量不能超过 15 个

**示例值**：["cli_a10fbf7e94b8d01d"]

**数据校验规则**：

- 最大长度：`5`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >group_message_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	群消息形式

**示例值**："chat"

**可选值有**：
<md-enum>
<md-enum-item key="chat" >对话消息</md-enum-item>
<md-enum-item key="thread" >话题消息</md-enum-item>
</md-enum>

**默认值**：`chat`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >chat_mode</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	群模式

**可选值有**：
- `group`：群组

**示例值**："group"

**默认值**：`group`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >chat_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	群类型

**可选值有**：
- `private`：私有群
- `public`：公开群

**示例值**："private"

**默认值**：`private`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >join_message_visibility</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	入群消息可见性

**可选值有**：
- `only_owner`：仅群主和管理员可见
- `all_members`：所有成员可见
- `not_anyone`：任何人均不可见

**示例值**："all_members"

**默认值**：`all_members`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >leave_message_visibility</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	退群消息可见性

**可选值有**：
- `only_owner`：仅群主和管理员可见
- `all_members`：所有成员可见
- `not_anyone`：任何人均不可见

**示例值**："all_members"

**默认值**：`all_members`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >membership_approval</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	加群审批

**可选值有**：
- `no_approval_required`：无需审批
- `approval_required`：需要审批

**示例值**："no_approval_required"

**默认值**：`no_approval_required`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >restricted_mode_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >restricted_mode_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	保密模式设置
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	保密模式是否开启

**注意**：
- status为true时，screenshot_has_permission_setting、download_has_permission_setting、message_has_permission_setting不能全为all_members。
- status为false时，screenshot_has_permission_setting、download_has_permission_setting、message_has_permission_setting不能存在not_anyone。

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >screenshot_has_permission_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	允许截屏录屏

**示例值**："all_members"

**可选值有**：
<md-enum>
<md-enum-item key="all_members" >所有成员允许截屏录屏</md-enum-item>
<md-enum-item key="not_anyone" >所有成员禁止截屏录屏</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >download_has_permission_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	允许下载消息中图片、视频和文件

**示例值**："all_members"

**可选值有**：
<md-enum>
<md-enum-item key="all_members" >所有成员允许下载资源</md-enum-item>
<md-enum-item key="not_anyone" >所有成员禁止下载资源</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >message_has_permission_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	允许复制和转发消息

**示例值**："all_members"

**可选值有**：
<md-enum>
<md-enum-item key="all_members" >所有成员允许复制和转发消息</md-enum-item>
<md-enum-item key="not_anyone" >所有成员禁止复制和转发消息</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >urgent_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	谁可以加急

**示例值**："all_members"

**可选值有**：
<md-enum>
<md-enum-item key="only_owner" >仅群主和管理员</md-enum-item>
<md-enum-item key="all_members" >所有成员</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >video_conference_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	谁可以发起视频会议

**示例值**："all_members"

**可选值有**：
<md-enum>
<md-enum-item key="only_owner" >仅群主和管理员</md-enum-item>
<md-enum-item key="all_members" >所有成员</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >edit_permission</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	谁可以编辑群信息

**示例值**："all_members"

**可选值有**：
<md-enum>
<md-enum-item key="only_owner" >仅群主和管理员</md-enum-item>
<md-enum-item key="all_members" >所有成员</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >hide_member_count_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	隐藏群成员人数设置

**示例值**："all_members"

**可选值有**：
<md-enum>
<md-enum-item key="all_members" >所有群成员可见</md-enum-item>
<md-enum-item key="only_owner" >仅群主群管理员可见</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "avatar": "default-avatar_44ae0ca3-e140-494b-956f-78091e348435",
    "name": "测试群名称",
    "description": "测试群描述",
    "i18n_names": {
        "zh_cn": "群聊",
        "en_us": "group chat",
        "ja_jp": "グループチャット"
    },
    "owner_id": "ou_7d8a6e6df7621556ce0d21922b676706ccs",
    "user_id_list": [
        "ou_7d8a6e6df7621556ce0d21922b676706ccs"
    ],
    "bot_id_list": [
        "cli_a10fbf7e94b8d01d"
    ],
    "group_message_type": "chat",
    "chat_mode": "group",
    "chat_type": "private",
    "join_message_visibility": "all_members",
    "leave_message_visibility": "all_members",
    "membership_approval": "no_approval_required",
    "restricted_mode_setting": {
        "status": false,
        "screenshot_has_permission_setting": "all_members",
        "download_has_permission_setting": "all_members",
        "message_has_permission_setting": "all_members"
    },
    "urgent_setting": "all_members",
    "video_conference_setting": "all_members",
    "edit_permission": "all_members",
    "hide_member_count_setting": "all_members"
}
</md-code-json>
:::



## 响应





### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	错误码，非 0 表示失败
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >msg</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	错误描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >data</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >\-</md-text>
	</md-dt-td>
	<md-dt-td>
	\-
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >chat_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群 ID，详情参见：[群ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >avatar</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群头像 URL
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >description</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >i18n_names</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_names</md-text>
	</md-dt-td>
	<md-dt-td>
	群国际化名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >zh_cn</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	中文名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >en_us</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	英文名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >ja_jp</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	日文名
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >owner_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群主 ID，ID值与查询参数中的 ==user_id_type== 对应；不同 ID 的说明参见 [用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)

**注意**：当群主是机器人时，该字段不返回
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >owner_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群主 ID 对应的ID类型，与查询参数中的 ==user_id_type== 相同。取值为：`open_id`、`user_id`、`union_id`其中之一

**注意**：当群主是机器人时，该字段不返回
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >urgent_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	谁可以加急

**可选值有**：
<md-enum>
<md-enum-item key="only_owner" >仅群主和管理员</md-enum-item>
<md-enum-item key="all_members" >所有成员</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >video_conference_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	谁可以发起视频会议

**可选值有**：
<md-enum>
<md-enum-item key="only_owner" >仅群主和管理员</md-enum-item>
<md-enum-item key="all_members" >所有成员</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >add_member_permission</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	拉 用户或机器人 入群权限

**可选值有**：
- `only_owner`：仅群主和管理员
- `all_members`：所有成员
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >share_card_permission</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群分享权限

**可选值有**：
- `allowed`：允许
- `not_allowed`：不允许
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >at_all_permission</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	at 所有人权限

**可选值有**：
- `only_owner`：仅群主和管理员
- `all_members`：所有成员
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >edit_permission</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群编辑权限

**可选值有**：
- `only_owner`：仅群主和管理员
- `all_members`：所有成员
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >group_message_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群消息形式

**可选值有**：
- `chat`：对话消息
- `thread`：话题消息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >chat_mode</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群模式

**可选值有**：
- `group`：群组
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >chat_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群类型

**可选值有**：
- `private`：私有群
- `public`：公开群
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >chat_tag</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群标签，如有多个，则按照下列顺序返回第一个

**可选值有**：
- `inner`：内部群
- `tenant`：公司群
- `department`：部门群
- `edu`：教育群
- `meeting`：会议群
- `customer_service`：客服群
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >external</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否是外部群
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >tenant_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	租户在Lark上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >join_message_visibility</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	入群消息可见性

**可选值有**：
- `only_owner`：仅群主和管理员可见
- `all_members`：所有成员可见
- `not_anyone`：任何人均不可见
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >leave_message_visibility</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	出群消息可见性

**可选值有**：
- `only_owner`：仅群主和管理员可见
- `all_members`：所有成员可见
- `not_anyone`：任何人均不可见
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >membership_approval</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	加群审批

**可选值有**：
- `no_approval_required`：无需审批
- `approval_required`：需要审批
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >moderation_permission</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	发言权限

**可选值有**：
- `only_owner`：仅群主和管理员
- `all_members`：所有成员
- `moderator_list`：指定群成员
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >restricted_mode_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >restricted_mode_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	保密模式设置
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	保密模式是否开启
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >screenshot_has_permission_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	允许截屏录屏

**可选值有**：
<md-enum>
<md-enum-item key="all_members" >所有成员允许截屏录屏</md-enum-item>
<md-enum-item key="not_anyone" >所有成员禁止截屏录屏</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >download_has_permission_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	允许下载消息中图片、视频和文件

**可选值有**：
<md-enum>
<md-enum-item key="all_members" >所有成员允许下载资源</md-enum-item>
<md-enum-item key="not_anyone" >所有成员禁止下载资源</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >message_has_permission_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	允许复制和转发消息

**可选值有**：
<md-enum>
<md-enum-item key="all_members" >所有成员允许复制和转发消息</md-enum-item>
<md-enum-item key="not_anyone" >所有成员禁止复制和转发消息</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >hide_member_count_setting</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	隐藏群成员人数设置

**可选值有**：
<md-enum>
<md-enum-item key="all_members" >所有群成员可见</md-enum-item>
<md-enum-item key="only_owner" >仅群主群管理员可见</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "msg": "success",
    "data": {
        "chat_id": "oc_a0553eda9014c201e6969b478895c230",
        "avatar": "https://p3-lark-file.byteimg.com/img/lark-avatar-staging/default-avatar_44ae0ca3-e140-494b-956f-78091e348435~100x100.jpg",
        "name": "测试群名称",
        "description": "测试群描述",
        "i18n_names": {
            "zh_cn": "群聊",
            "en_us": "group chat",
            "ja_jp": "グループチャット"
        },
        "owner_id": "ou_7d8a6e6df7621556ce0d21922b676706ccs",
        "owner_id_type": "open_id",
        "urgent_setting": "all_members",
        "video_conference_setting": "all_members",
        "add_member_permission": "all members",
        "share_card_permission": "allowed",
        "at_all_permission": "all members",
        "edit_permission": "all members",
        "group_message_type": "chat",
        "chat_mode": "group",
        "chat_type": "private",
        "chat_tag": "inner",
        "external": false,
        "tenant_key": "736588c9260f175e",
        "join_message_visibility": "all_members",
        "leave_message_visibility": "all_members",
        "membership_approval": "no_approval_required",
        "moderation_permission": "all_members",
        "restricted_mode_setting": {
            "status": false,
            "screenshot_has_permission_setting": "all_members",
            "download_has_permission_setting": "all_members",
            "message_has_permission_setting": "all_members"
        },
        "hide_member_count_setting": "all_members"
    }
}
</md-code-json>
:::



### 错误码
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">HTTP状态码</md-th>
            <md-th style="width: 15%;">错误码</md-th>
            <md-th style="width: 30%;">描述</md-th>
            <md-th style="width: 30%;">排查建议</md-th>
        </md-tr>
    </md-thead>
  <md-tbody>

<md-tr>
  <md-td>400</md-td>
  <md-td>232001</md-td>
  <md-td>Your request contains an invalid request parameter.</md-td>
  <md-td>参数错误，参考本文档检查输入参数。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232004</md-td>
  <md-td>Such an app does NOT exist.</md-td>
  <md-td>作为操作者的 app_id 不存在，请联系[技术支持](https://applink.larksuite.com/TLJsX982)。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232019</md-td>
  <md-td>The request has been rate limited.</md-td>
  <md-td>触发群限流，请控制请求的速度，详情参见[频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232020</md-td>
  <md-td>Name can NOT be less than two characters for public chats.</md-td>
  <md-td>公开群名称太短，不得少于2个字符。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232021</md-td>
  <md-td>Bot can NOT be found.</md-td>
  <md-td>未找到机器人。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232022</md-td>
  <md-td>User has already resigned.	</md-td>
  <md-td>邀请的用户已离职，请检查邀请的群成员ID。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232023</md-td>
  <md-td>Chat information review failed while updating the chat.	</md-td>
  <md-td>群组相关信息审核没有通过，请检查群名称或群描述中是否存在敏感内容。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232025</md-td>
  <md-td>Bot ability is not activated.</md-td>
  <md-td>[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)未启用 。可在[开发者后台](https://open.larksuite.com/app) -> 应用能力 -> 添加应用能力页面添加机器人功能，发布新版本后生效。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232030</md-td>
  <md-td>Your request specifies a user_id which is invalid.</md-td>
  <md-td>请检查user_id是否正确。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232032</md-td>
  <md-td>The operator who will create the chat and the designated chat owner must be in the same tenant.</md-td>
  <md-td>指定的群主须与本接口的操作者在同一个租户中。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232033</md-td>
  <md-td>The operator or invited bots does NOT have the authority to manage external chats without the scope.</md-td>
  <md-td>暂不支持创建外部群。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232034</md-td>
  <md-td>The app is unavailable or inactivated by the tenant.</md-td>
  <md-td>应用在本租户下未安装或未启用。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232037</md-td>
  <md-td>The operator or invited bots does NOT have the authority to manage chat labels without the scope.</md-td>
  <md-td>操作者或受邀的机器人没有权限管理群Label，请检查权限配置。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232042</md-td>
  <md-td>Public group chat's name should not be empty.</md-td>
  <md-td>创建公开群时需要指定群名称。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232043</md-td>
  <md-td>Your request contains unavailable ids.</md-td>
  <md-td>请检查传入的 ID 列表。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232057</md-td>
  <md-td>The operator tenant doesn't have the permission to use restricted mode.</md-td>
  <md-td>操作者所属的租户没有权限使用保密模式，请联系租户管理员。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232068</md-td>
  <md-td>The group is being created, please wait a moment.</md-td>
  <md-td>群组正在创建中，请稍等。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232069</md-td>
  <md-td>current chat type unsupported to set public.</md-td>
  <md-td>不支持群类型为public的外部群。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232091</md-td>
  <md-td>Due to the security control requirements of this tenant, this tenant does not allow public group.</md-td>
  <md-td>租户管理员设置不允许创建公开群。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232095</md-td>
  <md-td>The operator does NOT have the authority to manage ocic type tag without the scope.</md-td>
  <md-td>没有权限操作ocic类型的群。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232078</md-td>
  <md-td>The operator tenant doesn't have the permission to use hide_member_count_setting.</md-td>
  <md-td>操作者所属租户无权使用隐藏群成员数量配置。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232079</md-td>
  <md-td>Chat type not supported by hide_member_count_setting.</md-td>
  <md-td>群类型不支持设置隐藏群成员数量。</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




