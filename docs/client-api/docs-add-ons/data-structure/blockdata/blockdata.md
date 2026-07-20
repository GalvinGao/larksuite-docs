---
document_id: '7270779605446803462'
directory_id: '7270719284443430917'
title: BlockData
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/blockdata
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- BlockData
- BlockData
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/blockdata
---

# BlockData
Block 的详细数据定义，不同 Block 类型有不同的数据定义。以下是公共属性：
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
<md-td>algin</md-td>
<md-td>string</md-td>
<md-td>否</md-td>
<md-td>Block 的对齐方式，可选值包括：
- 左对齐 - left
- 居中对齐 - center
- 右对齐 - right
  
注意，尽管该属性在公共属性上，但并非所有 Block 的视图都支持该属性，目前支持的 Block 类型包括：
- TEXT
- HEADING1
- HEADING2
- HEADING3
- HEADING4
- HEADING5
- HEADING6
- HEADING7
- HEADING8
- HEADING9
- BULLET
- ORDERED
- TODO
- QUOTE
- IMAGE
- CHAT_CARD
  </md-td>
</md-tr>
</md-tbody>
</md-table>
:::
