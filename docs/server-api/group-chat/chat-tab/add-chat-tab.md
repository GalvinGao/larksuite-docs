---
document_id: '7134990230498639877'
directory_id: '7130513101061423109'
title: 添加会话标签页
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/create
breadcrumb:
- Server API
- Group Chat
- Chat tab
- Add chat tab
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:09:17Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/create
---

# 添加会话标签页

添加自定义会话标签页。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=chat.tab&method=create)

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
- 只允许添加类型为`doc`和`url`的会话标签页
- 添加doc类型时，操作者（access token对应的身份）需要拥有对应文档的权限
- tab_config字段当前只对`url`类型的会话标签页生效
- 在开启 ==仅群主和管理员可管理标签页== 的设置时，仅群主和群管理员可以添加会话标签页
- 操作内部群时，操作者须与群组在同一租户下
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
      <md-td>https://open.larksuite.com/open-apis/im/v1/chats/:chat_id/chat_tabs</md-td>
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
            <md-perm name="im:chat.tabs:write_only" desc="操作群会话标签页" support_app_types="custom,isv" tags="">操作群会话标签页</md-perm>
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
	<md-text type="field-name" >chat_tabs</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >chat.tab\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	会话标签页

**注意**：一个群内最多只允许添加20个自定义会话标签页
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >tab_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	Tab名称

**注意**：会话标签页的名称不能超过30个字符（最多 10 个汉字）

**示例值**："文档"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >tab_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	Tab类型

**示例值**："doc"

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


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >tab_content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >chat_tab_content</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	Tab内容
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	URL类型

**示例值**："https://www.larksuite.com"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >doc</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	Doc链接

**示例值**："https://example.larksuite.com/wiki/wikcnPIcqWjJQwkwDzrB9t40123xz"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >meeting_minute</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	会议纪要

**示例值**："https://example.larksuite.com/docs/doccnvIXbV22i6hSD3utar4123dx"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >tab_config</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >chat_tab_config</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	Tab的配置
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >icon_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	群Tab图标

**示例值**："img_v2_b99741-7628-4abd-aad0-b881e4db83ig"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >is_built_in</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	群tab是否App内嵌打开

**示例值**：false
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "chat_tabs": [
        {
            "tab_name": "文档",
            "tab_type": "doc",
            "tab_content": {
                "url": "https://www.larksuite.com",
                "doc": "https://example.larksuite.com/wiki/wikcnPIcqWjJQwkwDzrB9t40123xz",
                "meeting_minute": "https://example.larksuite.com/docs/doccnvIXbV22i6hSD3utar4123dx"
            },
            "tab_config": {
                "icon_key": "img_v2_b99741-7628-4abd-aad0-b881e4db83ig",
                "is_built_in": false
            }
        }
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
               "tab_id": "7101214603622940633",
               "tab_type": "message"
            },
            {
                "tab_id": "7101214603622940671",
                "tab_name": "文档",
                "tab_type": "doc",
                "tab_content": {
                    "doc": "https://example.larksuite.com/wiki/wikcnPIcqWjJQwkwDzrB9t40123xz"
                }
            },
            {
                "tab_id": "7158333373373759422",
                "tab_name": "测试",
                "tab_type": "url",
                "tab_content": {
                    "url": "https://www.test.cn"
                },
                "tab_config": {
                    "icon_key": "img_v2_b99741-7628-4abd-aad0-b881e4db83ig",
                    "is_built_in": true
                }
            }
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
  <md-td>232025</md-td>
  <md-td>Bot ability is not activated.</md-td>
  <md-td>应用需要开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232033</md-td>
  <md-td>The operator or invited bots does NOT have the authority to manage external chats without the scope.</md-td>
  <md-td>没有权限操作外部群。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232034</md-td>
  <md-td>The app is unavailable or inactivated by the tenant.</md-td>
  <md-td>应用在本租户下未安装或未启用。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232046</md-td>
  <md-td>The number of chat tabs has reached the limit.</md-td>
  <md-td>自定义的会话标签页个数超过上限20个。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232047</md-td>
  <md-td>The length of the tab name reaches the limit.</md-td>
  <md-td>会话标签页名称过长。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232048</md-td>
  <md-td>The chat tab content is illegal.</md-td>
  <md-td>会话标签页内容非法。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232050</md-td>
  <md-td>Operate chat tab unsupported chat type.</md-td>
  <md-td>请求的群类型不支持会话标签页。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232051</md-td>
  <md-td>The operator does not have doc permission.</md-td>
  <md-td>操作者必须拥有文档权限。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232055</md-td>
  <md-td>The operator does not have chat tab, chat menu, chat widget manage permission</md-td>
  <md-td>没有会话标签页、会话菜单和小组件的管理权限，请检查群设置中“谁可以给管理会话标签页、会话菜单和小组件”是否开启了“仅群主和管理员”。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232056</md-td>
  <md-td>The operator is not the image's owner, no permission to complete the request.</md-td>
  <md-td>机器人需要使用自己上传的图片。</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




