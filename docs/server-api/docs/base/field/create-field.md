---
document_id: '7073819570391040005'
directory_id: '7072190414392246278'
title: 新增字段
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/create
breadcrumb:
- Server API
- Docs
- Base
- Field
- Create field
document_type: ReferenceDocumentType
updated_at: 2025-07-28T02:01:40Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/create
---

# 新增字段

在多维表格数据表中新增一个字段。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app.table.field&method=create)

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

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields |
| HTTP Method | POST |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm><br>base:field:create |

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


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >table_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格数据表的唯一标识。获取方式：
- 你可通过多维表格 URL 获取 `table_id`，下图高亮部分即为当前数据表的 `table_id`
- 也可通过[列出数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/list)接口获取 `table_id`

  ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/18741fe2a0d3cafafaf9949b263bb57d_yD1wkOrSju.png?height=746&lazyload=true&maxWidth=700&width=2976)

**示例值**："tblsRc9GRRXKqhvW"
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
	<md-text type="field-name" >client_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作。此值为空表示将发起一次新的请求，此值非空表示幂等的进行更新操作。

**示例值**：fe599b60-450f-46ff-b2ef-9f6675625b97
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
	<md-text type="field-name" >field_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	多维表格字段名称。名称中的首尾空格将会被去除。

**示例值**："任务名称"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
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
	要新增的字段类型。不支持新增 19 查找引用字段类型。

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
<md-enum-item key="18" >单项关联</md-enum-item>
<md-enum-item key="20" >公式（不支持设置公式表达式）</md-enum-item>
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


<md-dt-tr level="0">
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
	字段属性，了解如何填写字段，参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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


<md-dt-tr level="2">
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
	选项名称

**示例值**："红色"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
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
	选项 ID，创建字段时不允许指定 ID。

**示例值**："optKl35lnG"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
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


<md-dt-tr level="1">
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
	数字和公式字段的显示格式。详情参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。


**示例值**："0"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	日期、创建时间、最后更新时间字段的显示格式。默认为 "yyyy/MM/dd"。枚举值如下所示：
- "yyyy/MM/dd"：2021/1/30
- "yyyy-MM-dd HH:mm"：2021/1/30 14:00
- "MM-dd"：1月30日
- "MM/dd/yyyy"：2021/1/30
- "dd/MM/yyyy"：2021/1/30"

**示例值**："yyyy/MM/dd"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	对于新记录，是否自动填写创建时间。默认为 false。

**示例值**：false
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	人员字段中是否允许添加多个成员，或单向关联、双向关联字段中是否允许添加多个记录。默认为 true。

**示例值**：false
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
	否
	</md-dt-td>
	<md-dt-td>
	单向关联、双向关联字段中关联的数据表的 ID

**示例值**："tblsRc9GRRXKqhvW"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	双向关联字段中，关联的数据表中对应的双向关联字段名称

**示例值**："table1-双向关联"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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


<md-dt-tr level="2">
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


<md-dt-tr level="2">
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
	自定义编号规则列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
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
<md-enum-item key="system_number" >自增数字的位数</md-enum-item>
<md-enum-item key="fixed_text" >固定字符</md-enum-item>
<md-enum-item key="created_time" >创建时间</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
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
	规则类型对应的值。
- 若规则类型为 `"type": "system_number"`，value 为范围在 1-9 的整数，表示自增数字的位数
- 若规则类型为 `"type": "fixed_text"`，value 为范围在 20 个字符以内的固定字符
- 若规则类型为 `"type": "created_time"`，value 用于指定日期的格式。可选值如下所示：
    - "yyyyMMdd"：日期为 20220130 的格式
    - "yyyyMM"：日期为 202201 的格式
    - "yyyy"：日期为 2022 的格式
    - "MMdd"：日期为 130 的格式，表示 1 月 30 日
    - "MM"：日期为 1 的格式，表示月份
    - "dd"：日期为 30 的格式

**示例值**："yyyyMMdd"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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


<md-dt-tr level="2">
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
<md-enum-item key="only_mobile" >仅允许移动端实时定位</md-enum-item>
<md-enum-item key="not_limit" >无限制，可输入任意地理位置</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	公式字段的表达式。

**示例值**："bitable::$table[tblNj92WQBAasdEf].$field[fldMV60rYs]*2"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	条码展示类型字段支持的配置
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
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
	是否允许手动录入。默认为 true

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
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
	是否允许移动端录入。默认为 true

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	进度和评分字段的数据范围最小值。不同字段类型中，该参数的必填属性和取值范围不同，详情参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。

**示例值**：0
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	进度和评分字段的数据范围最大值。不同字段类型中，该参数的必填属性和取值范围不同，详情参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。

**示例值**：10
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	进度字段是否允许自定义进度条值，默认为 false。

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	货币的具体类型，枚举值如下所示：
- CNY：人民币，货币符号为 ¥
- USD：美元，货币符号为 $
- EUR：欧元，货币符号为 €
- GBP：英镑，货币符号为 £
- AED：阿联酋迪拉姆，货币符号为 dh
- AUD：澳大利亚元，货币符号为 $
- BRL：巴西雷亚尔，货币符号为 R$
- CAD：加拿大元，货币符号为 $
- CHF：瑞士法郎，货币符号为 CHF
- HKD：港元，货币符号为 $
- INR：印度卢比，货币符号为 ₹
- IDR：印尼盾，货币符号为 Rp
- JPY：日元，货币符号为 ¥
- KRW：韩元，货币符号为 ₩
- MOP：澳门元，货币符号为 MOP$
- MXN：墨西哥比索，货币符号为 $
- MYR：马来西亚令吉，货币符号为 RM
- PHP：菲律宾比索，货币符号为 ₱
- PLN：波兰兹罗提，货币符号为 zł
- RUB：俄罗斯卢布，货币符号为 ₽
- SGD：新加坡元，货币符号为 $
- THB：泰国铢，货币符号为 ฿
- TRY：土耳其里拉，货币符号为 ₺
- TWD：新台币，货币符号为 NT$
- VND：越南盾，货币符号为 ₫

**示例值**："CNY"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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


<md-dt-tr level="2">
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
	评分的图标，默认为 "star"。枚举值如下所示：

- star：星星
- heart：爱心
- thumbsup：赞
- fire：火
- smile：笑脸
- lightning：闪电
- flower：花
- number：数字

**示例值**："star"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.field.property.type</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	设置公式字段的数据类型

**注意**：非所有多维表格都支持该能力。请参考[获取多维表格元数据](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/get)接口返回的formula_type 判断，当 `formula_type` 等于 2 时，表示需要设置该字段。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >data_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	公式字段对应的数据类型

**示例值**：1

**可选值有**：
<md-enum>
<md-enum-item key="1" >文本（默认值）、条码</md-enum-item>
<md-enum-item key="2" >数字（默认值）、进度、货币、评分</md-enum-item>
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
	<md-text type="field-name" >ui_property</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.field.property.type.ui_property</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	公式数据属性信息
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

**数据校验规则**：

- 长度范围：`0` ～ `20` 字符
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
	数字、公式字段的显示格式

**示例值**："0"

**数据校验规则**：

- 长度范围：`0` ～ `50` 字符
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

**示例值**：1

**数据校验规则**：

- 取值范围：`0` ～ `1`
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

**示例值**：100

**数据校验规则**：

- 取值范围：`1` ～ `100`
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
	日期、创建时间、最后更新时间字段的显示格式

**示例值**："yyyy/MM/dd"

**数据校验规则**：

- 长度范围：`0` ～ `50` 字符
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
	<md-text type="field-name" >ui_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	公式字段在界面上的展示类型，例如进度字段是数字的一种展示形态。了解更多，参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。

**示例值**："Progress"

**可选值有**：
<md-enum>
<md-enum-item key="Number" >数字</md-enum-item>
<md-enum-item key="Progress" >进度</md-enum-item>
<md-enum-item key="Currency" >货币</md-enum-item>
<md-enum-item key="Rating" >评分</md-enum-item>
<md-enum-item key="DateTime" >日期</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
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


<md-dt-tr level="1">
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
	是否禁止同步该描述，只在新增、修改字段时生效。枚举值：
- true：表示禁止同步该描述内容到表单的问题描述
- false：允许同步该描述

**示例值**：true

**默认值**：`true`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
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
	字段描述内容

**示例值**："这是一个字段描述"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
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
	字段在界面上的展示类型，例如进度字段是数字的一种展示形态。了解更多，参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。

**示例值**："Progress"

**可选值有**：
<md-enum>
<md-enum-item key="Text" >文本</md-enum-item>
<md-enum-item key="Email" >邮箱地址</md-enum-item>
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

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "field_name":"文本",
    "type":1
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
	<md-text type="field-name" >field</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.field</md-text>
	</md-dt-td>
	<md-dt-td>
	字段
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
	多维表格字段名称
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
	多维表格字段类型

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
<md-enum-item key="18" >关联</md-enum-item>
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
	<md-text type="field-name" >property</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.field.property</md-text>
	</md-dt-td>
	<md-dt-td>
	字段属性，具体可参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。
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
	选项名
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
	选项 ID，创建时不允许指定 ID
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
	选项颜色
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
	数字、公式字段的显示格式
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
	日期、创建时间、最后更新时间字段的显示格式
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
	日期字段中新纪录自动填写创建时间
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
	人员字段中允许添加多个成员，单向关联、双向关联中允许添加多个记录
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
	单向关联、双向关联字段中关联的数据表的 ID
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
	单向关联、双向关联字段中关联的数据表的名称
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
	双向关联字段中关联的数据表中对应的双向关联字段的名称
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
	自动编号类型

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
	自动编号的可选规则项类型

**可选值有**：
<md-enum>
<md-enum-item key="system_number" >自增数字位，value 范围 1-9</md-enum-item>
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
	与自动编号的可选规则项类型相对应的取值
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
	地理位置输入限制

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
	公式字段的表达式
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
	是否允许手动录入
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
	是否允许移动端录入
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
	进度、评分等字段的数据范围最小值
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
	进度、评分等字段的数据范围最大值
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
	进度等字段是否支持自定义范围
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
	货币币种
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
	评分字段的符号展示
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.field.property.type</md-text>
	</md-dt-td>
	<md-dt-td>
	设置公式字段的数据类型

**注意**：非所有多维表格都支持该能力。请参考[获取多维表格元数据](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/get)接口返回的formula_type 判断，当 `formula_type` 等于 2 时，表示需要设置该字段。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >data_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	公式字段对应的数据类型

**可选值有**：
<md-enum>
<md-enum-item key="1" >多行文本（默认值）、条码</md-enum-item>
<md-enum-item key="2" >数字（默认值）、进度、货币、评分</md-enum-item>
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


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >ui_property</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.field.property.type.ui_property</md-text>
	</md-dt-td>
	<md-dt-td>
	公式数据属性
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >currency_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	货币币种
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >formatter</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	数字、公式字段的显示格式
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >range_customize</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	进度等字段是否支持自定义范围
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >min</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >number(float)</md-text>
	</md-dt-td>
	<md-dt-td>
	进度、评分等字段的数据范围最小值
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >max</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >number(float)</md-text>
	</md-dt-td>
	<md-dt-td>
	进度、评分等字段的数据范围最大值
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >date_formatter</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	日期、创建时间、最后更新时间字段的显示格式
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >rating</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >rating</md-text>
	</md-dt-td>
	<md-dt-td>
	评分字段的相关设置
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="6">
	<md-dt-td>
	<md-text type="field-name" >symbol</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	评分字段的符号展示
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >ui_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	公式字段在界面上的展示类型，例如进度字段是数字的一种展示形态。了解更多，参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。

**可选值有**：
<md-enum>
<md-enum-item key="Number" >数字</md-enum-item>
<md-enum-item key="Progress" >进度</md-enum-item>
<md-enum-item key="Currency" >货币</md-enum-item>
<md-enum-item key="Rating" >评分</md-enum-item>
<md-enum-item key="DateTime" >日期</md-enum-item>
</md-enum>
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
	是否禁止同步，如果为true，表示禁止同步该描述内容到表单的问题描述（只在新增、修改字段时生效）
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
	字段描述内容
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >is_primary</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否是索引列
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >field_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格字段 ID
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
	字段在界面上的展示类型，例如进度字段是数字的一种展示形态。了解更多，参考[字段编辑指南](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide)。

**可选值有**：
<md-enum>
<md-enum-item key="Text" >文本</md-enum-item>
<md-enum-item key="Email" >邮箱地址</md-enum-item>
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
	<md-text type="field-name" >is_hidden</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否是隐藏字段
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
        "field": {
            "field_id": "fld4bocNLY",
            "field_name": "文本",
            "type": 1,
            "property": null
        }
    },
    "msg": "Success"
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 200 | 1254000 | WrongRequestJson | 请求体错误 |
| 200 | 1254001 | WrongRequestBody | 请求体错误 |
| 200 | 1254002 | Fail | 导致报 1254002 错误码的场景较多，请参考以下建议排查：<br>- 如果单次操作的内容变更较大，请尝试在单次操作中减少数据量<br>- 如果你并发调用了接口，请尝试控制请求间隔，稍后重试<br>- 如果在知识库（wiki）中创建多维表格，请检查你是否使用了知识库[创建知识空间节点](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/create)接口创建多维表格。在此场景下不能使用[创建多维表格](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/create)接口<br>- 请检查接口参数是否有误。例如，在分页查询多维表格时，传递了无效的 page_token，或传递了错误的数据表的 table_id<br>- 如果该报错偶尔发生，可能是服务器超时或不稳定，请重试解决 |
| 200 | 1254003 | WrongBaseToken | app_token 错误 |
| 200 | 1254004 | WrongTableId | table_id 错误。table_id 是多维表格数据表的唯一标识。获取方式：<br>- 你可通过多维表格 URL 获取 `table_id`，下图高亮部分即为当前数据表的 `table_id`<br>- 也可通过[列出数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/list)接口获取 `table_id`<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/18741fe2a0d3cafafaf9949b263bb57d_yD1wkOrSju.png?height=746&lazyload=true&maxWidth=700&width=2976) |
| 200 | 1254005 | WrongViewId | view_id 错误。view_id 是多维表格中视图的唯一标识。获取方式：<br>- 在多维表格的 URL 地址栏中，`view_id` 是下图中高亮部分：<br>![view_id.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/140668632c97e0095832219001d17c54_DJMgVH9x2S.png?height=748&lazyload=true&width=2998)<br>- 通过[列出视图](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/list)接口获取。暂时无法获取到嵌入到云文档中的多维表格的 `view_id`。<br>**注意**：<br>当 `filter` 参数 或 `sort` 参数不为空时，请求视为对数据表中的全部数据做条件过滤，指定的 `view_id` 会被忽略。 |
| 200 | 1254006 | WrongRecordId | record_id 错误。record_id 是数据表中一条记录的唯一标识。通过[查询记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search)接口获取 |
| 200 | 1254007 | EmptyValue | 空值 |
| 200 | 1254008 | EmptyView | 空视图 |
| 200 | 1254009 | WrongFieldId | field_id 错误。field_id 是数据表中一个字段的唯一标识。通过[列出字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/list)接口获取 |
| 200 | 1254010 | ReqConvError | 请求错误 |
| 200 | 1254012 | NotSupportFieldOrView | 不支持的字段或视图。注意数据表的初始索引字段仅支持以下类型：<br>- 1：文本<br>- 2：数字<br>- 5：日期<br>- 13：电话号码<br>- 15：超链接<br>- 20：公式<br>- 22：地理位置 |
| 200 | 1254013 | TableNameDuplicated | 表名重复 |
| 200 | 1254014 | FieldNameDuplicated | 字段名重复 |
| 200 | 1254015 | FieldTypeValueNotMatch | 字段类型和值不匹配 |
| 200 | 1254026 | EmptyOptionName | 选项名不能为空 |
| 400 | 1254028 | EmptyFieldName | 字段名为空 |
| 400 | 1254029 | InvalidFieldName | 字段名无效 |
| 200 | 1254030 | TooLargeResponse | 响应体过大 |
| 400 | 1254036 | Base is copying, please try again later. | 复制多维表格为异步操作，该错误码表示当前多维表格仍在复制中，在复制期间无法操作当前多维表格。需要等待复制完成后再操作 |
| 400 | 1254037 | Invalid client token, make sure that it complies with the specification. | 幂等键格式错误，需要传入 uuidv4 格式 |
| 200 | 1254040 | BaseTokenNotFound | app_token 不存在 |
| 200 | 1254041 | TableIdNotFound | table_id 不存在。获取方式：<br>- 你可通过多维表格 URL 获取 `table_id`，下图高亮部分即为当前数据表的 `table_id`<br>- 也可通过[列出数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/list)接口获取 `table_id`<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/18741fe2a0d3cafafaf9949b263bb57d_yD1wkOrSju.png?height=746&lazyload=true&maxWidth=700&width=2976) |
| 200 | 1254042 | ViewIdNotFound | view_id 不存在。获取方式：<br>- 在多维表格的 URL 地址栏中，`view_id` 是下图中高亮部分：<br>![view_id.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/140668632c97e0095832219001d17c54_DJMgVH9x2S.png?height=748&lazyload=true&width=2998)<br>- 通过[列出视图](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/list)接口获取。暂时无法获取到嵌入到云文档中的多维表格的 `view_id`。<br>**注意**：<br>当 `filter` 参数 或 `sort` 参数不为空时，请求视为对数据表中的全部数据做条件过滤，指定的 `view_id` 会被忽略。 |
| 200 | 1254043 | RecordIdNotFound | record_id 不存在。record_id 是数据表中一条记录的唯一标识。请通过[查询记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search)接口获取 |
| 200 | 1254044 | FieldIdNotFound | field_id  不存在。field_id  是数据表中一个字段的唯一标识。通过[列出字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/list)接口获取 |
| 200 | 1254060 | TextFieldConvFail | 文本字段错误 |
| 200 | 1254061 | NumberFieldConvFail | 数字字段错误 |
| 200 | 1254062 | SingleSelectFieldConvFail | 单选字段错误 |
| 200 | 1254063 | MultiSelectFieldConvFail | 多选字段错误 |
| 200 | 1254064 | DatetimeFieldConvFail | 日期字段错误 |
| 200 | 1254065 | CheckboxFieldConvFail | 复选框字段错误 |
| 200 | 1254066 | UserFieldConvFail | 人员字段有误。原因可能是：<br>- `user_id_type` 参数指定的 ID 类型与传入的 ID 类型不匹配<br>- 传入了不识别的类型或结构，目前只支持填写 `id` 参数，且需要传入数组<br>- 跨应用传入了 `open_id`。如果跨应用传入 ID，建议使用 `user_id`。不同应用获取的 `open_id` 不能交叉使用<br>- 若想对人员字段传空，可传 null |
| 200 | 1254067 | LinkFieldConvFail | 关联字段错误 |
| 200 | 1254070 | ActionValidateFailed | Action验证失败 |
| 400 | 1254080 | TextFieldPropertyError | 文本字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254081 | NumberFieldPropertyError | 数字字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254082 | SingleSelectFieldPropertyError | 单选字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254083 | MultiSelectFieldPropertyError | 多选字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254084 | DateFieldPropertyError | 日期字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254085 | CheckboxFieldPropertyError | 复选框字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254086 | UserFieldPropertyError | 人员字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254087 | URLFieldPropertyError | 超链接字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254088 | AttachFieldPropertyError | 附件字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254089 | LinkFieldPropertyError | 单向关联字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254090 | LookUpFieldPropertyError | 查找引用字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254091 | FormulaFieldPropertyError | 公式字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254092 | DuplexLinkFieldPropertyError | 双向关联字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254093 | CreatedTimeFieldPropertyError | 创建时间字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254094 | ModifiedTimeFieldPropertyError | 最后更新时间字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254095 | CreatedUserFieldPropertyError | 创建人字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 400 | 1254096 | ModifiedUserFieldPropertyError | 修改人字段property错误。请参考[字段编辑指南-字段的属性 property](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/guide#887ee1cd)查看字段的 property 结构。 |
| 200 | 1254100 | TableExceedLimit | 数据表或仪表盘数量超限。每个多维表格中，数据表加仪表盘的数量最多为 100 个 |
| 200 | 1254101 | ViewExceedLimit | 视图数量超限, 限制200个 |
| 200 | 1254102 | FileExceedLimit | 文件数量超限 |
| 200 | 1254103 | RecordExceedLimit | 记录数量超限, 限制20,000条 |
| 200 | 1254104 | RecordAddOnceExceedLimit | 单次添加记录数量超限, 单次调用最多更新 1,000 条记录 |
| 200 | 1254105 | ColumnExceedLimit | 字段数量超限 |
| 200 | 1254130 | TooLargeCell | 格子内容过大 |
| 200 | 1254290 | TooManyRequest | 请求过快，稍后重试 |
| 200 | 1254291 | Write conflict | 在同一个数据表中，并发调用了读写接口或请求过快，出现冲突。请参考以下建议解决：<br>- 确保没有并发调用多维表格读写相关接口<br>- 若操作量较大，建议在接口与接口之间增加 0.5 或 1 秒的延迟，也可在报错中增加重试逻辑，确保业务的稳定性<br>- 对于写接口，可以将接口中的查询参数 `ignore_consistency_check` 设置为 true，表示在读写操作时，暂时忽略一致性检查，以提高性能 |
| 200 | 1254301 | OperationTypeError | 多维表格未开启高级权限或不支持开启高级权限 |
| 200 | 1255001 | InternalError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255002 | RpcError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255003 | MarshalError | 序列化错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255004 | UmMarshalError | 反序列化错误 |
| 200 | 1255005 | ConvError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 400 | 1255006 | Client token conflict, please generate a new client token and try again. | 幂等键冲突，需要重新随机生成一个幂等键 |
| 504 | 1255040 | Request timed out, please try again later | 请求超时，进行重试 |
| 400 | 1254607 | Data not ready, please try again later | 该报错一般是由于前置操作未执行完成，或本次操作数据太大，服务器计算超时导致。遇到该错误码时，建议等待一段时间后重试。通常有以下几种原因：<br>- **编辑操作频繁**：开发者对多维表格的编辑操作非常频繁。可能会导致由于等待前置操作处理完成耗时过长而超时的情况。多维表格底层对数据表的处理基于版本维度的串行方式，不支持并发。因此，并发请求时容易出现此类错误，不建议开发者对单个数据表进行并发请求。<br>- **批量操作负载重**：开发者在多维表格中进行批量新增、删除等操作时，如果数据表的数据量非常大，可能会导致单次请求耗时过长，最终导致请求超时。建议开发者适当降低批量请求的 page_size 以减少请求耗时。<br>- **资源分配与计算开销**：资源分配是基于单文档维度的，如果读接口涉及公式计算、排序等计算逻辑，会占用较多资源。例如，并发读取一个文档下的多个数据表也可能导致该文档阻塞。 |
| 403 | 1254302 | Permission denied. | 调用身份缺少多维表格的高级权限。你需给予调用身份数据表的 **可管理** 权限或多维表格的 **可管理** 等权限，再重新调用。具体步骤如下所示：<br>- 对用户授予高级权限，你可在 **多维表格高级权限设置** 中添加用户，为用户开通足够权限；或在多维表格页面右上方 **分享** 入口为当前用户添加可管理权限。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/df3911b4f747d75914f35a46962d667d_dAsfLjv3QC.png?height=546&lazyload=true&maxWidth=550)<br>- 对应用授予高级权限，你需通过多维表格页面右上方 **「...」** -> **「...更多」** ->**「添加文档应用」** 入口为应用添加可管理权限。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/22c027f63c540592d3ca8f41d48bb107_CSas7OYJBR.png?height=1994&lazyload=true&maxWidth=550&width=3278)<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9f3353931fafeea16a39f0eb887db175_0tjzC9P3zU.png?height=728&lazyload=true&maxWidth=550&width=890)<br>**注意**：<br>在 **添加文档应用** 前，你需确保目标应用至少开通了一个多维表格的 [API 权限](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/scope-list)。否则你将无法在文档应用窗口搜索到目标应用。<br>- 你也可以在 **多维表格高级权限设置** 中添加用户或一个包含应用的群组，给予这个群自定义的读写等权限。 |
| 403 | 1254304 | Permission denied. | 权限不足。请检查多维表格是否开启了高级权限，如果开启高级权限，调用身份需要有多维表格的可管理权限 |
| 403 | 1254608 | ReqRecommited | 基于同一个多维表格版本重复提交了更新请求（传入了相同的 client_token），常见于并发或时间间隔极短的请求，例如并发将一个视图的信息更新为相同的内容。建议稍后重试 |





