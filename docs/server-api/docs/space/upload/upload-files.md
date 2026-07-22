---
document_id: '7028022131297304581'
directory_id: '7148629329634689030'
title: 上传文件
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_all
breadcrumb:
- Server API
- Docs
- Space
- Upload
- Upload Files
document_type: ReferenceDocumentType
updated_at: 2022-10-08T09:38:11Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_all
---

# 上传文件

向云空间指定目录下上传一个小文件。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=file&method=upload_all)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">
请不要使用这个接口上传大于20MB的文件，如果有这个需求可以尝试使用[分片上传接口](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/multipart-upload-file-/introduction)。
</md-alert>
:::

:::html
<md-alert type="tip">
该接口支持调用频率上限为5QPS
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/v1/files/upload_all |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:file" desc="上传、下载文件到云空间" support_app_types="custom,isv" tags="">上传、下载文件到云空间</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **示例值**："multipart/form-data; boundary=---7MA4YWxkTrZu0gW" |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

### 请求体

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 30%;">名称</md-dt-th>
      <md-dt-th style="width: 20%;">类型</md-dt-th>
      <md-dt-th style="width: 10%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 50%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >file_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	文件名。

**示例值**："demo.pdf"

**数据校验规则**：

- 最大长度：`250` 字符
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >parent_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	上传点类型。

**示例值**："explorer"

**可选值有**：
<md-enum>
<md-enum-item key="explorer" >云空间。</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >parent_node</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	文件夹token，
获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)

**示例值**："fldbcO1UuPz8VwnpPx5a92abcef"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >size</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	文件大小（以字节为单位）。

**示例值**：1024

**数据校验规则**：

- 最大值：`20971520`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >checksum</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	文件adler32校验和(可选)。

**示例值**："123423882374238912356"
	</md-dt-td>
</md-dt-tr>


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
	文件二进制内容。

**示例值**：file binary
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::

### cURL示例
```shell
curl --location --request POST 'https://open.larksuite.com/open-apis/drive/v1/files/upload_all' \
--header 'Authorization: Bearer t-e13d5ec1954e82e458f3ce04491c54ea8c9abcef' \
--header 'Content-Type: multipart/form-data' \
--form 'file_name="demo.pdf"' \
--form 'parent_type="explorer"' \
--form 'parent_node="fldbcO1UuPz8VwnpPx5a92abcef"' \
--form 'size="1024"' \
--form 'file=@"/path/demo.pdf"'
```


### Python示例
```python
import os
import requests
from requests_toolbelt import MultipartEncoder

def upload_file():
    file_path = "/path/demo.pdf"
    file_size = os.path.getsize(file_path)
    url = "https://open.larksuite.com/open-apis/drive/v1/files/upload_all"
    form = {'file_name': 'demo.pdf',
            'parent_type': 'explorer',
            'parent_node': 'fldbcO1UuPz8VwnpPx5a92abcef',
            'size': str(file_size),
            'file': (open(file_path, 'rb'))}  
    multi_form = MultipartEncoder(form)
    headers = {
        'Authorization': 'Bearer t-e13d5ec1954e82e458f3ce04491c54ea8c9abcef',  ## 获取tenant_access_token, 需要替换为实际的token
    }
    headers['Content-Type'] = multi_form.content_type
    response = requests.request("POST", url, headers=headers, data=multi_form)

if __name__ == '__main__':
    upload_file()
```

### 请求体示例

```HTTP
---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="file_name";

demo.pdf
---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="parent_type";

explorer
---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="parent_node";

fldbcO1UuPz8VwnpPx5a92abcef
---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="size";

1024
---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="checksum";

123423882374238912356
---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="file";
Content-Type: application/octet-stream

file binary
---7MA4YWxkTrZu0gW
```



## 响应



### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 30%;">名称</md-dt-th>
      <md-dt-th style="width: 20%;">类型</md-dt-th>
      <md-dt-th style="width: 50%;">描述</md-dt-th>
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
	<md-text type="field-name" >file_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	新创建文件的 token
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
        "file_token": "boxcnrHpsg1QDqXAAAyachabcef"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 200 | 1061001 | internal error. | 服务内部错误，包括超时，错误码没处理。 |
| 400 | 1061002 | params error. | 请检查请求参数是否正确。 |
| 404 | 1061003 | not found. | 请确认对应资源是否存在。 |
| 403 | 1061004 | forbidden. | 请确认当前身份是否有对应上传节点的的权限，如用户是否有上传到指定doc的编辑权限。 |
| 400 | 1061021 | upload id expire. | 上传事务过期，请重头开始上传。 |
| 500 | 1061022 | file version conflict. | 文件版本号冲突。 |
| 400 | 1061041 | parent node has been deleted. | 请确认上传点未被删除。 |
| 400 | 1061042 | parent node out of limit. | 在当前上传点上传过多素材，请更换上传点。 |
| 400 | 1061043 | file size beyond limit. | 请检查文件长度以避免超出限制。[具体限制请参考](https://www.feishu.cn/hc/zh-CN/articles/360049067549) |
| 400 | 1061044 | parent node not exist. | 请确认上传点是否存在。 |
| 200 | 1061045 | can retry. | 内部可重试错误，请稍后重试。 |
| 400 | 1061109 | file name cqc not passed. | 请确保上传的文件和文件名合规。 |
| 400 | 1061101 | file quota exceeded. | 租户容量超限，请确保租户有足够容量进行上传。 |
| 403 | 1061500 | mount node point kill. | 挂载点不存在。 |
| 400 | 1062007 | upload user not match. | 请确保当前请求身份和上传任务的身份为同一个。 |
| 400 | 1062008 | checksum param Invalid. | 请确保文件/文件块的checksum正确。 |
| 400 | 1062009 | the actual size is inconsistent with the parameter declaration size. | 实际传输的文件大小和参数说明的大小不符合一致。 |
| 400 | 1062010 | block missing, please upload all blocks. | 部分文件分片缺失，请确保所有文件分片上传完成。 |
| 400 | 1062011 | block num out of bounds. | 上传过多文件分片，请确保上传的为对应文件。 |
| 400 | 1061061 | user quota exceeded. | 个人容量超限，请确保个人有足够容量进行上传。 |
| 403 | 1061073 | no scope auth. | 没有申请接口权限。 |
| 200 | 1064230 | locked for data migration | 数据迁移中，暂时无法上传。 |
| 400 | 1062505 | parent node out of size. | 云空间单树大小超限制（40W限制 ）。 |
| 400 | 1062506 | parent node out of depth. | 云空间目录深度超限制（15限制）。 |
| 400 | 1062507 | parent node out of sibling num. | 云空间目录下挂载数量超过限制（单层**1500**限制 ）。 |





