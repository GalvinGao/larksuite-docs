---
document_id: '7035816914552324102'
directory_id: '7021842990278148101'
title: 查询批量消息推送和阅读人数
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/batch_message/read_user
breadcrumb:
- Server API
- Messaging
- Batch message
- Query the number of pushers and readers of batch messages
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:08:34Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/batch_message/read_user
---

# 查询批量消息推送和阅读人数

发送批量消息请求后，可以通过该接口查询批量消息推送的总人数和阅读人数。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=batch_message&method=read_user)

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
- 只能查询**30天内**通过[批量发送消息](/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM)接口产生的消息
- 该接口返回的数据为查询时刻的快照数据
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/im/v1/batch_messages/:batch_message_id/read_user |
| HTTP Method | GET |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="im:message:send_multi_depts" desc="给一个或多个部门的成员批量发消息" support_app_types="custom,isv" tags="">给一个或多个部门的成员批量发消息</md-perm><br><md-perm name="im:message:send_multi_users" desc="给多个用户批量发消息" support_app_types="custom,isv" tags="">给多个用户批量发消息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




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
	<md-text type="field-name" >batch_message_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	待查询的批量消息任务 ID，通过调用[批量发送消息接口](	/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM)的返回值`message_id`中得到

**示例值**："bm_dc13264520392913993dd051dba21dcf"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::

### 请求示例
```
curl --location --request GET 'https://open.larksuite.com/open-apis/im/v1/batch_messages/bm_dc13264520392913993dd051dba21dcf/read_user' \
--header 'Authorization: Bearer t-e811da948d9a2803433be10458f650c842799970'
```



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
	<md-text type="field-name" >read_user</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >batch_message_read_user</md-text>
	</md-dt-td>
	<md-dt-td>
	批量发送消息的用户阅读情况
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >read_count</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	已读的人数
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >total_count</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	推送的总人数
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
    "data": {
        "read_user": {
            "read_count": "10",
            "total_count": "100"
        }
    },
    "msg": "ok"
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 230001 | Your request contains an invalid request parameter. | 参数错误，请根据接口返回的错误信息并参考文档检查输入参数。 |
| 400 | 230012 | Bot is NOT the sender of the message. | 机器人不是消息的发送者。 |
| 400 | 230027 | Lack of necessary permissions. | 请根据本文档中的权限要求部分补充所需权限。 |
| 400 | 230030 | The message id is invalid, record not found. | 检查传入的batch_message_id是否正确。 |





