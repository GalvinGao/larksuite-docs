---
document_id: '7026663896463261701'
directory_id: '7002892512470777862'
title: 下载文件
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/get
breadcrumb:
- Server API
- Messaging
- File message
- Download file
document_type: ReferenceDocumentType
updated_at: 2024-06-05T08:08:34Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/get
---

# 下载文件

下载文件接口，只能下载应用自己上传的文件。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=im&version=v1&resource=file&method=get)

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
注意事项:
- 需要开启[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
- 只能下载机器人自己上传的文件
- 下载用户发送的资源，请使用[获取消息中的资源文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-resource/get)接口
- 下载的资源大小不能超过100M
- 如果需要Content-Disposition header，发起请求的时候需要在header中设置Content-Type为application/json
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/im/v1/files/:file_key |
| HTTP Method | GET |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | 无 |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




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
	<md-text type="field-name" >file_key</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	文件的key，通过[上传文件](	/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/create)接口上传图片后获得

**示例值**："file_456a92d6-c6ea-4de4-ac3f-7afcf44ac78g"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





## 响应



HTTP状态码为 200 时，表示成功

返回文件二进制流



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 234001 | Invalid request param. | 检查请求参数是否正确。 |
| 401 | 234002 | Unauthorized. | 鉴权失败，联系Oncall解决。 |
| 400 | 234008 | The app is not the resource sender | 应用不是资源的所有者。 |
| 400 | 234005 | Image has been deleted | 资源不存在。 |
| 400 | 234037 | Downloaded file size exceeds limit. | 不允许下载的资源大小超过100MB限制。 |
| 400 | 234041 | Tenant master key has been deleted, please contact the tenant administrator. | 租户加密密钥被删除，请联系租户管理员。 |
| 400 | 234042 | Hybrid deployment tenant storage error, such as full storage space, please contact tenant administrator. | 请求出现混布租户存储错误，如存储空间已满等，请联系租户管理员或技术支持。 |





