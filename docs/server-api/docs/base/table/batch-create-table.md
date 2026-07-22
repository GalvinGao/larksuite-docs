---
document_id: '7073819570391187461'
directory_id: '7072190414392393734'
title: 新增多个数据表
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/batch_create
breadcrumb:
- Server API
- Docs
- Base
- Table
- Batch create table
document_type: ReferenceDocumentType
updated_at: 2026-03-12T13:48:56Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/batch_create
---

# 新增多个数据表

新增多个数据表，仅可指定数据表名称。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app.table&method=batch_create)

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

调用此接口前，请确保当前调用身份（tenant_access_token 或 user_access_token）已有多维表格的编辑等文档权限，否则接口将返回 HTTP 403 或 400 状态码。了解更多，参考[如何为应用或用户开通文档权限](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#16c6475a)。

## 使用限制

每个多维表格中，数据表与仪表盘的总数量上限为 100。


## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/bitable/v1/apps/:app_token/tables/batch_create |
| HTTP Method | POST |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="base:table:create" desc="新增数据表" support_app_types="custom,isv" tags="">新增数据表</md-perm><br><md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom,isv" tags="">获取用户 user ID</md-perm> |

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


**示例值**："appbcbWCzen6D8dezhoCH2RpMAh"
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
	<md-text type="field-name" >tables</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >req_table\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	tables
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
	否
	</md-dt-td>
	<md-dt-td>
	数据表名称。该字段必填。

**注意**：
- 名称中的首尾空格将会被默认去除。
- 数据表名称不可以包含 `/ \ ? * :  [ ]` 等特殊字符。

**示例值**："一个新的数据表"

**数据校验规则**：

- 长度范围：`1` ～ `100` 字符
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "tables": [
        {
            "name": "一个新的数据表"
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
	<md-text type="field-name" >table_ids</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	table ids
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
        "table_ids": [
            "tblIovTTN2eIW2hn"
        ]
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 200 | 1254000 | WrongRequestJson | 请求体错误 |
| 200 | 1254001 | WrongRequestBody | 请求体错误 |
| 200 | 1254002 | Fail | 内部错误，有疑问可咨询客服 |
| 200 | 1254003 | WrongBaseToken | app_token 错误 |
| 200 | 1254004 | WrongTableId | table_id 错误 |
| 200 | 1254005 | WrongViewId | view_id 错误 |
| 200 | 1254006 | WrongRecordId | 检查 record_id |
| 200 | 1254007 | EmptyValue | 空值 |
| 200 | 1254008 | EmptyView | 空视图 |
| 200 | 1254009 | WrongFieldId | 字段 id 错误 |
| 200 | 1254010 | ReqConvError | 请求错误 |
| 400 | 1254013 | TableNameDuplicated | 表名重复 |
| 200 | 1254030 | TooLargeResponse | 响应体过大 |
| 400 | 1254036 | Base is copying, please try again later. | 复制多维表格为异步操作，该错误码表示当前多维表格仍在复制中，在复制期间无法操作当前多维表格。需要等待复制完成后再操作 |
| 200 | 1254040 | BaseTokenNotFound | app_token 不存在 |
| 200 | 1254041 | TableIdNotFound | table_id 不存在 |
| 200 | 1254042 | ViewIdNotFound | view_id 不存在 |
| 200 | 1254043 | RecordIdNotFound | record_id 不存在 |
| 200 | 1254044 | FieldIdNotFound | field_id  不存在 |
| 200 | 1254060 | TextFieldConvFail | 多行文本字段错误 |
| 200 | 1254061 | NumberFieldConvFail | 数字字段错误 |
| 200 | 1254062 | SingleSelectFieldConvFail | 单选字段错误 |
| 200 | 1254063 | MultiSelectFieldConvFail | 多选字段错误 |
| 200 | 1254064 | DatetimeFieldConvFail | 日期字段错误 |
| 200 | 1254065 | CheckboxFieldConvFail | 复选框字段错误 |
| 200 | 1254066 | UserFieldConvFail | 人员字段有误。原因可能是：<br>- `user_id_type` 参数指定的 ID 类型与传入的 ID 类型不匹配<br>- 传入了不识别的类型或结构，目前只支持填写 `id` 参数，且需要传入数组<br>- 跨应用传入了 `open_id`。如果跨应用传入 ID，建议使用 `user_id`。不同应用获取的 `open_id` 不能交叉使用 |
| 200 | 1254067 | LinkFieldConvFail | 关联字段错误 |
| 200 | 1254100 | TableExceedLimit | 数据表或仪表盘数量超限。每个多维表格中，数据表加仪表盘的数量最多为 100 个 |
| 200 | 1254101 | ViewExceedLimit | 视图数量超限, 限制200个 |
| 200 | 1254102 | FileExceedLimit | 超限 |
| 200 | 1254103 | RecordExceedLimit | 记录数量超限, 限制20,000条 |
| 200 | 1254104 | RecordAddOnceExceedLimit | 单次添加记录数量超限, 限制500条 |
| 200 | 1254130 | TooLargeCell | 格子内容过大 |
| 200 | 1254290 | TooManyRequest | 请求过快，稍后重试 |
| 200 | 1254291 | Write conflict | 同一个数据表(table) 不支持并发调用写接口，请检查是否存在并发调用写接口。写接口包括：新增、修改、删除记录；新增、修改、删除字段；修改表单；修改视图等。 |
| 200 | 1254301 | OperationTypeError | 多维表格未开启高级权限或不支持开启高级权限 |
| 403 | 1254302 | The role has no permissions. | 调用身份缺少多维表格的高级权限。你需要为调用身份授予高级权限：<br>- 对用户授予高级权限，你需要在多维表格页面右上方 **分享** 入口为当前用户添加可管理权限。![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/df3911b4f747d75914f35a46962d667d_dAsfLjv3QC.png?height=546&lazyload=true&maxWidth=550)<br>- 对应用授予高级权限，你需通过多维表格页面右上方 **「...」** -> **「...更多」** ->**「添加文档应用」** 入口为应用添加可管理权限。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/22c027f63c540592d3ca8f41d48bb107_CSas7OYJBR.png?height=1994&maxWidth=550&width=3278)<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9f3353931fafeea16a39f0eb887db175_0tjzC9P3zU.png?maxWidth=550)<br>**注意**：<br>在 **添加文档应用** 前，你需确保目标应用至少开通了一个多维表格的 [API 权限](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/scope-list)。否则你将无法在文档应用窗口搜索到目标应用。<br>- 你也可以在 **多维表格高级权限设置** 中添加用户或一个包含应用的群组, 给予这个群自定义的读写等权限。 |
| 200 | 1255001 | InternalError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255002 | RpcError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255003 | MarshalError | 序列化错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255004 | UmMarshalError | 反序列化错误 |
| 200 | 1255005 | ConvError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 504 | 1255040 | 请求超时 | 进行重试 |





