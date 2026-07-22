---
document_id: '7242548239428337669'
directory_id: '7241898543983099909'
title: 上传勋章图片
full_path: /uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge_image/create
breadcrumb:
- Server API
- Admin
- Enterprise badge
- Badge management
- Upload a badge image
document_type: ReferenceDocumentType
updated_at: 2023-06-09T05:06:07Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge_image/create
---

# 上传勋章图片

通过该接口可以上传勋章详情图、挂饰图的文件，获取对应的文件key。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=admin&version=v1&resource=badge_image&method=create)

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
| HTTP URL | https://open.larksuite.com/open-apis/admin/v1/badge_images |
| HTTP Method | POST |
| 接口频率限制 | [20 次/分钟](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="admin:badge" desc="查看、创建、编辑勋章信息和上传勋章图片" support_app_types="custom" tags="">查看、创建、编辑勋章信息和上传勋章图片</md-perm> |

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
	<md-text type="field-name" >image_file</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >file</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	勋章图片的文件，仅支持 PNG 格式，320 x 320 像素，大小不超过 1024 KB。

**示例值**：file binary
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >image_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	图片的类型

**示例值**：1

**可选值有**：
<md-enum>
<md-enum-item key="1" >勋章详情图</md-enum-item>
<md-enum-item key="2" >勋章挂饰图</md-enum-item>
</md-enum>

**数据校验规则**：

- 取值范围：`1` ～ `2`
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例

```HTTP
---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="image_file";
Content-Type: application/octet-stream


---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="image_type";

1
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
	<md-text type="field-name" >image_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	图片的key
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
        "image_key": "f02a98aa-1413-4af6-93ab-431ba9e5f2cg"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 1051000 | unknown server error | 服务内部错误，请稍后重试 |
| 400 | 1051001 | request contain invalid param | 请求中包含非法参数 |
| 403 | 1051002 | request to exceed authority | 请求发生越权 |
| 400 | 1051100 | the image size is too large | 上传的图片大小不符合要求 |
| 400 | 1051101 | the image format is illegal | 上传的图片格式不符合要求 |
| 400 | 1051109 | the image height is too small | 图片尺寸高度过小 |
| 400 | 1051110 | the image height is too large | 图片尺寸高度过大 |
| 400 | 1051111 | the aspect ratio of the image is too small | 图片尺寸宽高比过小 |
| 400 | 1051112 | the aspect ratio of the image is too large | 图片尺寸宽高比过大 |
| 403 | 1051003 | the tenant's current payment plan does not support the use of this function | 该租户当前套餐不能使用勋章功能点 |





