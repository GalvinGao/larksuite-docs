---
document_id: '7543849381893574065'
directory_id: '7539767820226658310'
title: 分页批量查询国家/地区
full_path: /uAjLw4CM/ukTMukTMukTM/mdm-v3/country_region/list
breadcrumb:
- Server API
- Feishu Master Data Management
- Common Data
- country region
- Pagination Batch Query Country Region
document_type: ReferenceDocumentType
updated_at: 2025-08-29T03:48:36Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/mdm-v3/country_region/list
---

# 分页批量查询国家/地区

分页批量查询国家/地区。资源介绍请参考[概述](/document/uAjLw4CM/ukTMukTMukTM/mdm-v3/country_region/resource-definition)。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=mdm&version=v3&resource=country_region&method=list)

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



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/mdm/v3/country_regions |
| HTTP Method | GET |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | mdm:country_region:read |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




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
	<md-text type="field-name" >languages</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	希望返回的语言种类，支持格式如下：
- 中文：zh-CN
- 英文：en-US
- 日文：ja-JP
<br>对于多语文本字段，传入特定语言，将会返回对应语言文本

**示例值**：en-US

**数据校验规则**：

- 长度范围：`0` ～ `100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >fields</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	需要的查询字段集

**示例值**：name

**数据校验规则**：

- 长度范围：`1` ～ `100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >limit</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	查询页大小

**示例值**：10

**数据校验规则**：

- 取值范围：`1` ～ `1000`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >offset</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	查询起始位置

**示例值**：0

**数据校验规则**：

- 取值范围：`0` ～ `100000`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >return_count</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	是否返回总数

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >page_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果

**示例值**：eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=
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
	<md-text type="field-name" >filter</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >filter</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	过滤参数
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >logic</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	逻辑关系<br>同一层级的多个expression由logic参数决定使用“与/或”条件<br>0=and，1=or

**示例值**："0"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >expressions</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >expression\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	过滤条件

**数据校验规则**：

- 长度范围：`0` ～ `100`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >field</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	字段名

**示例值**："name"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >operator</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	运算符。<br>0=等于，1=不等于，2=大于，3=大于等于，4=小于，5=小于等于，6=属于任意，7=不属于任意，8=匹配，9=前缀匹配，10=后缀匹配，11=为空，12=不为空

**示例值**："0"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >value</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	字段值
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >string_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	字符串值

**示例值**："安道尔"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >bool_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	布尔值

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >int_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	整型值

**示例值**："111"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >string_list_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	字符串列表值

**示例值**：["1"]
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >int_list_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	整型列表值

**示例值**：["1"]
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >common</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >common</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	此参数可忽略
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "filter": {
        "logic": "0",
        "expressions": [
            {
                "field": "name",
                "operator": "0",
                "value": {
                    "string_value": "安道尔",
                    "bool_value": true,
                    "int_value": "111",
                    "string_list_value": [
                        "1"
                    ],
                    "int_list_value": [
                        "1"
                    ]
                }
            }
        ]
    },
    "common": {}
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
	<md-text type="field-name" >data</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >country_region\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	国家/地区目录列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >alpha_3_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	三位字母代码。与ISO国家代码的三位代码一致。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >alpha_2_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	两位字母代码。与ISO国家代码的二位代码一致。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >numeric_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	数字代码。与ISO国家代码的Numeric代码一致
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_string</md-text>
	</md-dt-td>
	<md-dt-td>
	国家/地区名称
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
	入参languages中排序第一的语言对应的值。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >multilingual_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >map&lt;string, string&gt;</md-text>
	</md-dt-td>
	<md-dt-td>
	入参languages中所有语言对应的值。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >return_language</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	value实际返回的值对应的语言，如"zh-CN"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >mdm_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	主数据编码（系统生成的唯一永久代码，格式为“MDCT+8位数字”）
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >full_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >i18n_string</md-text>
	</md-dt-td>
	<md-dt-td>
	国家/地区全称
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
	入参languages中排序第一的语言对应的值。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >multilingual_value</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >map&lt;string, string&gt;</md-text>
	</md-dt-td>
	<md-dt-td>
	入参languages中所有语言对应的值。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >return_language</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	value实际返回的值对应的语言，如"zh-CN"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >global_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	国际电话区号
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >status</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是否生效。0代表否，1代表是
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >continents</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >enum</md-text>
	</md-dt-td>
	<md-dt-td>
	所属大洲。可选值如下<br>1-亚洲，2-欧洲，3-非洲，4-北美洲，5-南美洲，6-大洋洲，7-南极洲）
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
	入参languages中排序第一的语言对应的值。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >multilingual_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >map&lt;string, string&gt;</md-text>
	</md-dt-td>
	<md-dt-td>
	入参languages中所有语言对应的值。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >total</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	总数
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >next_page_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	下一次分页参数
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
        "data": [
            {
                "alpha_3_code": "AND",
                "alpha_2_code": "AD",
                "numeric_code": "20",
                "name": {
                    "value": "安道尔",
                    "multilingual_value": {
                        "zh-CN": "安道尔"
                    },
                    "return_language": "zh-CN"
                },
                "mdm_code": "MDCT00000001",
                "full_name": {
                    "value": "安道尔公国",
                    "multilingual_value": {
                        "zh-CN": "安道尔公国"
                    },
                    "return_language": "zh-CN"
                },
                "global_code": "+376",
                "status": "1",
                "continents": {
                    "value": "2",
                    "multilingual_name": {
                        "zh-CN": "安道尔"
                    }
                }
            }
        ],
        "total": "0",
        "next_page_token": "token"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1640201 | invalid params | 参数非法，请检查参数 |
| 400 | 1640202 | param not found | 参数为空，请检查参数填写是否完整 |
| 400 | 1640203 | object field not found | 此字段不在查询范围内，请检查 |
| 400 | 1640204 | language is invalid | 非法语言，请重新填写 |
| 400 | 1640205 | params is required | 必填参数未填写，请检查 |
| 400 | 1640206 | limit too large | 查询数量过大，请限制在1000以内 |
| 400 | 1640207 | limit plus offset too large | 查询数量和起始位置加和过大，请限制在10000以内 |
| 400 | 1640208 | page token invalid | 分页码无效，请检查该参数 |
| 400 | 1640209 | record id not found | 未发现符合条件行级记录，请检查查询参数 |
| 400 | 1640210 | sort by null is not allowed | 排序参数填写有误，请检查 |
| 400 | 1640211 | biz codes is empty | 业务编码为空，请检查 |
| 400 | 1640212 | mdm codes is empty | 主数据编码为空，请检查 |
| 400 | 1640213 | biz id is invalid | 业务编码非法，请检查 |
| 500 | 1640101 | system error | 系统错误，联系Lark技术支持排查 |
| 500 | 1640102 | invalid generic method | 请求方法错误，联系Lark技术支持排查 |
| 500 | 1640103 | call meta service error | 请求元数据方法错误，联系Lark技术支持排查 |
| 500 | 1640104 | meta data not found | 找不到对应元数据，联系Lark技术支持排查 |
| 500 | 1640105 | data source request invalid | 数据库请求非法，联系Lark技术支持排查 |
| 500 | 1640106 | db error | 数据库错误，联系Lark技术支持排查 |
| 500 | 1640107 | func not supported yet | 服务不支持该方法，联系Lark技术支持排查 |
| 500 | 1640108 | field type not supported yet | 字段类型不支持，联系Lark技术支持排查 |
| 500 | 1640109 | validator not found | 未发现数据校验规则，联系Lark技术支持排查 |
| 500 | 1640110 | transformer not found | 未发现数据转换规则，联系Lark技术支持排查 |
| 500 | 1640111 | executor not found | 未发现执行器，联系Lark技术支持排查 |
| 500 | 1640112 | invalid data source type | 错误数据库类型，联系Lark技术支持排查 |
| 500 | 1640113 | data source not found | 未发现数据库，联系Lark技术支持排查 |
| 500 | 1640114 | data source invalid | 数据库无效，联系Lark技术支持排查 |
| 500 | 1640115 | meta setting not found | 未发现元数据配置，联系Lark技术支持排查 |
| 500 | 1640116 | generate page token error | 分页码生成错误，联系Lark技术支持排查 |
| 500 | 1640117 | record duplicate error | 行级记录重复，联系Lark技术支持排查 |
| 500 | 1640118 | generate idl error | idl生成错误，联系Lark技术支持排查 |
| 500 | 1640119 | producer not found error | 消息生产者错误，联系Lark技术支持排查 |
| 500 | 1640120 | custom func request error | 自定义执行器请求错误，联系Lark技术支持排查 |
| 500 | 1640121 | custom func response error | 自定义执行器响应错误，联系Lark技术支持排查 |





