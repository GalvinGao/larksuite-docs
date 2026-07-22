---
document_id: '7134990230498705413'
directory_id: '7130513101061423109'
title: 会话标签页排序
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/sort_tabs
breadcrumb:
- Server API
- Group Chat
- Chat tab
- Sort chat tab
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:09:18Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/sort_tabs
---

# 会话标签页排序

会话标签页排序。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=chat.tab&method=sort_tabs)

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
- 机器人或授权用户必须在群里
- 消息标签页强制固定为第一顺位，不参与排序，但是请求体中必须包含该标签页的Tab ID
- 操作内部群时，操作者须与群组在同一租户下
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/im/v1/chats/:chat_id/chat_tabs/sort_tabs |
| HTTP Method | POST |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="im:chat" desc="获取与更新群组信息" support_app_types="custom,isv" tags="">获取与更新群组信息</md-perm><br><md-perm name="im:chat.tabs:write_only" desc="操作群会话标签页" support_app_types="custom,isv" tags="">操作群会话标签页</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




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
	群ID，详情参见[群ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)

**注意**：支持群模式为`p2p`与`group`的群ID

**示例值**："oc_a0553eda9014c201e6969b478895c230"
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
	<md-text type="field-name" >tab_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	会话标签页ID列表，Tab ID可以在[添加会话标签页](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/create)与[拉取会话标签页](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/list_tabs)的返回值中获取

**注意**：必须包含该会话中全部的Tab ID

**示例值**：["6936075528890826780"]
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "tab_ids": [
        "6936075528890826780"
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
	<md-text type="field-name" >chat_tabs</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >chat.tab\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	会话标签页
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >tab_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Tab ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >tab_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Tab名称

**注意**：会话标签页的名称不能超过30个字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >tab_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Tab类型

**可选值有**：
<md-enum>
<md-enum-item key="message" >消息类型</md-enum-item>
<md-enum-item key="doc_list" >云文档列表</md-enum-item>
<md-enum-item key="doc" >文档</md-enum-item>
<md-enum-item key="pin" >Pin</md-enum-item>
<md-enum-item key="meeting_minute" >会议纪要</md-enum-item>
<md-enum-item key="chat_announcement" >群公告</md-enum-item>
<md-enum-item key="url" >URL</md-enum-item>
<md-enum-item key="file" >文件</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >tab_content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >chat_tab_content</md-text>
	</md-dt-td>
	<md-dt-td>
	Tab内容
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	URL类型
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >doc</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	Doc链接
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >meeting_minute</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	会议纪要
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >tab_config</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >chat_tab_config</md-text>
	</md-dt-td>
	<md-dt-td>
	Tab的配置
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >icon_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	群Tab图标
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >is_built_in</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	群tab是否App内嵌打开
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
    "msg": "ok",
    "data": {
        "chat_tabs": [
            {
                "tab_content": {},
                "tab_id": "7104164142520467475",
                "tab_type": "message"
            },
            {
                "tab_content": {},
                "tab_id": "7104164246245605395",
                "tab_type": "pin"
            },
            {
                "tab_content": {
                    "url": "https://www.larksuite.com/"
                },
                "tab_id": "7104168465417633811",
                "tab_name": "url2",
                "tab_type": "url"
            },
            {
                "tab_content": {
                    "doc": "https://example.larksuite.com/docx/doxbcjoYDoEtuwC0k0hryQBkSV1"
                },
                "tab_id": "7104168465379885076",
                "tab_name": "doc2",
                "tab_type": "doc"
            },
            {
                "tab_content": {
                    "url": "https://www.larksuite.com/"
                },
                "tab_id": "7104168141097287699",
                "tab_name": "url1",
                "tab_type": "url"
            },
            {
                "tab_content": {
                    "doc": "https://example.larksuite.com/docx/doxbcjoYDoEtuwC0k0hryQBkSV1"
                },
                "tab_id": "7104168141063716884",
                "tab_name": "doc1",
                "tab_type": "doc"
            }
        ]
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 232001 | Your request contains an invalid request parameter. | 参数错误，参考本文档检查输入参数。 |
| 400 | 232006 | Your request specifies a chat_id which is invalid. | 无效的 chat_id，请检查chat_id是否正确。 |
| 400 | 232009 | Your request specifies a chat which has already been dissolved. | 群组已被解散。 |
| 400 | 232010 | Operator and chat can NOT be in different tenants. | 操作者和被操作的群组应该在同一租户下。 |
| 400 | 232011 | Operator can NOT be out of the chat. | 操作者需要在群组中。 |
| 400 | 232025 | Bot ability is not activated. | 应用需要开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。 |
| 400 | 232033 | The operator or invited bots does NOT have the authority to manage external chats without the scope. | 没有权限操作外部群。 |
| 400 | 232034 | The app is unavailable or inactivated by the tenant. | 应用在本租户下未安装或未启用。 |
| 400 | 232050 | Operate chat tab unsupported chat type. | 请求的群类型不支持会话标签页。 |
| 400 | 232055 | The operator does not have chat tab, chat menu, chat widget manage permission | 没有会话标签页、会话菜单和小组件的管理权限，请检查群设置中“谁可以给管理会话标签页、会话菜单和小组件”是否开启了“仅群主和管理员”。 |





