---
document_id: '7152073576123662341'
directory_id: '6907567266540732418'
title: 获取文件夹下的清单
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list
breadcrumb:
- Server API
- Docs
- Space
- Folder
- List items in folder
document_type: ReferenceDocumentType
updated_at: 2023-08-03T02:02:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list
---

# 获取文件夹下的清单

获取用户云空间中指定文件夹下的文件清单。清单类型包括文件、各种在线文档（文档、电子表格、多维表格、思维笔记）、文件夹和快捷方式。该接口支持分页，但是不会递归获取子文件夹的清单。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=file&method=list)

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
让应用（tenant_access_token）访问个人云空间中的文件夹请参阅[常见问题](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/faq)第三点。
</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/drive/v1/files |
| HTTP Method | GET |
| 接口频率限制 | [特殊频控](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云空间中所有文件</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




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
	<md-text type="field-name" >page_size</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	分页大小

**示例值**：10

**数据校验规则**：

- 最大值：`200`
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

**示例值**：MTY1NTA3MTA1OXw3MTA4NDc2MDc1NzkyOTI0Nabcef
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
	文件夹的token（若不填写该参数或填写空字符串，则默认获取用户云空间下的清单，且不支持分页）

**示例值**：fldbcO1UuPz8VwnpPx5a9abcef
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
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
	<md-text type="field-name" >files</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >file\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	文件夹清单列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	文件标识
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
	文件名
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
	文件类型

**可选值有**：

- `doc`：旧版文档

- `sheet`：表格

- `mindnote`：思维导图

- `bitable`：多维表格

- `file`：文件

- `docx`：新版文档

- `folder`：文件夹
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >parent_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	父文件夹标识
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
	在浏览器中查看的链接
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >shortcut_info</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >shortcut_info</md-text>
	</md-dt-td>
	<md-dt-td>
	快捷方式文件信息
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >target_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	快捷方式指向的原文件类型

**可选值有**：

- `doc`：旧版文档

- `sheet`：表格

- `mindnote`：思维导图

- `bitable`：多维表格

- `file`：文件

- `docx`：新版文档
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >target_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	快捷方式指向的原文件token
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
	分页标记，当 has_more 为 true 时，会同时返回下一次遍历的page_token，否则则不返回
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >has_more</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否还有更多项
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
        "files": [
            {
                "name": "test pdf.pdf",
                "parent_token": "fldbcO1UuPz8VwnpPx5a9abcef",
                "token": "boxbc0dGSMu23m7QkC1bvabcef",
                "type": "file",
                "url": "https://larksuite.com/file/boxbc0dGSMu23m7QkC1bvabcef"
            },
            {
                "name": "test file.docx",
                "parent_token": "fldcnCEG903UUB4fUqfysdabcef",
                "shortcut_info": {
                    "target_token": "boxcnRPaXpD4I6Je9t8k8babcef",
                    "target_type": "file"
                },
                "token": "nodcnVkiLQ6LD4CsUEaANrabcef",
                "type": "shortcut",
                "url": "https://larksuite.com/file/boxcnRPaXpD4I6Je9t8k8babcef"
            },
            {
                "name": "test docx",
                "parent_token": "fldcnCEG903UUB4fUqfysdabcef",
                "token": "doxcntan34DX4QoKJu7jJyabcef",
                "type": "docx",
                "url": "https://larksuite.com/docx/doxcntan34DX4QoKJu7jJyabcef"
            },
            {
                "name": "test sheet",
                "parent_token": "fldcnCEG903UUB4fUqfysdabcef",
                "token": "shtcnOko1Ad0HU48HH8KHuabcef",
                "type": "sheet",
                "url": "https://larksuite.com/sheets/shtcnOko1Ad0HU48HH8KHuabcef"
            }
        ],
        "has_more": false
    },
    "msg": "success"
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 500 | 1061001 | internal error. | 服务内部错误，包括超时，错误码没处理。 |
| 400 | 1061002 | params error. | 请检查请求参数是否正确。 |
| 404 | 1061003 | not found. | 请确认对应资源是否存在。 |
| 403 | 1061004 | forbidden. | 请确认当前身份是否有对应上传节点的的权限，如用户是否有上传到指定doc的编辑权限。 |
| 401 | 1061005 | auth failed. | 请使用正确身份访问该接口。 |
| 404 | 1061007 | file has been delete. | 请确认对应节点未被删除。 |
| 400 | 1064001 | page size out of limit. | 请检查分页大小是否超过限制。 |





