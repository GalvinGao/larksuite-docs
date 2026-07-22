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

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| text | object | 是 | 文本数据 |
| ∟elements | object | 是 | 文本元素数组， 其中 TextElement 类型是下面类型中的一种：<br>- TextRun<br>- MentionUser<br>- MentionDoc<br>- Reminder<br>- File<br>- InlineBlock<br>- Equation |
| plain_text | string | 是 | 返回文档上显示的文本数据，包括（reminder，InlineBlock， MentionUser，Equation， MentionDoc等） |

