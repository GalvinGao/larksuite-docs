---
document_id: '7026663890609537030'
directory_id: '7002892512470597638'
title: 获取消息中的资源文件
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-resource/get
breadcrumb:
- Server API
- Messaging
- Message management
- Obtain resource files in messages
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:08:21Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-resource/get
---

# 获取消息中的资源文件

获取消息中的资源文件，包括音频，视频，图片和文件，**暂不支持表情包资源下载**。当前仅支持 100M 以内的资源文件的下载。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=message.resource&method=get)

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
文件类型可通过 response header 中`Content-Type`字段获取
</md-alert>
:::

## 前提条件
调用接口前，请确保： 
- 你已为应用开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
- 机器人和消息已在同一会话中

## 使用限制

该接口暂不支持获取合并转发消息中的子消息的资源文件。

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
      <md-td>https://open.larksuite.com/open-apis/im/v1/messages/:message_id/resources/:file_key</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
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
            <md-perm name="im:message" desc="获取与发送单聊、群组消息" support_app_types="custom,isv" tags="">获取与发送单聊、群组消息</md-perm>
            <md-perm name="im:message:readonly" desc="获取单聊、群组消息" support_app_types="custom,isv" tags="">获取单聊、群组消息</md-perm>
            <md-perm name="im:message.history:readonly" desc="获取单聊、群组的历史消息" support_app_types="custom" tags="history">获取单聊、群组的历史消息</md-perm>
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
	<md-text type="field-name" >message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	待查询资源对应的消息ID。

**示例值**："om_dc13264520392913993dd051dba21dcf"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >file_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	待查询资源的key。可以调用[获取指定消息的内容](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/get)接口，通过消息ID查询消息内容中的资源Key。

**注意**：请求的 file_key 和 message_id 需要匹配

**示例值**："file_456a92d6-c6ea-4de4-ac3f-7afcf44ac78g"
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
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	资源类型

 **可选值有：**
- `image`: 对应消息中的图片或富文本消息中的图片。
- `file`: 对应消息中的 文件、音频、视频（表情包除外）。

**示例值**：image
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::

### 请求示例
```bash
curl --location --request GET 'https://open.larksuite.com/open-apis/im/v1/messages/om_aa0944122e2ec23545d9a93df0ceec02/resources/img_v2_98f3923d-3351-4017-8156-ee34d92196bj?type=image' \
--header 'Authorization: Bearer t-9fef198b83bfdb975f09297cf14c4a63a117c52e' 
```
```bash
curl --location --request GET 'https://open.larksuite.com/open-apis/im/v1/messages/om_4b6abce3eb423be3c2dfd11d4ca9e97b/resources/file_v2_83e69b47-5d67-4e21-bfb4-66f8bf80b7ej?type=file' \
--header 'Authorization: Bearer t-9fef198b83bfdb975f09297cf14c4a63a117c52e' 
```



## 响应



HTTP状态码为 200 时，表示成功

返回文件二进制流



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
  <md-td>230110</md-td>
  <md-td>Action unavailable as the message has been deleted.</md-td>
  <md-td>消息已经删除.</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234001</md-td>
  <md-td>Invalid request param.</md-td>
  <md-td>检查请求参数是否正确。</md-td>
</md-tr>


<md-tr>
  <md-td>401</md-td>
  <md-td>234002</md-td>
  <md-td>Unauthorized.</md-td>
  <md-td>鉴权失败，联系Oncall解决。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234003</md-td>
  <md-td>File not in message.</md-td>
  <md-td>该资源不属于当前消息，请检查消息ID和资源Key。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234004</md-td>
  <md-td>App not in chat.</md-td>
  <md-td>应用不在消息所在的群组中，请检查消息ID是否正确。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234009</md-td>
  <md-td>Lack of necessary permissions.</md-td>
  <md-td>暂不支持在外部群中进行本操作。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234019</md-td>
  <md-td>scope CheckAppTenant fail.</md-td>
  <md-td>请重试。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234037</md-td>
  <md-td>Downloaded file size exceeds limit.</md-td>
  <md-td>不允许下载的资源大小超过100MB限制。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234038</md-td>
  <md-td>Do not allow downloading of message resources in restricted mode.</md-td>
  <md-td>不能下载保密消息中资源文件，请检查消息是否已被设置为保密，或群组开启了防泄密模式。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234040</md-td>
  <md-td>The message is invisible to the operator.</md-td>
  <md-td>该消息对操作者不可见，请联系群主或群管理员检查群设置中是否关闭了”新成员可查看历史消息“。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234041</md-td>
  <md-td>Tenant master key has been deleted, please contact the tenant administrator.</md-td>
  <md-td>租户加密密钥被删除，请联系租户管理员。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234042</md-td>
  <md-td>Hybrid deployment tenant storage error, such as full storage space, please contact tenant administrator.</md-td>
  <md-td>请求出现混布租户存储错误，如存储空间已满等，请联系租户管理员或技术支持。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>234043</md-td>
  <md-td>Unsupported message type.</md-td>
  <md-td>不支持的消息类型，如合并转发消息、消息卡片。</md-td>
</md-tr>


  </md-tbody>
</md-table>
:::




