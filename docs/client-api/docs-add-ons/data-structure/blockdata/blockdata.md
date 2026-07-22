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

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| algin | string | 否 | Block 的对齐方式，可选值包括：<br>- 左对齐 - left<br>- 居中对齐 - center<br>- 右对齐 - right<br>注意，尽管该属性在公共属性上，但并非所有 Block 的视图都支持该属性，目前支持的 Block 类型包括：<br>- TEXT<br>- HEADING1<br>- HEADING2<br>- HEADING3<br>- HEADING4<br>- HEADING5<br>- HEADING6<br>- HEADING7<br>- HEADING8<br>- HEADING9<br>- BULLET<br>- ORDERED<br>- TODO<br>- QUOTE<br>- IMAGE<br>- CHAT_CARD |

