---
document_id: '7026663890609389574'
directory_id: '7012181176438243333'
title: 更新群公告信息
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/patch
breadcrumb:
- Server API
- Group Chat
- Group announcement
- Update group announcement info
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:09:17Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/patch
---

# 更新群公告信息

更新会话中的群公告信息，更新公告信息的格式和更新[旧版云文档](/document/ukTMukTMukTM/uAzM5YjLwMTO24CMzkjN)格式相同，不支持新版文档格式。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=chat.announcement&method=patch)

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
- 操作者需要拥有群公告文档的阅读权限
- 获取内部群信息时，操作者须与群组在同一租户下
- 若群开启了 ==仅群主和群管理员可编辑群信息== 配置，群主/群管理员 或 创建群组且具备 ==更新应用所创建群的群信息== 权限的机器人，可更新群公告
- 若群未开启 ==仅群主和群管理员可编辑群信息== 配置，所有成员可以更新群公告
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
      <md-td>https://open.larksuite.com/open-apis/im/v1/chats/:chat_id/announcement</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>PATCH</md-td>
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
            <md-perm name="im:chat.announcement:write_only" desc="更新群公告内容" support_app_types="custom,isv" tags="">更新群公告内容</md-perm>
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
	待修改公告的群 ID，详情参见[群ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)

**注意**：不支持P2P单聊

**示例值**："oc_5ad11d72b830411d72b836c20"
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
	<md-text type="field-name" >revision</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	文档当前版本号 int64 类型，[获取群公告信息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/get)接口会返回

**注意**：传入的版本号和最新版本号的差距不能超过100

**示例值**："12"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >requests</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	修改文档请求的序列化字段

更新公告信息的格式和更新[云文档](/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN)格式相同

**示例值**：["{\"requestType\":\"InsertBlocksRequestType\",\"insertBlocksRequest\":{\"payload\":\"{\\\"blocks\\\":[{\\\"type\\\":\\\"paragraph\\\",\\\"paragraph\\\":{\\\"elements\\\":[{\\\"type\\\":\\\"textRun\\\",\\\"textRun\\\":{\\\"text\\\":\\\"Docs API Sample Content\\\",\\\"style\\\":{}}}],\\\"style\\\":{}}}]}\",\"location\":{\"zoneId\":\"0\",\"index\":0, \"endOfZone\": true}}}"]
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "revision": "12",
    "requests": [
        "{\"requestType\":\"InsertBlocksRequestType\",\"insertBlocksRequest\":{\"payload\":\"{\\\"blocks\\\":[{\\\"type\\\":\\\"paragraph\\\",\\\"paragraph\\\":{\\\"elements\\\":[{\\\"type\\\":\\\"textRun\\\",\\\"textRun\\\":{\\\"text\\\":\\\"Docs API Sample Content\\\",\\\"style\\\":{}}}],\\\"style\\\":{}}}]}\",\"location\":{\"zoneId\":\"0\",\"index\":0, \"endOfZone\": true}}}"
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


  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "msg": "success",
    "data": {}
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
  <md-td>232002</md-td>
  <md-td>No Permission: Only chat owner or admin can edit chat information in the current situation.</md-td>
  <md-td>只允许群主或群管理可以编辑群信息。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232003</md-td>
  <md-td>Chat announcement can NOT be found in chat information.</md-td>
  <md-td>群公告信息异常。</md-td>
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
  <md-td>232018</md-td>
  <md-td>Updating announcement failed.</md-td>
  <md-td>更新公告失败，请确认错误具体信息，可参考文档[编辑文档内容](/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN)。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232019</md-td>
  <md-td>The request has been rate limited.</md-td>
  <md-td>触发群限流，请控制请求的速度，详情参见[频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)。</md-td>
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
  <md-td>应用没有权限操作外部群。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232034</md-td>
  <md-td>The app is unavailable or inactivated by the tenant.</md-td>
  <md-td>应用在本租户下未安装或未启用。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232066</md-td>
  <md-td>The operator does not have the doc permission.</md-td>
  <md-td>操作者没有文档的阅读权限，请添加文档权限后重试。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>232097</md-td>
  <md-td>Unable to operate docx type chat announcement.</md-td>
  <md-td>本接口无法操作 docx 类型的群公告，请参考"更新新版本群公告"接口。</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




