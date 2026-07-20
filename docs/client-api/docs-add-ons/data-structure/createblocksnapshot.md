---
document_id: '7270779605450293254'
directory_id: '7270719284443660293'
title: CreateBlockSnapshot
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/CreateBlockSnapshot
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- CreateBlockSnapshot
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/CreateBlockSnapshot
---

# CreateBlockSnapshot
创建 Block 时传入的数据对象，跟 [BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot)类似，但只有 `type` 和 `data` 属性。
| **名称** | **数据类型**  | **是否必填** | **描述**                                                                                          |
| ------ | --------- | -------- | ----------------------------------------------------------------------------------------------- |
| type   | BlockType | 是        | Block 的类型，[见【数据对象 - BlockType】](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockType)   |
| data   | BlockData | 是        | Block 的详细数据，[见【数据对象 - BlockData】](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/blockdata)
