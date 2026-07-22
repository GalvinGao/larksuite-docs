---
document_id: '7239978910498275334'
directory_id: '7143912179455426565'
title: 复制仪表盘
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-dashboard/copy
breadcrumb:
- Server API
- Docs
- Base
- Dashboards
- Copy dashboards
document_type: ReferenceDocumentType
updated_at: 2025-07-28T02:01:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-dashboard/copy
---

# 复制仪表盘

基于现有仪表盘复制出新的仪表盘。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app.dashboard&method=copy)

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

</md-alert>
:::

## 前提条件

调用此接口前，请确保当前调用身份（tenant_access_token 或 user_access_token）已有原多维表格的阅读权限，否则接口将返回 HTTP 403 或 400 状态码。了解更多，参考[如何为应用或用户开通文档权限](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#16c6475a)。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/bitable/v1/apps/:app_token/dashboards/:block_id/copy |
| HTTP Method | POST |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm><br>base:dashboard:copy |

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
	<md-text type="field-name" >app_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格 App 的唯一标识。不同形态的多维表格，其 app_token 的获取方式不同：
- 如果多维表格的 URL 以 ==**larksuite.com/base**== 开头，该多维表格的 `app_token` 是 base 之后的字符串

- 如果多维表格的 URL 以 ==**larksuite.com/wiki**== 开头，你需调用知识库相关[获取知识空间节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)接口获取多维表格的 app_token。当 `obj_type` 的值为 `bitable` 时，`obj_token` 字段的值才是多维表格的 `app_token`


**示例值**："basbcldP5xZeskcHDFZQfeToydb"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >block_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格仪表盘的唯一标识，以 blk 开头。获取方式：

- 在多维表格的 URL 地址栏中，`block_id` 是下图中高亮部分：
  
  ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a32979b410e8915c588bf68cd7f13e2e_byQ36msTdQ.png?height=799&lazyload=true&maxWidth=300&width=1602)

  
- 通过[列出仪表盘](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-dashboard/list)接口获取

**示例值**："blkEsvEEaNllY2UV"
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
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	新的仪表盘名称

**示例值**："New Dashboard"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "name": "New Dashboard"
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
	<md-text type="field-name" >block_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	新的仪表盘的 block_id
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
	新的仪表盘名称
	</md-dt-td>
</md-dt-tr>


  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "block_id": "blkXsf60PxMdJLQN",
    "name": "New Dashboard"
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1254000 | WrongRequestJson | 请求体错误 |
| 400 | 1254001 | WrongRequestBody | 请求体错误 |
| 400 | 1254002 | Fail | 导致报 1254002 错误码的场景较多，请参考以下排查建议排查：<br>- 如果日志内容显示 changeset size exceed the limit, len:xxxxx，表示单次操作的内容变更较大，你需要在单次操作中减少数据量<br>- 如果日志内容显示 lock not obtained 或 data not ready，表示并发调用接口导致抢锁失败出错。你需控制请求间隔，稍后重试<br>- 如果在知识库（wiki）中创建多维表格，请检查你是否使用了知识库[创建知识空间节点](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/create)接口创建多维表格。在此场景下不能使用[创建多维表格](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/create)接口<br>- 请检查接口参数是否有误。例如，在分页查询多维表格时，传递了无效的 page_token，或传递了错误的数据表的 table_id<br>- 如果该报错偶尔发生，可能是服务器超时或不稳定，请重试解决 |
| 400 | 1254003 | WrongBaseToken | app_token 错误 |
| 400 | 1254004 | WrongTableId | table_id 错误 |
| 400 | 1254005 | WrongViewId | view_id 错误 |
| 400 | 1254006 | WrongRecordId | 检查 record_id |
| 400 | 1254007 | EmptyValue | 空值 |
| 400 | 1254008 | EmptyView | 空视图 |
| 400 | 1254009 | WrongFieldId | 字段 id 错误 |
| 400 | 1254010 | ReqConvError | 请求错误 |
| 400 | 1254016 | InvalidSort | Sort参数错误 |
| 400 | 1254018 | InvalidFilter | filter 参数错误。请参考[记录过滤参数填写指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/record-filter-guide)了解如何填写 filter 参数。 |
| 400 | 1254019 | InvalidViewType | 视图类型无效 |
| 400 | 1254020 | ViewNameDuplicated | 视图名重复 |
| 400 | 1254021 | EmptyViewName | 视图名为空 |
| 400 | 1254022 | InvalidViewName | 视图名无效 |
| 400 | 1254030 | TooLargeResponse | 响应体过大 |
| 400 | 1254032 | Invalid role name, make sure that it complies with the specification. | 自定义角色名无效 |
| 400 | 1254033 | Duplicated role name, use another name instead. | 自定义角色名重复 |
| 400 | 1254036 | Base is copying, please try again later. | 复制多维表格为异步操作，该错误码表示当前多维表格仍在复制中，在复制期间无法操作当前多维表格。需要等待复制完成后再操作。 |
| 400 | 1254038 | Invalid name, make sure that it complies with the specification. | 名称不规范，名称中不能包含 "[]"，并且不能超过 100 个字符 |
| 400 | 1254039 | Duplicated name, use another name instead. | 名称与现有仪表盘名称重复，更换名称后重试 |
| 404 | 1254040 | BaseTokenNotFound | app_token 不存在 |
| 404 | 1254041 | TableIdNotFound | table_id 不存在 |
| 404 | 1254042 | ViewIdNotFound | view_id 不存在 |
| 404 | 1254043 | RecordIdNotFound | record_id 不存在 |
| 404 | 1254044 | FieldIdNotFound | field_id  不存在 |
| 404 | 1254045 | FieldNameNotFound | 字段名称不存在。请检查:<br>- 接口中字段名称和多维表格中的字段名称是否完全匹配。如果难以排查，建议你调用[列出字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/list)接口获取字段名称，因为根据表格页面的 UI 名称可能会忽略空格、换行或特殊符号等差异。<br>- 表格是否开启了高级权限但调用身份缺少对应字段的权限。你需要为调用身份授予高级权限：<br>- 对用户授予高级权限，你需要在多维表格页面右上方 **分享** 入口为当前用户添加可管理权限。![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/df3911b4f747d75914f35a46962d667d_dAsfLjv3QC.png?height=546&lazyload=true&maxWidth=550)<br>- 对应用授予高级权限，你需通过多维表格页面右上方 **「...」** -> **「...更多」** ->**「添加文档应用」** 入口为应用添加可管理权限。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/22c027f63c540592d3ca8f41d48bb107_CSas7OYJBR.png?height=1994&maxWidth=550&width=3278)<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9f3353931fafeea16a39f0eb887db175_0tjzC9P3zU.png?maxWidth=550)<br>**注意**：<br>在 **添加文档应用** 前，你需确保目标应用至少开通了一个多维表格的 [API 权限](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/scope-list)。否则你将无法在文档应用窗口搜索到目标应用。<br>- 你也可以在 **多维表格高级权限设置** 中添加用户或一个包含应用的群组, 给予这个群自定义的读写等权限。 |
| 404 | 1254047 | Role id is not found. | role_id 不存在 |
| 400 | 1254048 | MemberNotFound | member 不存在 |
| 404 | 1254049 | Form field is not found. | form_field_id 不存在 |
| 404 | 1254050 | Source block is not found. | 传入了不存在的仪表盘 block_id |
| 400 | 1254060 | TextFieldConvFail | 多行文本字段错误 |
| 400 | 1254061 | NumberFieldConvFail | 数字字段错误 |
| 400 | 1254062 | SingleSelectFieldConvFail | 单选字段错误 |
| 400 | 1254063 | MultiSelectFieldConvFail | 多选字段错误 |
| 400 | 1254064 | DatetimeFieldConvFail | 日期字段错误 |
| 400 | 1254065 | CheckboxFieldConvFail | 复选框字段错误 |
| 400 | 1254066 | UserFieldConvFail | 人员字段有误。原因可能是：<br>- `user_id_type` 参数指定的 ID 类型与传入的 ID 类型不匹配<br>- 传入了不识别的类型或结构，目前只支持填写 `id` 参数，且需要传入数组<br>- 跨应用传入了 `open_id`。如果跨应用传入 ID，建议使用 `user_id`。不同应用获取的 `open_id` 不能交叉使用 |
| 400 | 1254067 | LinkFieldConvFail | 关联字段错误 |
| 400 | 1254100 | TableExceedLimit | 数据表或仪表盘数量超限。每个多维表格中，数据表加仪表盘的数量最多为 100 个 |
| 400 | 1254101 | ViewExceedLimit | 视图数量超限, 限制200个 |
| 400 | 1254103 | RecordExceedLimit | 记录数量超限, 限制20,000条 |
| 400 | 1254104 | RecordAddOnceExceedLimit | 单次添加记录数量超限, 限制500条 |
| 400 | 1254110 | Role exceeds limit | 自定义角色数量超限，限制30条 |
| 400 | 1254130 | TooLargeCell | 格子内容过大 |
| 429 | 1254290 | TooManyRequest | 请求过快，稍后重试 |
| 400 | 1254291 | Write conflict | 同一个数据表(table) 不支持并发调用写接口，请检查是否存在并发调用写接口。写接口包括：新增、修改、删除记录；新增、修改、删除字段；修改表单；修改视图等。 |
| 400 | 1254301 | OperationTypeError | 多维表格未开启高级权限或不支持开启高级权限 |
| 403 | 1254302 | Permission denied. | 调用身份缺少多维表格的高级权限。你需要为调用身份授予高级权限：<br>- 对用户授予高级权限，你需要在多维表格页面右上方 **分享** 入口为当前用户添加可管理权限。![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/df3911b4f747d75914f35a46962d667d_dAsfLjv3QC.png?height=546&lazyload=true&maxWidth=550)<br>- 对应用授予高级权限，你需通过多维表格页面右上方 **「...」** -> **「...更多」** ->**「添加文档应用」** 入口为应用添加可管理权限。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/22c027f63c540592d3ca8f41d48bb107_CSas7OYJBR.png?height=1994&maxWidth=550&width=3278)<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9f3353931fafeea16a39f0eb887db175_0tjzC9P3zU.png?maxWidth=550)<br>**注意**：<br>在 **添加文档应用** 前，你需确保目标应用至少开通了一个多维表格的 [API 权限](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/scope-list)。否则你将无法在文档应用窗口搜索到目标应用。<br>- 你也可以在 **多维表格高级权限设置** 中添加用户或一个包含应用的群组, 给予这个群自定义的读写等权限。 |
| 400 | 1254607 | Data not ready, please try again later. | 该报错一般是由于前置操作未执行完成，或本次操作数据太大，服务器计算超时导致。遇到该错误码时，建议等待一段时间后重试。通常有以下几种原因：<br>- **编辑操作频繁**：开发者对多维表格的编辑操作非常频繁。可能会导致由于等待前置操作处理完成耗时过长而超时的情况。多维表格底层对数据表的处理基于版本维度的串行方式，不支持并发。因此，并发请求时容易出现此类错误，不建议开发者对单个数据表进行并发请求。<br>- **批量操作负载重**：开发者在多维表格中进行批量新增、删除等操作时，如果数据表的数据量非常大，可能会导致单次请求耗时过长，最终导致请求超时。建议开发者适当降低批量请求的 page_size 以减少请求耗时。<br>- **资源分配与计算开销**：资源分配是基于单文档维度的，如果读接口涉及公式计算、排序等计算逻辑，会占用较多资源。例如，并发读取一个文档下的多个数据表也可能导致该文档阻塞。 |
| 400 | 1255001 | InternalError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 400 | 1255002 | RpcError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 400 | 1255003 | MarshalError | 序列化错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 400 | 1255004 | UmMarshalError | 反序列化错误 |
| 400 | 1255005 | ConvError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 504 | 1255040 | 请求超时 | 进行重试 |





