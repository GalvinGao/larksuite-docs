---
document_id: '7520565167073591302'
directory_id: '7520163178001596422'
title: 全量更新卡片实体
full_path: /uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/update
breadcrumb:
- Server API
- Lark Card
- Card
- Update card entity
document_type: ReferenceDocumentType
updated_at: 2025-06-27T09:56:03Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/update
---

# 全量更新卡片实体

传入全新的卡片 JSON 数据，覆盖更新指定的卡片实体的所有内容。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=cardkit&version=v1&resource=card&method=update)

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

## 使用限制

- 本接口仅支持[卡片 JSON 2.0 结构](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-json-v2-structure)。
- 调用该接口时，不支持将卡片设置为独享卡片模式。即不支持将卡片 JSON 数据中的 `update_multi` 属性设置为 `false`。
- 调用该接口的应用身份（tenant_access_token）需与创建目标卡片实体的应用身份一致。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/cardkit/v1/cards/:card_id |
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

**示例值**："7355372766134157313"

**数据校验规则**：

- 长度范围：`1` ～ `20` 字符
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
	<md-text type="field-name" >card</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >card</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	更新后的完整卡片 JSON 内容。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	卡片数据的类型。取固定值 `card_json`。

**示例值**："card_json"

**可选值有**：
<md-enum>
<md-enum-item key="card_json" >卡片 JSON 数据类型</md-enum-item>
</md-enum>

**数据校验规则**：

- 长度范围：`1` ～ `50` 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >data</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	卡片 JSON 数据的内容。

**注意**：
- 仅支持 JSON 2.0 版本的卡片结构。

- 以下示例值未转义，使用时请注意将其转为 JSON 序列化后的字符串。

**示例值**："{\"schema\":\"2.0\",\"header\":{\"title\":{\"content\":\"项目进度更新提醒\",\"tag\":\"plain_text\"}},\"body\":{\"elements\":[{\"tag\":\"markdown\",\"content\":\"截至今日，项目完成度已达80%\"}]}}"

**数据校验规则**：

- 长度范围：`1` ～ `1000000` 字符
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
	幂等 ID，可通过传入唯一的 UUID 以保证相同批次的操作只进行一次。

**示例值**："a0d69e20-1dd1-458b-k525-dfeca4015204"

**数据校验规则**：

- 长度范围：`1` ～ `64` 字符
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


**数据校验规则**：int32 范围（ `1`~`2147483647`）内的正整数

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
    "card": {
        "type": "card_json",
        "data": "{\"schema\":\"2.0\",\"header\":{\"title\":{\"content\":\"项目进度更新提醒\",\"tag\":\"plain_text\"}},\"body\":{\"elements\":[{\"tag\":\"markdown\",\"content\":\"截至今日，项目完成度已达80%\"}]}}"
    },
    "uuid": "a0d69e20-1dd1-458b-k525-dfeca4015204",
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
| 400 | 200860 | Card content exceeds limit. | 卡片体积超限。请将卡片大小控制在 30KB 以内。 |
| 400 | 300301 | Duplicate element_id in card component. | 卡片内部组件 ID （即 element_id）重复。 |
| 400 | 300302 | update_multi property is false | 卡片全局属性 update_multi 为 false。在流式更新模式下，卡片全局属性 update_multi 需设置为 true。 |
| 400 | 300303 | Only schema 2.0 is supported | 该接口仅支持 Schema v2.0 结构。详情参考[卡片 JSON 2.0 结构](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-json-v2-structure)。 |
| 400 | 200220 | Failed to generate card content | 生成卡片内容失败。请检查卡片 JSON 格式是否有误。 |
| 400 | 300305 | The number of card components exceeds 200 | 超出卡片组件限制。卡片 JSON 2.0 结构中，一张卡片最多支持 200 个元素（如 tag 为 plain_text 的文本元素）或组件。请将组件和元素的数量之和控制在 200 个以内。 |
| 400 | 300307 | The card DSL is empty | 卡片 JSON 数据为空。请检查数据。 |
| 400 | 300311 | The current application does not have permission to update/use this card | 当前应用没有更新或使用该卡片的权限。请使用创建该卡片实体的应用调用相关 OpenAPI 发送、操作卡片。 |
| 400 | 300317 | The sequence number for operating on the card did not increment consecutively | 操作卡片的序号（sequence）未按顺序递增。请确保在通过卡片 OpenAPI 操作同一张卡片时，sequence 的值相较于上一次操作严格递增。 |



