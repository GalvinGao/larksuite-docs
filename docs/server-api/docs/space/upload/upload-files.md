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
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/drive/v1/files/upload_all</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom,isv"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
            
      </md-th>
      <md-td>
            <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
            <md-perm name="drive:file" desc="上传、下载文件到云空间" support_app_types="custom,isv" tags="">上传、下载文件到云空间</md-perm>
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 35%;">名称</md-th>
      <md-th style="width: 13%;">类型</md-th>
       <md-th style="width: 15%;" filters="是,否" >必填</md-th>
      <md-th  style="width: 37%;">描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
或
<md-tag mode="inline" type="token-user">user_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"

[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)

</md-td>
</md-tr>
<md-tr>
<md-td>Content-Type</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>**示例值**："multipart/form-data; boundary=---7MA4YWxkTrZu0gW"</md-td>
</md-tr>
</md-tbody>
</md-table>
:::

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
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">HTTP状态码</md-th>
            <md-th style="width: 15%;">错误码</md-th>
            <md-th style="width: 30%;">描述</md-th>
            <md-th style="width: 30%;">排查建议</md-th>
        </md-tr>
    </md-thead>
  <md-tbody>

<md-tr>
  <md-td>200</md-td>
  <md-td>1061001</md-td>
  <md-td>internal error.</md-td>
  <md-td>服务内部错误，包括超时，错误码没处理。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1061002</md-td>
  <md-td>params error.</md-td>
  <md-td>请检查请求参数是否正确。</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1061003</md-td>
  <md-td>not found.</md-td>
  <md-td>请确认对应资源是否存在。</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1061004</md-td>
  <md-td>forbidden.</md-td>
  <md-td>请确认当前身份是否有对应上传节点的的权限，如用户是否有上传到指定doc的编辑权限。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1061021</md-td>
  <md-td>upload id expire.</md-td>
  <md-td>上传事务过期，请重头开始上传。</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1061022</md-td>
  <md-td>file version conflict.</md-td>
  <md-td>文件版本号冲突。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1061041</md-td>
  <md-td>parent node has been deleted.</md-td>
  <md-td>请确认上传点未被删除。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1061042</md-td>
  <md-td>parent node out of limit.</md-td>
  <md-td>在当前上传点上传过多素材，请更换上传点。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1061043</md-td>
  <md-td>file size beyond limit.</md-td>
  <md-td>请检查文件长度以避免超出限制。[具体限制请参考](https://www.feishu.cn/hc/zh-CN/articles/360049067549)</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1061044</md-td>
  <md-td>parent node not exist.</md-td>
  <md-td>请确认上传点是否存在。</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1061045</md-td>
  <md-td>can retry.</md-td>
  <md-td>内部可重试错误，请稍后重试。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1061109</md-td>
  <md-td>file name cqc not passed.</md-td>
  <md-td>请确保上传的文件和文件名合规。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1061101</md-td>
  <md-td>file quota exceeded.</md-td>
  <md-td>租户容量超限，请确保租户有足够容量进行上传。</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1061500</md-td>
  <md-td>mount node point kill.</md-td>
  <md-td>挂载点不存在。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1062007</md-td>
  <md-td>upload user not match.</md-td>
  <md-td>请确保当前请求身份和上传任务的身份为同一个。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1062008</md-td>
  <md-td>checksum param Invalid.</md-td>
  <md-td>请确保文件/文件块的checksum正确。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1062009</md-td>
  <md-td>the actual size is inconsistent with the parameter declaration size.</md-td>
  <md-td>实际传输的文件大小和参数说明的大小不符合一致。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1062010</md-td>
  <md-td>block missing, please upload all blocks.</md-td>
  <md-td>部分文件分片缺失，请确保所有文件分片上传完成。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1062011</md-td>
  <md-td>block num out of bounds.</md-td>
  <md-td>上传过多文件分片，请确保上传的为对应文件。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1061061</md-td>
  <md-td>user quota exceeded.</md-td>
  <md-td>个人容量超限，请确保个人有足够容量进行上传。</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1061073</md-td>
  <md-td>no scope auth.</md-td>
  <md-td>没有申请接口权限。</md-td>
</md-tr>


<md-tr>
  <md-td>200</md-td>
  <md-td>1064230</md-td>
  <md-td>locked for data migration</md-td>
  <md-td>数据迁移中，暂时无法上传。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1062505</md-td>
  <md-td>parent node out of size.</md-td>
  <md-td>云空间单树大小超限制（40W限制 ）。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1062506</md-td>
  <md-td>parent node out of depth.</md-td>
  <md-td>云空间目录深度超限制（15限制）。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1062507</md-td>
  <md-td>parent node out of sibling num.</md-td>
  <md-td>云空间目录下挂载数量超过限制（单层**1500**限制 ）。</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




