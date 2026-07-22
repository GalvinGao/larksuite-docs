---
document_id: '7073819570391089157'
directory_id: '7072190414392393734'
title: 新增一个数据表
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/create
breadcrumb:
- Server API
- Docs
- Base
- Table
- Create table
document_type: ReferenceDocumentType
updated_at: 2026-03-12T13:48:55Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/create
---

# 新增一个数据表

新增一个数据表，支持传入数据表名称、视图名称和字段。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app.table&method=create)

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
| HTTP URL | https://open.larksuite.com/open-apis/bitable/v1/apps/:app_token/tables |
| HTTP Method | POST |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="base:table:create" desc="新增数据表" support_app_types="custom,isv" tags="">新增数据表</md-perm><br><md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm> |

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

**数据校验规则**：

- 最小长度：`1` 字符
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
	<md-text type="field-name" >table</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >req_table</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	数据表
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


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >default_view_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	默认表格视图的名称，不填则默认为“表格视图 1”。

注意：

- 名称中的首尾空格将会被去除。
- 名称中不允许包含 [ ] 两个字符

**示例值**："表格视图"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >fields</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.create_header\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	数据表的初始字段。了解如何填写字段，参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。

**注意**：

- 如果 `default_view_name` 字段和 `fields` 字段都不填写，将会创建一个仅包含索引字段的空数据表。
- 数据表的第一个字段为索引字段。索引字段仅支持以下类型：
    - 1：多行文本
    - 2：数字
  - 5：日期
  - 13：电话号码
  - 15：超链接
  - 20：公式
  - 22：地理位置 

**数据校验规则**：

- 长度范围：`1` ～ `300`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >field_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	字段名称

**示例值**："问题描述"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	字段类型。不支持新增 19 查找引用字段类型。

**示例值**：1

**可选值有**：
<md-enum>
<md-enum-item key="1" >文本</md-enum-item>
<md-enum-item key="2" >数字</md-enum-item>
<md-enum-item key="3" >单选</md-enum-item>
<md-enum-item key="4" >多选</md-enum-item>
<md-enum-item key="5" >日期</md-enum-item>
<md-enum-item key="7" >复选框</md-enum-item>
<md-enum-item key="11" >人员</md-enum-item>
<md-enum-item key="13" >电话号码</md-enum-item>
<md-enum-item key="15" >超链接</md-enum-item>
<md-enum-item key="17" >附件</md-enum-item>
<md-enum-item key="18" >单向关联</md-enum-item>
<md-enum-item key="20" >公式</md-enum-item>
<md-enum-item key="21" >双向关联</md-enum-item>
<md-enum-item key="22" >地理位置</md-enum-item>
<md-enum-item key="23" >群组</md-enum-item>
<md-enum-item key="1001" >创建时间</md-enum-item>
<md-enum-item key="1002" >最后更新时间</md-enum-item>
<md-enum-item key="1003" >创建人</md-enum-item>
<md-enum-item key="1004" >修改人</md-enum-item>
<md-enum-item key="1005" >自动编号</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >ui_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	字段在界面上的展示类型，例如 Progress 进度字段是数字的一种展示形态

**示例值**："Progress"

**可选值有**：
<md-enum>
<md-enum-item key="Text" >文本</md-enum-item>
<md-enum-item key="Barcode" >条码</md-enum-item>
<md-enum-item key="Number" >数字</md-enum-item>
<md-enum-item key="Progress" >进度</md-enum-item>
<md-enum-item key="Currency" >货币</md-enum-item>
<md-enum-item key="Rating" >评分</md-enum-item>
<md-enum-item key="SingleSelect" >单选</md-enum-item>
<md-enum-item key="MultiSelect" >多选</md-enum-item>
<md-enum-item key="DateTime" >日期</md-enum-item>
<md-enum-item key="Checkbox" >复选框</md-enum-item>
<md-enum-item key="User" >人员</md-enum-item>
<md-enum-item key="GroupChat" >群组</md-enum-item>
<md-enum-item key="Phone" >电话号码</md-enum-item>
<md-enum-item key="Url" >超链接</md-enum-item>
<md-enum-item key="Attachment" >附件</md-enum-item>
<md-enum-item key="SingleLink" >单向关联</md-enum-item>
<md-enum-item key="Formula" >公式</md-enum-item>
<md-enum-item key="DuplexLink" >双向关联</md-enum-item>
<md-enum-item key="Location" >地理位置</md-enum-item>
<md-enum-item key="CreatedTime" >创建时间</md-enum-item>
<md-enum-item key="ModifiedTime" >最后更新时间</md-enum-item>
<md-enum-item key="CreatedUser" >创建人</md-enum-item>
<md-enum-item key="ModifiedUser" >修改人</md-enum-item>
<md-enum-item key="AutoNumber" >自动编号</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >property</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.field.property</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	字段属性
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >options</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.field.property.option\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	单选、多选字段的选项信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
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
	选项名

**示例值**："红色"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	选项 ID，创建时不可指定 ID

**示例值**："optKl35lnG"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >color</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	选项颜色，详情参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。

**示例值**：0

**数据校验规则**：

- 取值范围：`0` ～ `54`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >formatter</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	数字、公式字段的显示格式。详情参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。

**示例值**："0"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >date_formatter</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	日期、创建时间、最后更新时间字段的显示格式。详情参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。

**示例值**："日期格式"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >auto_fill</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	日期字段中新纪录自动填写创建时间

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >multiple</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	人员字段中允许添加多个成员，单向关联、双向关联中允许添加多个记录

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >table_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	单向关联、双向关联字段中关联的数据表的id

**示例值**："tblsRc9GRRXKqhvW"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >table_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	单向关联、双向关联字段中关联的数据表的名字

**示例值**："table2"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >back_field_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	双向关联字段中关联的数据表中对应的双向关联字段的名字

**示例值**："table1-双向关联"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >auto_serial</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.field.property.auto_serial</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	自动编号类型
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
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
	自动编号类型

**示例值**："auto_increment_number"

**可选值有**：
<md-enum>
<md-enum-item key="custom" >自定义编号</md-enum-item>
<md-enum-item key="auto_increment_number" >自增数字</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >options</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.field.property.auto_serial.options\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	自动编号规则列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
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
	自动编号的可选规则项类型

**示例值**："created_time"

**可选值有**：
<md-enum>
<md-enum-item key="system_number" >自增数字位,value范围1-9</md-enum-item>
<md-enum-item key="fixed_text" >固定字符，最大长度：20</md-enum-item>
<md-enum-item key="created_time" >创建时间，支持格式 "yyyyMMdd"、"yyyyMM"、"yyyy"、"MMdd"、"MM"、"dd"</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	与自动编号的可选规则项类型相对应的取值

**示例值**："yyyyMMdd"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >location</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.field.property.location</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	地理位置输入方式
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >input_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	地理位置输入限制

**示例值**："not_limit"

**可选值有**：
<md-enum>
<md-enum-item key="only_mobile" >只允许移动端上传</md-enum-item>
<md-enum-item key="not_limit" >无限制</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >formula_expression</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	公式字段的表达式

**示例值**："bitable::$table[tblNj92WQBAasdEf].$field[fldMV60rYs]*2"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >allowed_edit_modes</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >allowed_edit_modes</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	字段支持的编辑模式
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >manual</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	是否允许手动录入

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >scan</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	是否允许移动端录入

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >min</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >number(float)</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	进度、评分等字段的数据范围最小值

**示例值**：0
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >max</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >number(float)</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	进度、评分等字段的数据范围最大值

**示例值**：10
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >range_customize</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	进度等字段是否支持自定义范围

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >currency_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	货币币种

**示例值**："CNY"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >rating</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >rating</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	评分字段的相关设置
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >symbol</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	评分字段的符号展示

**示例值**："star"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >description</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.field.description</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	字段的描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >disable_sync</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	是否禁止同步，如果为true，表示禁止同步该描述内容到表单的问题描述

**示例值**：true

**默认值**：`true`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >text</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	字段描述内容，支持换行\n

**示例值**："请按 name_id 格式填写\n例如：“Alice_20202020”"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
  "table": {
    "name": "数据表名称",
    "default_view_name": "默认的表格视图",
    "fields": [
      {
        "field_name": "索引字段",
        "type": 1
      },
      {
        "field_name": "单选",
        "type": 3,
        "ui_type": "SingleSelect",
        "property": {
          "options": [
            {
              "name": "Enabled",
              "color": 0
            },
            {
              "name": "Disabled",
              "color": 1
            },
            {
              "name": "Draft",
              "color": 2
            }
          ]
        }
      }
    ]
  }
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
	<md-text type="field-name" >table_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格数据表的 ID
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >default_view_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	默认表格视图的 ID。该字段仅在请求参数中填写了`default_view_name` 或 `fields` 字段才会返回
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >field_id_list</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	数据表初始字段的 ID 列表，该字段仅在请求参数中填写了 `fields` 才会返回
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
		"table_id": "tblDBTWm6Es84d8c",
		"default_view_id": "vewUuKOz2R",
		"field_id_list": [
			"fldhr2hBEA"
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
| 200 | 1254002 | Fail | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1254003 | WrongBaseToken | app_token 错误 |
| 200 | 1254004 | WrongTableId | table_id 错误 |
| 200 | 1254007 | EmptyValue | 空值 |
| 200 | 1254008 | EmptyView | 空视图 |
| 200 | 1254009 | WrongFieldId | 字段 id 错误 |
| 200 | 1254010 | ReqConvError | 请求错误 |
| 400 | 1254012 | NotSupportFieldOrView | 不支持的字段或视图。注意数据表的初始索引字段仅支持以下类型：<br>- 1：多行文本<br>- 2：数字<br>- 5：日期<br>- 13：电话号码<br>- 15：超链接<br>- 20：公式<br>- 22：地理位置 |
| 200 | 1254013 | TableNameDuplicated | 表名重复 |
| 400 | 1254014 | FieldNameDuplicated | 字段名重复 |
| 400 | 1254021 | EmptyViewName | 视图名为空 |
| 400 | 1254022 | InvalidViewName | 视图名无效 |
| 400 | 1254029 | InvalidFieldName | 字段名无效 |
| 200 | 1254030 | TooLargeResponse | 响应体过大 |
| 200 | 1254036 | Base is copying, please try again later. | 复制多维表格为异步操作，该错误码表示当前多维表格仍在复制中，在复制期间无法操作当前多维表格。需要等待复制完成后再操作 |
| 200 | 1254040 | BaseTokenNotFound | app_token 不存在 |
| 200 | 1254041 | TableIdNotFound | table_id 不存在 |
| 200 | 1254044 | FieldIdNotFound | field_id  不存在 |
| 200 | 1254060 | TextFieldConvFail | 多行文本字段错误 |
| 200 | 1254061 | NumberFieldConvFail | 数字字段错误 |
| 200 | 1254062 | SingleSelectFieldConvFail | 单选字段错误 |
| 200 | 1254063 | MultiSelectFieldConvFail | 多选字段错误 |
| 200 | 1254064 | DatetimeFieldConvFail | 日期字段错误 |
| 200 | 1254065 | CheckboxFieldConvFail | 复选框字段错误 |
| 200 | 1254066 | UserFieldConvFail | 人员字段有误。原因可能是：<br>- `user_id_type` 参数指定的 ID 类型与传入的 ID 类型不匹配<br>- 传入了不识别的类型或结构，目前只支持填写 `id` 参数，且需要传入数组<br>- 跨应用传入了 `open_id`。如果跨应用传入 ID，建议使用 `user_id`。不同应用获取的 `open_id` 不能交叉使用<br>- 若想对人员字段传空，可传 null |
| 200 | 1254067 | LinkFieldConvFail | 关联字段错误 |
| 200 | 1254100 | TableExceedLimit | 数据表或仪表盘数量超限。每个多维表格中，数据表加仪表盘的数量最多为 100 个 |
| 200 | 1254101 | ViewExceedLimit | 视图数量超限, 限制200个 |
| 200 | 1254130 | TooLargeCell | 格子内容过大 |
| 200 | 1254290 | TooManyRequest | 请求过快，稍后重试 |
| 200 | 1254291 | Write conflict | 同一个数据表(table) 不支持并发调用写接口，请检查是否存在并发调用写接口。写接口包括：新增、修改、删除记录；新增、修改、删除字段；修改表单；修改视图等。 |
| 200 | 1254301 | OperationTypeError | 多维表格未开启高级权限或不支持开启高级权限 |
| 403 | 1254302 | Permission denied. | 调用身份缺少多维表格的高级权限。你需要为调用身份授予高级权限：<br>- 对用户授予高级权限，你需要在多维表格页面右上方 **分享** 入口为当前用户添加可管理权限。![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/df3911b4f747d75914f35a46962d667d_dAsfLjv3QC.png?height=546&lazyload=true&maxWidth=550)<br>- 对应用授予高级权限，你需通过多维表格页面右上方 **「...」** -> **「...更多」** ->**「添加文档应用」** 入口为应用添加可管理权限。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/22c027f63c540592d3ca8f41d48bb107_CSas7OYJBR.png?height=1994&maxWidth=550&width=3278)<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9f3353931fafeea16a39f0eb887db175_0tjzC9P3zU.png?maxWidth=550)<br>**注意**：<br>在 **添加文档应用** 前，你需确保目标应用至少开通了一个多维表格的 [API 权限](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/scope-list)。否则你将无法在文档应用窗口搜索到目标应用。<br>- 你也可以在 **多维表格高级权限设置** 中添加用户或一个包含应用的群组, 给予这个群自定义的读写等权限。 |
| 400 | 1254607 | Data not ready, please try again later | 该报错一般是由于前置操作未执行完成，或本次操作数据太大，服务器计算超时导致。遇到该错误码时，建议等待一段时间后重试。通常有以下几种原因：<br>- **编辑操作频繁**：开发者对多维表格的编辑操作非常频繁。可能会导致由于等待前置操作处理完成耗时过长而超时的情况。多维表格底层对数据表的处理基于版本维度的串行方式，不支持并发。因此，并发请求时容易出现此类错误，不建议开发者对单个数据表进行并发请求。<br>- **批量操作负载重**：开发者在多维表格中进行批量新增、删除等操作时，如果数据表的数据量非常大，可能会导致单次请求耗时过长，最终导致请求超时。建议开发者适当降低批量请求的 page_size 以减少请求耗时。<br>- **资源分配与计算开销**：资源分配是基于单文档维度的，如果读接口涉及公式计算、排序等计算逻辑，会占用较多资源。例如，并发读取一个文档下的多个数据表也可能导致该文档阻塞。 |
| 200 | 1255001 | InternalError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255002 | RpcError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255003 | MarshalError | 序列化错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255004 | UmMarshalError | 反序列化错误 |
| 200 | 1255005 | ConvError | 服务内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |





