---
document_id: '7270779605447180294'
directory_id: '7270719284443267077'
title: TextBlockData
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/textblockdata/textblockdata
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- BlockData
- TextBlockData
- TextBlockData
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/textblockdata/textblockdata
---

# TextBlockData
文本类 Block 数据结构，继承于 [BlockData](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/blockdata)

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
<md-td>text</md-td>
<md-td>object</md-td>
<md-td>是</md-td>
<md-td>文本数据</md-td>
</md-tr>
<md-tr>
<md-td>∟elements</md-td>
<md-td>object</md-td>
<md-td>是</md-td>
<md-td>文本元素数组， 其中 TextElement 类型是下面类型中的一种：
- TextRun
- MentionUser
- MentionDoc
- Reminder
- File
- InlineBlock
- Equation
  </md-td>
</md-tr>
<md-tr>
<md-td>plain_text</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>返回文档上显示的文本数据，包括（reminder，InlineBlock， MentionUser，Equation， MentionDoc等）</md-td>
</md-tr>
</md-tbody>
</md-table>
:::
