---
document_id: '7139727756258033670'
directory_id: '7122028361538994182'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/file/overview
breadcrumb:
- Server API
- Approval
- Native approval document
- Resource introduction
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/file/overview
---

# 资源介绍
当审批表单中有图片或附件控件时，开发者需在创建审批实例前通过审批上传文件接口将文件上传到审批系统。

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: ;">名称</md-dt-th>
      <md-dt-th style="width: ;">类型</md-dt-th>
      <md-dt-th style="width: ;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	文件名（需包含文件扩展名）

**示例值**："文件.doc"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>

	<md-dt-td>
	文件类型

**示例值**："attachment"

**可选值有**：
<md-enum>
<md-enum-item key="attachment" >attachment</md-enum-item>
<md-enum-item key="image" >image</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >content</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >file</md-text>
	</md-dt-td>

	<md-dt-td>
	文件

**示例值**：123.doc
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::

## 数据示例
```json
{
	"name":"123.doc",
	"type":"attachment",
	"content":123.doc
}
```
