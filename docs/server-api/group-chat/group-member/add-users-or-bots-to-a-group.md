---
document_id: '7026663890609586182'
directory_id: '7002892512470745094'
title: 将用户或机器人拉入群聊
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/create
breadcrumb:
- Server API
- Group Chat
- Group member
- Add users or bots to a group
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:09:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/create
---

# 将用户或机器人拉入群聊

将用户或机器人拉入群聊。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=chat.members&method=create)

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
 - 如需拉用户进群，需要机器人对用户有[可用性](/document/home/introduction-to-scope-and-authorization/availability)
- 如需拉机器人进群，需要被拉的机器人开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
 - 机器人或授权用户必须在群组中
- 外部租户不能被加入到内部群中
- 操作内部群时，操作者须与群组在同一租户下
 - 在开启 ==仅群主和群管理员可添加群成员== 的设置时，仅有群主/管理员 或 创建群组且具备 ==更新应用所创建群的群信息== 权限的机器人，可以拉用户或者机器人进群
 - 在未开启 ==仅群主和群管理员可添加群成员== 的设置时，所有群成员都可以拉用户或机器人进群
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
      <md-td>https://open.larksuite.com/open-apis/im/v1/chats/:chat_id/members</md-td>
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
            <md-perm name="im:chat.members:write_only" desc="添加、移除群成员" support_app_types="custom,isv" tags="">添加、移除群成员</md-perm>
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
或
<md-tag mode="inline" type="token-user">user_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"

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



### 路径参数
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
	<md-text type="field-name" >chat_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群 ID，详情参见[群ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)

**注意**：仅支持群模式为`group`、`topic`的群组ID

**示例值**："oc_a0553eda9014c201e6969b478895c230"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
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
	<md-text type="field-name" >member_id_type</md-text>
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
<md-enum-item key="open_id" >标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)</md-enum-item>
<md-enum-item key="union_id" >标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)</md-enum-item>
<md-enum-item key="user_id" >标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)</md-enum-item>
<md-enum-item key="app_id" >Lark开放平台应用的唯一标识。在创建应用时，由系统自动生成，用户不能自行修改。[了解更多：如何获取应用的 App ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-app-id)</md-enum-item>
</md-enum>

**默认值**：`open_id`

**当值为 `user_id`，字段权限要求**：
<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >succeed_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	出现不可用ID后的处理方式 0/1/2

**默认值**：`0`

**示例值**：0

**可选值有**：
<md-enum>
<md-enum-item key="0" >兼容之前的策略，不存在/不可见的 ID 会拉群失败，并返回错误响应。存在已离职 ID 时，会将其他可用 ID 拉入群聊，返回拉群成功的响应。</md-enum-item>
<md-enum-item key="1" >将参数中可用的 ID 全部拉入群聊，返回拉群成功的响应，并展示剩余不可用的 ID 及原因。</md-enum-item>
<md-enum-item key="2" >参数中只要存在任一不可用的 ID ，就会拉群失败，返回错误响应，并展示出不可用的 ID。</md-enum-item>
</md-enum>
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
	<md-text type="field-name" >id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	成员ID列表。邀请用户进群时推荐使用 OpenID，获取方式可参考文档[如何获取 Open ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)；邀请机器人进群时需填写应用的App ID，请参考[如何获取应用的 App ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-app-id)

**注意**：
- 成员列表不可为空
- 列表中填写的成员ID类型应与 ==member_id_type== 参数中选择的类型相对应
- 每次请求最多拉50个用户且不超过群人数上限。对于已认证企业的Lark的群人数默认上限：普通群5000人，会议群3000人，话题群5000人。若租户管理员配置了群人数上限，则群人数上限为该人数上限
- 最多同时邀请5个机器人，且邀请后群组中机器人数量不能超过 15 个

**示例值**：["4d7a3c6g"]
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "id_list": [
        "4d7a3c6g"
    ]
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
	<md-text type="field-name" >invalid_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	无效成员列表

**注意**：
- 当`success_type=0`时，`invalid_id_list`只包含已离职的用户ID
- 当`success_type=1`时，`invalid_id_list`中包含已离职的、不可见的、应用未激活的ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >not_existed_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	ID不存在的成员列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >pending_approval_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	等待群主或管理员审批的成员ID列表
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
        "invalid_id_list": [
            "4d7a3c6g"
        ],
        "not_existed_id_list": [
            "4d7a3c6g"
        ],
        "pending_approval_id_list": [
            "4d7a3c6g"
        ]
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
  <md-td>232006</md-td>
  <md-td>Your request specifies a chat_id which is invalid.</md-td>
  <md-td>无效的 chat_id，请检查chat_id是否正确。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232009</md-td>
  <md-td>Your request specifies a chat which has already been dissolved.</md-td>
  <md-td>群组已被解散。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232010</md-td>
  <md-td>Operator and chat can NOT be in different tenants.</md-td>
  <md-td>操作者和被操作的群组应该在同一租户下。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232011</md-td>
  <md-td>Operator can NOT be out of the chat.</md-td>
  <md-td>操作者需要在群组中。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232013</md-td>
  <md-td>You have reached the limit of maximum number of members a chat can have.</md-td>
  <md-td>加入群组时，群成员数量已达到上限。对于已认证企业的Lark的群人数默认上限：普通群5000人，会议群3000人，话题群5000人。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232017</md-td>
  <md-td>No Permission: If the operator is NOT owner or creator with the scope, the operator can NOT complete the request.</md-td>
  <md-td>操作者在群中不具备群主、群创建者等身份，无法完成本操作。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232019</md-td>
  <md-td>The request has been rate limited.</md-td>
  <md-td>触发群限流，请控制请求的速度，详情参见[频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232024</md-td>
  <md-td>Users do not have the visibility of the app, or the operator does not have collaboration permissions with the target users.</md-td>
  <md-td>机器人对用户没有可见性，或操作者与用户间没有协作权限。前者可在[开发者后台](https://open.larksuite.com/app)-应用发布-版本管理与发布 编辑应用对用户的可见性并发布；后者请检查是否与目标用户有协作权限，如屏蔽、未添加未联系人等。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232025</md-td>
  <md-td>Bot ability is not activated.</md-td>
  <md-td>应用需要开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232027</md-td>
  <md-td>There are no valid members in the ID list specified in your request.</md-td>
  <md-td>成员ID列表为空或不存在有效的成员。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232028</md-td>
  <md-td>External members can Not be added to an internal group chat.</md-td>
  <md-td>外租户成员不能被加入内部群。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232033</md-td>
  <md-td>The operator or invited bots does NOT have the authority to manage external chats without the scope.</md-td>
  <md-td>没有权限操作外部群，且暂不支持申请该权限。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232034</md-td>
  <md-td>The app is unavailable or inactivated by the tenant.</md-td>
  <md-td>应用在本租户下未安装或未启用。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232043</md-td>
  <md-td>Your request contains unavailable ids.</md-td>
  <md-td>请检查传入的 ID 列表。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232044</md-td>
  <md-td>You have reached maximum number of chat members set by admin.</md-td>
  <md-td>群成员人数已达到租户管理员配置的上限，如需提高上限请向租户管理员申请。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232090</md-td>
  <md-td>Unsupported chat type.</md-td>
  <md-td>不支持的群类型，无法完成本操作。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>99992351</md-td>
  <md-td>Your request contains not existed id.</md-td>
  <md-td>部分 open_id 不存在，请检查后重试。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>99992360</md-td>
  <md-td>Your request contains not existed id.</md-td>
  <md-td>部分 user_id 不存在，请检查后重试。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>99992364</md-td>
  <md-td>Your request contains not existed id.</md-td>
  <md-td>部分 union_id 不存在，请检查后重试。</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




