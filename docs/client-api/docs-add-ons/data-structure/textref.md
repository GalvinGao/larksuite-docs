---
document_id: '7270779605451571206'
directory_id: '7270719284443660293'
title: TextRef
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/TextRef
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- TextRef
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/TextRef
---

# TextRef
文本的引用对象。
:::html
<md-table>
<md-thead>
<md-tr>
<md-th>名称</md-th>
<md-th>数据类型</md-th>
<md-th>是否必填</md-th>
<md-th>描述</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>blockRef</md-td>
<md-td>[BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef)</md-td>
<md-td>是</md-td>
<md-td>对应的 Block 引用</md-td>
</md-tr>
<md-tr>
<md-td>range</md-td>
<md-td>[number, number]</md-td>
<md-td>是</md-td>
<md-td>文本范围，假设数组为 [start, end]，则：
  - start > end 时，是非法值，会降级为 [start, start]
  - start = end 时，表示文本位置
  - start < end 时，表示文本范围

start 和 end 超出 Block 的文本内容时，则设置为文本内容的最后位置。
</md-td>
</md-tr>
</md-tbody>
</md-table>
:::
