---
document_id: '7348285063958478854'
directory_id: '7346424663796760582'
title: 识别文件中的日本驾驶证
full_path: /uAjLw4CM/ukTMukTMukTM/document_ai-v1/jp_driving_license/recognize
breadcrumb:
- Server API
- AI
- Document AI
- Recognize Japanese Driving License
- Identify Japanese driving license in documents
document_type: ReferenceDocumentType
updated_at: 2024-03-20T07:50:09Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/document_ai-v1/jp_driving_license/recognize
---

# 识别文件中的日本驾驶证

日本驾驶证识别接口，支持JPG/JPEG/PNG/BMP四种文件类型的一次性的识别。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=document_ai&version=v1&resource=jp_driving_license&method=recognize)

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
| HTTP URL | https://open.larksuite.com/open-apis/document_ai/v1/jp_driving_license/recognize |
| HTTP Method | POST |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="document_ai:jp_driving_license" desc="日本驾驶证识别" support_app_types="custom,isv" tags="">日本驾驶证识别</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
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
	识别的日本驾驶证源文件

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
	<md-text type="field-name" >jp_driving_license</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >jp_driving_license</md-text>
	</md-dt-td>
	<md-dt-td>
	日本驾驶证信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >entities</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >jp_driving_license_entity\[\]</md-text>
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
<md-enum-item key="card_number" >驾驶证号</md-enum-item>
<md-enum-item key="full_name" >姓名</md-enum-item>
<md-enum-item key="birthday" >生日</md-enum-item>
<md-enum-item key="birthday_2nd" >生日(非本国常用表达方式)</md-enum-item>
<md-enum-item key="address" >地址</md-enum-item>
<md-enum-item key="effective_date" >有效期</md-enum-item>
<md-enum-item key="issue_date" >颁发日期</md-enum-item>
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
        "jp_driving_license": {
            "entities": [
                {
                    "type": "card_number",
                    "value": "12**56"
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
| 400 | 2110001 | Param is invalid | 输入文件错误，参考文档检查输入参数 |
| 400 | 2110002 | No valid entity | 未检测出票据信息，参考文档检查输入文件是否有效 |
| 500 | 2110010 | Internal error, please try later. | 后端服务异常或网络异常，可重新请求 |
| 400 | 2110003 | You have reached the Intelligent document parsing limit. To continue using this function, please contact sales to purchase more. | 智能文档解析次数已达使用上限，如需继续使用，请联系销售购买 |





