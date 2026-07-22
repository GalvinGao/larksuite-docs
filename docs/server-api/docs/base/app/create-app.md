---
document_id: '7195149421070401541'
directory_id: '7072190414392295430'
title: 创建多维表格
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/create
breadcrumb:
- Server API
- Docs
- Base
- App
- Create App
document_type: ReferenceDocumentType
updated_at: 2026-03-12T13:48:50Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/create
---

# 创建多维表格

在指定目录下创建多维表格{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app&method=create)

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



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/bitable/v1/apps |
| HTTP Method | POST |
| 接口频率限制 | [20 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="base:app:create" desc="创建多维表格" support_app_types="custom,isv" tags="">创建多维表格</md-perm><br><md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




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
	否
	</md-dt-td>
	<md-dt-td>
	多维表格App名字

**示例值**："一篇新的多维表格"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >folder_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	多维表格App归属文件夹

**示例值**："fldbco*****CIMltVc"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >time_zone</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	文档时区，说明见：https://bytedance.larkoffice.com/docx/YKRndTM7VoyDqpxqqeEcd67MnEf

**示例值**："Asia/Macau"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "name":"一篇新的多维表格",
    "folder_token": "fldbco*****CIMltVc"
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
	<md-text type="field-name" >app</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app</md-text>
	</md-dt-td>
	<md-dt-td>
	返回响应体
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >app_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格的 app_token
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格的名字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >folder_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格 App 归属文件夹
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
	多维表格 App URL
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >default_table_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	默认的表格id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >time_zone</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	文档时区，说明见：https://bytedance.larkoffice.cn/docx/YKRndTM7VoyDqpxqqeEcd67MnEf
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
        "app": {
            "app_token": "S404b*****e9PQsYDWYcNryFn0g",
            "default_table_id": "tblxxxxxxxxoumSQ",
            "folder_token": "fldbco*****CIMltVc",
            "name": "一篇新的多维表格",
            "url": "https://by*****ce.larksuite.com/base/S404b*****e9PQsYDWYcNryFn0g"
        }
    },
    "msg": "success"
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
| 200 | 1254025 | InvalidCopyTypes | Copy Type参数无效 |
| 200 | 1254030 | TooLargeResponse | 响应体过大 |
| 400 | 1254031 | InvalidAppName | 名称不能超过255个字符 |
| 400 | 1254036 | Base is copying, please try again later. | 多维表格副本复制中，稍后重试 |
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
| 200 | 1254066 | UserFieldConvFail | 人员字段错误 |
| 200 | 1254067 | LinkFieldConvFail | 关联字段错误 |
| 200 | 1254100 | TableExceedLimit | 数据表数量超限, 限制300个 |
| 200 | 1254101 | ViewExceedLimit | 视图数量超限, 限制200个 |
| 200 | 1254102 | FileExceedLimit | 超限 |
| 200 | 1254103 | RecordExceedLimit | 记录数量超限, 限制20,000条 |
| 200 | 1254104 | RecordAddOnceExceedLimit | 单次添加记录数量超限, 限制500条 |
| 200 | 1254130 | TooLargeCell | 格子内容过大 |
| 200 | 1254290 | TooManyRequest | 请求过快，稍后重试 |
| 200 | 1254291 | Write conflict | 同一个数据表(table) 不支持并发调用写接口，请检查是否存在并发调用写接口。写接口包括：新增、修改、删除记录；新增、修改、删除字段；修改表单；修改视图等。 |
| 200 | 1254301 | OperationTypeError | 多维表格未开启高级权限或不支持开启高级权限 |
| 200 | 1254304 | PermNotAllow | 仅企业版和旗舰版Lark支持行列权限 |
| 403 | 1254701 | DriveNodePermNotAllow | 对目标云空间节点没有权限 |
| 404 | 1254702 | DriveNodeNotExist | 云空间节点不存在，检查参数正确性 |
| 400 | 1254800 | ParamInvalid | 参数无效，请检查入参的正确性 |
| 200 | 1255001 | InternalError | 内部错误，有疑问可咨询客服 |
| 200 | 1255002 | RpcError | 内部错误，有疑问可咨询客服 |
| 200 | 1255003 | MarshalError | 序列化错误，有疑问可咨询客服 |
| 200 | 1255004 | UmMarshalError | 反序列化错误 |
| 200 | 1255005 | ConvError | 内部错误，有疑问可咨询客服处 |
| 504 | 1255040 | 请求超时 | 进行重试 |
| 400 | 1254292 | MountNodeOutOfSiblingNum | 父目录下挂载的节点数量超出限制 |





