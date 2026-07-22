---
document_id: '7296037675742724102'
directory_id: '7278235764373651461'
title: 识别文件中的票据
full_path: /uAjLw4CM/ukTMukTMukTM/document_ai-v1/invoice/recognize
breadcrumb:
- Server API
- AI
- Document AI
- Recognize invoice
- recognize invoice
document_type: ReferenceDocumentType
updated_at: 2024-03-20T07:50:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/document_ai-v1/invoice/recognize
---

# 识别文件中的票据

票据识别接口，支持JPG/JPEG/PNG/PDF四种文件类型的一次性的识别。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=document_ai&version=v1&resource=invoice&method=recognize)

:::html
<md-alert type="tip">
单租户限流：10QPS，同租户下的应用没有限流，共享本租户的 10QPS 限流
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
| HTTP URL | https://open.larksuite.com/open-apis/document_ai/v1/invoice/recognize |
| HTTP Method | POST |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="document_ai:invoice:recognize" desc="识别票据" support_app_types="custom,isv" tags="">识别票据</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **示例值**："multipart/form-data; boundary=---7MA4YWxkTrZu0gW" |




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
	<md-text type="field-name" >file</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >file</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	识别的票据文件

**示例值**：file binary
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例

```HTTP
---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="file";
Content-Type: application/octet-stream


---7MA4YWxkTrZu0gW
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
	<md-text type="field-name" >invoice</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >invoice</md-text>
	</md-dt-td>
	<md-dt-td>
	票据信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >entities</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >invoice_entity\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	识别出的实体类型
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
	识别的字段种类

**可选值有**：
<md-enum>
<md-enum-item key="customer_name" >客户名称</md-enum-item>
<md-enum-item key="invoice_id" >发票 ID</md-enum-item>
<md-enum-item key="invoice_date" >发票开具日</md-enum-item>
<md-enum-item key="due_date" >发票支付截止日</md-enum-item>
<md-enum-item key="vendor_name" >供应商名称</md-enum-item>
<md-enum-item key="vendor_address" >供应商地址</md-enum-item>
<md-enum-item key="vendor_address_recipient" >供应商地址收件人</md-enum-item>
<md-enum-item key="billing_address" >账单邮寄地址</md-enum-item>
<md-enum-item key="billing_address_recipient" >账单邮寄地址收件人</md-enum-item>
<md-enum-item key="shipping_address" >送货地址</md-enum-item>
<md-enum-item key="shipping_address_recipient" >送货地址收件人</md-enum-item>
<md-enum-item key="sub_total" >小计</md-enum-item>
<md-enum-item key="total_tax" >总税额</md-enum-item>
<md-enum-item key="invoice_total" >发票合计</md-enum-item>
<md-enum-item key="sub_total_format" >小计 - 格式化结果，例：CNY 100</md-enum-item>
<md-enum-item key="sub_total_amount" >小计 - 纯数字结果，例：100</md-enum-item>
<md-enum-item key="sub_total_currency_code" >小计 - 货币代码，例：CNY</md-enum-item>
<md-enum-item key="sub_total_currency_symbol" >小计 - 货币标识，例：￥</md-enum-item>
<md-enum-item key="total_tax_format" >总金额 - 格式化结果，例：CNY 100</md-enum-item>
<md-enum-item key="total_tax_amount" >总金额 - 纯数字结果，例：100</md-enum-item>
<md-enum-item key="total_tax_currency_code" >总金额 - 货币代码，例：CNY</md-enum-item>
<md-enum-item key="total_tax_currency_symbol" >总金额 - 货币标识，例：￥</md-enum-item>
<md-enum-item key="invoice_total_format" >发票合计 - 格式化结果，例：CNY 100</md-enum-item>
<md-enum-item key="invoice_total_amount" >发票合计 - 纯数字结果，例：100</md-enum-item>
<md-enum-item key="invoice_total_currency_code" >发票合计 - 货币代码，例：CNY</md-enum-item>
<md-enum-item key="invoice_total_currency_symbol" >发票合计 - 货币标识，例：￥</md-enum-item>
<md-enum-item key="invoice_date_format" >发票开具日 - 格式化结果 yyyy-mm-dd</md-enum-item>
<md-enum-item key="due_date_format" >发票支付截止日 - 格式化结果 yyyy-mm-dd</md-enum-item>
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
	识别出字段的文本信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >items</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >items_entity\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	识别出的费用明细
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >amount</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >currency_entity</md-text>
	</md-dt-td>
	<md-dt-td>
	金额
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	原文字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >format</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	格式化结果
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
	货币代码
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >currency_symbol</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	货币标识
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >amount</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	纯数字结果
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >date</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >common_entity</md-text>
	</md-dt-td>
	<md-dt-td>
	日期
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	原文字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >description</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >common_entity</md-text>
	</md-dt-td>
	<md-dt-td>
	描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	原文字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >product_code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >common_entity</md-text>
	</md-dt-td>
	<md-dt-td>
	产品代码
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	原文字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >quantity</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >common_entity</md-text>
	</md-dt-td>
	<md-dt-td>
	数量
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	原文字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >tax</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >currency_entity</md-text>
	</md-dt-td>
	<md-dt-td>
	税额
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	原文字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >format</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	格式化结果
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
	货币代码
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >currency_symbol</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	货币标识
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >amount</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	纯数字结果
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >tax_rate</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >common_entity</md-text>
	</md-dt-td>
	<md-dt-td>
	税率
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	原文字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >unit</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >common_entity</md-text>
	</md-dt-td>
	<md-dt-td>
	单位
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	原文字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >unit_price</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >currency_entity</md-text>
	</md-dt-td>
	<md-dt-td>
	单价
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	原文字
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >format</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	格式化结果
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
	货币代码
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >currency_symbol</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	货币标识
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="5">
	<md-dt-td>
	<md-text type="field-name" >amount</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	纯数字结果
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
        "invoice": {
            "entities": [
                {
                    "type": "customer_name",
                    "value": "Tom",
                    "items": [
                        {
                            "amount": {
                                "content": "￥100",
                                "format": "￥100",
                                "currency_code": "CNY",
                                "currency_symbol": "￥",
                                "amount": "100"
                            },
                            "date": {
                                "content": "this is an example"
                            },
                            "description": {
                                "content": "this is an example"
                            },
                            "product_code": {
                                "content": "this is an example"
                            },
                            "quantity": {
                                "content": "this is an example"
                            },
                            "tax": {
                                "content": "￥100",
                                "format": "￥100",
                                "currency_code": "CNY",
                                "currency_symbol": "￥",
                                "amount": "100"
                            },
                            "tax_rate": {
                                "content": "this is an example"
                            },
                            "unit": {
                                "content": "this is an example"
                            },
                            "unit_price": {
                                "content": "￥100",
                                "format": "￥100",
                                "currency_code": "CNY",
                                "currency_symbol": "￥",
                                "amount": "100"
                            }
                        }
                    ]
                }
            ]
        }
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 2110001 | param is invalid | 输入文件错误，参考文档检查输入参数 |
| 400 | 2110002 | no valid entity | 未检测出票据信息，参考文档检查输入文件是否有效 |
| 500 | 2110010 | internal error, please try later | 后端服务异常或网络异常，可重新请求 |
| 400 | 2110003 | You have reached the Intelligent document parsing limit. To continue using this function, please contact sales to purchase more. | 智能文档解析次数已达使用上限，如需继续使用，请联系销售购买 |





