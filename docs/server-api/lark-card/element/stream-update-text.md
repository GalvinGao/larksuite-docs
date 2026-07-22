---
document_id: '7520565507028975628'
directory_id: '7520163178001629190'
title: 流式更新文本
full_path: /uAjLw4CM/ukTMukTMukTM/cardkit-v1/card-element/content
breadcrumb:
- Server API
- Lark Card
- Element
- Stream update text
document_type: ReferenceDocumentType
updated_at: 2025-06-27T09:56:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card-element/content
---

# 流式更新文本

对卡片中的普通文本元素（tag 为 plain_text 的元素）或富文本组件（tag 为 markdown 的组件）传入全量文本内容，以实现“打字机”式的文字输出效果。参考[流式更新卡片](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/streaming-updates-openapi-overview)，了解流式更新文本的完整流程。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=cardkit&version=v1&resource=card.element&method=content)

:::html
<md-alert type="tip">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="error">

</md-alert>
:::

## 输出效果说明

若旧文本为传入的新文本的前缀子串，新增文本将在旧文本末尾继续以打字机效果输出；若新旧文本前缀不同，全量文本将直接上屏输出，无打字机效果。

## 使用限制

- 本接口仅支持[卡片 JSON 2.0 结构](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-json-v2-structure)。
- 调用该接口时，不支持将卡片设置为独享卡片模式。即不支持将卡片 JSON 数据中的 `update_multi` 属性设置为 `false`。
- 调用该接口的应用身份（tenant_access_token）需与创建目标卡片实体的应用身份一致。

## 前提条件

调用该接口时，需确保已开启卡片的流式更新模式，即将 `streaming_mode` 设为 `true`。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/cardkit/v1/cards/:card_id/elements/:element_id/content |
| HTTP Method | PUT |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="cardkit:card:write" desc="创建与更新卡片" support_app_types="custom,isv" tags="">创建与更新卡片</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
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
	<md-text type="field-name" >card_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	卡片实体 ID。通过[创建卡片实体](/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/create)获取

**示例值**："7355439197428236291"

**数据校验规则**：

- 长度范围：`1` ～ `20` 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >element_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	普通文本组件或富文本组件的 ID。对应卡片 JSON 中的 `element_id` 属性，由开发者自定义。

**示例值**："markdown_1"

**数据校验规则**：

- 长度范围：`1` ～ `64` 字符
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
	<md-text type="field-name" >uuid</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	幂等 ID，可通过传入唯一的 UUID 以保证相同批次的操作只进行一次。

**示例值**："a0d69e20-1dd1-458b-k525-dfeca4015204"

**数据校验规则**：

- 长度范围：`1` ～ `64` 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	新的全量文本内容。使用时请注意转义为字符串。

**注意**：
- 若 content 中含有代码块，你需将代码块前后的空格去掉，否则可能导致代码渲染失败。
- 若旧文本为传入的新文本的前缀子串，新增文本将在旧文本末尾继续以打字机效果输出；若新旧文本前缀不同，全量文本将直接上屏输出，无打字机效果。

**示例值**："这是更新后的文本内容。将以打字机式的效果输出"

**数据校验规则**：

- 长度范围：`1` ～ `100000` 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >sequence</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	操作卡片的序号。用于保证多次更新的时序性。

**注意**：
请确保在通过卡片 OpenAPI 操作同一张卡片时，sequence 的值相较于上一次操作严格递增。


**数据校验规则**：int32 范围（ `1`~`2147483647`）内的正整数。

**示例值**：1
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "uuid": "a0d69e20-1dd1-458b-k525-dfeca4015204",
    "content": "这是更新后的文本内容。将以打字机式的效果输出",
    "sequence": 1
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

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 10002 | Your request contains an invalid request parameter. | 参数错误，请根据接口返回的错误信息并参考文档检查输入参数。 |
| 400 | 200740 | The card entity does not exist | 卡片实体不存在。请检查实体 ID 是否正确。 |
| 400 | 200750 | The card entity has expired | 卡片实体已过期。卡片实体的有效期为 14 天。即创建卡片实体超出 14 天后，你将无法调用相关接口操作卡片。请重新创建卡片实体。 |
| 400 | 200770 | UUID conflict | UUID 冲突。请传入唯一的 UUID 以保证相同批次的操作只进行一次。 |
| 400 | 200810 | The card is in an ongoing interaction and cannot be updated | 在用户点击卡片[请求回调](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-callback-communication)交互期间，卡片无法实现流式更新。请等待交互结束后再尝试更新卡片。 |
| 400 | 200850 | Card streaming timeout | 卡片流式更新模式因超时自动关闭。你可调用[更新卡片配置](/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/settings) 将 `streaming_mode` 字段值设置为 `true`。 |
| 400 | 200860 | Card content exceeds limit. | 卡片体积超限。请将卡片大小控制在 30KB 以内。 |
| 400 | 300302 | update_multi property is false | 卡片全局属性 update_multi 设置为了 false。在流式更新模式下，卡片全局属性 update_multi 需设置为 true。 |
| 400 | 200220 | Failed to generate card content | 生成卡片内容失败。请检查卡片 JSON 格式是否有误。 |
| 400 | 300307 | The card DSL is empty | 卡片 JSON 数据为空。请检查数据。 |
| 400 | 300309 | Card streaming closed | 流式更新模式为关闭状态。你可调用[更新卡片配置](/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/settings) 将 `streaming_mode` 字段值设置为 `true`。 |
| 400 | 300310 | Only update text | 该接口仅支持普通文本元素和富文本组件。请使用普通文本元素或富文本组件进行更新。 |
| 400 | 300311 | The current application does not have permission to update/use this card | 当前应用没有更新或使用该卡片的权限。仅支持创建卡片实体的应用调用相关 OpenAPI 发送、操作卡片。 |
| 400 | 300313 | Failed to update element properties | 更新组件属性失败。请根据接口返回的错误信息检查输入参数。 |
| 400 | 300317 | The sequence number for operating on the card did not increment consecutively | 操作卡片的序号（sequence）未按顺序递增。请确保在通过卡片 OpenAPI 操作同一张卡片时，sequence 的值相较于上一次操作严格递增。 |
| 400 | 300120 | Server Internal Error | 服务内部错误。请确保 `sequence` 依次递增，然后稍后重试。仍然出现可联系[技术支持](https://applink.larksuite.com/TLJpeNdW)。 |



